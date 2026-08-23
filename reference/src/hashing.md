# Hashing

## `Hash`
- **Without `via`**: Generates `impl Hash for Type`, calling the `hash` method on the underlying field.
- **With `via`**: Casts `&self` to `&via`, then hashes the `via` type.
