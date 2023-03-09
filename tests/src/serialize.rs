use deriving_via::DerivingVia;
use serde::{Serialize, Serializer};

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, Serialize(via = i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, Serialize(via = T))]
pub struct D<T: Serialize>(T);

#[test]
fn test() {
    use serde_json::json;

    let c = C(B(A(1)));
    assert_eq!(json!({ "c": c }).to_string(), r#"{"c":1}"#.to_owned());
    let d = D(1);
    assert_eq!(json!({ "d": d }).to_string(), r#"{"d":1}"#.to_owned());
}
