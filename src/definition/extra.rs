//! Additional types that aren't present in the Haskell package but are useful.
use super::{Block, Inline};

/// A utility type to provide better error messages when
/// an array of blocks doesn't match an expected pattern.
///
/// For example:
///
/// ```
/// use pandoc_types::definition::{extra::BlockType, Attr, Block, Inline};
///
/// fn parse_as_para(blocks: &[Block]) -> Result<&Vec<Inline>, String> {
///     match blocks {
///         [Block::Para(inlines)] => Ok(inlines),
///         unexpected => {
///             return Err(format!(
///                 "expected [Para] but found {:?}",
///                 unexpected.iter().map(BlockType::from).collect::<Vec<_>>(),
///             ))
///         }
///     }
/// }
///
/// assert_eq!(
///     parse_as_para(&[Block::CodeBlock(
///         Attr::default(),
///         "some very long string".into()
///     )]),
///     Err("expected [Para] but found [CodeBlock]".into())
/// );
/// ```
///
/// Note that if we didn't use `BlockType` the error message would include
/// the contained data of the CodeBlock which isn't actually relevant.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BlockType {
    Plain,
    Para,
    LineBlock,
    CodeBlock,
    RawBlock,
    BlockQuote,
    OrderedList,
    BulletList,
    DefinitionList,
    Header,
    HorizontalRule,
    Table,
    Figure,
    Div,
    Null,
}

impl From<&Block> for BlockType {
    fn from(block: &Block) -> Self {
        match block {
            Block::Plain(_) => Self::Plain,
            Block::Para(_) => Self::Para,
            Block::LineBlock(_) => Self::LineBlock,
            Block::CodeBlock(_, _) => Self::CodeBlock,
            Block::RawBlock(_, _) => Self::RawBlock,
            Block::BlockQuote(_) => Self::BlockQuote,
            Block::OrderedList(_, _) => Self::OrderedList,
            Block::BulletList(_) => Self::BulletList,
            Block::DefinitionList(_) => Self::DefinitionList,
            Block::Header(_, _, _) => Self::Header,
            Block::HorizontalRule => Self::HorizontalRule,
            Block::Table(_) => Self::Table,
            Block::Figure(_, _, _) => Self::Figure,
            Block::Div(_, _) => Self::Div,
            Block::Null => Self::Null,
        }
    }
}

/// A utility type to provide better error messages when
/// an array of inlines doesn't match an expected pattern.
///
/// For example:
///
/// ```
/// use pandoc_types::definition::{extra::InlineType, Attr, Block, Inline, Target};
///
/// fn parse_as_link(inlines: &[Inline]) -> Result<(&Vec<Inline>, &Target), String> {
///     match inlines {
///         [Inline::Link(_, label, target)] => Ok((label, target)),
///         unexpected => {
///             return Err(format!(
///                 "expected [Link] but found {:?}",
///                 unexpected.iter().map(InlineType::from).collect::<Vec<_>>(),
///             ))
///         }
///     }
/// }
///
/// assert_eq!(
///     parse_as_link(&[Inline::Underline(vec![Inline::Str(
///         "some very long string".into()
///     )],)]),
///     Err("expected [Link] but found [Underline]".into())
/// );
/// ```
///
/// Note that if we didn't use `InlineType` the error message would include
/// the contained text of the Underline which isn't actually relevant.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum InlineType {
    Str,
    Emph,
    Underline,
    Strong,
    Strikeout,
    Superscript,
    Subscript,
    SmallCaps,
    Quoted,
    Cite,
    Code,
    Space,
    SoftBreak,
    LineBreak,
    Math,
    RawInline,
    Link,
    Image,
    Note,
    Span,
}

impl From<&Inline> for InlineType {
    fn from(inline: &Inline) -> Self {
        match inline {
            Inline::Str(_) => Self::Str,
            Inline::Emph(_) => Self::Emph,
            Inline::Underline(_) => Self::Underline,
            Inline::Strong(_) => Self::Strong,
            Inline::Strikeout(_) => Self::Strikeout,
            Inline::Superscript(_) => Self::Superscript,
            Inline::Subscript(_) => Self::Subscript,
            Inline::SmallCaps(_) => Self::SmallCaps,
            Inline::Quoted(_, _) => Self::Quoted,
            Inline::Cite(_, _) => Self::Cite,
            Inline::Code(_, _) => Self::Code,
            Inline::Space => Self::Space,
            Inline::SoftBreak => Self::SoftBreak,
            Inline::LineBreak => Self::LineBreak,
            Inline::Math(_, _) => Self::Math,
            Inline::RawInline(_, _) => Self::RawInline,
            Inline::Link(_, _, _) => Self::Link,
            Inline::Image(_, _, _) => Self::Image,
            Inline::Note(_) => Self::Note,
            Inline::Span(_, _) => Self::Span,
        }
    }
}
