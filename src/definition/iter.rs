use super::{Block, Caption, Format, Inline, Pandoc, Row, Table};

/// A trait to iterate over the immediately contained blocks in a type.
pub trait IterBlocks<'a> {
    type Iter: Iterator<Item = &'a Block>;
    type IterMut: Iterator<Item = &'a mut Block>;

    /// Returns an iterator over the immediately contained blocks.
    fn iter_blocks(&'a self) -> Self::Iter;

    /// Returns an iterator over the immediately contained blocks, allowing each block to be modified.
    fn iter_blocks_mut(&'a mut self) -> Self::IterMut;
}

/// A trait to iterate over the immediately contained inlines in a type.
pub trait IterInlines<'a> {
    type Iter: Iterator<Item = &'a Inline>;
    type IterMut: Iterator<Item = &'a mut Inline>;

    /// Returns an iterator over the immediately contained inlines.
    fn iter_inlines(&'a self) -> Self::Iter;

    /// Returns an iterator over the immediately contained inlines, allowing each inline to be modified.
    fn iter_inlines_mut(&'a mut self) -> Self::IterMut;
}

impl<'a> IterBlocks<'a> for Block {
    type Iter = Box<dyn Iterator<Item = &'a Block> + 'a>;
    type IterMut = Box<dyn Iterator<Item = &'a mut Block> + 'a>;

    fn iter_blocks(&'a self) -> Self::Iter {
        Box::new(match self {
            Block::BlockQuote(blocks) => IterTypes::Iter(blocks.iter()),
            Block::Figure(_, _, blocks) => IterTypes::Iter(blocks.iter()),
            Block::Div(_, blocks) => IterTypes::Iter(blocks.iter()),
            Block::BulletList(items) => IterTypes::FlattenIter(items.iter().flatten()),
            Block::OrderedList(_, items) => IterTypes::FlattenIter(items.iter().flatten()),
            Block::DefinitionList(definitions) => IterTypes::FlatMap(
                definitions
                    .iter()
                    .map(|(_dt, dd)| dd.iter().flatten())
                    .flatten(),
            ),
            Block::Table(table) => IterTypes::Table(table.iter_blocks()),
            Block::Plain(_) => IterTypes::Empty,
            Block::Para(_) => IterTypes::Empty,
            Block::LineBlock(_) => IterTypes::Empty,
            Block::CodeBlock(_, _) => IterTypes::Empty,
            Block::RawBlock(_, _) => IterTypes::Empty,
            Block::Header(_, _, _) => IterTypes::Empty,
            Block::HorizontalRule => IterTypes::Empty,
            Block::Null => IterTypes::Empty,
        })
    }

    fn iter_blocks_mut(&'a mut self) -> Self::IterMut {
        Box::new(match self {
            Block::BlockQuote(blocks) => IterTypes::Iter(blocks.iter_mut()),
            Block::Figure(_, _, blocks) => IterTypes::Iter(blocks.iter_mut()),
            Block::Div(_, blocks) => IterTypes::Iter(blocks.iter_mut()),
            Block::BulletList(items) => IterTypes::FlattenIter(items.iter_mut().flatten()),
            Block::OrderedList(_, items) => IterTypes::FlattenIter(items.iter_mut().flatten()),
            Block::DefinitionList(definitions) => IterTypes::FlatMap(
                definitions
                    .iter_mut()
                    .map(|(_dt, dd)| dd.iter_mut().flatten())
                    .flatten(),
            ),
            Block::Table(table) => IterTypes::Table(table.iter_blocks_mut()),
            Block::Plain(_) => IterTypes::Empty,
            Block::Para(_) => IterTypes::Empty,
            Block::LineBlock(_) => IterTypes::Empty,
            Block::CodeBlock(_, _) => IterTypes::Empty,
            Block::RawBlock(_, _) => IterTypes::Empty,
            Block::Header(_, _, _) => IterTypes::Empty,
            Block::HorizontalRule => IterTypes::Empty,
            Block::Null => IterTypes::Empty,
        })
    }
}

impl<'a> IterBlocks<'a> for Table {
    type Iter = Box<dyn Iterator<Item = &'a Block> + 'a>;
    type IterMut = Box<dyn Iterator<Item = &'a mut Block> + 'a>;

    fn iter_blocks(&'a self) -> Self::Iter {
        Box::new(
            self.caption.long.iter().chain(
                self.head
                    .rows
                    .iter()
                    .chain(
                        self.bodies
                            .iter()
                            .flat_map(|b| b.head.iter().chain(b.body.iter()))
                            .chain(self.foot.rows.iter()),
                    )
                    .flat_map(|Row { cells, .. }| {
                        cells.iter().flat_map(|cell| cell.content.iter())
                    }),
            ),
        )
    }

    fn iter_blocks_mut(&'a mut self) -> Self::IterMut {
        Box::new(
            self.caption.long.iter_mut().chain(
                self.head
                    .rows
                    .iter_mut()
                    .chain(
                        self.bodies
                            .iter_mut()
                            .flat_map(|b| b.head.iter_mut().chain(b.body.iter_mut()))
                            .chain(self.foot.rows.iter_mut()),
                    )
                    .flat_map(|Row { cells, .. }| {
                        cells.iter_mut().flat_map(|cell| cell.content.iter_mut())
                    }),
            ),
        )
    }
}

/// Currently only yields blocks for `Inline::Note`.
impl<'a> IterBlocks<'a> for Inline {
    type Iter = std::slice::Iter<'a, Block>;
    type IterMut = std::slice::IterMut<'a, Block>;

    fn iter_blocks(&'a self) -> Self::Iter {
        match self {
            Inline::Note(blocks) => blocks.iter(),
            _ => [].iter(),
        }
    }

    fn iter_blocks_mut(&'a mut self) -> Self::IterMut {
        match self {
            Inline::Note(blocks) => blocks.iter_mut(),
            _ => [].iter_mut(),
        }
    }
}

impl<'a> IterBlocks<'a> for Pandoc {
    type Iter = std::slice::Iter<'a, Block>;
    type IterMut = std::slice::IterMut<'a, Block>;

    fn iter_blocks(&'a self) -> Self::Iter {
        self.blocks.iter()
    }

    fn iter_blocks_mut(&'a mut self) -> Self::IterMut {
        self.blocks.iter_mut()
    }
}

impl<'a> IterInlines<'a> for Block {
    type Iter = Box<dyn Iterator<Item = &'a Inline> + 'a>;
    type IterMut = Box<dyn Iterator<Item = &'a mut Inline> + 'a>;

    fn iter_inlines(&'a self) -> Self::Iter {
        Box::new(match self {
            Block::Plain(inlines) => IterTypes::Iter(inlines.iter()),
            Block::Para(inlines) => IterTypes::Iter(inlines.iter()),
            Block::LineBlock(lines) => IterTypes::FlattenIter(lines.iter().flatten()),
            Block::DefinitionList(definitions) => {
                IterTypes::FlatMap(definitions.iter().flat_map(|(dt, _)| dt))
            }
            Block::Header(_, _, inlines) => IterTypes::Iter(inlines.iter()),
            Block::Table(Table {
                caption: Caption { short, .. },
                ..
            }) => IterTypes::Table(short.iter().flatten()),
            Block::CodeBlock(_, _) => IterTypes::Empty,
            Block::RawBlock(_, _) => IterTypes::Empty,
            Block::BlockQuote(_) => IterTypes::Empty,
            Block::OrderedList(_, _) => IterTypes::Empty,
            Block::BulletList(_) => IterTypes::Empty,
            Block::HorizontalRule => IterTypes::Empty,
            Block::Figure(_, _, _) => IterTypes::Empty,
            Block::Div(_, _) => IterTypes::Empty,
            Block::Null => IterTypes::Empty,
        })
    }

    fn iter_inlines_mut(&'a mut self) -> Self::IterMut {
        Box::new(match self {
            Block::Plain(inlines) => IterTypes::Iter(inlines.iter_mut()),
            Block::Para(inlines) => IterTypes::Iter(inlines.iter_mut()),
            Block::LineBlock(lines) => IterTypes::FlattenIter(lines.iter_mut().flatten()),
            Block::DefinitionList(definitions) => {
                IterTypes::FlatMap(definitions.iter_mut().flat_map(|(dt, _)| dt))
            }
            Block::Header(_, _, inlines) => IterTypes::Iter(inlines.iter_mut()),
            Block::Table(Table {
                caption: Caption { short, .. },
                ..
            }) => IterTypes::Table(short.iter_mut().flatten()),
            Block::CodeBlock(_, _) => IterTypes::Empty,
            Block::RawBlock(_, _) => IterTypes::Empty,
            Block::BlockQuote(_) => IterTypes::Empty,
            Block::OrderedList(_, _) => IterTypes::Empty,
            Block::BulletList(_) => IterTypes::Empty,
            Block::HorizontalRule => IterTypes::Empty,
            Block::Figure(_, _, _) => IterTypes::Empty,
            Block::Div(_, _) => IterTypes::Empty,
            Block::Null => IterTypes::Empty,
        })
    }
}

impl<'a> IterInlines<'a> for Inline {
    type Iter = std::slice::Iter<'a, Inline>;
    type IterMut = std::slice::IterMut<'a, Inline>;

    fn iter_inlines(&'a self) -> Self::Iter {
        match self {
            Inline::Emph(inlines) => inlines.iter(),
            Inline::Underline(inlines) => inlines.iter(),
            Inline::Strong(inlines) => inlines.iter(),
            Inline::Strikeout(inlines) => inlines.iter(),
            Inline::Superscript(inlines) => inlines.iter(),
            Inline::Subscript(inlines) => inlines.iter(),
            Inline::SmallCaps(inlines) => inlines.iter(),
            Inline::Quoted(_, inlines) => inlines.iter(),
            Inline::Cite(_, inlines) => inlines.iter(),
            Inline::Link(_, inlines, _) => inlines.iter(),
            Inline::Image(_, inlines, _) => inlines.iter(),
            Inline::Span(_, inlines) => inlines.iter(),
            Inline::Str(_) => [].iter(),
            Inline::Code(_, _) => [].iter(),
            Inline::Space => [].iter(),
            Inline::SoftBreak => [].iter(),
            Inline::LineBreak => [].iter(),
            Inline::Math(_, _) => [].iter(),
            Inline::RawInline(_, _) => [].iter(),
            Inline::Note(_) => [].iter(),
        }
    }

    fn iter_inlines_mut(&'a mut self) -> Self::IterMut {
        match self {
            Inline::Emph(inlines) => inlines.iter_mut(),
            Inline::Underline(inlines) => inlines.iter_mut(),
            Inline::Strong(inlines) => inlines.iter_mut(),
            Inline::Strikeout(inlines) => inlines.iter_mut(),
            Inline::Superscript(inlines) => inlines.iter_mut(),
            Inline::Subscript(inlines) => inlines.iter_mut(),
            Inline::SmallCaps(inlines) => inlines.iter_mut(),
            Inline::Quoted(_, inlines) => inlines.iter_mut(),
            Inline::Cite(_, inlines) => inlines.iter_mut(),
            Inline::Link(_, inlines, _) => inlines.iter_mut(),
            Inline::Image(_, inlines, _) => inlines.iter_mut(),
            Inline::Span(_, inlines) => inlines.iter_mut(),
            Inline::Str(_) => [].iter_mut(),
            Inline::Code(_, _) => [].iter_mut(),
            Inline::Space => [].iter_mut(),
            Inline::SoftBreak => [].iter_mut(),
            Inline::LineBreak => [].iter_mut(),
            Inline::Math(_, _) => [].iter_mut(),
            Inline::RawInline(_, _) => [].iter_mut(),
            Inline::Note(_) => [].iter_mut(),
        }
    }
}

enum IterTypes<A, B, C, D> {
    Empty,
    Iter(A),
    FlattenIter(B),
    FlatMap(C),
    Table(D),
}

impl<A, B, C, D> Iterator for IterTypes<A, B, C, D>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
    C: Iterator<Item = A::Item>,
    D: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            IterTypes::Empty => None,
            IterTypes::Iter(iter) => iter.next(),
            IterTypes::FlattenIter(iter) => iter.next(),
            IterTypes::FlatMap(iter) => iter.next(),
            IterTypes::Table(iter) => iter.next(),
        }
    }
}

/// Converts the given element into a string with all formatting removed.
pub trait Stringify {
    /// Converts the given element into a string with all formatting removed.
    fn stringify(&self) -> String {
        let mut s = String::new();
        self.stringify_to(&mut s);
        s
    }

    /// Appends the stringified element to the given string.
    fn stringify_to(&self, str: &mut String);
}

impl Stringify for Inline {
    fn stringify_to(&self, str: &mut String) {
        // Should match the implementation of pandoc's Haskell API
        // https://hackage.haskell.org/package/pandoc/docs/src/Text.Pandoc.Shared.html#stringify
        match self {
            Inline::Space => str.push(' '),
            Inline::SoftBreak => str.push(' '),
            Inline::Str(x) => str.push_str(x),
            Inline::Code(_, x) => str.push_str(x),
            Inline::Math(_, x) => str.push_str(x),
            Inline::RawInline(Format(format), raw)
                if format == "html" && raw.starts_with("<br") =>
            {
                str.push(' ')
            }
            Inline::LineBreak => str.push('\n'),
            Inline::Quoted(super::QuoteType::SingleQuote, inlines) => {
                str.push('\u{2018}');
                for inline in inlines {
                    inline.stringify_to(str);
                }
                str.push('\u{2019}');
            }
            Inline::Quoted(super::QuoteType::DoubleQuote, inlines) => {
                str.push('\u{201C}');
                for inline in inlines {
                    inline.stringify_to(str);
                }
                str.push('\u{201D}');
            }
            other => {
                for inline in other.iter_inlines() {
                    inline.stringify_to(str);
                }
            }
        }
    }
}

impl<T> Stringify for T
where
    for<'a> &'a T: IntoIterator<Item = &'a Inline>,
{
    fn stringify_to(&self, str: &mut String) {
        for inline in self.into_iter() {
            inline.stringify_to(str);
        }
    }
}
