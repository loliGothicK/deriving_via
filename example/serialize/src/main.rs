use deriving_via::DerivingVia;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, DerivingVia)]
#[deriving(Display, From, Serialize(via: u32), Deserialize(via: u32))]
struct Id<T>(#[underlying] u32, std::marker::PhantomData<T>);

#[derive(Debug, DerivingVia)]
#[deriving(Display, From, FromStr, Serialize(via: String), Deserialize(via: String))]
struct Name<T>(#[underlying] String, std::marker::PhantomData<T>);

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Id<User>,
    name: Name<User>,
}

fn main() {
    let user = User {
        id: 1u32.into(),
        name: "mitama".parse().unwrap(),
    };
    let json = json!({ "user": user });
    println!("{json}");
}
