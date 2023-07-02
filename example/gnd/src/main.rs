use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(Display)]
pub struct B(A);

fn main() {
    let b = B(A(42));

    assert_eq!(b.to_string(), "42");
}
