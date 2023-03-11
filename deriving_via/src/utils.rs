use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;

pub(crate) type Constructor = TokenStream;
pub(crate) type Accessor = TokenStream;
pub(crate) type UnderlyingType = syn::Type;

pub(crate) fn extract_fields(ast: &syn::DeriveInput) -> (Accessor, UnderlyingType, Constructor) {
    let struct_name = &ast.ident;
    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            let fields = fields.iter().collect::<Vec<_>>();

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
                (accessor, field.ty.to_owned(), constructor)
            } else {
                match fields.iter().enumerate().find(|(_, field)| {
                    field
                        .attrs
                        .iter()
                        .any(|attr| attr.path.is_ident("underlying"))
                }) {
                    None => abort!(
                        ast,
                        "#[underlying] is required for multiple fields";
                        help = "Specify #[underlying] to the field";
                    ),
                    Some((idx, underlying)) => {
                        let ty = &underlying.ty;
                        let accessor = underlying.ident.as_ref().map_or_else(
                            || {
                                let idx = syn::Index::from(idx);
                                quote! { #idx }
                            },
                            |ident| quote! { #ident },
                        );
                        let defaults = fields
                            .iter()
                            .enumerate()
                            .filter(|(_idx, field)| field != &underlying)
                            .map(|(idx, field)| {
                                field.ident.as_ref().map_or_else(
                                    || {
                                        let idx = syn::Index::from(idx);
                                        quote! { #idx }
                                    },
                                    |ident| quote! { #ident },
                                )
                            })
                            .collect::<Vec<_>>();

                        let constructor = quote! { (|__| #struct_name { #accessor: __, #(#defaults: Default::default()),* }) };
                        (accessor, ty.to_owned(), constructor)
                    }
                }
            }
        }
        _ => abort!(
            ast,
            "input is not a struct";
            help = "#[derive(DerivingVia)] can only be used with structs";
        ),
    }
}
