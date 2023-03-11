use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let _generics = {
        let lt = &input.generics.lt_token;
        let params = &input.generics.params;
        let gt = &input.generics.gt_token;

        quote! { #lt #params #gt }
    };
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
                impl<__AsMutT: ?::core::marker::Sized, #generic_types> ::core::convert::AsMut<__AsMutT> for #struct_name #generic_params
                    where
                        #field_ty: ::core::convert::AsMut<__AsMutT>,
                        #predicates
                {
                    #[inline]
                    fn as_mut(&mut self) -> &mut __AsMutT {
                        <#field_ty as ::core::convert::AsMut<__AsMutT>>::as_mut(&mut self.#accessor)
                    }
                }
            }
        },
        |via| {
            quote! {
                impl<__AsMutT: ?::core::marker::Sized, #generic_types> ::core::convert::AsMut<__AsMutT> for #struct_name #generic_params
                    where
                        #via: ::core::convert::AsMut<__AsMutT>,
                        #predicates
                {
                    #[inline]
                    fn as_mut(&mut self) -> &mut __AsMutT {
                        <#via as ::core::convert::AsMut<__AsMutT>>::as_mut(&mut self.#accessor)
                    }
                }
            }
        },
    )
}
