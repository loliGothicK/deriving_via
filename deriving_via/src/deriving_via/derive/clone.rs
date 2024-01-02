use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, _, constructor) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                #[allow(clippy::clippy::non_canonical_clone_impl)]
                impl #impl_generics Clone for #struct_name #ty_generics #where_clause {
                    fn clone(&self) -> Self {
                        #constructor(self. #accessor .to_owned())
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #impl_generics Clone for #struct_name #ty_generics #where_clause {
                    fn clone(&self) -> Self {
                        let __: &#via = &self. #accessor;
                        __.to_owned().into()
                    }
                }
            }
        },
    )
}
