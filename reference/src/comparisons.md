# Comparisons

## `Eq`
- **Without `via`**: Generates `impl Eq for Type`.
- **With `via`**: Generates `impl Eq for Type`. No custom logic is needed other than the marker trait itself, but this relies on `PartialEq` being implemented properly.

## `Ord`
- **Without `via`**: Generates `impl Ord for Type`, delegating to the `cmp` method of the underlying type.
- **With `via`**: Casts `&self` and `&other` to `&via`, then delegates to `Ord::cmp` on the `via` type.

## `PartialEq`
- **Without `via`**: Generates `impl PartialEq for Type`, delegating the `eq` method to the underlying type.
- **With `via`**: Casts both sides to `&via`, then delegates to `PartialEq::eq` on the `via` type.

## `PartialOrd`
- **Without `via`**: Generates `impl PartialOrd for Type`, delegating the `partial_cmp` method to the underlying type.
- **With `via`**: Casts both sides to `&via`, then delegates to `PartialOrd::partial_cmp` on the `via` type.
