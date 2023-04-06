use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, field_ty, _) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics ::core::convert::From<#struct_name #ty_generics> for #field_ty #where_clause {
                    fn from(__: #struct_name #ty_generics) -> #field_ty {
                        __.#accessor
                    }
                }
            }
        },
        |via| {
            // TODO: abort if violates the Orphan Rule
            quote! {
                impl #impl_generics ::core::convert::From<#struct_name #ty_generics> for #via #where_clause {
                    fn from(__: #struct_name #ty_generics) -> #via {
                        let de: &#via = &__;
                        de.to_owned()
                    }
                }
            }
        },
    )
}
