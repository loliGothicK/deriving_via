// ANCHOR: Display
use std::fmt::Display;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct Inner(i32);

#[derive(DerivingVia)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(Display(via: i32))]
pub struct Wrapper(Base);

#[derive(DerivingVia)]
#[deriving(Display)]
pub struct Outer<T: Display>(T);

pub fn main() {
    let wrapper = Wrapper(Base(Inner(1)));
    assert_eq!(wrapper.to_string(), 1.to_string());
    let outer = Outer(1);
    assert_eq!(outer.to_string(), 1.to_string());
}
// ANCHOR_END: Display

#[test]
fn test_main() {
    main();
}
