use std::fmt::*;

macro_rules! impl_for{
    ($this:ident, $($trait:ident),*) => {
        $(
            impl std::fmt::$trait for $this {
                fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    todo!()
                }
            }
        )*
    }
}

pub struct A;
impl_for!(A, Debug, Pointer, Display);

fn main(){
    let a = A;
    format!("the thing: {a:X}");
    //~^ ERROR cannot format `A` with format specifier `X` [E0277]
}