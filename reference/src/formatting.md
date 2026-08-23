# Formatting

## `Display`
- **Without `via`**: Generates `impl Display for Type`. It delegates formatting to the `.fmt(f)` method of the underlying field. The underlying type must implement `Display`.
- **With `via`**: Derives `Display` from the specified base type. The wrapper type is cast to `&via` and is formatted using the `Display` implementation of the `via` type.

## `Debug`
- **Without `via`**: Generates `impl Debug for Type`, delegating to the `Debug` implementation of the underlying field.
- **With `via`**: The wrapper type is cast to `&via`, and the resulting reference is formatted using the `Debug` implementation of the `via` type.
