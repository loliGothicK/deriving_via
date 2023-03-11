use proc_macro2::TokenStream;

use crate::impls::deriving_via::{add, mul};

pub(crate) fn extract(input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
    [add::extract(input, via.clone()), mul::extract(input, via)]
        .into_iter()
        .collect()
}
