use std::fmt::Display;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Display(via: T))]
#[allow(unused)]
pub struct D<T: Display>(T);

#[test]
fn test() {}
