# deriving_via

This library is a slightly more convenient version of [`derive_more`](https://docs.rs/derive_more/latest/derive_more/).

## Syntax

Derive `DerivingVia` and list the traits you want to derive in the `#[deriving]` attribute.

```rust
#[derive(DerivingVia)]
#[deriving(<Derive>...)]
struct Target(Base);
```

The syntax of `<Derive>` is defined as follows.

```text
Derive := <Trait> | <Trait>(via = <Type>)
```

## Example

In this example, the `Inner` does not derive the `Eq` or `Display`,
but `Outer` derives them via `i32`.

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
- Eq
- Ord
- Hash
- serde::Serialize
- serde::Deserialize
- Into
    - additional requirements: `Base: Into<Underlying>`
    - limitations: one hop
- From
    - additional requirements: `Base: From<Underlying>`
    - limitations: one hop
- TryFrom
    - additional requirements: `Base: From<Underlying>`
    - limitations: one hop
- FromStr
    - additional requirements: `Base: From<Underlying>`
    - limitations: one hop

## Caveat

DerivingVia using transitive case of _Type Coercion_.
Note that this is not fully supported yet.

See: https://doc.rust-lang.org/reference/type-coercions.html#coercion-types
