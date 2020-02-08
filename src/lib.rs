use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Pandoc {
    pub meta: HashMap<String, MetaValue>,
    pub blocks: Vec<Block>,
    #[serde(rename = "pandoc-api-version", default)]
    pub api_version: Option<Vec<i32>>,
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
    #[serde(rename = "citationId")]
    pub citation_id: String,
    #[serde(rename = "citationPrefix")]
    pub citation_prefix: Vec<Inline>,
    #[serde(rename = "citationSuffix")]
    pub citation_suffix: Vec<Inline>,
    #[serde(rename = "citationMode")]
    pub citation_mode: CitationMode,
    #[serde(rename = "citationNoteNum")]
    pub citation_note_num: i32,
    #[serde(rename = "citationHash")]
    pub citation_hash: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum CitationMode {
    AuthorInText,
    SuppressAuthor,
    NormalCitation,
}
