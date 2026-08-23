use deriving_via::DerivingVia;
// ANCHOR: PartialOrd
#[derive(DerivingVia)]
#[deriving(PartialOrd, PartialEq)]
pub struct Inner(i32);

pub fn main() {
    assert!(Inner(1) < Inner(2));
}
// ANCHOR_END: PartialOrd

#[test]
fn test_main() {
    main();
}
