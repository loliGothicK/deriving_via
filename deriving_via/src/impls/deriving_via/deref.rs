use proc_macro2::TokenStream;
use quote::quote;
use syn::GenericParam;

use crate::utils::extract_single_field;

pub(crate) fn extract(input: &syn::DeriveInput) -> TokenStream {
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
    let field_ident = &field.ident;
    let field_ty = &field.ty;

    field_ident.as_ref().map_or_else(
        || {
            quote! {
                impl #generics std::ops::Deref for #struct_name #generic_params #where_clause {
                    type Target = #field_ty;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
            }
        },
        |field_name| {
            quote! {
                impl #generics std::ops::Deref for #struct_name #generic_params #where_clause {
                    type Target = #field_ty;

                    fn deref(&self) -> &Self::Target {
                        &self. #field_name
                    }
                }
            }
        },
    )
}
