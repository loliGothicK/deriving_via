use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (accessor, ..) = extract_fields(input);
    let debug = struct_name.to_string();
    let field = accessor.to_string();
    let debug_body = if field == "0" {
        {
            quote! {
                f.debug_tuple(#debug)
                    .field(field)
                    .finish()
            }
        }
    } else {
        {
            quote! {
                f.debug_struct(#debug)
                    .field(#field, field)
                    .finish()
            }
        }
    };

    via.as_ref().map_or_else(
        || {
            quote! {
                impl #impl_generics ::core::fmt::Debug for #struct_name #ty_generics
                    #where_clause
                {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        let field = &self. #accessor;
                        #debug_body
                    }
                }
            }
        },
        |via| {
            quote! {
                impl #impl_generics ::core::fmt::Debug for #struct_name #ty_generics
                    #where_clause
                {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        let field: &#via = self;
                        #debug_body
                    }
                }
            }
        },
    )
}
