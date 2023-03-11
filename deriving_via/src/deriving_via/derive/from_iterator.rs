use proc_macro2::TokenStream;
use proc_macro_error::abort;
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
    let (_, _, constructor) = extract_fields(input);

    if let Some(via) = via {
        quote! {
            impl #generics ::core::iter::FromIterator<#via> for #struct_name #generic_params
                #where_clause
            {
                fn from_iter<I: ::core::iter::IntoIterator<Item=#via>>(iter: I) -> Self {
                    #constructor(iter.into_iter().collect())
                }
            }
        }
    } else {
        abort!(
            input,
            "#[deriving(FromIterator)] is not allowed";
            help = "Specify via as #[deriving(FromIterator(via: <ItemType>)])";
        );
    }
}
