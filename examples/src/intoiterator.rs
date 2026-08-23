// ANCHOR: IntoIterator
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(IntoIterator)]
pub struct Inner(Vec<i32>);

pub fn main() {
    let wrapper = Inner(vec![1, 2, 3]);
    for _ in wrapper {}
}
// ANCHOR_END: IntoIterator

#[test]
fn test_main() {
    main();
}
