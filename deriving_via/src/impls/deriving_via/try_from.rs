use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::extract_single_field;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);

    let field_ident = &field.ident;
    let field_ty = &field.ty;

    via.map_or_else(
        || {
            field_ident.as_ref().map_or_else(
                || {
                    quote! {
                        impl std::convert::TryFrom<#field_ty> for #struct_name {
                            type Error = <#field_ty as std::str::TryFrom>::Error;

                            fn try_from(__: #field_ty) -> std::result::Result<Self, Self::Error> {
                                Ok(Self(__.try_into()?))
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl std::convert::TryFrom<#field_ty> for #struct_name {
                            type Error = <#field_ty as std::str::TryFrom>::Error;

                            fn try_from(__: #field_ty) -> std::result::Result<Self, Self::Error> {
                                Ok(Self { #field_name: __.try_into()? })
                            }
                        }
                    }
                },
            )
        },
        |via| {
            field_ident.as_ref().map_or_else(
                || {
                    quote! {
                        impl std::convert::TryFrom<#field_ty> for #struct_name {
                            type Error = <#via as std::str::TryFrom>::Error;

                            fn try_from(__: #field_ty) -> std::result::Result<Self, Self::Error> {
                                let intermediate: #via = __.try_into()?;
                                Ok(Self(intermediate.into()))
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl std::convert::TryFrom<#field_ty> for #struct_name {
                            type Error = <#via as std::str::TryFrom>::Error;

                            fn try_from(__: #field_ty) -> std::result::Result<Self, Self::Error> {
                                let intermediate: #via = __.try_into()?;
                                Ok(Self { #field_name: intermediate.into() })
                            }
                        }
                    }
                },
            )
        },
    )
}
