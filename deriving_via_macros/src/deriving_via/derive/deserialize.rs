use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (_, ty_generics, where_clause) = input.generics.split_for_impl();
    let impl_generics = &input.generics.params;
    let impl_generics = quote! { <'de, #impl_generics> };
    let predicates = where_clause.map(|wc| &wc.predicates);
    let (_, field_ty, constructor) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics serde::Deserialize<'de> for #struct_name #ty_generics {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                        #predicates
                    {
                        Ok(#constructor(#field_ty::deserialize(deserializer)?.into()))
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #impl_generics serde::Deserialize<'de> for #struct_name #ty_generics {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                        #predicates
                    {
                        Ok(#via::deserialize(deserializer)?.into())
                    }
                }
            }
        },
    )
}
