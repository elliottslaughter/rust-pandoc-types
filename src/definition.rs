//! This module contatins the types from [Text.Pandoc.Definition] ported to Rust.
//!
//! [Text.Pandoc.Definition]: https://hackage.haskell.org/package/pandoc-types/docs/Text-Pandoc-Definition.html
use std::collections::HashMap;

pub use iter::*;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

pub mod extra;
mod iter;

const PANDOC_API_VERSION: [i32; 2] = [1, 23];

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pandoc {
    pub blocks: Vec<Block>,
    pub meta: HashMap<String, MetaValue>,
}

impl Serialize for Pandoc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut value = serializer.serialize_struct("Pandoc", 3)?;
        value.serialize_field("pandoc-api-version", &PANDOC_API_VERSION)?;
        value.serialize_field("meta", &self.meta)?;
        value.serialize_field("blocks", &self.blocks)?;
        value.end()
    }
}

impl<'a> Deserialize<'a> for Pandoc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        #[serde(rename = "Pandoc")]
        struct Inner {
            meta: HashMap<String, MetaValue>,
            blocks: Vec<Block>,
            #[serde(rename = "pandoc-api-version")]
            version: Vec<i32>,
        }

        let value = Inner::deserialize(deserializer)?;

        if value.version.len() < 2
            || value.version[0] != PANDOC_API_VERSION[0]
            || value.version[1] != PANDOC_API_VERSION[1]
        {
            return Err(serde::de::Error::custom(format!(
                "expected pandoc-api-version to start with {},{}",
                PANDOC_API_VERSION[0], PANDOC_API_VERSION[1]
            )));
        }

        Ok(Pandoc {
            meta: value.meta,
            blocks: value.blocks,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum MetaValue {
    MetaMap(HashMap<String, MetaValue>),
    MetaList(Vec<MetaValue>),
    MetaBool(bool),
    MetaString(String),
    MetaInlines(Vec<Inline>),
    MetaBlocks(Vec<Block>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum Block {
    /// Plain text, not a paragraph
    Plain(Vec<Inline>),
    /// Paragraph
    Para(Vec<Inline>),
    /// Multiple non-breaking lines
    LineBlock(Vec<Vec<Inline>>),
    /// Code block (literal) with attributes
    CodeBlock(Attr, String),
    /// Raw block
    RawBlock(Format, String),
    /// Block quote
    BlockQuote(Vec<Block>),
    /// Ordered list (attributes and a list of items, each a list of blocks)
    OrderedList(ListAttributes, Vec<Vec<Block>>),
    /// Bullet list (list of items, each a list of blocks)
    BulletList(Vec<Vec<Block>>),
    /// Definition list. Each list item is a pair consisting of a term (a list of inlines) and one or more definitions (each a list of blocks)
    DefinitionList(Vec<(Vec<Inline>, Vec<Vec<Block>>)>),
    /// Header - level (integer) and text (inlines)
    Header(i32, Attr, Vec<Inline>),
    /// Horizontal rule
    HorizontalRule,
    /// Table
    Table(Table),
    /// Figure
    Figure(Attr, Caption, Vec<Block>),
    /// Generic block container with attributes
    Div(Attr, Vec<Block>),
    /// Nothing
    Null,
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone, PartialEq, Default)]
pub struct Table {
    pub attr: Attr,
    pub caption: Caption,
    pub colspecs: Vec<ColSpec>,
    pub head: TableHead,
    pub bodies: Vec<TableBody>,
    pub foot: TableFoot,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum Inline {
    /// Text
    Str(String),
    /// Emphasized text
    Emph(Vec<Inline>),
    /// Underlined text
    Underline(Vec<Inline>),
    /// Strongly emphasized text
    Strong(Vec<Inline>),
    /// Strikeout text
    Strikeout(Vec<Inline>),
    /// Superscripted text
    Superscript(Vec<Inline>),
    /// Subscripted text
    Subscript(Vec<Inline>),
    /// Small caps text
    SmallCaps(Vec<Inline>),
    /// Quoted text
    Quoted(QuoteType, Vec<Inline>),
    /// Citation
    Cite(Vec<Citation>, Vec<Inline>),
    /// Inline code
    Code(Attr, String),
    /// Inter-word space
    Space,
    /// Soft line break
    SoftBreak,
    /// Hard line break
    LineBreak,
    /// TeX math
    Math(MathType, String),
    /// Raw inline
    RawInline(Format, String),
    /// Hyperlink: alt text (list of inlines), target
    Link(Attr, Vec<Inline>, Target),
    /// Image: alt text (list of inlines), target
    Image(Attr, Vec<Inline>, Target),
    /// Footnote or endnote
    Note(Vec<Block>),
    /// Generic inline container with attributes
    Span(Attr, Vec<Inline>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum Alignment {
    AlignLeft,
    AlignRight,
    AlignCenter,
    AlignDefault,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::AlignDefault
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum ColWidth {
    ColWidth(f64),
    ColWidthDefault,
}

impl Default for ColWidth {
    fn default() -> Self {
        Self::ColWidthDefault
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ColSpec(pub Alignment, pub ColWidth);

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone, PartialEq)]
pub struct Row {
    pub attr: Attr,
    pub cells: Vec<Cell>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone, PartialEq, Default)]
pub struct TableHead {
    pub attr: Attr,
    pub rows: Vec<Row>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone, PartialEq, Default)]
pub struct TableBody {
    pub attr: Attr,
    pub row_head_columns: i32,
    pub head: Vec<Row>,
    pub body: Vec<Row>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone, PartialEq, Default)]
pub struct TableFoot {
    pub attr: Attr,
    pub rows: Vec<Row>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone, PartialEq, Default)]
pub struct Caption {
    pub short: Option<Vec<Inline>>,
    pub long: Vec<Block>,
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone, PartialEq)]
pub struct Cell {
    pub attr: Attr,
    pub align: Alignment,
    pub row_span: i32,
    pub col_span: i32,
    pub content: Vec<Block>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            attr: Default::default(),
            align: Default::default(),
            row_span: 1,
            col_span: 1,
            content: Default::default(),
        }
    }
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone, PartialEq)]
pub struct ListAttributes {
    pub start_number: i32,
    pub style: ListNumberStyle,
    pub delim: ListNumberDelim,
}

impl Default for ListAttributes {
    fn default() -> Self {
        Self {
            start_number: 1,
            style: ListNumberStyle::default(),
            delim: ListNumberDelim::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum ListNumberStyle {
    DefaultStyle,
    Example,
    Decimal,
    LowerRoman,
    UpperRoman,
    LowerAlpha,
    UpperAlpha,
}

impl Default for ListNumberStyle {
    fn default() -> Self {
        Self::DefaultStyle
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum ListNumberDelim {
    DefaultDelim,
    Period,
    OneParen,
    TwoParens,
}

impl Default for ListNumberDelim {
    fn default() -> Self {
        Self::DefaultDelim
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Format(pub String);

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone, PartialEq, Default)]
pub struct Attr {
    pub identifier: String,
    pub classes: Vec<String>,
    pub attributes: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum QuoteType {
    SingleQuote,
    DoubleQuote,
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug, Clone, PartialEq)]
pub struct Target {
    pub url: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum MathType {
    DisplayMath,
    InlineMath,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Citation {
    pub citation_id: String,
    pub citation_prefix: Vec<Inline>,
    pub citation_suffix: Vec<Inline>,
    pub citation_mode: CitationMode,
    pub citation_note_num: i32,
    pub citation_hash: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum CitationMode {
    AuthorInText,
    SuppressAuthor,
    NormalCitation,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn version() {
        assert!(serde_json::from_value::<Pandoc>(json!({
            "pandoc-api-version": PANDOC_API_VERSION,
            "meta": {},
            "blocks": [],
        }))
        .is_ok());

        assert!(serde_json::from_value::<Pandoc>(json!({
            "pandoc-api-version": [],
            "meta": {},
            "blocks": [],
        }))
        .is_err());

        assert!(serde_json::from_value::<Pandoc>(json!({
            "pandoc-api-version": [PANDOC_API_VERSION[0], PANDOC_API_VERSION[1] + 1],
            "meta": {},
            "blocks": [],
        }))
        .is_err());
    }
}
