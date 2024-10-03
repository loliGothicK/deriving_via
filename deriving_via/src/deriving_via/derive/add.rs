use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::GenericParam;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, _, constructor) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics ::core::ops::Add for #struct_name #ty_generics #where_clause {
                    type Output = Self;

                    fn add(self, other: Self) -> Self {
                        #constructor((self.#accessor + other.#accessor).into())
                    }
                }
                impl #impl_generics ::core::ops::Sub for #struct_name #ty_generics #where_clause {
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
                            Self: ::core::convert::From<<#via as ::core::ops::Add>::Output>,
                    }
                },
                |where_clause| {
                    quote! {
                        #where_clause
                            Self: ::core::convert::From<<#via as ::core::ops::Add>::Output>,
                    }
                },
            );
            let where_clause_for_sub = where_clause.as_ref().map_or_else(
                || {
                    quote! {
                        where
                            Self: ::core::convert::From<<#via as ::core::ops::Sub>::Output>,
                    }
                },
                |where_clause| {
                    quote! {
                        #where_clause
                            Self: ::core::convert::From<<#via as ::core::ops::Sub>::Output>,
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
                impl #impl_generics ::core::ops::Add for #struct_name #ty_generics #where_clause_for_add {
                    type Output = Self;

                    fn add(self, other: Self) -> Self {
                        let lhs: &#via = &self;
                        let rhs: &#via = &other;
                        (lhs.to_owned() + rhs.to_owned()).into()
                    }
                }
                impl #impl_generics ::core::ops::Sub for #struct_name #ty_generics #where_clause_for_sub {
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
