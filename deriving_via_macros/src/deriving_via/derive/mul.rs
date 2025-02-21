use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::GenericParam;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, _, constructor) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics ::core::ops::Mul for #struct_name #ty_generics #where_clause {
                    type Output = Self;

                    fn mul(self, other: Self) -> Self {
                        #constructor((self.#accessor * other.#accessor).into())
                    }
                }
                impl #impl_generics ::core::ops::Div for #struct_name #ty_generics #where_clause {
                    type Output = Self;

                    fn div(self, other: Self) -> Self {
                        #constructor((self.#accessor / other.#accessor).into())
                    }
                }
            }
        },
        |via| {
            let where_clause_for_mul = where_clause.as_ref().map_or_else(
                || {
                    quote! {
                        where
                            Self: ::core::convert::From<<#via as ::core::ops::Mul>::Output>,
                    }
                },
                |where_clause| {
                    quote! {
                        #where_clause
                            Self: ::core::convert::From<<#via as ::core::ops::Mul>::Output>,
                    }
                },
            );
            let where_clause_for_div = where_clause.as_ref().map_or_else(
                || {
                    quote! {
                        where
                            Self: ::core::convert::From<<#via as ::core::ops::Div>::Output>,
                    }
                },
                |where_clause| {
                    quote! {
                        #where_clause
                            Self: ::core::convert::From<<#via as ::core::ops::Div>::Output>,
                    }
                },
            );
            let (where_clause_for_mul, where_clause_for_div) = if input.generics.params.iter().filter_map(|param| {
                match param {
                    GenericParam::Type(ty) => Some(ty.ident.to_string()),
                    _ => None,
                }
            })
                .collect_vec()
                .contains(&format!("{}", via.to_token_stream())) {
                (quote! { #where_clause_for_mul #via: Clone, }, quote! { #where_clause_for_div #via: Clone, })
            } else {
                (where_clause_for_mul, where_clause_for_div)
            };

            quote! {
                impl #impl_generics ::core::ops::Mul for #struct_name #ty_generics #where_clause_for_mul {
                    type Output = Self;

                    fn mul(self, other: Self) -> Self {
                        let lhs: &#via = &self;
                        let rhs: &#via = &other;
                        (lhs.to_owned() * rhs.to_owned()).into()
                    }
                }
                impl #impl_generics ::core::ops::Div for #struct_name #ty_generics #where_clause_for_div {
                    type Output = Self;

                    fn div(self, other: Self) -> Self {
                        let lhs: &#via = &self;
                        let rhs: &#via = &other;
                        (lhs.to_owned() / rhs.to_owned()).into()
                    }
                }
            }
        },
    )
}
