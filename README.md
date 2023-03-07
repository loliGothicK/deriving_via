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

## How DerivingVia works

`DerivingVia` uses transitive type coercion for type conversion.
All newtypes must be dereferenceable to the underlying type.

Therefore, `DerivingVia` automatically implies a `Deref` trait.


## Example

```rust
use deriving_via::DerivingVia;

#[derive(DerivingVia)] // for Deref
pub struct A(i32);

#[derive(DerivingVia)] // for Deref
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(Display(via = i32))]
pub struct C(B);

fn main() {
  let c = C(B(A(42)));
  println!("{c}"); // 42
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
