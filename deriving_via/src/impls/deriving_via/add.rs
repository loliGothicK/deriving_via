use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::GenericParam;

use crate::utils::extract_fields;

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
    let (accessor, _, constructor) = extract_fields(input);

    via.map_or_else(
        || {
            quote! {
                impl #generics std::ops::Add for #struct_name #generic_params #where_clause {
                    type Output = Self;

                    fn add(self, other: Self) -> Self {
                        #constructor((self.#accessor + other.#accessor).into())
                    }
                }
                impl #generics std::ops::Sub for #struct_name #generic_params #where_clause {
                    type Output = Self;

                    fn sub(self, other: Self) -> Self {
                        #constructor((self.#accessor - other.#accessor).into())
                    }
                }
            }
        },
        |via| {
            let where_clause_for_add = where_clause.as_ref().map_or_else(
                || {
                    quote! {
                        where
                            Self: std::convert::From<<#via as std::ops::Add>::Output>,
                    }
                },
                |where_clause| {
                    quote! {
                        #where_clause
                            Self: std::convert::From<<#via as std::ops::Add>::Output>,
                    }
                },
            );
            let where_clause_for_sub = where_clause.as_ref().map_or_else(
                || {
                    quote! {
                        where
                            Self: std::convert::From<<#via as std::ops::Sub>::Output>,
                    }
                },
                |where_clause| {
                    quote! {
                        #where_clause
                            Self: std::convert::From<<#via as std::ops::Sub>::Output>,
                    }
                },
            );
            let (where_clause_for_add, where_clause_for_sub) = if input.generics.params.iter().filter_map(|param| {
                match param {
                    GenericParam::Type(ty) => Some(ty.ident.to_string()),
                    _ => None,
                }
            })
                .collect_vec()
                .contains(&format!("{}", via.to_token_stream())) {
                (quote! { #where_clause_for_add #via: Clone, }, quote! { #where_clause_for_sub #via: Clone, })
            } else {
                (where_clause_for_add, where_clause_for_sub)
            };

            quote! {
                impl #generics std::ops::Add for #struct_name #generic_params #where_clause_for_add {
                    type Output = Self;

                    fn add(self, other: Self) -> Self {
                        let lhs: &#via = &self;
                        let rhs: &#via = &other;
                        (lhs.to_owned() + rhs.to_owned()).into()
                    }
                }
                impl #generics std::ops::Sub for #struct_name #generic_params #where_clause_for_sub {
                    type Output = Self;

                    fn sub(self, other: Self) -> Self {
                        let lhs: &#via = &self;
                        let rhs: &#via = &other;
                        (lhs.to_owned() - rhs.to_owned()).into()
                    }
                }
            }
        },
    )
}
