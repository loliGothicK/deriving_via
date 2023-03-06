use proc_macro_error::abort;

#[allow(unused)]
pub(crate) fn combine<const N: usize>(
    impls: [fn(&syn::DeriveInput) -> proc_macro2::TokenStream; N],
) -> impl FnOnce(&syn::DeriveInput) -> proc_macro::TokenStream {
    move |ast: &syn::DeriveInput| -> proc_macro::TokenStream {
        Vec::<proc_macro::TokenStream>::from_iter(impls.map(|derive| derive(ast).into()))
            .into_iter()
            .collect()
    }
}

#[allow(unused)]
pub(crate) fn extract_single_field(ast: &syn::DeriveInput) -> syn::Field {
    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            match fields.iter().cloned().collect::<Vec<_>>().as_slice() {
                [field] => field.clone(),
                _ => abort!(
                    ast,
                    "Not just one field";
                    help = "#[derive(NewtypeDeriving)] can only be used with one field";
                ),
            }
        }
        _ => abort!(
            ast,
            "input is not a struct";
            help = "#[derive(NewtypeDeriving)] can only be used with structs";
        ),
    }
}

#[allow(unused)]
pub(crate) fn extract_fields(ast: &syn::DeriveInput) -> impl IntoIterator<Item = syn::Field> {
    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => fields.clone(),
        _ => abort!(
            ast,
            "input is not a struct";
            help = "#[derive(NewtypeDeriving)] can only be used with structs";
        ),
    }
}
