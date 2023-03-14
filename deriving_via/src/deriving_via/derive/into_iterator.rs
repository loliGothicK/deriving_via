use proc_macro2::TokenStream;
use proc_macro_error::abort;
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
    let generic_params = &input.generics.params;
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
    let (accessor, filed_ty, _constructor) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #generics ::core::iter::IntoIterator for #struct_name #generic_types
                    #where_clause
                {
                    type Item = <#filed_ty as ::core::iter::IntoIterator>::Item;
                    type IntoIter = <#filed_ty as ::core::iter::IntoIterator>::IntoIter;
                    #[inline]
                    fn into_iter(self) -> Self::IntoIter {
                        <#filed_ty as ::core::iter::IntoIterator>::into_iter(self.#accessor)
                    }
                }

                impl<'__derivingViaLifetime, #generic_params> ::core::iter::IntoIterator for &'__derivingViaLifetime #struct_name #generic_types
                    #where_clause
                {
                    type Item = <&'__derivingViaLifetime #filed_ty as ::core::iter::IntoIterator>::Item;
                    type IntoIter = <&'__derivingViaLifetime #filed_ty as ::core::iter::IntoIterator>::IntoIter;
                    #[inline]
                    fn into_iter(self) -> Self::IntoIter {
                        <&'__derivingViaLifetime #filed_ty as ::core::iter::IntoIterator>::into_iter(&self.#accessor)
                    }
                }

                impl<'__derivingViaLifetime, #generic_params> ::core::iter::IntoIterator for &'__derivingViaLifetime mut #struct_name #generic_types
                    #where_clause
                {
                    type Item = <&'__derivingViaLifetime mut #filed_ty as ::core::iter::IntoIterator>::Item;
                    type IntoIter = <&'__derivingViaLifetime mut #filed_ty as ::core::iter::IntoIterator>::IntoIter;
                    #[inline]
                    fn into_iter(self) -> Self::IntoIter {
                        <&'__derivingViaLifetime mut #filed_ty as ::core::iter::IntoIterator>::into_iter(
                            &mut self.#accessor,
                        )
                    }
                }
            }
        },
        |_| {
            abort!(
                input,
                "#[deriving(IntoIterator(via))] is not allowed";
                help = "Please try #[deriving(Iter(via: <ItemType>)])";
            );
        },
    )
}
