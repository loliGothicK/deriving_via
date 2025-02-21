use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};

#[proc_macro_derive(Invoke)]
pub fn into_extract(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemEnum);

    let ident = &input.ident;
    let match_arms = input.variants.iter().map(|variant| {
        let module = variant
            .ident
            .to_token_stream()
            .to_string()
            .to_case(Case::Snake);
        let module = format_ident!("{}", module);
        quote! {
            #variant =>  #module :: extract
        }
    });

    let generated = quote! {
        impl #ident {
            fn invoke(self, input: &syn::DeriveInput, via: Option<syn::Type>) -> TokenStream {
                use #ident :: *;
                (match self {
                    #(#match_arms),*
                })(input, via)
            }
        }
    };

    generated.into()
}
