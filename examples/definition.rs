extern crate serde_json;

extern crate pandoc_types;

use pandoc_types::definition::*;

fn main() {
    let mut meta = Meta::null();
    meta.0.insert(String::from("title"), MetaValue::MetaInlines(vec![Inline::Str(String::from("a"))]));

    let doc = Pandoc(
        meta,
        vec![Block::Header(1,
                           Attr(String::from("a"), vec![], vec![]),
                           vec![Inline::Str(String::from("a"))])]);

    let s = serde_json::to_string(&doc).unwrap();
    println!("serialized = {}", s);

    let d: Pandoc = serde_json::from_str(&s).unwrap();
    println!("deserialized = {:?}", d);
}
