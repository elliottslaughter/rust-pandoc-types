extern crate serde_json;

extern crate pandoc_types;

use pandoc_types::definition::*;

fn main() {
    let mut meta = Meta::null();
    meta.0.insert("title".to_string(), MetaValue::MetaInlines(vec![Inline::Str("a".to_string())]));

    let doc = Pandoc(
        meta,
        vec![Block::Header(1,
                           Attr("a".to_string(), vec![], vec![]),
                           vec![Inline::Str("a".to_string())]),
             Block::Para(vec![Inline::Str("b".to_string())])]);

    let s = serde_json::to_string(&doc).unwrap();
    println!("serialized = {}", s);

    let d: Pandoc = serde_json::from_str(&s).unwrap();
    println!("deserialized = {:?}", d);
}
