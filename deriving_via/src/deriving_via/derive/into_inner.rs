use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, filed_ty, _) = extract_fields(input);
    let inner = via.unwrap_or(filed_ty);

    quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            fn into_inner(self) -> #inner
                where #inner: ::core::fmt::Debug
            {
                let inner: &#inner = &self.#accessor;
                inner.to_owned()
            }
        }
    }
}
