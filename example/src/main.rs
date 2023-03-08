use std::fmt::Display;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, Add(via = i32), Display(via = i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, Display(via = T))]
pub struct D<T: Display>(T);

fn main() {
    let c = C(B(A(42))) + C(B(A(42)));
    println!("{c}");

    let d = D("foo".to_owned());
    println!("{d}");
}
