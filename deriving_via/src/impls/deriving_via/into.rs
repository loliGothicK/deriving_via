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
                        impl From<#struct_name> for #field_ty {
                            fn from(__: #struct_name) -> Self {
                                __.0
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl From<#struct_name> for #field_ty {
                            fn from(__: #struct_name) -> Self {
                                __. #field_name
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl From<#struct_name> for #via {
                    fn from(__: #struct_name) -> Self {
                        let de: &#via = &*__;
                        de.to_owned()
                    }
                }
            }
        },
    )
}
