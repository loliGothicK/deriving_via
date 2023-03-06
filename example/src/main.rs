use deriving_via::DerivingVia;
use serde::Serializer;
use serde_json::json;

#[derive(DerivingVia)]
pub struct Inner(String);

#[derive(DerivingVia)]
#[deriving(Serialize(via = String))]
pub struct Outer(Inner);

fn main() {
    let value = Outer(Inner("42".to_owned()));
    let json = json!({ "key": value });
    println!("{json}");
}
