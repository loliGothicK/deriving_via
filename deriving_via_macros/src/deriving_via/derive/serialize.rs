use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, ..) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics serde::Serialize for #struct_name #ty_generics #where_clause {
                    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                        self.#accessor.serialize(serializer)
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #impl_generics serde::Serialize for #struct_name #ty_generics #where_clause {
                    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                        let de: &#via = self;
                        de.serialize(serializer)
                    }
                }
            }
        },
    )
}
