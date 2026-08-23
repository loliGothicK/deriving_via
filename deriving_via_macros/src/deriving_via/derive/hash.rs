use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, ..) = match extract_fields(input) {
        Ok(res) => res,
        Err(e) => return e,
    };

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics ::core::hash::Hash for #struct_name #ty_generics #where_clause {
                    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                        self.#accessor.hash(state);
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #impl_generics ::core::hash::Hash for #struct_name #ty_generics #where_clause {
                    fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                        let de: &#via = self;
                        de.hash(state);
                    }
                }
            }
        },
    )
}
