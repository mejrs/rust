#![crate_type = "lib"]
#![feature(rustc_attrs)]

#[rustc_as_ptr] //~ERROR
pub struct X;
