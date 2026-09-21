//@ check-fail
//@ compile-flags: --crate-type=lib

// Regression test for issue 142649
pub fn public() {
    #[deprecated] 0
    //~^ ERROR mismatched types
    //~| ERROR attribute cannot be used on expressions
    //~| WARN previously accepted
}
