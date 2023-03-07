# deriving_via

This library is a slightly more convenient version of [`derive_more`](https://docs.rs/derive_more/latest/derive_more/).

## Syntax

```rust
#[derive(DerivingVia)]
#[deriving(<Derive>...)]
struct Target(Base);
```

```text
Derive := <Trait> | <Trait>(via = <Type>)
```

## Example

In this example, the `Inner` does not derive the `Eq` or `Display`,
but `Outer` derives them through the `i32`.

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

## Available Derives

```rust
struct Base(Underlying);

#[derive(DerivingVia)]
#[deriving(<Derive>...)]
struct Target(Base);
```

- Display
- Into
  - additional requires: `Base: Into<Underlying>`
- From
  - additional requires: `Base: From<Underlying>`
- Eq
- Ord
- TryFrom
  - additional requires: `Base: From<Underlying>`
- FromStr
  - additional requires: `Base: From<Underlying>`
- Hash
- serde::Serialize
- serde::Deserialize
