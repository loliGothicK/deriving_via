use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();
    let impl_generics = &input.generics.params;
    let impl_generics = quote! { <__IdxT, #impl_generics> };
    let predicates = where_clause.map(|wc| &wc.predicates);
    let (accessor, field_ty, _) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics ::core::ops::IndexMut<__IdxT> for #struct_name #ty_generics
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
                impl #impl_generics ::core::ops::IndexMut<__IdxT> for #struct_name #ty_generics
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
