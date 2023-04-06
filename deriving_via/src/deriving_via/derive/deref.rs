use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (accessor, ty, _) = extract_fields(input);

    quote! {
        impl #impl_generics ::core::ops::Deref for #struct_name #ty_generics #where_clause {
            type Target = #ty;

            fn deref(&self) -> &Self::Target {
                &self.#accessor
            }
        }
    }
}
