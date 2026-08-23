use proc_macro2::TokenStream;
use quote::quote;

use super::super::utils::extract_fields;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (_, field_ty, _constructor) = match extract_fields(input) {
        Ok(res) => res,
        Err(e) => return e,
    };

    via.as_ref().map_or_else(
        || {
            syn::Error::new_spanned(
                input,
                "#[deriving(TryFrom)] without `via` is infallible; consider using `#[deriving(From)]` instead, which automatically provides `TryFrom`.",
            )
            .to_compile_error()
        },
        |via| {
            quote! {
                #[allow(clippy::infallible_try_from)]
                impl #impl_generics ::core::convert::TryFrom<#field_ty> for #struct_name #ty_generics #where_clause {
                    type Error = <#via as ::core::convert::TryFrom<#field_ty>>::Error;

                    fn try_from(__: #field_ty) -> ::core::result::Result<Self, Self::Error> {
                        let intermediate: #via = __.try_into()?;
                        Ok(intermediate.into())
                    }
                }
            }
        },
    )
}
