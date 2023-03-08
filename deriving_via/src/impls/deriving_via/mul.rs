use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::extract_single_field;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl std::ops::Mul for #struct_name {
                            type Output = Self;

                            fn mul(self, other: Self) -> Self {
                                Self((self.0 * other.0).into())
                            }
                        }
                        impl std::ops::Div for #struct_name {
                            type Output = Self;

                            fn div(self, other: Self) -> Self {
                                Self((self.0 / other.0).into())
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl std::ops::Mul for #struct_name {
                            type Output = Self;

                            fn mul(self, other: Self) -> Self {
                                Self {
                                    #field_name: (self.#field_name * other.#field_name).into()
                                }
                            }
                        }
                        impl std::ops::Div for #struct_name {
                            type Output = Self;

                            fn div(self, other: Self) -> Self {
                                Self {
                                    #field_name: (self.#field_name / other.#field_name).into()
                                }
                            }
                        }
                    }
                },
            )
        },
        |via| {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl std::ops::Mul for #struct_name {
                            type Output = Self;

                            fn mul(self, other: Self) -> Self {
                                let lhs: &#via = &*self;
                                let rhs: &#via = &*self;
                                Self((lhs.to_owned() * rhs.to_owned()).into())
                            }
                        }
                        impl std::ops::Div for #struct_name {
                            type Output = Self;

                            fn div(self, other: Self) -> Self {
                                let lhs: &#via = &*self;
                                let rhs: &#via = &*self;
                                Self((lhs.to_owned() / rhs.to_owned()).into())
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl std::ops::Mul for #struct_name {
                            type Output = Self;

                            fn mul(self, other: Self) -> Self {
                                let lhs: &#via = &*self;
                                let rhs: &#via = &*self;
                                Self {
                                    #field_name: (lhs.to_owned() * rhs.to_owned()).into()
                                }
                            }
                        }
                        impl std::ops::Div for #struct_name {
                            type Output = Self;

                            fn div(self, other: Self) -> Self {
                                let lhs: &#via = &*self;
                                let rhs: &#via = &*self;
                                Self {
                                    #field_name: (lhs.to_owned() / rhs.to_owned()).into()
                                }
                            }
                        }
                    }
                },
            )
        },
    )
}
