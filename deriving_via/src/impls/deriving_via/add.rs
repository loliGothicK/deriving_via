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
                        impl #generics std::ops::Add for #struct_name #generic_params #where_clause {
                            type Output = Self;

                            fn add(self, other: Self) -> Self {
                                Self((self.0 + other.0).into())
                            }
                        }
                        impl #generics std::ops::Sub for #struct_name #generic_params #where_clause {
                            type Output = Self;

                            fn sub(self, other: Self) -> Self {
                                Self((self.0 - other.0).into())
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl #generics std::ops::Add for #struct_name #generic_params #where_clause {
                            type Output = Self;

                            fn add(self, other: Self) -> Self {
                                Self {
                                    #field_name: (self.#field_name + other.#field_name).into()
                                }
                            }
                        }
                        impl #generics std::ops::Sub for #struct_name #generic_params #where_clause {
                            type Output = Self;

                            fn sub(self, other: Self) -> Self {
                                Self {
                                    #field_name: (self.#field_name - other.#field_name).into()
                                }
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl #generics std::ops::Add for #struct_name #generic_params #where_clause {
                    type Output = Self;

                    fn add(self, other: Self) -> Self {
                        let lhs: &#via = &self;
                        let rhs: &#via = &other;
                        (lhs.to_owned() + rhs.to_owned()).into()
                    }
                }
                impl #generics std::ops::Sub for #struct_name #generic_params #where_clause {
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
