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
                        impl From<#field_ty> for #struct_name {
                            fn from(__: #field_ty) -> Self {
                                Self(__)
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl From<#field_ty> for #struct_name {
                            fn from(#field_name: #field_ty) -> Self {
                                Self{ #field_name }
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
                        impl From<#via> for #struct_name {
                            fn from(__: #via) -> Self {
                                Self(__.into())
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl From<#via> for #struct_name {
                            fn from(__: #via) -> Self {
                                Self{ #field_name: __.into() }
                            }
                        }
                    }
                },
            )
        },
    )
}
