// ANCHOR: Deserialize
use deriving_via::DerivingVia;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(From, Deserialize(via: i32))]
#[transitive(i32 -> Inner -> Base -> Wrapper)]
pub struct Wrapper(Base);

#[derive(DerivingVia)]
#[deriving(From, Deserialize(via: T))]
pub struct Outer<T: for<'a> Deserialize<'a>>(T);

pub fn main() {
    let _: Wrapper = serde_json::from_str("1").unwrap();
}
// ANCHOR_END: Deserialize

#[test]
fn test_main() {
    main();
}
