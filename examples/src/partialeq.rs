use deriving_via::DerivingVia;
// ANCHOR: PartialEq
#[derive(DerivingVia)]
#[deriving(PartialEq)]
pub struct Inner(i32);

pub fn main() {
    assert!(Inner(1) == Inner(1));
}
// ANCHOR_END: PartialEq

#[test]
fn test_main() {
    main();
}
