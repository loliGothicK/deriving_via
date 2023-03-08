use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use super::partial_eq;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let generics = {
        let lt = &input.generics.lt_token;
        let params = &input.generics.params;
        let gt = &input.generics.gt_token;

        quote! { #lt #params #gt }
    };
    let generic_params = {
        let lt = &input.generics.lt_token;
        let params = input.generics.params.iter().filter_map(|p| match p {
            GenericParam::Type(ty) => Some(&ty.ident),
            _ => None,
        });
        let gt = &input.generics.gt_token;

        quote! { #lt #(#params),* #gt }
    };
    let where_clause = &input.generics.where_clause;

    let impl_eq =
        quote! { impl #generics ::core::cmp::Eq for #struct_name #generic_params #where_clause {} };

    [impl_eq, partial_eq::extract(input, via)]
        .into_iter()
        .collect()
}
