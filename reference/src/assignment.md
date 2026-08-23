# Assignment Operations

## `AddAssign` (and `SubAssign`)
- **Without `via`**: Delegates `add_assign` and `sub_assign` to the underlying type.
- **With `via`**: Casts to `&mut via` and performs the assignment operation on the `via` type.

## `MulAssign` (and `DivAssign`)
- **Without `via`**: Delegates `mul_assign` and `div_assign` to the underlying type.
- **With `via`**: Casts to `&mut via` and performs the assignment operation on the `via` type.
