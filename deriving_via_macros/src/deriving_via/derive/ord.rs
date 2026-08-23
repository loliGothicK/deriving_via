use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    if let Err(e) = extract_fields(input) {
        return e;
    }

    [
        impl_ord(input, via.as_ref()),
        super::partial_ord::extract(input, via),
    ]
    .into_iter()
    .collect()
}

fn impl_ord(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, ..) = match extract_fields(input) {
        Ok(res) => res,
        Err(e) => return e,
    };

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics Ord for #struct_name #ty_generics #where_clause {
                    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                        self.#accessor.cmp(&other.#accessor)
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #impl_generics Ord for #struct_name #ty_generics #where_clause{
                    fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                        let left: &#via = self;
                        let right: &#via = other;
                        left.cmp(right)
                    }
                }
            }
        },
    )
}
