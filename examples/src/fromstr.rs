// ANCHOR: FromStr
use std::str::FromStr;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From, FromStr)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(From, FromStr)]
pub struct Base(Inner);

#[derive(DerivingVia)]
#[deriving(From, FromStr(via: i32))]
#[transitive(i32 -> Inner -> Base -> Wrapper)]
pub struct Wrapper(Base);

#[derive(DerivingVia)]
#[deriving(From, FromStr(via: T))]
pub struct Outer<T: FromStr>(T);

pub fn main() {
    let _: Wrapper = Wrapper::from_str("42").unwrap();
    let _: Outer<i32> = Outer::from_str("42").unwrap();
}
// ANCHOR_END: FromStr

#[test]
fn test_main() {
    main();
}
