# Other

## `Clone`
- **Without `via`**: Generates `impl Clone for Type`. Clones the underlying field and wraps it.
- **With `via`**: Clones as the `via` type, then uses `.into()` to convert back to `Self`.

## `Copy`
- Generates `impl Copy for Type` and `impl Clone for Type` (using the same logic as `Clone` above).

## `Default`
- **Without `via`**: Generates `impl Default for Type`. Uses `Underlying::default().into()` to construct the instance.
- **With `via`**: Uses `via::default().into()` to construct the instance.

## `Deref`
- Generates `impl Deref for Type` where `Target = Underlying`.
- **Note:** The `via` attribute is not supported here.

## `DerefMut`
- Generates `impl DerefMut for Type`.
- **Note:** The `via` attribute is not supported here.

## `IntoInner`
- **Without `via`**: Generates an inherent method `pub fn into_inner(self) -> Underlying` which returns a clone of the underlying type.
- **With `via`**: Generates an inherent method `pub fn into_inner(self) -> via` returning a clone of the underlying type cast to `&via`. The underlying (or `via`) type must implement `Debug` (due to the macro's internal requirements) and `Clone`.
