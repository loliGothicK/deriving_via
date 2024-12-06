use std::fmt::Debug;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, Debug(via: i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, Debug(via: T))]
pub struct D<T: Debug>(T);

#[test]
fn test() {
    assert_eq!(format!("{:?}", C(B(A(1)))), "C(1)".to_string());
    assert_eq!(format!("{:?}", D(1)), "D(1)".to_string());
}
