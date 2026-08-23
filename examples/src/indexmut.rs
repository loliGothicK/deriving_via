// ANCHOR: IndexMut
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Index, IndexMut)]
pub struct Inner(Vec<i32>);

pub fn main() {
    let mut wrapper = Inner(vec![1, 2, 3]);
    wrapper[0] = 42;
}
// ANCHOR_END: IndexMut

#[test]
fn test_main() {
    main();
}
