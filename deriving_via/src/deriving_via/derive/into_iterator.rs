use proc_macro2::TokenStream;
use proc_macro_error2::abort;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let generics_introducer = &input.generics.params;
    let generics_introducer = quote! { <'__derivingViaLifetime, #generics_introducer> };
    let (accessor, filed_ty, _constructor) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics ::core::iter::IntoIterator for #struct_name #ty_generics
                    #where_clause
                {
                    type Item = <#filed_ty as ::core::iter::IntoIterator>::Item;
                    type IntoIter = <#filed_ty as ::core::iter::IntoIterator>::IntoIter;
                    #[inline]
                    fn into_iter(self) -> Self::IntoIter {
                        <#filed_ty as ::core::iter::IntoIterator>::into_iter(self.#accessor)
                    }
                }

                impl #generics_introducer ::core::iter::IntoIterator for &'__derivingViaLifetime #struct_name #ty_generics
                    #where_clause
                {
                    type Item = <&'__derivingViaLifetime #filed_ty as ::core::iter::IntoIterator>::Item;
                    type IntoIter = <&'__derivingViaLifetime #filed_ty as ::core::iter::IntoIterator>::IntoIter;
                    #[inline]
                    fn into_iter(self) -> Self::IntoIter {
                        <&'__derivingViaLifetime #filed_ty as ::core::iter::IntoIterator>::into_iter(&self.#accessor)
                    }
                }

                impl #generics_introducer ::core::iter::IntoIterator for &'__derivingViaLifetime mut #struct_name #ty_generics
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
