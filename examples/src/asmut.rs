// ANCHOR: AsMut
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From, AsMut, Eq)]
pub struct Wrapper(Vec<i32>);

#[allow(unused)]
#[derive(DerivingVia)]
#[deriving(From, AsMut(via: Vec<T>))]
pub struct Outer<T>(Vec<T>);

pub fn main() {
    let mut wrapper = Wrapper(vec![1, 2, 4]);
    let _: &mut [i32] = wrapper.as_mut();
}
// ANCHOR_END: AsMut

#[test]
fn test_main() {
    main();
}
