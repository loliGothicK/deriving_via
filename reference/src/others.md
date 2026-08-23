# Others

## `Clone`
Derives the `std::clone::Clone` trait to allow creating a deep or explicit copy of the type.

- **Without `via`**: Generates `impl Clone for Type`. Clones the underlying field and wraps it.
  - **Requires**: The underlying type must implement `Clone`.
- **With `via`**: Clones as the `via` type, then uses `.into()` to convert back to `Self`.
  - **Requires**: The `via` type must implement `Clone`, and `Self` must implement `From<via>`.

### Example

```rust
{{#include ../../examples/src/clone.rs:Clone}}
```

## `Copy`
Derives the `std::marker::Copy` trait to indicate that values can be duplicated simply by copying bits.

- **Without `via`**: Generates `impl Copy for Type`. 
  - **Requires**: The underlying type must implement `Copy`.
- **Note:** The `via` attribute is not supported here.

### Example

```rust
{{#include ../../examples/src/copy.rs:Copy}}
```

## `Default`
Derives the `std::default::Default` trait to provide a default value for the type.

- **Without `via`**: Generates `impl Default for Type`, returning `Type(Underlying::default())`.
  - **Requires**: The underlying type must implement `Default`.
- **With `via`**: Generates `impl Default for Type`, returning `Type(via::default().into())`.
  - **Requires**: The `via` type must implement `Default`, and `Self` must implement `From<via>`.

### Example

```rust
{{#include ../../examples/src/default.rs:Default}}
```

## `Deref`
Derives the `std::ops::Deref` trait, implicitly derived by the macro to allow the newtype to be treated as a reference to its underlying type.

- **Without `via`**: Generates `impl Deref for Type`.
  - **Requires**: None (works unconditionally).
- **Note:** The `via` attribute is not supported here.

### Example

```rust
{{#include ../../examples/src/deref.rs:Deref}}
```

## `IntoInner`
Generates an inherent method `into_inner` to extract or unwrap the inner value.

- **Without `via`**: Generates an inherent method `pub fn into_inner(self) -> Underlying` which returns a clone of the underlying type.
  - **Requires**: The underlying type must implement `Clone`.
- **With `via`**: Generates an inherent method `pub fn into_inner(self) -> via` returning a clone of the underlying type cast to `&via`.
  - **Requires**: The underlying (or `via`) type must implement `Debug` (due to the macro's internal requirements) and `Clone`.

### Example

```rust
{{#include ../../examples/src/intoinner.rs:IntoInner}}
```
