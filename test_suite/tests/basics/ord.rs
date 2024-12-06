use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Ord(via: i32), Eq(via: i32))]
pub struct A(i32);

#[derive(Debug, DerivingVia)]
#[deriving(
    Eq(via: u32),
    Ord(via: u32),
)]
pub struct Id<T>(#[underlying] u32, std::marker::PhantomData<T>);

#[test]
fn test() {
    assert!(A(1) < A(2));
}
