# deriving_via

```rust
use deriving_via::DerivingVia;
use serde::{Serializer, Deserializer};
use serde_json::json;

#[derive(DerivingVia)]
#[deriving(From, Into)]
pub struct Inner(String);

#[derive(DerivingVia)]
#[deriving(Serialize(via = String), Deserialize(via = String))]
pub struct Outer(Inner);

fn main() {
    let value = Outer(Inner("42".to_owned()));
    let json = json!({ "key": value });
    println!("{json}");
    // output:
    // { "key": "value" }
}
```
