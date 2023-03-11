use std::{collections::hash_map::DefaultHasher, hash::Hash};

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct A(i32);

#[derive(DerivingVia)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(Hash(via: i32))]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(Hash)]
pub struct D<T: Hash>(T);

#[test]
fn test() {
    let mut hasher = DefaultHasher::new();
    let c = C(B(A(42)));
    c.hash(&mut hasher);
    let d = D(42);
    d.hash(&mut hasher);
}
