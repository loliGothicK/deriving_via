use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (_, field_ty, constructor) = match extract_fields(input) {
        Ok(res) => res,
        Err(e) => return e,
    };

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics From<#field_ty> for #struct_name #ty_generics #where_clause {
                    fn from(__: #field_ty) -> Self {
                        #constructor(__)
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #impl_generics From<#via> for #struct_name #ty_generics #where_clause {
                    fn from(__: #via) -> Self {
                        __.into()
                    }
                }
            }
        },
    )
}
