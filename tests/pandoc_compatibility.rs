//! This test checks that the JSON we produce actually matches what
//! Pandoc itself supports. We do this by running roundtrips through
//! Pandoc and back, making sure that the document before and after is
//! identical and that there are no errors.
//!
//! This requires that Pandoc be installed and on PATH.

use pandoc_types::definition::Pandoc;
use serde_json;

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
fn inline() {
    check_roundtrip_stability(
        r#"
str
*emph*
**strong**
~~strikeout~~
^superscript^
~subscript~
<span style="font-variant:small-caps;">caps</span>
'single'
"double"
[see @cite]
`code`
line break: \
$math$
\rawlatex{something}
[link](http://pandoc.org)
![](image.png)
[^footnote]

[^footnote]: <span class="asdf">span</span>
"#,
    );
}
