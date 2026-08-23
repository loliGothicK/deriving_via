// ANCHOR: Index
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Index)]
pub struct Inner(Vec<i32>);

pub fn main() {
    let wrapper = Inner(vec![1, 2, 3]);
    let _ = wrapper[0];
}
// ANCHOR_END: Index

#[test]
fn test_main() {
    main();
}
