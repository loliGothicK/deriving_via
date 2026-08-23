# Comparisons

## `Eq`
Derives the `std::cmp::Eq` marker trait, which indicates that the equality logic forms an equivalence relation.

- **Without `via`**: Generates `impl Eq for Type`.
  - **Requires**: The underlying type must implement `Eq`.
- **With `via`**: Generates `impl Eq for Type`. No custom logic is needed other than the marker trait itself, but this relies on `PartialEq` being implemented properly.
  - **Requires**: The `via` type must implement `Eq`.

### Example

```rust
{{#include ../../examples/src/eq.rs:Eq}}
```

## `Ord`
Derives the `std::cmp::Ord` trait to provide a total ordering between values.

- **Without `via`**: Generates `impl Ord for Type`.
  - **Requires**: The underlying type must implement `Ord`.
- **With `via`**: Generates `impl Ord for Type`.
  - **Requires**: The `via` type must implement `Ord`.

### Example

```rust
{{#include ../../examples/src/ord.rs:Ord}}
```

## `PartialEq`
Derives the `std::cmp::PartialEq` trait to provide partial equality comparisons (used by `==` and `!=`).

- **Without `via`**: Generates `impl PartialEq for Type`, delegating the `eq` method to the underlying type.
  - **Requires**: The underlying type must implement `PartialEq`.
- **With `via`**: Casts both sides to `&via`, then delegates to `PartialEq::eq` on the `via` type.
  - **Requires**: The `via` type must implement `PartialEq`.

### Example

```rust
{{#include ../../examples/src/partialeq.rs:PartialEq}}
```

## `PartialOrd`
Derives the `std::cmp::PartialOrd` trait to provide partial ordering comparisons (used by `<`, `>`, `<=`, `>=`).

- **Without `via`**: Generates `impl PartialOrd for Type`, delegating `partial_cmp` to the underlying type.
  - **Requires**: The underlying type must implement `PartialOrd`.
- **With `via`**: Casts both sides to `&via` and delegates to `PartialOrd::partial_cmp` on the `via` type.
  - **Requires**: The `via` type must implement `PartialOrd`.

### Example

```rust
{{#include ../../examples/src/partialord.rs:PartialOrd}}
```
