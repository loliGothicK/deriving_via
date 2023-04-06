![](https://raw.githubusercontent.com/LoliGothick/mitama-lab-static/main/public/DerivingVia.svg)

------------------------
[![Matrix Test](https://github.com/LoliGothick/deriving_via/actions/workflows/ci.yml/badge.svg)](https://github.com/LoliGothick/deriving_via/actions/workflows/ci.yml)
[![crate-name at crates.io](https://img.shields.io/crates/v/deriving_via.svg)](https://crates.io/crates/deriving_via)
[![crate-name at docs.rs](https://docs.rs/deriving_via/badge.svg)](https://docs.rs/deriving_via)
------------------------

This library is a slightly more convenient version of [`derive_more`](https://docs.rs/derive_more/latest/derive_more/) for newtype pattern.

=> [Deriving via](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/deriving_via.html)

## Basic Usage

`#[derive(DerivingVia)]` and then write the `#[deriving]` attribute on struct and list the trait you want to derive in it.

### simple

Derives `From<i32> for D` and `Display for D`.

```rust
#[derive(DerivingVia)]
#[deriving(From, Display)]
pub struct D(i32);
```

### with generics

```rust
#[derive(DerivingVia)]
#[deriving(From, Display)]
pub struct D<T: Display>(T);
```

### with newtype pattern

If you have more than one field, specify `#[underlying]` for one.
Note that the other fields require default initialisation by the `Default` trait.

```rust
#[derive(DerivingVia)]
#[deriving(From, Display)]
pub struct Test<T>(#[underlying] i32, std::marker::PhantomData<T>);
```

## Syntax

Derive `DerivingVia` and list the traits you want to derive in the `#[deriving]` attribute.

```rust
#[derive(DerivingVia)]
#[deriving(<Derive>...)]
struct Target(Base);
```

The syntax of `<Derive>` is defined as follows.

```text
Derive := <Trait> | <Trait>(via: <Type>)
```

## Deriving Via

Using the deriving via feature, it is possible to generate derives from the impl of a base of a multi-layered wrapped type.

`DerivingVia` uses transitive type coercion for type conversion.
All newtypes must be dereferenceable to the underlying type.
Therefore, `DerivingVia` automatically generates a `Deref` trait.

### Example

```rust
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct A(i32);

#[derive(DerivingVia)]
pub struct B(A);

#[derive(DerivingVia)]
#[deriving(Display(via: i32))]
pub struct C(B);

fn main() {
  let c = C(B(A(42)));
  println!("{c}"); // 42
}
```

`Deref` trait works transitive, but how we re-constructs a `Self` type?
Unfortunately, no convenience mechanism exists in the language,
so it is necessary to teach how to revert using the `#[transitive]` attribute.
Some trait require `#[transitive]` attribute (see Available Derives section).

### Example

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
#[deriving(From, Add(via: i32), Display(via: i32))]
#[transitive(i32 -> A -> B -> C)]
pub struct C(B);

fn main() {
  let c = C(B(A(42))) + C(B(A(42)));
  println!("{c}");
}
```

## Available Derives

```rust
struct Base(Underlying);

#[derive(DerivingVia)]
#[deriving(<Derive>)]
struct Target(Base);
```

- fmt
  - `Display`
    - requires: `Base: Display` or `(via = <Type>) and Type: Display`
- ops
  - `Eq`
    - requires: `Base: Eq` or `(via = <Type>) and Type: Eq`
  - `Ord`
    - requires: `Base: Ord` or `(via = <Type>) and Type: Ord`
  - `Add`-lile (Add, Sub)
    - requires: `Base: From<Underlying>`
    - limitations: one hop or `#[transitive]`
  - `Mul`-like (Mul, Div)
    - requires: `Base: From<Underlying>`
    - limitations: one hop or `#[transitive]`
  - `Arithmetic` (Add, Sub, Mul, Div)
    - requires: `Base: From<Underlying>`
    - limitations: one hop or `#[transitive]`
  - `Index`
    - requires: `Base: Index` or `(via = <Type>) and Type: Index`
  - `IndexMut`
    - requires: `Base: IndexMut` or `(via = <Type>) and Type: IndexMut`
  - `DerefMut`
    - requires: `Base: DerefMut` or `(via = <Type>) and Type: DerefMut`
- hash
  - `Hash`
    - requires: `Base: Hash` or `(via = <Type>) and Type: Hash`
- serde
  - `Serialize`
    - requires: `Base: Serialize` or `(via = <Type>) and Type: Serialize`
  - `Deserialize`
    - requires: `Base: Deserialize` or `(via = <Type>) and Type: Deserialize`
- convert
  - `AsRef`
  - `AsMut`
  - `FromIterator`
    - requires: `(via: <ItemType>)`
  - `IntoIterator`
    - requires: `Base: IntoIterator` or `(via: <Type>), Type: IntoIterator`
  - `Into`
    - requires: `Base: Into<Underlying>`
    - limitations: one hop or `#[transitive]`
  - `From`
    - limitations: one hop or `#[transitive]`
  - `TryFrom`
    - requires: `Base: From<Underlying>`
    - limitations: one hop or `#[transitive]`
  - `FromStr`
    - requires: `Base: From<Underlying>`
    - limitations: one hop or `#[transitive]`
- impls
  - Iter
    - requires: `Base: IntoIterator and Base dereferenceable to slice` or `(via: <Type>), Type: IntoIterator and Type dereferenceable to slice`


## Caveat

DerivingVia using transitive case of _Type Coercion_.
According to rumours, transitive _Type Coercion_ is not fully supported yet.

See: https://doc.rust-lang.org/reference/type-coercions.html#coercion-types
