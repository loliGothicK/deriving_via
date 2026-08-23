// ANCHOR: Arithmetic
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(From, Arithmetic(via: i32))]
#[transitive(i32 -> Inner -> Base -> Wrapper)]
pub struct Wrapper(Base);

pub fn main() {
    let _ = Wrapper(Base(Inner(1))) + Wrapper(Base(Inner(1)));
    let _ = Wrapper(Base(Inner(1))) - Wrapper(Base(Inner(1)));
    let _ = Wrapper(Base(Inner(1))) * Wrapper(Base(Inner(1)));
    let _ = Wrapper(Base(Inner(1))) / Wrapper(Base(Inner(1)));
}
// ANCHOR_END: Arithmetic

#[test]
fn test_main() {
    main();
}
