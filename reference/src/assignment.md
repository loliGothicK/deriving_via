# Assignment Operations

## `AddAssign`
Derives the `std::ops::AddAssign` trait to allow using the `+=` operator.

- **Without `via`**: Delegates `add_assign` to the underlying type.
  - **Requires**: The underlying type must implement `AddAssign`.
- **With `via`**: Casts to `&mut via` and performs the assignment operation on the `via` type.
  - **Requires**: The `via` type must implement `AddAssign` and `Clone`.

### Example

```rust
{{#include ../../examples/src/addassign.rs:AddAssign}}
```

## `SubAssign`
Derives the `std::ops::SubAssign` trait to allow using the `-=` operator.

- **Without `via`**: Delegates `sub_assign` to the underlying type.
  - **Requires**: The underlying type must implement `SubAssign`.
- **With `via`**: Casts to `&mut via` and performs the assignment operation on the `via` type.
  - **Requires**: The `via` type must implement `SubAssign` and `Clone`.

### Example

```rust
{{#include ../../examples/src/addassign.rs:AddAssign}}
```

## `MulAssign`
Derives the `std::ops::MulAssign` trait to allow using the `*=` operator.

- **Without `via`**: Generates `impl MulAssign for Type`, delegating to `<Underlying as MulAssign>::mul_assign`.
  - **Requires**: The underlying type must implement `MulAssign`.
- **With `via`**: Performs `MulAssign` on the `via` type.
  - **Requires**: The `via` type must implement `MulAssign` and `Clone`. (Note: due to an internal macro quirk, it may incorrectly require `DivAssign` to be in scope).

### Example

```rust
{{#include ../../examples/src/mulassign.rs:MulAssign}}
```

## `DivAssign`
Derives the `std::ops::DivAssign` trait to allow using the `/=` operator.

- **Without `via`**: Generates `impl DivAssign for Type`, delegating to `<Underlying as DivAssign>::div_assign`.
  - **Requires**: The underlying type must implement `DivAssign`.
- **With `via`**: Performs `DivAssign` on the `via` type.
  - **Requires**: The `via` type must implement `DivAssign` and `Clone`.

### Example

```rust
{{#include ../../examples/src/mulassign.rs:MulAssign}}
```
