use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};
use syn::{
    parse::{Parse, ParseStream},
    parse2,
    punctuated::Punctuated,
};

mod add;
mod arithmetic;
mod as_ref;
mod deref;
mod deserialize;
mod display;
mod eq;
mod from;
mod from_iterator;
mod from_str;
mod hash;
mod into;
mod mul;
mod ord;
mod partial_eq;
mod partial_ord;
mod serialize;
mod try_from;

#[derive(Debug, typed_builder::TypedBuilder)]
struct Derive {
    path: syn::Path,
    #[builder(default, setter(strip_option))]
    via: Option<syn::Type>,
}

#[derive(Debug, Default)]
struct DerivingAttributes(Vec<Derive>);

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
}

impl DerivingAttributes {
    fn from_attribute(attribute: &syn::Attribute) -> syn::Result<Self> {
        #[derive(Debug)]
        enum Derive {
            Single(Single),
            DerivingVia(DerivingVia),
        }

        impl FromIterator<Derive> for Vec<crate::impls::deriving_via::Derive> {
            fn from_iter<T: IntoIterator<Item = Derive>>(iter: T) -> Self {
                type Target = crate::impls::deriving_via::Derive;
                iter.into_iter()
                    .flat_map(|derive| -> Vec<Target> {
                        match derive {
                            Derive::Single(Single { path }) => {
                                vec![Target::builder().path(path).build()]
                            }
                            Derive::DerivingVia(DerivingVia {
                                derive, via: path, ..
                            }) => {
                                vec![Target::builder().path(derive).via(path).build()]
                            }
                        }
                    })
                    .collect()
            }
        }

        #[derive(Debug)]
        struct Single {
            path: syn::Path,
        }

        #[derive(Debug)]
        struct DerivingVia {
            derive: syn::Path,
            via: syn::Type,
        }

        impl From<syn::Path> for Single {
            fn from(path: syn::Path) -> Self {
                Self { path }
            }
        }

        impl From<(syn::Path, syn::Type)> for DerivingVia {
            fn from((derive, via): (syn::Path, syn::Type)) -> Self {
                Self { derive, via }
            }
        }

        fn try_parse(input: syn::Expr) -> syn::Result<Derive> {
            use syn::Expr::{Assign, Call, Path};
            match input {
                Path(derive) => AvailableDerives::iter()
                    .any(|available_derive| derive.path.is_ident(available_derive.into()))
                    .then(|| Derive::Single(Single::from(derive.path.clone())))
                    .ok_or_else(|| syn::Error::new_spanned(derive, "unavailable derive")),
                Call(syn::ExprCall { func, args, .. }) => match &*func {
                    Path(path) => match args.iter().collect::<Vec<_>>().as_slice() {
                        [Assign(syn::ExprAssign { left, right, .. })] => {
                            if let (Path(keyword), ty) = (&**left, &**right) {
                                if keyword.path.is_ident("via") {
                                    return if let Ok(ty) = parse2(ty.into_token_stream()) {
                                        Ok(Derive::DerivingVia(DerivingVia::from((
                                            path.path.clone(),
                                            ty,
                                        ))))
                                    } else {
                                        Err(syn::Error::new_spanned(ty, "expected: <Type>"))
                                    };
                                }
                            }
                            Err(syn::Error::new_spanned(args, "expected: (via = <Type>)"))
                        }
                        _ => Err(syn::Error::new_spanned(args, "expected: (via = <Type>)")),
                    },
                    _ => Err(syn::Error::new_spanned(*func, "unavailable custom option")),
                },
                expr => Err(syn::Error::new_spanned(expr, "expected: (<...>)")),
            }
        }

        let expr: syn::Expr = parse2(attribute.tokens.to_owned()).unwrap();
        use syn::Expr::{Paren, Tuple};
        Ok(Self(
            match expr {
                Paren(expr) => try_parse(*expr.expr).map(|derive| vec![derive]),
                Tuple(items) => items.elems.into_iter().map(try_parse).collect(),
                expr => Err(syn::Error::new_spanned(expr, "expected: (<Item>, ...)")),
            }?
            .into_iter()
            .collect(),
        ))
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
) -> impl FnOnce(&syn::DeriveInput, Option<&syn::Type>) -> TokenStream {
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
    }
}

impl DerivingAttributes {
    fn into_token_stream(self, input: &syn::DeriveInput) -> TokenStream {
        self.0
            .into_iter()
            .map(|derive| {
                AvailableDerives::iter()
                    .filter_map(|ad| {
                        derive
                            .path
                            .is_ident(ad.into())
                            .then(|| extractor(ad)(input, derive.via.as_ref()))
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
