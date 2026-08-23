// ANCHOR: FromIterator
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(FromIterator(via: i32))]
pub struct Inner(Vec<i32>);

#[allow(unused)]
#[derive(DerivingVia)]
#[deriving(FromIterator(via: T))]
pub struct Outer<T>(Vec<T>);

pub fn main() {
    let _: Inner = vec![1, 2, 3].into_iter().collect();
}
// ANCHOR_END: FromIterator

#[test]
fn test_main() {
    main();
}
