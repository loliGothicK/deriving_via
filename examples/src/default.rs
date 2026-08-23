// ANCHOR: Default
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Default(via: T))]
pub struct Outer<T: Default>(T);

pub fn main() {
    let _ = Outer::<i32>::default();
}
// ANCHOR_END: Default

#[test]
fn test_main() {
    main();
}
