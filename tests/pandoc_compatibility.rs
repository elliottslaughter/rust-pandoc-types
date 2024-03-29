//! This test checks that the JSON we produce actually matches what
//! Pandoc itself supports. We do this by running roundtrips through
//! Pandoc and back, making sure that the document before and after is
//! identical and that there are no errors.
//!
//! This requires that Pandoc be installed and on PATH.

use pandoc_types::definition::{Block, Inline, IterBlocks, IterInlines, Pandoc, Stringify};

use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

fn pandoc_convert(input: &str, from: &str, to: &str) -> io::Result<String> {
    let process = Command::new("pandoc")
        .arg("-s")
        .arg("-f")
        .arg(from)
        .arg("-t")
        .arg(to)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    process.stdin.unwrap().write_all(input.as_bytes())?;

    let mut s = String::new();
    process.stdout.unwrap().read_to_string(&mut s).map(|_| s)
}

fn check_roundtrip_stability(md_1: &str) {
    // Do an initial roundtrip to settle whitespace issues.
    let md_2 = pandoc_convert(md_1, "markdown", "markdown").unwrap();

    // Now do a roundtrip through our own parser and back.
    let json_2 = pandoc_convert(&md_2, "markdown", "json").unwrap();
    let doc_2: Pandoc = serde_json::from_str(&json_2).unwrap();
    let json_3 = serde_json::to_string(&doc_2).unwrap();
    let json_4 = pandoc_convert(&json_3, "json", "json").unwrap();
    let doc_4: Pandoc = serde_json::from_str(&json_4).unwrap();
    assert_eq!(doc_2, doc_4);
}

#[test]
fn pandoc_available() {
    pandoc_convert("", "markdown", "json").unwrap();
}

#[test]
fn empty() {
    check_roundtrip_stability("");
}

#[test]
fn title() {
    check_roundtrip_stability(
        r#"% title
% author
% date
"#,
    );
}

#[test]
fn meta() {
    check_roundtrip_stability(
        r#"
---
title: asdf
author: qwer
date: zxcv
foo: bar
fun:
  - 2
  - times
  - 2
---
"#,
    );
}

#[test]
fn para() {
    check_roundtrip_stability(
        r#"
first paragraph

second paragraph
"#,
    );
}

#[test]
fn line_block() {
    check_roundtrip_stability(
        r#"
| line
| block
"#,
    );
}

#[test]
fn code_block() {
    check_roundtrip_stability(
        r#"
```bash
$ echo hi
$ uname -a
```
"#,
    );
}

#[test]
fn raw_block() {
    check_roundtrip_stability(
        r#"
```bash
\begin{enumerate}
\item one
\item two
\end{enumerate}
```
"#,
    );
}

#[test]
fn block_quote() {
    check_roundtrip_stability(
        r#"
> "Don't worry about what anybody else is going to do. The best way to
> predict the future is to invent it."
>
> --- Alan Kay
"#,
    );
}

#[test]
fn lists() {
    check_roundtrip_stability(
        r#"
 1. d
    e
 2. f
      * g

 a. 1
 b. 2
 c. 3

  * a
  * b
  * c

fun

: something you do with friends

"#,
    );
}

#[test]
fn headers() {
    check_roundtrip_stability(
        r#"
# a

## b

### c

#### d

##### e

###### f
"#,
    );
}

#[test]
fn horizontal_rule() {
    check_roundtrip_stability(
        r#"
----
"#,
    );
}

#[test]
fn table() {
    check_roundtrip_stability(
        r#"
  right left    center
------- ------ --------
      1 2         3
      4 5         6
"#,
    );
}

#[test]
fn div() {
    check_roundtrip_stability(
        r#"
<div id="foo" class="bar">

  * 1
  * 2
  * 3

</div>
"#,
    );
}

#[test]
fn inline_roundtrip() {
    check_roundtrip_stability(concat!(
        include_str!("inlines.txt"),
        r#"
[^footnote]

[^footnote]: <span class="asdf">span</span>
"#,
    ));
}

#[test]
fn markdown_reader_more() {
    check_roundtrip_stability(include_str!("markdown-reader-more.txt"));
}

#[test]
fn tables() {
    check_roundtrip_stability(include_str!("tables.txt"));
}

#[test]
fn testsuite() {
    check_roundtrip_stability(include_str!("testsuite.txt"));
}

#[test]
fn stringify() {
    let json = pandoc_convert(include_str!("inlines.txt"), "markdown", "json").unwrap();
    let pandoc: Pandoc = serde_json::from_str(&json).unwrap();
    match &pandoc.blocks[..] {
        [Block::Para(inlines)] => {
            assert_eq!(inlines.stringify(), "str emph underline strong strikeout superscript subscript caps ‘single’ “double” [see @cite] code line break:\nmath  link alt");
        }
        _ => panic!("expected inlines.txt to return only one Para"),
    }
}

fn make_blocks_uppercase<'a>(blocks: impl Iterator<Item = &'a mut Block>) {
    for block in blocks {
        make_blocks_uppercase(block.iter_blocks_mut());
        make_inlines_uppercase(block.iter_inlines_mut());
    }
}

fn make_inlines_uppercase<'a>(inlines: impl Iterator<Item = &'a mut Inline>) {
    for inline in inlines {
        if let Inline::Str(text) = inline {
            *text = text.to_uppercase()
        }
        make_inlines_uppercase(inline.iter_inlines_mut());
        make_blocks_uppercase(inline.iter_blocks_mut());
    }
}

#[test]
fn iter_mut_blocks() {
    let json = pandoc_convert(include_str!("testsuite.txt"), "markdown", "json").unwrap();
    let mut doc: Pandoc = serde_json::from_str(&json).unwrap();
    make_blocks_uppercase(doc.blocks.iter_mut());
    let json = serde_json::to_string(&doc).unwrap();
    let markdown = pandoc_convert(&json, "json", "markdown").unwrap();
    assert_eq!(markdown, include_str!("testsuite_uppercase.txt"));
}

#[test]
fn iter_mut_tables() {
    let json = pandoc_convert(include_str!("tables.txt"), "markdown", "json").unwrap();
    let mut doc: Pandoc = serde_json::from_str(&json).unwrap();
    make_blocks_uppercase(doc.blocks.iter_mut());
    let json = serde_json::to_string(&doc).unwrap();
    let markdown = pandoc_convert(&json, "json", "markdown").unwrap();
    assert_eq!(markdown, include_str!("tables_uppercase.txt"));
}
