// ANCHOR: Iter
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Iter)]
pub struct Inner(Vec<i32>);

#[derive(DerivingVia)]
#[deriving(Iter(via: Vec<i32>))]
pub struct Wrapper(Inner);

#[allow(unused)]
#[derive(DerivingVia)]
#[deriving(Iter(via: Vec<T>))]
pub struct Outer<T>(Vec<T>);

pub fn main() {
    let wrapper = Wrapper(Inner(vec![1, 2, 3]));
    for _ in wrapper.iter() {}
}
// ANCHOR_END: Iter

#[test]
fn test_main() {
    main();
}
