use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
};

use crate::deriving_via::derive::*;

mod derive;
pub(crate) mod utils;

#[derive(EnumIter, IntoStaticStr, Clone, Copy)]
#[strum(serialize_all = "PascalCase")]
enum AvailableDerives {
    Display,
    Into,
    From,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    TryFrom,
    FromStr,
    Hash,
    Serialize,
    Deserialize,
    Add,
    Mul,
    Arithmetic,
    AsRef,
    FromIterator,
    Index,
}

#[derive(Debug)]
struct Deriving {
    path: syn::Path,
    via: Option<Via>,
}

impl Parse for Deriving {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(syn::token::Paren) {
            Ok(Deriving {
                path,
                via: Some(input.parse()?),
            })
        } else {
            Ok(Deriving { path, via: None })
        }
    }
}

#[derive(Debug, Clone)]
struct Via {
    via: syn::ExprType,
}

impl From<Via> for syn::Type {
    fn from(via: Via) -> Self {
        *via.via.ty
    }
}

impl Parse for Via {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        let _ = syn::parenthesized!(content in input);

        let via = Via {
            via: content.parse()?,
        };

        if let Some(via) = via
            .via
            .expr
            .clone()
            .into_token_stream()
            .to_string()
            .eq("via")
            .then(|| via.to_owned())
        {
            Ok(via)
        } else {
            abort!(
                via.via.expr,
                "Unexpected token";
                help = format!("expected: `via`, got: `{}`", via.via.expr.to_token_stream());
            );
        }
    }
}

struct DerivingAttributes {
    derivings: Punctuated<Deriving, syn::Token![,]>,
}

impl Parse for DerivingAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _ = syn::parenthesized!(content in input);

        Ok(DerivingAttributes {
            derivings: content.parse_terminated(Deriving::parse)?,
        })
    }
}

struct Transitive {
    #[allow(unused)]
    paren_token: syn::token::Paren,
    types: Punctuated<syn::Type, syn::Token![->]>,
}

impl Parse for Transitive {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Transitive {
            paren_token: syn::parenthesized!(content in input),
            types: content.parse_terminated(syn::Type::parse)?,
        })
    }
}

impl DerivingAttributes {
    fn from_attribute(attr: &syn::Attribute) -> syn::Result<Self> {
        parse2(attr.tokens.to_owned())
    }
}

impl Transitive {
    fn from_attribute(attr: &syn::Attribute) -> syn::Result<Self> {
        parse2(attr.tokens.to_owned())
    }
}

pub(crate) fn impl_deriving_via(input: &syn::DeriveInput) -> TokenStream {
    input
        .attrs
        .iter()
        .map(|attr| {
            if attr.path.is_ident("deriving") {
                match DerivingAttributes::from_attribute(attr) {
                    Ok(deriving) => deriving.into_token_stream(input),
                    Err(err) => err.to_compile_error(),
                }
            } else if attr.path.is_ident("transitive") {
                match Transitive::from_attribute(attr) {
                    Ok(transitive) => transitive.into_token_stream(input),
                    Err(err) => err.to_compile_error(),
                }
            } else {
                syn::Error::new_spanned(attr, "unknown attribute").to_compile_error()
            }
        })
        .chain(std::iter::once_with(|| deref::extract(input)))
        .collect()
}

fn extractor(
    target: AvailableDerives,
) -> impl FnOnce(&syn::DeriveInput, Option<syn::Type>) -> TokenStream {
    use AvailableDerives::*;
    match target {
        Display => display::extract,
        Into => into::extract,
        From => from::extract,
        PartialEq => partial_eq::extract,
        Eq => eq::extract,
        PartialOrd => partial_ord::extract,
        Ord => ord::extract,
        TryFrom => try_from::extract,
        FromStr => from_str::extract,
        Hash => hash::extract,
        Serialize => serialize::extract,
        Deserialize => deserialize::extract,
        Add => add::extract,
        Mul => mul::extract,
        Arithmetic => arithmetic::extract,
        AsRef => as_ref::extract,
        FromIterator => from_iterator::extract,
        Index => index::extract,
    }
}

impl DerivingAttributes {
    fn into_token_stream(self, input: &syn::DeriveInput) -> TokenStream {
        self.derivings
            .into_iter()
            .map(|derive| {
                AvailableDerives::iter()
                    .filter_map(|ad| {
                        derive.path.is_ident(ad.into()).then(|| {
                            extractor(ad)(input, derive.via.as_ref().cloned().map(Into::into))
                        })
                    })
                    .collect::<Vec<_>>()
                    .first()
                    .cloned()
                    .unwrap_or_else(|| {
                        syn::Error::new_spanned(derive.path, "Sorry, unsupported Derive")
                            .to_compile_error()
                    })
            })
            .collect()
    }
}

impl Transitive {
    fn into_token_stream(self, _: &syn::DeriveInput) -> TokenStream {
        if self.types.len() < 3 {
            return syn::Error::new_spanned(self.types, "transitive must have three or more types")
                .to_compile_error();
        }
        let from_type = self.types.first().unwrap();
        let self_type = self.types.last().unwrap();
        let types = &self.types.iter().collect::<Vec<_>>()[1..];
        quote! {
            impl From<#from_type> for #self_type {
                fn from(__: #from_type) -> Self {
                    #(let __: #types = __.into();)*
                    __
                }
            }
        }
    }
}
