use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::extract_single_field;

pub(crate) fn extract(input: &syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field_ident = &field.ident;
    let field_ty = &field.ty;

    field_ident.as_ref().map_or_else(
        || {
            quote! {
                impl std::ops::Deref for #struct_name {
                    type Target = #field_ty;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
            }
        },
        |field_name| {
            quote! {
                impl std::ops::Deref for #struct_name {
                    type Target = #field_ty;

                    fn deref(&self) -> &Self::Target {
                        &self. #field_name
                    }
                }
            }
        },
    )
}
