use std::ops::{Add, Div, Mul, Sub};

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From, Arithmetic)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, Arithmetic(via = i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, Arithmetic(via = T))]
pub struct D<T: Add + Sub + Mul + Div + Clone>(T);

#[test]
fn test() {
    let _ = C(B(A(1))) + C(B(A(1)));
    let _ = C(B(A(1))) - C(B(A(1)));
    let _ = C(B(A(1))) * C(B(A(1)));
    let _ = C(B(A(1))) / C(B(A(1)));

    let _ = D(1) + D(1);
    let _ = D(1) - D(1);
    let _ = D(1) * D(1);
    let _ = D(1) / D(1);
}
