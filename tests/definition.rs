extern crate pandoc_types;

use pandoc_types::definition::*;

#[test]
fn meta_null() {
    assert!(Meta::null().is_null());
}
