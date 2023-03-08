use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use crate::utils::extract_single_field;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let generics = {
        let lt = &input.generics.lt_token;
        let params = &input.generics.params;
        let gt = &input.generics.gt_token;

        quote! { #lt #params #gt }
    };
    let generic_params = {
        let lt = &input.generics.lt_token;
        let params = input.generics.params.iter().filter_map(|p| match p {
            GenericParam::Type(ty) => Some(&ty.ident),
            _ => None,
        });
        let gt = &input.generics.gt_token;

        quote! { #lt #(#params),* #gt }
    };
    let where_clause = &input.generics.where_clause;
    let field = extract_single_field(input);
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl #generics std::hash::Hash for #struct_name #generic_params #where_clause {
                            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                                self.0.hash(state);
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl #generics std::hash::Hash for #struct_name #generic_params #where_clause {
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
                impl #generics std::hash::Hash for #struct_name #generic_params #where_clause {
                    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                        let de: &#via = self;
                        de.hash(state);
                    }
                }
            }
        },
    )
}
