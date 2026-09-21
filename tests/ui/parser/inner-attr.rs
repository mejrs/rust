#[feature(lang_items)] //~ ERROR crate-level attribute should be an inner attribute

#![recursion_limit="100"] //~ ERROR an inner attribute is not permitted following an outer attribute
fn main() {}
