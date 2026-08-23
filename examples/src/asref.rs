// ANCHOR: AsRef
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(From, AsRef(via: i32))]
#[transitive(i32 -> Inner -> Base -> Wrapper)]
pub struct Wrapper(Base);

#[derive(DerivingVia)]
#[deriving(From, AsRef(via: T))]
pub struct Outer<T>(T);

pub fn main() {
    let wrapper = Wrapper(Base(Inner(1)));
    let _ = wrapper.as_ref();
    let outer = Outer(1);
    let _ = outer.as_ref();
}
// ANCHOR_END: AsRef

#[test]
fn test_main() {
    main();
}
