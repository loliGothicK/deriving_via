// ANCHOR: Copy
use std::io::Stderr;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From, Clone(via: T))]
pub struct Outer<T: Clone>(T);

#[derive(DerivingVia)]
#[deriving(From, Into, Copy)]
pub struct Super<T>(#[underlying] i32, std::marker::PhantomData<T>);

pub fn main() {
    let _: Outer<i32> = Outer(1).clone();
    let super_val = Super(1, std::marker::PhantomData);
    let _: Super<Stderr> = super_val;
}
// ANCHOR_END: Copy

#[test]
fn test_main() {
    main();
}
