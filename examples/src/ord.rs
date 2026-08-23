use deriving_via::DerivingVia;
// ANCHOR: Ord
#[derive(DerivingVia)]
#[deriving(Ord(via: i32), Eq(via: i32))]
pub struct Inner(i32);

#[allow(unused)]
#[derive(Debug, DerivingVia)]
#[deriving(
    Eq(via: u32),
    Ord(via: u32),
)]
pub struct Id<T>(#[underlying] u32, std::marker::PhantomData<T>);

pub fn main() {
    assert!(Inner(1) < Inner(2));
}
// ANCHOR_END: Ord

#[test]
fn test_main() {
    main();
}
