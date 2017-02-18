extern crate serde_json;
extern crate pandoc_types;

use pandoc_types::definition::{Pandoc};

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
    let json_1 = pandoc_convert(md_1, "markdown", "json").unwrap();
    let md_2 = pandoc_convert(&json_1, "json", "markdown").unwrap();

    // Now do a roundtrip through our own parser and back.
    let json_2 = pandoc_convert(&md_2, "markdown", "json").unwrap();
    let doc_2 : Pandoc = serde_json::from_str(&json_2).unwrap();
    let json_3 = serde_json::to_string(&doc_2).unwrap();
    let md_3 = pandoc_convert(&json_3, "json", "markdown").unwrap();
    let json_4 = pandoc_convert(&md_3, "markdown", "json").unwrap();
    let doc_4 : Pandoc = serde_json::from_str(&json_4).unwrap();
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
    check_roundtrip_stability(r#"% title
% author
% date
"#);
}

#[test]
fn headers() {
    check_roundtrip_stability(r#"
# a

b

## c

d

### e

f
"#);
}

#[test]
fn lists() {
    check_roundtrip_stability(r#"
  * a
  * b
  * c

 1. d
    e
 2. f
      * g
"#);
}
