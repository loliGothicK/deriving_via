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

Therefore, `DerivingVia` automatically generates a `Deref` trait.

`Deref` trait works transitive, but how we re-constructs a `Self` type?
Unfortunately, no convenience mechanism exists in the language,
so it is necessary to teach how to revert using the `#[transitive]` attribute.

## Example

```rust
use std::fmt::Display;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(From)]
pub struct A(i32);

#[derive(DerivingVia)]
#[deriving(From)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(From, Add(via = i32), Display(via = i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

#[derive(DerivingVia)]
#[deriving(From, Display(via = T))]
pub struct D<T: Display>(T);

fn main() {
  let c = C(B(A(42))) + C(B(A(42)));
  println!("{c}");

  let d = D("foo".to_owned());
  println!("{d}");
}
```

## Available Derives

```rust
struct Base(Underlying);

#[derive(DerivingVia)]
#[deriving(<Derive>...)]
struct Target(Base);
```

- `Display`
- `Eq`
- `Ord`
- `Hash`
- `serde::Serialize`
- `serde::Deserialize`
- `Into`
  - additional requirements: `Base: Into<Underlying>`
  - limitations: one hop or `#[transitive]`
- `From`
  - additional requirements: `Base: From<Underlying>`
  - limitations: one hop or `#[transitive]`
- `TryFrom`
  - additional requirements: `Base: From<Underlying>`
  - limitations: one hop or `#[transitive]`
- `FromStr`
  - additional requirements: `Base: From<Underlying>`
  - limitations: one hop or `#[transitive]`
- `Add`-lile (Add, Sub)
  - additional requirements: `Base: From<Underlying>`
  - limitations: one hop or `#[transitive]`
- `Mul`-like (Mul, Div)
  - additional requirements: `Base: From<Underlying>`
  - limitations: one hop or `#[transitive]`
- `Arithmetic` (Add, Sub, Mul, Div)
  - additional requirements: `Base: From<Underlying>`
  - limitations: one hop or `#[transitive]`

## Caveat

DerivingVia using transitive case of _Type Coercion_.
Note that this is not fully supported yet.

See: https://doc.rust-lang.org/reference/type-coercions.html#coercion-types
