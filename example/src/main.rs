use std::fmt::Display;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, Add(via = i32), Mul(via = i32), Display(via = i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, Display(via = T))]
pub struct D<T: Display>(T);

#[derive(DerivingVia)]
#[deriving(Display(via = i32))]
pub struct Test<T>(#[underlying] i32, std::marker::PhantomData<T>);

fn main() {
    let c = C(B(A(42))) + C(B(A(42)));
    println!("{c}");

    let d = D("foo".to_owned());
    println!("{d}");

    let test = Test::<i32>(42, Default::default());
    println!("{test}");
}
