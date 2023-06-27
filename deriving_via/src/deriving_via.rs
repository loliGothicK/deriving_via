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

#[derive(deriving_via_impl::Invoke, EnumIter, IntoStaticStr, Clone, Copy)]
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
    AsMut,
    IndexMut,
    IntoIterator,
    Iter,
    AddAssign,
    MulAssign,
    Default,
    IntoInner,
}

mod keyword {
    syn::custom_keyword!(deriving);
    syn::custom_keyword!(transitive);
}

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

#[derive(Clone)]
struct Via {
    via: syn::FnArg,
}

impl From<Via> for syn::Type {
    fn from(via: Via) -> Self {
        use syn::FnArg::*;
        match via.via {
            Receiver(_) => unreachable!(""),
            Typed(typed) => *typed.ty,
        }
    }
}

impl Parse for Via {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        let _ = syn::parenthesized!(content in input);

        let via = Via {
            via: content.parse()?,
        };

        use syn::FnArg::*;

        match &via.via {
            Receiver(_) => abort!(
                via.via,
                "Unexpected token";
                help = "expected: `via`, got: `self`";
            ),
            Typed(typed) => typed
                .pat
                .to_token_stream()
                .to_string()
                .eq("via")
                .then_some(via.to_owned())
                .ok_or(syn::Error::new_spanned(
                    &via.via,
                    format!(
                        "expected: `via`, got: `{}`",
                        typed.pat.clone().into_token_stream()
                    ),
                )),
        }
    }
}

struct DerivingAttributes {
    derivings: Punctuated<Deriving, syn::Token![,]>,
}

impl Parse for DerivingAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _ = input.parse::<keyword::deriving>()?;
        let _ = syn::parenthesized!(content in input);

        Ok(DerivingAttributes {
            derivings: content.parse_terminated(Deriving::parse, syn::Token![,])?,
        })
    }
}

struct Transitive {
    types: Punctuated<syn::Type, syn::Token![->]>,
}

impl Parse for Transitive {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _ = input.parse::<keyword::transitive>()?;
        let _ = syn::parenthesized!(content in input);

        Ok(Transitive {
            types: content.parse_terminated(syn::Type::parse, syn::Token![->])?,
        })
    }
}

impl DerivingAttributes {
    fn from_attribute(attr: &syn::Attribute) -> syn::Result<Self> {
        parse2(attr.meta.to_token_stream())
    }
}

impl Transitive {
    fn from_attribute(attr: &syn::Attribute) -> syn::Result<Self> {
        parse2(attr.meta.to_token_stream())
    }
}

pub(crate) fn impl_deriving_via(input: &syn::DeriveInput) -> TokenStream {
    input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr
                .meta
                .to_token_stream()
                .to_string()
                .starts_with("deriving")
            {
                Some(match DerivingAttributes::from_attribute(attr) {
                    Ok(deriving) => deriving.into_token_stream(input),
                    Err(err) => err.to_compile_error(),
                })
            } else if attr
                .meta
                .to_token_stream()
                .to_string()
                .starts_with("transitive")
            {
                Some(match Transitive::from_attribute(attr) {
                    Ok(transitive) => transitive.into_token_stream(input),
                    Err(err) => err.to_compile_error(),
                })
            } else {
                None
            }
        })
        .chain([deref::extract(input), deref_mut::extract(input, None)])
        .collect()
}

impl DerivingAttributes {
    fn into_token_stream(self, input: &syn::DeriveInput) -> TokenStream {
        self.derivings
            .into_iter()
            .map(|derive| {
                AvailableDerives::iter()
                    .filter_map(|kind| {
                        derive.path.is_ident(kind.into()).then(|| {
                            kind.invoke(input, derive.via.as_ref().cloned().map(Into::into))
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
    fn into_token_stream(self, input: &syn::DeriveInput) -> TokenStream {
        if self.types.len() < 3 {
            return syn::Error::new_spanned(self.types, "transitive must have three or more types")
                .to_compile_error();
        }
        let from_type = self.types.first().unwrap();
        let self_type = self.types.last().unwrap();

        if input.ident != self_type.to_token_stream().to_string() {
            return syn::Error::new_spanned(
                self_type,
                "transitive must end with the same type as the derive input",
            )
            .to_compile_error();
        }

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
