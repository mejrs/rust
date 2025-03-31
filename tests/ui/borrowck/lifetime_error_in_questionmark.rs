//@ compile-flags: -Z track-diagnostics

#![crate_type= "lib"]

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct MyError<Src> {
    stuff: Src,
}
impl<T: Debug> Display for MyError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        Debug::fmt(self, f)
    }
}
impl<T: Debug> Error for MyError<T> {}

// like `anyhow::Error`
#[derive(Debug)]
struct StaticError;

impl<E> From<E> for StaticError
where
    E: Error + Send + Sync + 'static,
{
    fn from(_error: E) -> Self {
        StaticError
    }
}

fn parse<X>(stuff: X) -> Result<(), MyError<X>> {
    Err(MyError { stuff })
}

fn foo() -> Result<(), StaticError> {
    let stuff = vec![1_u8, 2, 3, 4];
    parse(&stuff)?;
    //~^ ERROR `stuff` does not live long enough
    Ok(())
}
