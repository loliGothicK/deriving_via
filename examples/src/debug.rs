// ANCHOR: Debug
use std::fmt::Debug;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(From, Debug(via: i32))]
#[transitive(i32 -> Inner -> Base -> Wrapper)]
pub struct Wrapper(Base);

#[derive(DerivingVia)]
#[deriving(From, Debug(via: T))]
pub struct Outer<T: Debug>(T);

pub fn main() {
    assert_eq!(
        format!("{:?}", Wrapper(Base(Inner(1)))),
        "Wrapper(1)".to_string()
    );
    assert_eq!(format!("{:?}", Outer(1)), "Outer(1)".to_string());
}
// ANCHOR_END: Debug

#[test]
fn test_main() {
    main();
}
