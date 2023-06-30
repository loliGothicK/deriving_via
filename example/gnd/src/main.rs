use std::fmt::Display;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct A(i32);

impl Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A({})", self.0)
    }
}

#[derive(DerivingVia)]
#[deriving(Display(via: i32))]
pub struct B(A);

fn main() {
    let b = B(A(42));

    assert_eq!(b.to_string(), "42");
}
