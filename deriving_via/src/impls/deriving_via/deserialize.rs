use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use crate::utils::extract_single_field;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let _generics = {
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
    let predicates = input
        .generics
        .where_clause
        .as_ref()
        .map(|wc| &wc.predicates);
    let generics_params = &input.generics.params;
    let field = extract_single_field(input);
    let field_ty = &field.ty;
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl<'de, #generics_params> serde::Deserialize<'de> for #struct_name #generic_params {
                            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                            where
                                D: Deserializer<'de>,
                                #predicates
                            {
                                Ok(#struct_name(field_ty::deserialize(deserializer)?.into()))
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl<'de, #generics_params> serde::Deserialize<'de> for #struct_name #generic_params {
                            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                            where
                                D: Deserializer<'de>,
                                #predicates
                            {
                                Ok(#struct_name {
                                    #field_name: #field_ty::deserialize(deserializer)?.into()
                                })
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
                        impl<'de, #generics_params> serde::Deserialize<'de> for #struct_name #generic_params {
                            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                            where
                                D: Deserializer<'de>,
                                #predicates
                            {
                                Ok(#struct_name(#via::deserialize(deserializer)?.into()))
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl<'de, #generics_params> serde::Deserialize<'de> for #struct_name #generic_params {
                            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                            where
                                D: Deserializer<'de>,
                                #predicates
                            {
                                Ok(#struct_name {
                                    #field_name: #via::deserialize(deserializer)?.into()
                                })
                            }
                        }
                    }
                },
            )
        },
    )
}
