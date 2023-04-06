use proc_macro2::TokenStream;
use quote::quote;

use super::partial_eq;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let impl_eq = quote! { impl #impl_generics ::core::cmp::Eq for #struct_name #ty_generics #where_clause {} };

    [impl_eq, partial_eq::extract(input, via)]
        .into_iter()
        .collect()
}
