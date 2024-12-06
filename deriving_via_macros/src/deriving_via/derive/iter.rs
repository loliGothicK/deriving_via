use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, filed_ty, _) = extract_fields(input);
    let collection_ty = via.unwrap_or(filed_ty);

    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            fn iter(&self) -> ::core::slice::Iter<'_, <#collection_ty as IntoIterator>::Item> {
                let slice: &[<#collection_ty as IntoIterator>::Item] = &self.#accessor;
                slice.iter()
            }
        }
    }
}
