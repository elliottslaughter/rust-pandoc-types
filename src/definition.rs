use std::collections::HashMap;

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::ser::SerializeStruct;

const PANDOC_API_VERSION: &'static [i32] = &[1, 17, 0, 5];

// For the basic Pandoc struct, it's possible to use attributes to get
// it to serialize properly. The commented code below shows how to do
// this. The reason not to want to go this way is:
//
//  1. We're forced to add a hidden field to the struct to encode the
//     version.
//  2. This doesn't generalize to other types, because the
//     transformations that need to be applied are more complicated.

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub struct Pandoc {
//     #[serde(rename = "pandoc-api-version")]
//     version: Version,
//     pub meta: Meta,
//     pub blocks: Vec<Block>,
// }

// impl Pandoc {
//     pub fn new(meta: Meta, blocks: Vec<Block>) -> Self {
//         Pandoc {
//             version: Version(Vec::from(PANDOC_API_VERSION)),
//             meta: meta,
//             blocks: blocks,
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Pandoc(pub Meta, pub Vec<Block>);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct SerializedPandoc {
    meta: Meta,
    blocks: Vec<Block>,
    #[serde(rename = "pandoc-api-version")]
    version: Vec<i32>,
}

impl Serialize for Pandoc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = SerializedPandoc {
            meta: self.0.clone(),
            blocks: self.1.clone(),
            version: Vec::from(PANDOC_API_VERSION),
        };

        value.serialize(serializer)

        // FIXME: The alternative is to implement serialize properly
        // here. This is alright for the serializer but gets really
        // painful for the deserializer.

        // let mut value = serializer.serialize_struct("Pandoc", 3)?;
        // value.serialize_field("pandoc-api-version", PANDOC_API_VERSION)?;
        // value.serialize_field("meta", &self.0)?;
        // value.serialize_field("blocks", &self.1)?;
        // value.end()
    }
}

impl Deserialize for Pandoc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer {
        let value = SerializedPandoc::deserialize(deserializer)?;
        // FIXME: Should check this, but need a better error.
        // assert!(value.version == PANDOC_API_VERSION);
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
pub enum MetaValue {
    MetaMap(HashMap<String, MetaValue>),
    MetaList(Vec<MetaValue>),
    MetaBool(bool),
    MetaString(String),
    MetaInlines(Vec<Inline>),
    MetaBlocks(Vec<Block>),
}

// FIXME: This approach works (though ugly) for serialization, but
// doesn't work at all for deserialization. Looks like it's going to
// be manual deserializers for us.

// #[derive(Debug, Clone, PartialEq)]
// pub enum MetaValue {
//     MetaMap(HashMap<String, MetaValue>),
//     MetaList(Vec<MetaValue>),
//     MetaBool(bool),
//     MetaString(String),
//     MetaInlines(Vec<Inline>),
//     MetaBlocks(Vec<Block>),
// }

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// struct SerializedContent<T> where T: Serialize, T: Deserialize {
//     t: String,
//     c: T,
// }

// impl Serialize for MetaValue {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
//         match *self {
//             MetaValue::MetaMap(ref _0) =>
//                 SerializedContent { t: String::from("MetaMap"), c: (_0.clone(),) }.serialize(serializer),
//             MetaValue::MetaList(ref _0) =>
//                 SerializedContent { t: String::from("MetaList"), c: (_0.clone(),) }.serialize(serializer),
//             MetaValue::MetaBool(ref _0) =>
//                 SerializedContent { t: String::from("MetaBool"), c: (_0.clone(),) }.serialize(serializer),
//             MetaValue::MetaString(ref _0) =>
//                 SerializedContent { t: String::from("MetaString"), c: (_0.clone(),) }.serialize(serializer),
//             MetaValue::MetaInlines(ref _0) =>
//                 SerializedContent { t: String::from("MetaInlines"), c: (_0.clone(),) }.serialize(serializer),
//             MetaValue::MetaBlocks(ref _0) =>
//                 SerializedContent { t: String::from("MetaBlocks"), c: (_0.clone(),) }.serialize(serializer),
//         }
//     }
// }

// impl Deserialize for MetaValue {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer {
//         let value: SerializedContent<HashMap<String, MetaValue>> = SerializedContent::deserialize(deserializer)?;
//         let result = match value.t.as_str() {
//             "MetaMap" => MetaValue::MetaMap(value.c),
//             // FIXME: Problem: We can't deserialize the content eagerly because we don't know what it is.
//             _ => panic!(),
//         };
//         Ok(result)
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    Table(Vec<Inline>, Vec<Alignment>, Vec<f64>, Vec<TableCell>, Vec<Vec<TableCell>>),
    Div(Attr, Vec<Block>),
    Null,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
pub enum Alignment {
    AlignLeft,
    AlignRight,
    AlignCenter,
    AlignDefault,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListAttributes(pub i32, pub ListNumberStyle, pub ListNumberDelim);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
