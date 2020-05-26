use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

const PANDOC_API_VERSION: &'static [i32] = &[1, 20];

#[derive(Debug, Clone, PartialEq)]
pub struct Pandoc(pub Meta, pub Vec<Block>);

impl Serialize for Pandoc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut value = serializer.serialize_struct("Pandoc", 3)?;
        value.serialize_field("pandoc-api-version", PANDOC_API_VERSION)?;
        value.serialize_field("meta", &self.0)?;
        value.serialize_field("blocks", &self.1)?;
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
            meta: Meta,
            blocks: Vec<Block>,
            #[serde(rename = "pandoc-api-version")] version: Vec<i32>,
        }

        let value = Inner::deserialize(deserializer)?;
        // FIXME: Should check this, but need a better error.
        assert!(
            value.version[0] == PANDOC_API_VERSION[0] && value.version[1] == PANDOC_API_VERSION[1]
        );
        Ok(Pandoc(value.meta, value.blocks))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Meta(pub HashMap<String, MetaValue>);

impl Meta {
    pub fn null() -> Meta {
        Meta(HashMap::new())
    }

    pub fn is_null(&self) -> bool {
        self.0.is_empty()
    }

    pub fn lookup(&self, key: &String) -> Option<&MetaValue> {
        self.0.get(key)
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
    Plain(Vec<Inline>),
    Para(Vec<Inline>),
    LineBlock(Vec<Vec<Inline>>),
    CodeBlock(Attr, String),
    RawBlock(Format, String),
    BlockQuote(Vec<Block>),
    OrderedList(ListAttributes, Vec<Vec<Block>>),
    BulletList(Vec<Vec<Block>>),
    DefinitionList(Vec<(Vec<Inline>, Vec<Vec<Block>>)>),
    Header(i32, Attr, Vec<Inline>),
    HorizontalRule,
    Table(
        Vec<Inline>,
        Vec<Alignment>,
        Vec<f64>,
        Vec<TableCell>,
        Vec<Vec<TableCell>>,
    ),
    Div(Attr, Vec<Block>),
    Null,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum Inline {
    Str(String),
    Emph(Vec<Inline>),
    Strong(Vec<Inline>),
    Strikeout(Vec<Inline>),
    Superscript(Vec<Inline>),
    Subscript(Vec<Inline>),
    SmallCaps(Vec<Inline>),
    Quoted(QuoteType, Vec<Inline>),
    Cite(Vec<Citation>, Vec<Inline>),
    Code(Attr, String),
    Space,
    SoftBreak,
    LineBreak,
    Math(MathType, String),
    RawInline(Format, String),
    Link(Attr, Vec<Inline>, Target),
    Image(Attr, Vec<Inline>, Target),
    Note(Vec<Block>),
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListAttributes(pub i32, pub ListNumberStyle, pub ListNumberDelim);

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum ListNumberDelim {
    DefaultDelim,
    Period,
    OneParen,
    TwoParens,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Format(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Attr(pub String, pub Vec<String>, pub Vec<(String, String)>);

impl Attr {
    pub fn null() -> Attr {
        Attr(String::new(), Vec::new(), Vec::new())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TableCell(pub Vec<Block>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum QuoteType {
    SingleQuote,
    DoubleQuote,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Target(pub String, pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum MathType {
    DisplayMath,
    InlineMath,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Citation {
    #[serde(rename = "citationId")] pub citation_id: String,
    #[serde(rename = "citationPrefix")] pub citation_prefix: Vec<Inline>,
    #[serde(rename = "citationSuffix")] pub citation_suffix: Vec<Inline>,
    #[serde(rename = "citationMode")] pub citation_mode: CitationMode,
    #[serde(rename = "citationNoteNum")] pub citation_note_num: i32,
    #[serde(rename = "citationHash")] pub citation_hash: i32,
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

    #[test]
    fn meta_null() {
        assert!(Meta::null().is_null());
    }
}
