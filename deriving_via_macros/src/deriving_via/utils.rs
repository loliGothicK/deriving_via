use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) type Constructor = TokenStream;
pub(crate) type Accessor = TokenStream;
pub(crate) type UnderlyingType = syn::Type;

pub(crate) fn extract_fields(
    ast: &syn::DeriveInput,
) -> Result<(Accessor, UnderlyingType, Constructor), TokenStream> {
    let struct_name = &ast.ident;
    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let fields = fields.iter().collect_vec();

            if fields.len() == 1 {
                let field = fields.first().unwrap();
                let accessor = field
                    .ident
                    .as_ref()
                    .map(|ident| {
                        quote! { #ident }
                    })
                    .unwrap_or_else(|| quote! { 0 });

                let constructor = field
                    .ident
                    .as_ref()
                    .map(|ident| {
                        quote! { (|__| #struct_name { #ident: __ }) }
                    })
                    .unwrap_or_else(|| {
                        quote! { (|__| #struct_name(__)) }
                    });
                Ok((accessor, field.ty.to_owned(), constructor))
            } else {
                match fields
                    .iter()
                    .enumerate()
                    .filter(|(_, field)| {
                        field
                            .attrs
                            .iter()
                            .any(|attr| attr.path().is_ident("underlying"))
                    })
                    .collect_vec()
                    .as_slice()
                {
                    [(idx, underlying)] => {
                        let ty = &underlying.ty;
                        let accessor = underlying.ident.as_ref().map_or_else(
                            || {
                                let idx = syn::Index::from(*idx);
                                quote! { #idx }
                            },
                            |ident| quote! { #ident },
                        );
                        let defaults = fields
                            .iter()
                            .enumerate()
                            .filter(|(i, field)| {
                                match (field.ident.as_ref(), underlying.ident.as_ref()) {
                                    (Some(x), Some(y)) if x != y => true,
                                    (None, None) => i != idx,
                                    _ => false,
                                }
                            })
                            .map(|(idx, field)| {
                                field.ident.as_ref().map_or_else(
                                    || {
                                        let idx = syn::Index::from(idx);
                                        quote! { #idx }
                                    },
                                    |ident| quote! { #ident },
                                )
                            })
                            .collect_vec();

                        let constructor = quote! { (|__| #struct_name { #accessor: __, #(#defaults: Default::default()),* }) };
                        Ok((accessor, ty.to_owned(), constructor))
                    }
                    [] => Err(syn::Error::new_spanned(
                        ast,
                        "#[underlying] is required for multiple fields: Specify #[underlying] to \
                         the field.",
                    )
                    .to_compile_error()),
                    _ => Err(syn::Error::new_spanned(
                        ast,
                        "multiple #[underlying] specifier is not allowed: Specify #[underlying] \
                         to only one field.",
                    )
                    .to_compile_error()),
                }
            }
        }
        _ => Err(syn::Error::new_spanned(
            ast,
            "input is not a struct: #[derive(DerivingVia)] can only be used with structs",
        )
        .to_compile_error()),
    }
}
