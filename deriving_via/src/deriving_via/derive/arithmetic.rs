use proc_macro2::TokenStream;

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    [
        super::add::extract(input, via.clone()),
        super::mul::extract(input, via),
    ]
    .into_iter()
    .collect()
}
