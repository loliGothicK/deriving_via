# Arithmetic Operations

When deriving arithmetic operations, the computed result generally needs to be converted back into the wrapper type. Therefore, `Self: From<UnderlyingOutput>` is typically required. When `via` is specified, the `#[transitive]` attribute may be required.

## `Add` (and `Sub`)
**Note:** Deriving `Add` generates implementations for both `Add` and `Sub`.
- **Without `via`**: Delegates `add` and `sub` to the underlying field. The result is converted back to the wrapper type using `Self::From`.
- **With `via`**: Both operands are cloned as the `via` type, and the addition/subtraction is performed. The result is converted to `Self` using `.into()`. Requires `Self: From<<via as Add>::Output>`.

## `Mul` (and `Div`)
**Note:** Deriving `Mul` generates implementations for both `Mul` and `Div`.
- **Without `via`**: Delegates `mul` and `div` to the underlying field. The result is returned after being converted via `Self::From`.
- **With `via`**: Both operands are cloned as the `via` type for multiplication/division. The result is converted to `Self` using `.into()`. Requires `Self: From<<via as Mul>::Output>`.

## `Arithmetic`
- A convenient derive option for generating `Add`, `Sub`, `Mul`, and `Div` simultaneously. It implicitly delegates to the `Add` and `Mul` macros.
