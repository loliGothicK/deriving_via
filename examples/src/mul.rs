// ANCHOR: Mul
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From, Mul(via: i32))]
#[transitive(i32 -> Inner -> Wrapper)]
pub struct Wrapper(Inner);

#[allow(unused)]
#[derive(DerivingVia)]
#[deriving(From, Mul(via: T))]
pub struct Outer<T: std::ops::Mul + std::ops::Div + Clone>(T);

pub fn main() {
    let _ = Wrapper(Inner(1)) * Wrapper(Inner(1));
}
// ANCHOR_END: Mul

#[test]
fn test_main() {
    main();
}
