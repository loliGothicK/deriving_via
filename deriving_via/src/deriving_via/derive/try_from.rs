use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (_, field_ty, constructor) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics std::convert::TryFrom<#field_ty> for #struct_name #ty_generics #where_clause {
                    type Error = <#field_ty as std::str::TryFrom>::Error;

                    fn try_from(__: #field_ty) -> std::result::Result<Self, Self::Error> {
                        Ok(#constructor(__.try_into()?))
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #impl_generics std::convert::TryFrom<#field_ty> for #struct_name #ty_generics #where_clause {
                    type Error = <#via as std::str::TryFrom>::Error;

                    fn try_from(__: #field_ty) -> std::result::Result<Self, Self::Error> {
                        let intermediate: #via = __.try_into()?;
                        Ok(intermediate.into())
                    }
                }
            }
        },
    )
}
