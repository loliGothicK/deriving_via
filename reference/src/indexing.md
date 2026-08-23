# Indexing

## `Index`
- **Without `via`**: Generates `impl<__IdxT> Index<__IdxT> for Type`, delegating to the underlying field.
- **With `via`**: Casts `&self` to `&via`, then indexes into the `via` type.

## `IndexMut`
- **Without `via`**: Generates `impl<__IdxT> IndexMut<__IdxT> for Type`, delegating to the underlying field.
- **With `via`**: Casts `&mut self` to `&mut via`, then performs mutable indexing on the `via` type.
