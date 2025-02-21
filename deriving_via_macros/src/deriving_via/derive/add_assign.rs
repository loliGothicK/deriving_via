use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::GenericParam;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (accessor, _, _) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics ::core::ops::AddAssign for #struct_name #ty_generics #where_clause {
                    fn add_assign(&mut self, rhs: Self) {
                        self.#accessor.add_assign(rhs.#accessor);
                    }
                }
                impl #impl_generics ::core::ops::SubAssign for #struct_name #ty_generics #where_clause {
                    fn sub_assign(&mut self, rhs: Self) {
                        self.#accessor.sub_assign(rhs.#accessor);
                    }
                }
            }
        },
        |via| {
            let where_clause_for_add = where_clause.as_ref().map_or_else(
                || {
                    quote! {
                        where
                            #via: ::core::ops::AddAssign,
                    }
                },
                |where_clause| {
                    quote! {
                        #where_clause
                            #via: ::core::ops::AddAssign,
                    }
                },
            );
            let where_clause_for_sub = where_clause.as_ref().map_or_else(
                || {
                    quote! {
                        where
                            #via: ::core::ops::SubAssign,
                    }
                },
                |where_clause| {
                    quote! {
                        #where_clause
                            #via: ::core::ops::SubAssign,
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
                impl #impl_generics ::core::ops::AddAssign for #struct_name #ty_generics #where_clause_for_add {
                    fn add_assign(&mut self, rhs: Self) {
                        let lhs: &mut #via = self;
                        let rhs: &#via = &rhs;
                        lhs.sub_assign(rhs.to_owned());
                    }
                }
                impl #impl_generics ::core::ops::SubAssign for #struct_name #ty_generics #where_clause_for_sub {
                    fn sub_assign(&mut self, rhs: Self) {
                        let lhs: &mut #via = self;
                        let rhs: &#via = &rhs;
                        lhs.sub_assign(rhs.to_owned());
                    }
                }
            }
        },
    )
}
