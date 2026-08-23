// ANCHOR: From
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Wrapper(Base);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Outer<T>(T);

pub fn main() {
    let inner = Inner(1);
    let base: Base = From::from(inner);
    let wrapper: Wrapper = From::from(base);

    let _: Outer<Wrapper> = From::from(wrapper);
}
// ANCHOR_END: From

#[test]
fn test_main() {
    main();
}
