# Hashing

## `Hash`
Derives the `std::hash::Hash` trait to allow the type to be hashed (e.g., for use in `HashMap` or `HashSet`).

- **Without `via`**: Generates `impl Hash for Type`, calling the `hash` method on the underlying field.
  - **Requires**: The underlying type must implement `Hash`.
- **With `via`**: Casts `&self` to `&via`, then hashes the `via` type.
  - **Requires**: The `via` type must implement `Hash`.

### Example

```rust
{{#include ../../examples/src/hash.rs:Hash}}
```
