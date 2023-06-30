![](https://raw.githubusercontent.com/LoliGothick/mitama-lab-static/main/public/DerivingVia.svg)

------------------------
[![Matrix Test](https://github.com/LoliGothick/deriving_via/actions/workflows/ci.yml/badge.svg)](https://github.com/LoliGothick/deriving_via/actions/workflows/ci.yml)
[![crate-name at crates.io](https://img.shields.io/crates/v/deriving_via.svg)](https://crates.io/crates/deriving_via)
[![crate-name at docs.rs](https://docs.rs/deriving_via/badge.svg)](https://docs.rs/deriving_via)
------------------------

This library is a slightly more convenient version of `derive` for newtype pattern.
The library provides features such as Generalised Newtype Deriving, which allows methods of the base type of newtype to be invoked by transitive application of `Deref` traits.
It also allows derives to be generated based on a specific base implementation using the _Deriving Via_ feature.
=> See also [Generalised derived instances for newtypes](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/newtype_deriving.html) and [Deriving via](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/deriving_via.html).

[The Rust Reference](https://doc.rust-lang.org/std/ops/trait.Deref.html) says:
> Deref should only be implemented for smart pointers to avoid confusion.

However, this is the only way to do it, as there is no mechanism such as Generalised Newtype Deriving available.
I consider it acceptable to use `Deref` for the newtype pattern.
Please use this library if and only if you agree with this idea.

## Generalised Newtype Deriving by Deref trait

The `DerivingVia` macro generates the `Deref` trait.
Therefore, repeatedly dereferencing the receiver-type even if the method call is directly ineligible as a syntax.
In other words, if the type derives `DerivingVia`, it can be treated as an _UNDERLING TYPE_.
This works for method calls in general. This is similar to what smart pointers do.
Types that derive `DerivingVia` will behave as _Smart Wrappers_.

### Example

```rust
#[derive(DerivingVia)]
pub struct Foo(i32);

fn main() {
  let foo = Foo(42);

  // This works because of Deref trait.
  // ToOwned trait is implemented for i32.
  // Foo is dereferenced to i32 and to_owned for i32 is called. 
  let i: i32 = foo.to_owned();
}
```

`Foo` desn't implement `ToOwned` trait, but `i32` implements `ToOwned` trait.
`foo.to_owned()` will deref receiver (`foo`) if it doesn't work directly.
`foo` is dereferenced to `i32` and `to_owned()` is called for `i32`.

## Deriving Via

Using the deriving via feature, it is possible to generate derives from the impl of a **specific** base of a multi-layered wrapped type.

### Example

This example is not use _Deriving Via_ feature.

```rust
use std::fmt::Display;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct A(i32);

impl Display for A {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "A({})", self.0)
  }
}

#[derive(DerivingVia)]
pub struct B(A);

fn main() {
  let b = B(A(42));

  // `b.to_string()` uses `A::Display` impl (most nearest impl). 
  assert_eq!(b.to_string(), "A(42)");
}
```

This example is use _Deriving Via_ feature.
`B` derives `Display` trait from `i32` impl.

```rust
use std::fmt::Display;

use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct A(i32);

impl Display for A {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "A({})", self.0)
  }
}

#[derive(DerivingVia)]
#[deriving(Display(via: i32))] // a new line
pub struct B(A);

fn main() {
  let b = B(A(42));

  // `b.to_string()` uses `B::Display` impl directly.
  assert_eq!(b.to_string(), "42");
}
```
## transitive attribute

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
  let c: C = C(B(A(42))) + C(B(A(42)));
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
  - IntoInner
    - requires: `Base: Clone` or `(via: <Type>), Type: Clone`

## Caveat

DerivingVia using transitive case of _Type Coercion_.
According to rumours, transitive _Type Coercion_ is not fully supported yet.

See: https://doc.rust-lang.org/reference/type-coercions.html#coercion-types
