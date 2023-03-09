use std::fmt::Display;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct A(i32);

#[derive(DerivingVia)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(Display(via = i32))]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(Display)]
pub struct D<T: Display>(T);

#[test]
fn test() {
    let c = C(B(A(1)));
    assert_eq!(c.to_string(), 1.to_string());
    let d = D(1);
    assert_eq!(d.to_string(), 1.to_string());
}
