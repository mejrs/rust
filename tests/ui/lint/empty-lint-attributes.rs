// Empty (and reason-only) lint attributes are legal—although we may want to
// lint them in the future (Issue #55112).

#![allow()] //~ ERROR unused attribute
#![warn(reason = "observationalism")] //~ ERROR unused attribute

#[forbid()] //~ ERROR unused attribute
fn devoir() {}

#[deny(reason = "ultion")] //~ ERROR unused attribute
fn waldgrave() {}

fn main() {}
