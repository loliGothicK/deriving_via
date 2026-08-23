# Serde

## `Serialize`
- **Without `via`**: Generates `impl Serialize for Type`, delegating to the underlying field.
- **With `via`**: Casts `&self` to `&via`, then serializes the `via` type.

## `Deserialize`
- **Without `via`**: Generates `impl Deserialize<'de> for Type`. It deserializes into the underlying type and wraps it.
- **With `via`**: Deserializes into the `via` type, then uses `.into()` to convert it to `Self`. Requires `Self: From<via>`.
