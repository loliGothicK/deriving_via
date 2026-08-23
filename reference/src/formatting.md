# Formatting

## `Display`
Derives the `std::fmt::Display` trait to provide user-facing string formatting (used by `{}`).

- **Without `via`**: Generates `impl Display for Type`. It delegates formatting to the `.fmt(f)` method of the underlying field.
  - **Requires**: The underlying type must implement `Display`.
- **With `via`**: Derives `Display` from the specified base type. The wrapper type is cast to `&via` and is formatted using the `Display` implementation of the `via` type.
  - **Requires**: The `via` type must implement `Display`.

### Example

```rust
{{#include ../../examples/src/display.rs:Display}}
```

## `Debug`
Derives the `std::fmt::Debug` trait to provide programmer-facing formatting (used by `{:?}`).

- **Without `via`**: Generates `impl Debug for Type`, which delegates the `fmt` call to `<Underlying as Debug>::fmt`.
  - **Requires**: The underlying type must implement `Debug`.
- **With `via`**: Generates `impl Debug for Type`, casting the reference to `&via` and calling its `fmt` method.
  - **Requires**: The `via` type must implement `Debug`.

### Example

```rust
{{#include ../../examples/src/debug.rs:Debug}}
```
