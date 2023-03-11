use itertools::Itertools;
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
    let generic_params = {
        let lt = &input.generics.lt_token;
        let params = input
            .generics
            .params
            .iter()
            .filter_map(|p| match p {
                GenericParam::Type(ty) => Some(&ty.ident),
                _ => None,
            })
            .collect_vec();
        let gt = &input.generics.gt_token;

        let params = &params[..];
        quote! { #lt #(#params),* #gt }
    };
    let where_clause = input.generics.where_clause.as_ref();
    let (accessor, _field_ty, _) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #generics ::core::ops::DerefMut for #struct_name #generic_params
                    #where_clause
                {
                    #[inline]
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.#accessor
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #generics ::core::ops::DerefMut for #struct_name #generic_params
                    #where_clause
                {
                    #[inline]
                    fn deref_mut(&mut self) -> &mut #via {
                        self
                    }
                }
            }
        },
    )
}
