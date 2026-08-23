// ANCHOR: MulAssign
use std::ops::DivAssign;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From, MulAssign(via: i32))]
pub struct Wrapper(Inner);

pub fn main() {
    let mut wrapper = Wrapper(Inner(2));
    wrapper *= Wrapper(Inner(3));
}
// ANCHOR_END: MulAssign

#[test]
fn test_main() {
    main();
}
