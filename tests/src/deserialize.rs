use deriving_via::DerivingVia;
use serde::{Deserialize, Serialize};

#[derive(DerivingVia)]
#[deriving(From, Serialize, Deserialize)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, Serialize(via: i32), Deserialize(via: i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, Serialize(via: T), Deserialize(via: T))]
pub struct D<T: Serialize + for<'a> Deserialize<'a>>(T);

#[test]
fn transitive() {
    let c = C(B(A(1)));

    let serialized = serde_json::to_string(&c).unwrap();

    assert_eq!(serialized, "1".to_owned());

    let deserialized: C = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.0 .0 .0, 1);
}

#[test]
fn generics() {
    let d = D(1);

    let serialized = serde_json::to_string(&d).unwrap();

    assert_eq!(serialized, "1".to_owned());

    let deserialized: D<i32> = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized.0, 1);
}
