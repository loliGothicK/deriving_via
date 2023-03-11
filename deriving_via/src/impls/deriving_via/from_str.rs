use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use crate::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
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
    let (_, field_ty, constructor) = extract_fields(input);

    match via.as_ref().unwrap_or(&field_ty) {
        syn::Type::Path(path) if path.path.is_ident("String") => {
            quote! {
                impl #generics std::str::FromStr for #struct_name #generic_params #where_clause {
                    type Err = std::convert::Infallible;

                    fn from_str(__: &str) -> std::result::Result<Self, Self::Err> {
                        Ok(#constructor(__.to_owned()))
                    }
                }
            }
        }
        ty => {
            quote! {
                impl #generics std::str::FromStr for #struct_name #generic_params #where_clause {
                    type Err = <#ty as std::str::FromStr>::Err;

                    fn from_str(__: &str) -> std::result::Result<Self, Self::Err> {
                        let intermediate: #ty = __.parse()?;
                        Ok(intermediate.into())
                    }
                }
            }
        }
    }
}
