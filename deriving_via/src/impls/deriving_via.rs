use proc_macro2::TokenStream;
use quote::ToTokens;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};
use syn::parse2;

mod add;
mod arithmetic;
mod as_ref;
mod deref;
mod deserialize;
mod display;
mod eq;
mod from;
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
}

impl DerivingAttributes {
    fn from_attributes(attributes: &[syn::Attribute]) -> syn::Result<Self> {
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

        Ok(Self(
            attributes
                .iter()
                .filter_map(|attr| attr.path.is_ident("deriving").then_some(&attr.tokens))
                .cloned()
                .map(|tokens| -> syn::Result<Vec<_>> {
                    let expr: syn::Expr = parse2(tokens).unwrap();
                    use syn::Expr::{Paren, Tuple};
                    match expr {
                        Paren(expr) => try_parse(*expr.expr).map(|derive| vec![derive]),
                        Tuple(items) => items.elems.into_iter().map(try_parse).collect(),
                        expr => Err(syn::Error::new_spanned(expr, "expected: (<Item>, ...)")),
                    }
                })
                .collect::<syn::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect(),
        ))
    }
}

pub(crate) fn impl_generalised_newtype_deriving(input: &syn::DeriveInput) -> TokenStream {
    match DerivingAttributes::from_attributes(&input.attrs) {
        Ok(attributes) => attributes.into_token_stream(input),
        Err(err) => err.to_compile_error(),
    }
}

fn extractor(
    target: AvailableDerives,
) -> impl FnOnce(&syn::DeriveInput, Option<&syn::Type>) -> TokenStream {
    match target {
        AvailableDerives::Display => display::extract,
        AvailableDerives::Into => into::extract,
        AvailableDerives::From => from::extract,
        AvailableDerives::PartialEq => partial_eq::extract,
        AvailableDerives::Eq => eq::extract,
        AvailableDerives::PartialOrd => partial_ord::extract,
        AvailableDerives::Ord => ord::extract,
        AvailableDerives::TryFrom => try_from::extract,
        AvailableDerives::FromStr => from_str::extract,
        AvailableDerives::Hash => hash::extract,
        AvailableDerives::Serialize => serialize::extract,
        AvailableDerives::Deserialize => deserialize::extract,
        AvailableDerives::Add => add::extract,
        AvailableDerives::Mul => mul::extract,
        AvailableDerives::Arithmetic => arithmetic::extract,
        AvailableDerives::AsRef => as_ref::extract,
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
            .chain(std::iter::once_with(|| deref::extract(input)))
            .collect()
    }
}
