// See https://github.com/rust-lang/rust/issues/95151
#[track_caller]
//~^ ERROR attribute cannot be used on macro defs
//~| WARN previously accepted
macro_rules! _foo {
    () => {};
}

fn main() {
}
