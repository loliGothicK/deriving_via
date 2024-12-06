use std::{collections::hash_map::DefaultHasher, hash::Hash};

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(Hash)]
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
    B(A(1)).hash(&mut hasher);
    c.hash(&mut hasher);
    let d = D(42);
    d.hash(&mut hasher);
}
