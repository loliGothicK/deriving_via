use proc_macro2::TokenStream;
use quote::quote;

use crate::deriving_via::utils::extract_fields;

pub(crate) fn extract(fmt: syn::LitStr, input: &syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, ..) = extract_fields(input);

    quote! {
        impl #impl_generics ::core::fmt::Debug for #struct_name #ty_generics
            #where_clause
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let field = &self. #accessor;
                write!(f, #fmt, field)
            }
        }
    }
}
