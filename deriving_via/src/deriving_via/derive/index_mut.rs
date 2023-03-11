use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (generic_params, generic_types) = {
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
        (quote! { #lt #(#params),* #gt }, quote! { #(#params),* })
    };
    let where_clause = input.generics.where_clause.as_ref();
    let predicates = where_clause.map(|wc| &wc.predicates);
    let (accessor, field_ty, _) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl<__IdxT, #generic_types> ::core::ops::IndexMut<__IdxT> for #struct_name #generic_params
                    where
                        #field_ty: ::core::ops::IndexMut<__IdxT>,
                        #predicates
                {
                    #[inline]
                    fn index_mut(&mut self, idx: __IdxT) -> &mut Self::Output {
                        <#field_ty as ::core::ops::IndexMut<__IdxT>>::index_mut(&mut self.#accessor, idx)
                    }
                }
            }
        },
        |via| {
            quote! {
                impl<__IdxT, #generic_types> ::core::ops::IndexMut<__IdxT> for #struct_name #generic_params
                    where
                        #via: ::core::ops::IndexMut<__IdxT>,
                        #predicates
                {
                    #[inline]
                    fn index_mut(&mut self, idx: __IdxT) -> &mut Self::Output {
                        let mut de: &mut #via = &mut self;
                        <#via as ::core::ops::IndexMut<__IdxT>>::index_mut(&mut de, idx)
                    }
                }
            }
        },
    )
}
