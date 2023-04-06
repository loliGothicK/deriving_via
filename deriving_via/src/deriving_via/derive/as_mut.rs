use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();
    let impl_generics = &input.generics.params;
    let impl_generics = quote! { <__AsMutT: ?::core::marker::Sized, #impl_generics> };
    let predicates = where_clause.map(|wc| &wc.predicates);
    let (accessor, field_ty, _) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics ::core::convert::AsMut<__AsMutT> for #struct_name #ty_generics
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
                impl #impl_generics ::core::convert::AsMut<__AsMutT> for #struct_name #ty_generics
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
