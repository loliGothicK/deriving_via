use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let generics = {
        let lt = &input.generics.lt_token;
        let params = &input.generics.params;
        let gt = &input.generics.gt_token;

        quote! { #lt #params #gt }
    };
    let generic_types = {
        let lt = &input.generics.lt_token;
        let params = input.generics.params.iter().filter_map(|p| match p {
            GenericParam::Type(ty) => Some(&ty.ident),
            _ => None,
        });
        let gt = &input.generics.gt_token;

        quote! { #lt #(#params),* #gt }
    };
    let where_clause = &input.generics.where_clause;
    let (accessor, filed_ty, _) = extract_fields(input);

    let collection_ty = via.unwrap_or(filed_ty);
    quote! {
        impl #generics #struct_name #generic_types
            #where_clause
        {
            fn iter(&self) -> ::core::slice::Iter<'_, <#collection_ty as IntoIterator>::Item> {
                let hoge: &[<#collection_ty as IntoIterator>::Item] = &self.#accessor;
                hoge.iter()
            }
        }
    }
}
