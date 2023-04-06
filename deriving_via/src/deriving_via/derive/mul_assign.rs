use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::GenericParam;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, _, _) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics ::core::ops::MulAssign for #struct_name #ty_generics #where_clause {
                    fn mul_assign(&mut self, rhs: Self) {
                        self.#accessor.mul_assign(rhs.#accessor);
                    }
                }
                impl #impl_generics ::core::ops::DivAssign for #struct_name #ty_generics #where_clause {
                    fn div_assign(&mut self, rhs: Self) {
                        self.#accessor.div_assign(rhs.#accessor);
                    }
                }
            }
        },
        |via| {
            let where_clause_for_mul = where_clause.as_ref().map_or_else(
                || {
                    quote! {
                        where
                            #via: ::core::ops::MulAssign,
                    }
                },
                |where_clause| {
                    quote! {
                        #where_clause
                            #via: ::core::ops::MulAssign,
                    }
                },
            );
            let where_clause_for_div = where_clause.as_ref().map_or_else(
                || {
                    quote! {
                        where
                            #via: ::core::ops::DivAssign,
                    }
                },
                |where_clause| {
                    quote! {
                        #where_clause
                            #via: ::core::ops::DivAssign,
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
                impl #impl_generics ::core::ops::MulAssign for #struct_name #ty_generics #where_clause_for_mul {
                    fn mul_assign(&mut self, rhs: Self) {
                        let lhs: &mut #via = self;
                        let rhs: &#via = &rhs;
                        lhs.div_assign(rhs.to_owned());
                    }
                }
                impl #impl_generics ::core::ops::DivAssign for #struct_name #ty_generics #where_clause_for_div {
                    fn div_assign(&mut self, rhs: Self) {
                        let lhs: &mut #via = self;
                        let rhs: &#via = &rhs;
                        lhs.div_assign(rhs.to_owned());
                    }
                }
            }
        },
    )
}
