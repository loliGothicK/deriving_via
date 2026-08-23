// ANCHOR: Clone
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(From, Clone(via: i32))]
#[transitive(i32 -> Inner -> Base -> Wrapper)]
pub struct Wrapper(Base);

#[derive(DerivingVia)]
#[deriving(From, Clone(via: T))]
pub struct Outer<T: Clone>(T);

#[derive(DerivingVia)]
#[deriving(From, Into, Clone)]
pub struct Super<T>(#[underlying] i32, std::marker::PhantomData<T>);

pub fn main() {
    let _: Wrapper = Wrapper(Base(Inner(1))).clone();
    let _: Outer<i32> = Outer(1).clone();
    let super_val = Super(1, std::marker::PhantomData);
    let _: Super<Base> = super_val.to_owned();
}
// ANCHOR_END: Clone

#[test]
fn test_main() {
    main();
}
