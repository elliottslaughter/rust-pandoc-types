use std::collections::HashMap;

const PANDOC_API_VERSION: &'static [i32] = &[1, 17, 0, 5];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Pandoc {
    pub meta: Meta,
    pub blocks: Vec<Block>,
    #[serde(rename = "pandoc-api-version")]
    version: Vec<i32>,
}

impl Pandoc {
    pub fn new(meta: Meta, blocks: Vec<Block>) -> Pandoc {
        Pandoc {
            meta: meta,
            blocks: blocks,
            version: PANDOC_API_VERSION.to_owned()
        }
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
#[serde(tag = "t")]
pub enum MetaValue {
    MetaMap { c: HashMap<String, MetaValue> },
    MetaList { c: Vec<MetaValue> },
    MetaBool { c: bool },
    MetaString { c: String },
    MetaInlines { c: Vec<Inline> },
    MetaBlocks { c: Vec<Block> },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t")]
pub enum Block {
    Plain { c: Vec<Inline> },
    Para { c: Vec<Inline> },
    LineBlock { c: Vec<Vec<Inline>> },
    CodeBlock { c: (Attr, String) },
    RawBlock { c: (Format, String) },
    BlockQuote { c: Vec<Block> },
    OrderedList { c: (ListAttributes, Vec<Vec<Block>>) },
    BulletList { c: Vec<Vec<Block>> },
    DefinitionList { c: Vec<(Vec<Inline>, Vec<Vec<Block>>)> },
    Header { c: (i32, Attr, Vec<Inline>) },
    HorizontalRule,
    Table { c: (Vec<Inline>, Vec<Alignment>, Vec<f64>, Vec<TableCell>, Vec<Vec<TableCell>>) },
    Div { c: (Attr, Vec<Block>) },
    Null,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t")]
pub enum Inline {
    Str { c: String },
    Emph { c: Vec<Inline> },
    Strong { c: Vec<Inline> },
    Strikeout { c: Vec<Inline> },
    Superscript { c: Vec<Inline> },
    Subscript { c: Vec<Inline> },
    SmallCaps { c: Vec<Inline> },
    Quoted { c: (QuoteType, Vec<Inline>) },
    Cite { c: (Vec<Citation>, Vec<Inline>) },
    Space,
    SoftBreak,
    LineBreak,
    Math { c: (MathType, String) },
    RawInline { c: (Format, String) },
    Link { c: (Attr, Vec<Inline>, Target) },
    Image { c: (Attr, Vec<Inline>, Target) },
    Note { c: Vec<Block> },
    Span { c: (Attr, Vec<Inline>) },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t")]
pub enum Alignment {
    AlignLeft,
    AlignRight,
    AlignCenter,
    AlignDefault,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListAttributes(pub i32, pub ListNumberStyle, pub ListNumberDelim);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "t")]
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
#[serde(tag = "t")]
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
pub enum QuoteType {
    SingleQuote,
    DoubleQuote,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Target(pub String, pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MathType {
    DisplayMath,
    InlineMath,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Citation {
    pub citation_id: String,
    pub citation_prefix: Vec<Inline>,
    pub citation_suffix: Vec<Inline>,
    pub citation_mode: CitationMode,
    pub citation_note_num: i32,
    pub citation_hash: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
