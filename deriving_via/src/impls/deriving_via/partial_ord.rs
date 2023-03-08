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
                        impl PartialOrd for #struct_name {
                            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                                self.0.partial_cmp(&other.0)
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl PartialOrd for #struct_name {
                            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                                self.#field_name.partial_cmp(&other.#field_name)
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl PartialOrd for #struct_name {
                    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                        let left: &#via = &*self;
                        let right: &#via = &*other;
                        left.partial_cmp(right)
                    }
                }
            }
        },
    )
}
