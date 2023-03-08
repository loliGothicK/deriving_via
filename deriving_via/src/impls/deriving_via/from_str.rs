use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::extract_single_field;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);

    let field_name = &field.ident;
    let field_ty = &field.ty;

    match via.unwrap_or(field_ty) {
        syn::Type::Path(path) if path.path.is_ident("String") => field_name
            .as_ref()
            .map(|field_name| {
                quote! {
                    impl std::str::FromStr for #struct_name {
                        type Err = std::convert::Infallible;

                        fn from_str(__: &str) -> std::result::Result<Self, Self::Err> {
                            Ok(Self { #field_name: __.to_owned() })
                        }
                    }
                }
            })
            .unwrap_or_else(|| {
                quote! {
                    impl std::str::FromStr for #struct_name {
                        type Err = std::convert::Infallible;

                        fn from_str(__: &str) -> std::result::Result<Self, Self::Err> {
                            Ok(Self(__.to_owned()))
                        }
                    }
                }
            }),
        ty => field_name
            .as_ref()
            .map(|field_name| {
                quote! {
                    impl std::str::FromStr for #struct_name {
                        type Err = <#ty as std::str::FromStr>::Err;

                        fn from_str(__: &str) -> std::result::Result<Self, Self::Err> {
                            let intermediate: #ty = __.parse()?;
                            Ok(Self { #field_name: intermediate.into() })
                        }
                    }
                }
            })
            .unwrap_or_else(|| {
                quote! {
                    impl std::str::FromStr for #struct_name {
                        type Err = <#ty as std::str::FromStr>::Err;

                        fn from_str(__: &str) -> std::result::Result<Self, Self::Err> {
                            let intermediate: #ty = __.parse()?;
                            Ok(Self(intermediate.into()))
                        }
                    }
                }
            }),
    }
}
