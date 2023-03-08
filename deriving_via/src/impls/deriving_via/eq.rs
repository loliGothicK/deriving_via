use proc_macro2::TokenStream;
use quote::quote;

use super::partial_eq;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let impl_eq = quote! { impl ::core::cmp::Eq for #struct_name {} };

    [impl_eq, partial_eq::extract(input, via)]
        .into_iter()
        .collect()
}
