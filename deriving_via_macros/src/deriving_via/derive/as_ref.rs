use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let generics_introducer = &input.generics.params;
    let generics_introducer = quote! { <__AsRefT: ?::core::marker::Sized, #generics_introducer> };
    let predicates = where_clause.map(|wc| &wc.predicates);
    let (accessor, field_ty, _) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #generics_introducer ::core::convert::AsRef<__AsRefT> for #struct_name #ty_generics
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
                impl #impl_generics ::core::convert::AsRef<#via> for #struct_name #ty_generics
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
