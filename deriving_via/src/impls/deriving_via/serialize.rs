use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::extract_single_field;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl serde::Serialize for #struct_name {
                            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                                self.0.serialize(serializer)
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl serde::Serialize for #struct_name {
                            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                                self.#field_name.serialize(serializer)
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl serde::Serialize for #struct_name {
                    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                        let de: &#via = &*self;
                        de.serialize(serializer)
                    }
                }
            }
        },
    )
}
