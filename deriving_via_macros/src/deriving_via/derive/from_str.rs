use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (_, field_ty, constructor) = extract_fields(input);

    match via.as_ref().unwrap_or(&field_ty) {
        syn::Type::Path(path) if path.path.is_ident("String") => {
            quote! {
                impl #impl_generics ::core::str::FromStr for #struct_name #ty_generics
                    #where_clause
                {
                    type Err = ::core::convert::Infallible;

                    fn from_str(__: &str) -> ::core::result::Result<Self, Self::Err> {
                        Ok(#constructor(__.to_owned()))
                    }
                }
            }
        }
        ty => {
            quote! {
                impl #impl_generics ::core::str::FromStr for #struct_name #ty_generics
                    #where_clause
                {
                    type Err = <#ty as ::core::str::FromStr>::Err;

                    fn from_str(__: &str) -> ::core::result::Result<Self, Self::Err> {
                        let intermediate: #ty = __.parse()?;
                        Ok(intermediate.into())
                    }
                }
            }
        }
    }
}
