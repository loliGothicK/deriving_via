# Arithmetic Operations

When deriving arithmetic operations, the computed result generally needs to be converted back into the wrapper type. Therefore, `Self: From<UnderlyingOutput>` is typically required. When `via` is specified, the `#[transitive]` attribute may be required.

## `Add`
Derives the `std::ops::Add` trait to allow using the `+` operator.

- **Without `via`**: Delegates `add` to the underlying field. The result is converted back to the wrapper type using `Self::From`.
  - **Requires**: The underlying type must implement `Add`, and `Self` must implement `From<<Underlying as Add>::Output>`.
- **With `via`**: Both operands are cloned as the `via` type, and the addition is performed. The result is converted to `Self` using `.into()`.
  - **Requires**: The `via` type must implement `Add` and `Clone`, and `Self` must implement `From<<via as Add>::Output>`.

### Example

```rust
{{#include ../../examples/src/add.rs:Add}}
```

## `Sub`
Derives the `std::ops::Sub` trait to allow using the `-` operator.

- **Without `via`**: Generates `impl Sub for Type`, delegating to `<Underlying as Sub>::sub`.
  - **Requires**: The underlying type must implement `Sub`, and `Self` must implement `From<<Underlying as Sub>::Output>`.
- **Note:** The `via` attribute is not supported here.

### Example

```rust
{{#include ../../examples/src/arithmetic.rs:Arithmetic}}
```

## `Mul`
Derives the `std::ops::Mul` trait to allow using the `*` operator.

- **Without `via`**: Generates `impl Mul for Type`, delegating to `<Underlying as Mul>::mul`.
  - **Requires**: The underlying type must implement `Mul`, and `Self` must implement `From<<Underlying as Mul>::Output>`.
- **With `via`**: Casts to `&via`, performs `Mul` on the `via` type, and converts back to `Self` using `From`.
  - **Requires**: The `via` type must implement `Mul` and `Clone`, and `Self` must implement `From<<via as Mul>::Output>`.

### Example

```rust
{{#include ../../examples/src/mul.rs:Mul}}
```

## `Div`
Derives the `std::ops::Div` trait to allow using the `/` operator.

- **Without `via`**: Generates `impl Div for Type`.
  - **Requires**: The underlying type must implement `Div`, and `Self` must implement `From<<Underlying as Div>::Output>`.
- **Note:** The `via` attribute is not supported here.

### Example

```rust
{{#include ../../examples/src/arithmetic.rs:Arithmetic}}
```

## `Arithmetic`
A convenience macro that derives `Add`, `Sub`, `Mul`, and `Div` simultaneously.

- **Without `via`**: Derives `Add`, `Sub`, `Mul`, and `Div` simultaneously using the underlying type.
  - **Requires**: The underlying type must implement `Add`, `Sub`, `Mul`, and `Div`. `Self` must implement `From` for all of their outputs.
- **With `via`**: Derives all four traits using the `via` type.
  - **Requires**: The `via` type must implement `Add`, `Sub`, `Mul`, `Div`, and `Clone`. `Self` must implement `From` for all of their outputs.

### Example

```rust
{{#include ../../examples/src/arithmetic.rs:Arithmetic}}
```
