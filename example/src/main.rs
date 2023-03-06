use deriving_via::DerivingVia;
use serde_json::json;
use serde::Serializer;

#[derive(DerivingVia)]
pub struct Inner(String);

#[derive(DerivingVia)]
#[deriving(Serialize(via = String))]
pub struct Outer(Inner);

fn main() {
    let foo = Outer(Inner("42".to_owned()));
    let json = json!({ "key": foo });
    println!("{json}");
}
