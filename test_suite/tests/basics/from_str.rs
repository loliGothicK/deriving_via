use std::str::FromStr;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From, FromStr)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From, FromStr)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, FromStr(via: i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, FromStr(via: T))]
pub struct D<T: FromStr>(T);

#[test]
fn test() {
    let _: C = C::from_str("42").unwrap();
    let _: D<i32> = D::from_str("42").unwrap();
}
