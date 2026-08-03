#![crate_type = "lib"]

#[cfg(false)]
mod foo{
    #[a#repr(C)]
    //~^ ERROR attribute syntax is unstable [E0658]
    struct Foo;
}

#[cfg(false)]
fn foo(){
    let a#foo = 42;
    //~^ ERROR attribute syntax is unstable [E0658]
    //~| ERROR expected pattern, found `a#foo`
}
