![logo](https://raw.githubusercontent.com/LoliGothick/mitama-lab-static/main/public/DerivingVia.svg)

---

[![Matrix Test](https://github.com/LoliGothick/deriving_via/actions/workflows/ci.yml/badge.svg)](https://github.com/LoliGothick/deriving_via/actions/workflows/ci.yml)
[![crate-name at crates.io](https://img.shields.io/crates/v/deriving_via.svg)](https://crates.io/crates/deriving_via)
[![crate-name at docs.rs](https://docs.rs/deriving_via/badge.svg)](https://docs.rs/deriving_via)

---

Welcome to _Deriving via_, a library that offers the ability to adopt the Powerful Newtype Pattern in Rust. This library provides a practical way to automatically derive implementations for newtype wrappers, in the spirit of Haskell's [GeneralisedNewtypeDeriving](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/newtype_deriving.html) and [Deriving via](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/deriving_via.html) extensions.

According to [The Rust Reference](https://doc.rust-lang.org/std/ops/trait.Deref.html), the `Deref` trait is typically only implemented for smart pointers in Rust. However, this library deviates from that policy.
This library uses the `Deref` trait as a hack to implement the newtype pattern.
If you are comfortable with this approach, this library is for you.

_deriving via_ aims to be your tool of choice for handling newtype patterns in Rust. The library makes use of a `DerivingVia` macro to generate `Deref` trait implementations, which allow your types to behave as _Smart Wrappers_ by automatically dereferencing their _underlying types_.

Our library also introduces features such as explicit Generalised Newtype Deriving using the `#[deriving]` attribute and a way to specify base types for derive generation using the `#[deriving(Trait(via: Type))]` mechanism.

## Generalised Newtype Deriving by Deref trait (in general)

The `DerivingVia` macro generates the `Deref` trait implementation.
Therefore, even if the method call is not directly syntactically valid, the receiver type can be repeatedly dereferenced;
in other words, if the type is derived `DerivingVia`, it can be treated as an _UNDERLYING TYPE_.
This works for method calls in general. This is similar to what smart pointers do.
Types that derive `DerivingVia` will behave as _Smart Wrappers_.

### Example

`DerivingVia` macro generates `Deref` trait implementation.
Therefore, even if the method call is not directly syntactically valid, the receiver type can be repeatedly dereferenced.

```rust
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
pub struct Foo(i32);

fn main() {
  let foo = Foo(42);

  let i: i32 = foo.to_owned(); // This works.
}
```

`Foo` doesn't implement `Clone` trait; therefore, `foo.to_owned()` doesn't work directly.
However, `Foo` implements `Deref` trait; therefore, `foo` is dereferenced to `i32` and `to_owned()` is called for `i32`.

```rust
pub struct Foo(i32);

impl Deref for Foo {
  type Target = i32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn main() {
  let foo = Foo(42);

  // This works because of Deref trait.
  // ToOwned trait is implemented for i32.
  // Foo is dereferenced to i32 and to_owned for i32 is called.
  let i: i32 = foo.to_owned();
}
```

## Explicit Generalised Newtype Deriving

`#[deriving]` attribute is available for explicit Generalised Newtype Deriving.

### Example (GND)

```rust
use deriving_via::DerivingVia;

#[derive(DerivingVia)]
#[deriving(Display)]
pub struct Foo(i32);

fn main() {
  let foo = Foo(42);

  println!("{foo}"); // 42
}
```

## Deriving via

Using the _Deriving via_ feature, it is possible to generate derives from the implementation of a **specific** base of a multi-layered wrapped type.

### Example (Deriving via)

This example does not use _Deriving via_ feature.

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

This example uses _Deriving via_ feature.
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

## `#[transitive]` attribute

By the way, when you want to derive `Add`, you can dereference up to `i32`, but not from `i32` back to `Self`.
Therefore, you need to derive `From` from `i32` to `Self`.
You also need to specify the `#[transitive]` attribute to specify the order in which to return.
Some traits require `#[transitive]` attribute (see Available Derives section).

Note: `From<T> for T` is implemented by [generic implementations](https://doc.rust-lang.org/std/convert/trait.From.html#generic-implementations).

### Example (transitive)

The following example derives `Add` and `Display` for `C`.
To implement `Display`, it is sufficient to dereference `C` to `i32`.
However, to implement `Add`, it is necessary to dereference from `i32` back to `C`.
To do so, you need to derive `From` for every newtype.
In addition, you need to specify the order in which to return from `i32` to `C` using the `#[transitive]` attribute.

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
  - `Add`-like (Add, Sub)
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

DerivingVia using a transitive case of _Type Coercion_.
According to rumours, transitive _Type Coercion_ is not fully supported yet.

See: <https://doc.rust-lang.org/reference/type-coercions.html#coercion-types>
