use std::io::Stderr;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From, Clone(via: T))]
pub struct D<T: Clone>(T);

#[derive(DerivingVia)]
#[deriving(From, Into, Copy)]
pub struct E<T>(#[underlying] i32, std::marker::PhantomData<T>);

#[test]
fn test() {
    let _: D<i32> = D(1).clone();
    let e = E(1, std::marker::PhantomData);
    let _: E<Stderr> = e;
}
