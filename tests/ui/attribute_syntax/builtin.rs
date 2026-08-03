//@edition: 2021
//@compile-flags: -Z track-diagnostics
#![crate_type = "lib"]
#![feature(attribute_syntax)]
#![feature(macro_attr)]

mod x {
    #[macro_export]
    macro_rules! aaaaa {
        attr () {} => {}
    }
    pub use aaaaa as repr;
}

use x::repr;

#[a#repr]
struct Foo;
