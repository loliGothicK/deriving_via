# Serde

## `Serialize`
Derives the `serde::Serialize` trait to allow the type to be serialized into a data format (like JSON).

- **Without `via`**: Generates `impl Serialize for Type`, delegating to the underlying field.
  - **Requires**: The underlying type must implement `Serialize`.
- **With `via`**: Casts `&self` to `&via`, then serializes the `via` type.
  - **Requires**: The `via` type must implement `Serialize`.

### Example

```rust
{{#include ../../examples/src/serialize.rs:Serialize}}
```

## `Deserialize`
Derives the `serde::Deserialize` trait to allow the type to be deserialized from a data format (like JSON).

- **Without `via`**: Generates `impl<'de> Deserialize<'de> for Type`, which delegates to the underlying type's `Deserialize` implementation and then constructs the wrapper.
  - **Requires**: The underlying type must implement `Deserialize<'de>`.
- **With `via`**: Generates `impl<'de> Deserialize<'de> for Type`, which delegates to the `Deserialize` implementation of the `via` type and then constructs the wrapper using `Self::from`.
  - **Requires**: The `via` type must implement `Deserialize<'de>`, and `Self` must implement `From<via>`.

### Example

```rust
{{#include ../../examples/src/deserialize.rs:Deserialize}}
```
