use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (_, _, constructor) = match extract_fields(input) {
        Ok(res) => res,
        Err(e) => return e,
    };

    if let Some(via) = via {
        quote! {
            impl #impl_generics ::core::iter::FromIterator<#via> for #struct_name #ty_generics
                #where_clause
            {
                fn from_iter<I: ::core::iter::IntoIterator<Item=#via>>(iter: I) -> Self {
                    #constructor(iter.into_iter().collect())
                }
            }
        }
    } else {
        syn::Error::new_spanned(
            input,
            "#[deriving(FromIterator)] is not allowed: Specify via as \
             #[deriving(FromIterator(via: <ItemType>))]",
        )
        .to_compile_error()
    }
}
