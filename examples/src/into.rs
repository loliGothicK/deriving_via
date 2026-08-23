// ANCHOR: Into
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Into(via: i32))]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(Into(via: i32))]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(Into(via: i32))]
pub struct Wrapper(Base);

// Note: IntoInner is preferred for wrappers.
#[derive(DerivingVia)]
#[deriving(IntoInner)]
pub struct Outer<T: Clone>(T);

pub fn main() {
    let _: i32 = Wrapper(Base(Inner(42))).into();
    let _: i32 = Outer(42).into_inner();
}
// ANCHOR_END: Into

#[test]
fn test_main() {
    main();
}
