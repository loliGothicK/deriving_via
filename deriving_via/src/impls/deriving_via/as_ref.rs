use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use crate::utils::extract_fields;

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
    let where_clause = input.generics.where_clause.as_ref();
    let predicates = where_clause.map(|wc| &wc.predicates);
    let (accessor, field_ty, _) = extract_fields(input);

    via.map_or_else(
        || {
            quote! {
                impl<__AsRefT: ?::core::marker::Sized, #generic_params> ::core::convert::AsRef<__AsRefT> for #struct_name #generic_params
                where
                    #field_ty: ::core::convert::AsRef<__AsRefT>,
                    #predicates
                {
                    #[inline]
                    fn as_ref(&self) -> &__AsRefT {
                        <#field_ty as ::core::convert::AsRef<__AsRefT>>::as_ref(&self.#accessor)
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #generics ::core::convert::AsRef<#via> for #struct_name #generic_params
                    #where_clause
                {
                    #[inline]
                    fn as_ref(&self) -> &#via {
                        &self.#accessor
                    }
                }
            }
        },
    )
}
