use std::ops::{Div, Mul};

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, Mul(via = i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, Mul(via = T))]
pub struct D<T: Mul + Div + Clone>(T);

#[test]
fn test() {
    let _ = C(B(A(1))) * C(B(A(1)));
    let _ = C(B(A(1))) / C(B(A(1)));
    let _ = D(1) * D(1);
    let _ = D(1) / D(1);
}
