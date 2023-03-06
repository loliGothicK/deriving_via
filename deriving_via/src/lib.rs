mod impls;
mod utils;

extern crate proc_macro;
#[allow(unused)]
use proc_macro::TokenStream;

#[proc_macro_derive(DerivingVia, attributes(deriving))]
pub fn derive_generalised_newtype_deriving(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impls::deriving_via::impl_generalised_newtype_deriving(&ast).into()
}
