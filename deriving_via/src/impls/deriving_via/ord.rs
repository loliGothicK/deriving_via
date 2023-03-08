use proc_macro2::TokenStream;
use quote::quote;

use crate::{impls::deriving_via::partial_ord, utils::extract_single_field};

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    [impl_ord(input, via), partial_ord::extract(input, via)]
        .into_iter()
        .collect()
}

fn impl_ord(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl Ord for #struct_name {
                            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                                self.0.cmp(&other.0)
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl Ord for #struct_name {
                            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                                self.#field_name.cmp(&other.#field_name)
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl Ord for #struct_name {
                    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                        type De = <#via as std::ops::Deref>::Target;
                        let left: &De = &*self;
                        let right: &De = &*other;
                        left.cmp(right)
                    }
                }
            }
        },
    )
}
