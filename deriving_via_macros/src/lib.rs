//! # deriving_via_macros
//!
//! This library provides the `DerivingVia` derive macro for _Newtypes_ (Single-Field structs).
//! `DerivingVia` can be used like [_derive_more_](https://docs.rs/derive_more/latest/derive_more/).
//!
//! ## Generalised Newtype Deriving
//!
//! ### Example
//!
//! ```
//! use deriving_via_macros::DerivingVia;
//!
//! #[derive(DerivingVia)]
//! #[deriving(Display)]
//! struct Newtype(pub i32);
//!
//! let x = Newtype(0);
//!
//! println!("{x}");
//! ```
//!
//! ## Deriving Via
//!
//! Deriving Via allows deriving from beyond a multiply wrapped hierarchy
//! using transitive type conversion through `Deref`, `Into` or `From` traits.
//!
//! ### Example
//!
//! ```
//! use deriving_via_macros::DerivingVia;
//!
//! #[derive(DerivingVia)]
//! pub struct Inner(i32);
//!
//! #[derive(DerivingVia)]
//! #[deriving(Display(via: i32))]
//! pub struct Outer(Inner);
//!
//! let x = Outer(Inner(42));
//!
//! println!("{x}");
//! ```

mod deriving_via;

extern crate proc_macro;
#[allow(unused)]
use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;

#[proc_macro_error]
#[proc_macro_derive(DerivingVia, attributes(deriving, transitive, underlying))]
pub fn deriving_via(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    deriving_via::impl_deriving_via(&ast).into()
}
