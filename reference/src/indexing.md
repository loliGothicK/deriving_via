# Indexing

## `Index`
Derives the `std::ops::Index` trait to allow immutable container indexing operations (e.g., `container[index]`).

- **Without `via`**: Generates `impl<__IdxT> Index<__IdxT> for Type`, delegating to the underlying field.
  - **Requires**: The underlying type must implement `Index<T>`.
- **With `via`**: Casts `&self` to `&via`, then indexes into the `via` type.
  - **Requires**: The `via` type must implement `Index<T>`.

### Example

```rust
{{#include ../../examples/src/index.rs:Index}}
```

## `IndexMut`
Derives the `std::ops::IndexMut` trait to allow mutable container indexing operations (e.g., `container[index] = value`).

- **Without `via`**: Generates `impl IndexMut<__IdxT> for Type`, which delegates the `index_mut` call to the underlying type's `IndexMut` implementation.
  - **Requires**: The underlying type must implement `IndexMut<T>`.
- **Note:** The `via` attribute is not supported here.

### Example

```rust
{{#include ../../examples/src/indexmut.rs:IndexMut}}
```
