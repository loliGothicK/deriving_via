// ANCHOR: IntoInner
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From, IntoInner)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(From, IntoInner(via: i32))]
pub struct Wrapper(Base);

pub fn main() {
    let inner = Inner(1);
    let _: i32 = inner.into_inner();
    let wrapper = Wrapper(Base(Inner(1)));
    let _: i32 = wrapper.into_inner();
}
// ANCHOR_END: IntoInner

#[test]
fn test_main() {
    main();
}
