//@edition: 2021
//@compile-flags: -Z track-diagnostics
#![deny(unknown_diagnostic_attributes)]
#![crate_type = "lib"]
#![feature(attribute_syntax)]

mod diagnostic {}

#[a#diagnostic::doesnt_exist]
//~^ ERROR unknown diagnostic attribute
struct Foo;
