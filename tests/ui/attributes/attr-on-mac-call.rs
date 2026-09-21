//@ check-fail
// Regression test for https://github.com/rust-lang/rust/issues/145779

#![feature(register_tool)]
#![feature(sanitize)]

fn main() {
    #[export_name = "x"]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[unsafe(naked)]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[track_caller]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[used]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[target_feature(enable = "x")]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[deprecated]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[inline]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[link_name = "x"]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[link_section = "__TEXT,__text"]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[link_ordinal(42)]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[non_exhaustive]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[proc_macro]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[cold]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[no_mangle]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[deprecated]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[automatically_derived]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[macro_use]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[must_use]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[no_implicit_prelude]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[path = ""]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[ignore]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[should_panic]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[link_name = "x"]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    #[sanitize(address = "off")]
    //~^ ERROR attribute cannot be used on macro calls
    unreachable!();

    #[repr()]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    //~| ERROR unused attribute
    unreachable!();
    #[repr(u8)]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    unreachable!();
    #[repr(align(8))]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    unreachable!();
    #[repr(packed)]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    unreachable!();
    #[repr(C)]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    unreachable!();
    #[repr(Rust)]
    //~^ ERROR attribute cannot be used on macro calls
    //~| WARN previously accepted
    unreachable!();
    #[repr(simd)]
    //~^ ERROR attribute cannot be used on macro calls
    //~| ERROR SIMD types are experimental and possibly buggy
    unreachable!();
    #[register_tool(xyz)]
    //~^ ERROR crate-level attribute should be an inner attribute
    unreachable!();
    #[deprecated = concat!("woah", "dude")]
    //~^ ERROR attribute value must be a literal
    #[doc = concat!("woah", "dude")]
    unreachable!();
    #[doc = {
        let a = 1;
        let b = 1;
        let sum = a + b;
        assert_eq!(sum, 2);
    }]
    unreachable!();
}
