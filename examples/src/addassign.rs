// ANCHOR: AddAssign
use std::ops::{AddAssign, SubAssign};

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(From, AddAssign(via: i32))]
pub struct Wrapper(Base);

#[allow(unused)]
#[derive(DerivingVia)]
#[deriving(From, AddAssign(via: T))]
pub struct Outer<T: AddAssign + SubAssign + Clone>(T);

pub fn main() {
    let mut wrapper = Wrapper(Base(Inner(1)));
    wrapper += Wrapper(Base(Inner(1)));
    wrapper -= Wrapper(Base(Inner(1)));
}
// ANCHOR_END: AddAssign

#[test]
fn test_main() {
    main();
}
