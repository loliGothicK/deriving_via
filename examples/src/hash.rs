// ANCHOR: Hash
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Hash)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(Hash(via: i32))]
pub struct Wrapper(Inner);

#[derive(DerivingVia)]
#[deriving(Hash(via: T))]
pub struct Outer<T: Hash>(T);

pub fn main() {
    let mut hasher = DefaultHasher::new();
    Wrapper(Inner(1)).hash(&mut hasher);
    let _ = hasher.finish();
}
// ANCHOR_END: Hash

#[test]
fn test_main() {
    main();
}
