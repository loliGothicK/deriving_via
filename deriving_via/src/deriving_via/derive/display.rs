use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use super::super::utils::extract_fields;

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
    let (accessor, ..) = extract_fields(input);

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #generics ::core::fmt::Display for #struct_name #generic_params
                    #where_clause
                {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        write!(f, "{}", &self. #accessor)
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #generics ::core::fmt::Display for #struct_name #generic_params
                    #where_clause
                {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        let de: &#via = self;
                        write!(f, "{}", de)
                    }
                }
            }
        },
    )
}
