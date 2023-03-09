use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use crate::utils::extract_fields;

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
    let (accessor, field_ty, _) = extract_fields(input);

    via.map_or_else(
        || {
            quote! {
                impl #generics ::core::convert::From<#struct_name #generic_params> for #field_ty #where_clause {
                    fn from(__: #struct_name #generic_params) -> #field_ty {
                        __.#accessor
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #generics ::core::convert::From<#struct_name #generic_params> for #via #where_clause {
                    fn from(__: #struct_name #generic_params) -> #via {
                        let de: &#via = &__;
                        de.to_owned()
                    }
                }
            }
        },
    )
}
