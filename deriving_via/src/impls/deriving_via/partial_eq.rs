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
                        impl std::cmp::PartialEq for #struct_name {
                            fn eq(&self, other: &Self) -> bool {
                                self.0.eq(&other.0)
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl std::cmp::PartialEq for #struct_name {
                            fn eq(&self, other: &Self) -> bool {
                                self.#field_name.eq(&other.#field_name)
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl std::cmp::PartialEq for #struct_name {
                    fn eq(&self, other: &Self) -> bool {
                        let left: &#via = &*self;
                        let right: &#via = &*other;
                        left.eq(right)
                    }
                }
            }
        },
    )
}
