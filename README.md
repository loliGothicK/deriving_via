# deriving_via

```rust
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct Inner(i32);

#[derive(DerivingVia)]
#[deriving(Eq(via = i32), Display(via = i32))]
pub struct Outer(Inner);

fn main() {
    let x = Outer(Inner(42));
    let y = Outer(Inner(42));

    println!("{x} == {y} => {}", x == y);
    // 42 == 42 => true
}
```
