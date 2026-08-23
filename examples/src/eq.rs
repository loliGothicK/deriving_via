use deriving_via::DerivingVia;
// ANCHOR: Eq
#[derive(DerivingVia)]
pub struct Inner(i32);

#[derive(DerivingVia)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(Eq(via: i32))]
pub struct Wrapper(Base);

#[derive(DerivingVia)]
#[deriving(Eq)]
pub struct Outer<T: PartialEq + Eq>(T);

pub fn main() {
    assert!(Wrapper(Base(Inner(1))) == Wrapper(Base(Inner(1))));
    assert!(Outer(1) == Outer(1));
}
// ANCHOR_END: Eq

#[test]
fn test_main() {
    main();
}
