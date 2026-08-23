// ANCHOR: Add
use std::ops::Add;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(From, Add(via: i32))]
#[transitive(i32 -> Inner -> Base -> Wrapper)]
pub struct Wrapper(Base);

#[allow(unused)]
#[derive(DerivingVia)]
#[deriving(From, Add(via: T))]
pub struct Outer<T: Add + std::ops::Sub + Clone>(T);

pub fn main() {
    let _ = Wrapper(Base(Inner(1))) + Wrapper(Base(Inner(1)));
}
// ANCHOR_END: Add

#[test]
fn test_main() {
    main();
}
