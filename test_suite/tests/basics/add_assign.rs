use std::ops::{AddAssign, SubAssign};

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, AddAssign(via: i32))]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, AddAssign(via: T))]
pub struct D<T: AddAssign + SubAssign + Clone>(T);

#[test]
fn test() {
    let mut c = C(B(A(1)));
    c += C(B(A(1)));
    c -= C(B(A(1)));
}
