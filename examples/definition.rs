use pandoc_types::*;
use std::collections::HashMap;

fn main() {
    let mut meta = HashMap::new();
    meta.insert(
        "title".to_owned(),
        MetaValue::MetaInlines(vec![Inline::Str("a".to_owned())]),
    );

    let blocks = vec![
        Block::Header(
            1,
            Attr("a".to_owned(), vec![], vec![]),
            vec![Inline::Str("a".to_owned())],
        ),
        Block::Para(vec![Inline::Str("b".to_owned())]),
    ];

    let doc = Pandoc {
        meta,
        blocks,
        api_version: None
    };

    let s = serde_json::to_string(&doc).unwrap();
    println!("serialized = {}", s);

    let d: Pandoc = serde_json::from_str(&s).unwrap();
    println!("deserialized = {:?}", d);
}
