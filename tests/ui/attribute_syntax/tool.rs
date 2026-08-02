//@edition: 2021
//@compile-flags: -Z track-diagnostics

#![crate_type = "lib"]
#![feature(attribute_syntax)]

mod rustfmt {}

#[a#rustfmt::skip]
struct Foo;
