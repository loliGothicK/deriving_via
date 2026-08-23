# Iterators

## `IntoIterator`
Derives the `std::iter::IntoIterator` trait to allow the type to be converted into an iterator (e.g., for use in `for` loops).

- **Without `via`**: Generates `impl IntoIterator for Type`, delegating to `.into_iter()` on the underlying type.
  - **Requires**: The underlying type must implement `IntoIterator`.
- **Note:** The `via` attribute is not supported here.

### Example

```rust
{{#include ../../examples/src/intoiterator.rs:IntoIterator}}
```

## `FromIterator`
Derives the `std::iter::FromIterator` trait to allow the type to be constructed from an iterator (e.g., using `.collect()`).

- **With `via`**: Generates `impl FromIterator<__ItemT> for Type`. It creates an iterator from the `via` type (i.e. `<via as FromIterator<__ItemT>>::from_iter(iter)`), and then converts the resulting `via` object back to the `Self` type using `.into()`.
  - **Requires**: The `via` type must implement `FromIterator<T>`, and `Self` must implement `From<via>`.

### Example

```rust
{{#include ../../examples/src/fromiterator.rs:FromIterator}}
```

## `Iter`
Generates a convenient inherent `iter` method, often used as a shorthand to get a borrowed iterator.

- **Without `via`**: Generates an inherent `pub fn iter(&self)` method which delegates to the underlying type's `.iter()` method.
  - **Requires**: The underlying type must have an `.iter()` method.
- **With `via`**: Generates an inherent `pub fn iter(&self)` method which delegates to the `.iter()` method of the `via` type.
  - **Requires**: The `via` type must have an `.iter()` method.

### Example

```rust
{{#include ../../examples/src/iter.rs:Iter}}
```
