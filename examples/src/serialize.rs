// ANCHOR: Serialize
use deriving_via::DerivingVia;
use serde::Serialize;

#[derive(Serialize, DerivingVia)]
#[deriving(From)]
pub struct Inner(i32);

#[derive(DerivingVia)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(Serialize(via: i32))]
pub struct Wrapper(Base);

#[derive(DerivingVia)]
#[deriving(Serialize)]
pub struct Outer<T: Serialize>(T);

pub fn main() {
    let _ = serde_json::to_string(&Wrapper(Base(Inner(1)))).unwrap();
    let _ = serde_json::to_string(&Outer(1)).unwrap();
}
// ANCHOR_END: Serialize

#[test]
fn test_main() {
    main();
}
