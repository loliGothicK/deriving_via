# Iterators

## `IntoIterator`
- **Without `via`**: Generates `impl IntoIterator for Type`, delegating to `.into_iter()` on the underlying type.
- **With `via`**: Delegates to the `IntoIterator` implementation of the `via` type.

## `FromIterator`
- **With `via` only**: Generates `impl FromIterator<via> for Type`. Collects items into the underlying type and wraps it. The underlying type must implement `FromIterator<via>`.

## `Iter`
- Generates an inherent method: `fn iter(&self) -> core::slice::Iter<'_, Item>`.
- The underlying type (or the `via` type) must implement `IntoIterator` and be coercible to a slice (`&[Item]`).
