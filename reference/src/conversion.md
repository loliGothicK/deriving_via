# Conversion

## `AsRef`
- **Without `via`**: Generates a blanket `impl<__AsRefT> AsRef<__AsRefT> for Type`. The underlying type must implement `AsRef<__AsRefT>`.
- **With `via`**: Generates `impl AsRef<via> for Type`, which casts the reference to the underlying field into `&via` and returns it.

## `AsMut`
- **Without `via`**: Generates a blanket `impl<__AsMutT> AsMut<__AsMutT> for Type`. The underlying type must implement `AsMut<__AsMutT>`.
- **With `via`**: Generates a blanket `impl<__AsMutT> AsMut<__AsMutT> for Type`. The `via` type must implement `AsMut<__AsMutT>`. The implementation delegates to `<via as AsMut<__AsMutT>>::as_mut(&mut self.0)`.

## `From`
- **Without `via`**: Generates `impl From<Underlying> for Type`, wrapping the underlying type.
- **With `via`**: Generates `impl From<via> for Type`, calling `__.into()` to convert from `via` to `Self`. The `#[transitive]` attribute is typically needed if crossing multiple layers.

## `Into`
- **Without `via`**: Generates `impl From<Type> for Underlying`. (Note: this implicitly provides `Into<Underlying>` for `Type` via the standard library's blanket impl).
- **With `via`**: Generates `impl From<Type> for via` by casting `&Type` to `&via` and cloning it.

## `TryFrom`
- **Without `via`**: Generates `impl TryFrom<Underlying> for Type`.
- **With `via`**: Generates `impl TryFrom<via> for Type`. This delegates to `<Self as From<via>>::from` or utilizes transitive logic.

## `FromStr`
- **Without `via`**: Generates `impl FromStr for Type`. It parses a string into the underlying type and wraps it. Requires `Self: From<Underlying>`.
- **With `via`**: Parses a string into the `via` type, then calls `.into()` to convert to `Self`. Requires `Self: From<via>`.
