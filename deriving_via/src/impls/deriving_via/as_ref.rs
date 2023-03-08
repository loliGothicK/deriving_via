use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::extract_single_field;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<&syn::Type>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field_ty = &field.ty;
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl<__AsRefT: ?::core::marker::Sized> ::core::convert::AsRef<__AsRefT> for #struct_name
                        where
                            #field_ty: ::core::convert::AsRef<__AsRefT>,
                        {
                            #[inline]
                            fn as_ref(&self) -> &__AsRefT {
                                <#field_ty as ::core::convert::AsRef<__AsRefT>>::as_ref(&self.0)
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl<__AsRefT: ?::core::marker::Sized> ::core::convert::AsRef<__AsRefT> for #struct_name
                        where
                            #field_ty: ::core::convert::AsRef<__AsRefT>,
                        {
                            #[inline]
                            fn as_ref(&self) -> &__AsRefT {
                                <#field_ty as ::core::convert::AsRef<__AsRefT>>::as_ref(&self.#field_name)
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
                        impl ::core::convert::AsRef<#via> for #struct_name
                        {
                            #[inline]
                            fn as_ref(&self) -> &#via {
                                &*self.0
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl ::core::convert::AsRef<#via> for #struct_name
                        {
                            #[inline]
                            fn as_ref(&self) -> &#via {
                                &*self.#field_name
                            }
                        }
                    }
                },
            )
        },
    )
}
