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
                        impl std::hash::Hash for #struct_name {
                            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                                self.0.hash(state);
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl std::hash::Hash for #struct_name {
                            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                                self.#field_name.hash(state);
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl std::hash::Hash for #struct_name {
                    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                        let de: &#via = &*self;
                        de.hash(state);
                    }
                }
            }
        },
    )
}
