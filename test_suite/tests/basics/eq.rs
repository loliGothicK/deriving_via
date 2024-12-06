use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct A(i32);

#[derive(DerivingVia)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(Eq(via: i32))]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(Eq)]
pub struct D<T: PartialEq + Eq>(T);

#[test]
fn test() {
    assert!(C(B(A(1))) == C(B(A(1))));
    assert!(D(1) == D(1));
}
