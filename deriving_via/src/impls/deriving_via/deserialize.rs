use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::extract_single_field;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field_ty = &field.ty;
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl<'de> serde::Deserialize<'de> for #struct_name {
                            fn deserialize<D>(deserializer: D) -> Result<#struct_name, D::Error>
                            where
                                D: Deserializer<'de>,
                            {
                                Ok(#struct_name(field_ty::deserialize(deserializer)?.into()))
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl<'de> serde::Deserialize<'de> for #struct_name {
                            fn deserialize<D>(deserializer: D) -> Result<#struct_name, D::Error>
                            where
                                D: Deserializer<'de>,
                            {
                                Ok(#struct_name {
                                    #field_name: #field_ty::deserialize(deserializer)?.into()
                                })
                            }
                        }
                    }
                },
            )
        },
        |via| {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl<'de> serde::Deserialize<'de> for #struct_name {
                            fn deserialize<D>(deserializer: D) -> Result<#struct_name, D::Error>
                            where
                                D: Deserializer<'de>,
                            {
                                Ok(#struct_name(#via::deserialize(deserializer)?.into()))
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl<'de> serde::Deserialize<'de> for #struct_name {
                            fn deserialize<D>(deserializer: D) -> Result<#struct_name, D::Error>
                            where
                                D: Deserializer<'de>,
                            {
                                Ok(#struct_name {
                                    #field_name: #via::deserialize(deserializer)?.into()
                                })
                            }
                        }
                    }
                },
            )
        },
    )
}
