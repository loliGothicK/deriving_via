# Conversion

## `AsRef`
Derives the `std::convert::AsRef` trait to provide a cheap, reference-to-reference conversion.

- **Without `via`**: Generates a blanket `impl<__AsRefT> AsRef<__AsRefT> for Type`. The underlying type must implement `AsRef<__AsRefT>`.
  - **Requires**: The underlying type must implement `AsRef<T>`.
- **With `via`**: Generates `impl AsRef<via> for Type`, which casts the reference to the underlying field into `&via` and returns it.
  - **Requires**: The underlying type must be coercible to `&via` (e.g. implicitly via `Deref`).

### Example

```rust
{{#include ../../examples/src/asref.rs:AsRef}}
```

## `AsMut`
Derives the `std::convert::AsMut` trait to provide a cheap, mutable reference-to-mutable reference conversion.

- **Without `via`**: Generates a blanket `impl<__AsMutT> AsMut<__AsMutT> for Type`.
  - **Requires**: The underlying type must implement `AsMut<T>`.
- **With `via`**: Generates a blanket `impl<__AsMutT> AsMut<__AsMutT> for Type`. The implementation delegates to `<via as AsMut<__AsMutT>>::as_mut(&mut self.0)`.
  - **Requires**: The `via` type must implement `AsMut<T>`, and `Self` must implement `DerefMut` where `<Self as Deref>::Target == via`.

### Example

```rust
{{#include ../../examples/src/asmut.rs:AsMut}}
```

## `From`
Derives the `std::convert::From` trait for value-to-value conversion.

- **Without `via`**: Generates `impl From<Underlying> for Type`, wrapping the underlying type.
  - **Requires**: None (works unconditionally).
- **With `via`**: Generates `impl From<via> for Type`, calling `__.into()` to convert from `via` to `Self`. The `#[transitive]` attribute is typically needed if crossing multiple layers.
  - **Requires**: The `via` type must implement `Into<Underlying>`, or `Underlying` must implement `From<via>`.

### Example

```rust
{{#include ../../examples/src/from.rs:From}}
```

## `Into`
Derives the `std::convert::Into` trait by actually implementing `From<Type> for Target` (as `Into` is automatically provided by a blanket implementation).

- **Without `via`**: Generates `impl From<Type> for Underlying`.
  - **Requires**: The underlying type must implement `Clone`.
- **With `via`**: Generates `impl From<Type> for via` by casting `&Type` to `&via` and cloning it.
  - **Requires**: The `via` type must implement `Clone`.

### Example

```rust
{{#include ../../examples/src/into.rs:Into}}
```

## `TryFrom`
Derives the `std::convert::TryFrom` trait for fallible value-to-value conversion.

- **Without `via`**: Generates `impl TryFrom<Underlying> for Type`.
  - **Requires**: None (works unconditionally).
- **With `via`**: Generates `impl TryFrom<via> for Type`. This delegates to `<Self as From<via>>::from` or utilizes transitive logic.
  - **Requires**: `Self` must implement `From<via>`.

### Example

```rust
{{#include ../../examples/src/tryfrom.rs:TryFrom}}
```

## `FromStr`
Derives the `std::str::FromStr` trait to allow parsing the type from a string.

- **Without `via`**: Generates `impl FromStr for Type`. It parses a string into the underlying type and wraps it.
  - **Requires**: The underlying type must implement `FromStr`, and `Self` must implement `From<Underlying>`.
- **With `via`**: Parses a string into the `via` type, then calls `.into()` to convert to `Self`.
  - **Requires**: The `via` type must implement `FromStr`, and `Self` must implement `From<via>`.

### Example

```rust
{{#include ../../examples/src/fromstr.rs:FromStr}}
```
