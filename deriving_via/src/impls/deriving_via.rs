use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::extract_single_field;

#[derive(Debug, typed_builder::TypedBuilder)]
struct Derive {
    path: syn::Path,
    #[builder(default, setter(strip_option))]
    via: Option<syn::Path>,
}

#[derive(Debug, Default)]
struct DerivingAttributes(Vec<Derive>);

const AVAILABLE_DERIVES: [&str; 8] = [
    "Display",
    "Into",
    "From",
    "Eq",
    "Ord",
    "FromStr",
    "Hash",
    "Serialize",
];

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
            via: syn::Path,
        }

        impl From<syn::Path> for Single {
            fn from(path: syn::Path) -> Self {
                Self { path }
            }
        }

        impl From<(syn::Path, syn::Path)> for DerivingVia {
            fn from((derive, via): (syn::Path, syn::Path)) -> Self {
                Self { derive, via }
            }
        }

        fn try_parse(input: syn::Expr) -> syn::Result<Derive> {
            use syn::Expr::{Assign, Call, Path};
            match input {
                Path(derive) => AVAILABLE_DERIVES
                    .iter()
                    .any(|available_derive| derive.path.is_ident(available_derive))
                    .then(|| Derive::Single(Single::from(derive.path.clone())))
                    .ok_or_else(|| syn::Error::new_spanned(derive, "unavailable derive")),
                Call(syn::ExprCall { func, args, .. }) => match &*func {
                    Path(path) => match args.iter().collect::<Vec<_>>().as_slice() {
                        [Assign(syn::ExprAssign { left, right, .. })] => {
                            if let (Path(keyword), Path(via)) = (&**left, &**right) {
                                if keyword.path.is_ident("via") {
                                    return Ok(Derive::DerivingVia(DerivingVia::from((
                                        path.path.clone(),
                                        via.path.clone(),
                                    ))));
                                }
                            }
                            Err(syn::Error::new_spanned(
                                args,
                                "expected: (via = <TypeName>)",
                            ))
                        }
                        _ => Err(syn::Error::new_spanned(
                            args,
                            "expected: (via = <TypeName>)",
                        )),
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
                    let expr: syn::Expr = syn::parse2(tokens).unwrap();
                    use syn::Expr::{Paren, Tuple};
                    match expr {
                        Paren(expr) => try_parse(*expr.expr).map(|derive| vec![derive]),
                        Tuple(items) => items.elems.into_iter().map(try_parse).collect(),
                        expr => Err(syn::Error::new_spanned(
                            dbg!(expr),
                            "expected: (<Item>, ...)",
                        )),
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

impl DerivingAttributes {
    fn into_token_stream(self, input: &syn::DeriveInput) -> TokenStream {
        self.0
            .into_iter()
            .map(|derive| {
                derive
                    .path
                    .is_ident("TryFrom")
                    .then(|| impl_try_from(input))
                    .or_else(|| {
                        derive
                            .path
                            .is_ident("FromStr")
                            .then(|| impl_from_str(input))
                    })
                    .or_else(|| {
                        derive
                            .path
                            .is_ident("Display")
                            .then(|| impl_display(input, derive.via.as_ref()))
                    })
                    .or_else(|| {
                        derive
                            .path
                            .is_ident("Into")
                            .then(|| impl_into(input, derive.via.as_ref()))
                    })
                    .or_else(|| {
                        derive
                            .path
                            .is_ident("Eq")
                            .then(|| impl_eq(input, derive.via.as_ref()))
                    })
                    .or_else(|| {
                        derive
                            .path
                            .is_ident("Ord")
                            .then(|| impl_ord(input, derive.via.as_ref()))
                    })
                    .or_else(|| derive.path.is_ident("From").then(|| impl_from(input)))
                    .or_else(|| {
                        derive
                            .path
                            .is_ident("Hash")
                            .then(|| impl_hash(input, derive.via.as_ref()))
                    })
                    .or_else(|| {
                        derive
                            .path
                            .is_ident("Serialize")
                            .then(|| impl_serialize(input, derive.via.as_ref()))
                    })
                    .unwrap_or_else(|| {
                        syn::Error::new_spanned(derive.path, "Sorry, unsupported Derive")
                            .to_compile_error()
                    })
            })
            .chain(std::iter::once_with(|| impl_deref(input)))
            .collect()
    }
}

fn impl_try_from(input: &syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);

    let field_ident = &field.ident;
    let field_ty = &field.ty;

    field_ident.as_ref().map_or_else(
        || {
            quote! {
                impl std::convert::TryFrom<#field_ty> for #struct_name {
                    type Err = <#field_ty as std::str::TryFrom>::Err;

                    fn try_from(__: #field_ty) -> std::result::Result<Self, Self::Error> {
                        Ok(Self(__.try_into()?))
                    }
                }
            }
        },
        |field_name| {
            quote! {
                impl std::convert::TryFrom<#field_ty> for #struct_name {
                    type Error = validator::ValidationErrors;

                    fn try_from(__: #field_ty) -> std::result::Result<Self, Self::Error> {
                        Ok(Self { #field_name: __.try_into()? })
                    }
                }
            }
        },
    )
}

fn impl_from_str(input: &syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);

    let field_name = &field.ident;
    let field_ty = &field.ty;

    match &field_ty {
        syn::Type::Path(path) if path.path.is_ident("String") => field_name
            .as_ref()
            .map(|field_name| {
                quote! {
                    impl std::str::FromStr for #struct_name {
                        type Err = std::convert::Infallible;

                        fn from_str(__: &str) -> std::result::Result<Self, Self::Err> {
                            Ok(Self { #field_name: __.to_owned() })
                        }
                    }
                }
            })
            .unwrap_or_else(|| {
                quote! {
                    impl std::str::FromStr for #struct_name {
                        type Err = std::convert::Infallible;

                        fn from_str(__: &str) -> std::result::Result<Self, Self::Err> {
                            Ok(Self(__.to_owned()))
                        }
                    }
                }
            }),
        _ => field_name
            .as_ref()
            .map(|field_name| {
                quote! {
                    impl std::str::FromStr for #struct_name {
                        type Err = <#field_ty as std::str::FromStr>::Err;

                        fn from_str(__: &str) -> std::result::Result<Self, Self::Err> {
                            Ok(Self { #field_name: __.parse()? })
                        }
                    }
                }
            })
            .unwrap_or_else(|| {
                quote! {
                    impl std::str::FromStr for #struct_name {
                        type Err = <#field_ty as std::str::FromStr>::Err;

                        fn from_str(__: &str) -> std::result::Result<Self, Self::Err> {
                            Ok(Self(__.parse()?))
                        }
                    }
                }
            }),
    }
}

fn impl_deref(input: &syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field_ident = &field.ident;
    let field_ty = &field.ty;

    field_ident.as_ref().map_or_else(
        || {
            quote! {
                impl std::ops::Deref for #struct_name {
                    type Target = #field_ty;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
            }
        },
        |field_name| {
            quote! {
                impl std::ops::Deref for #struct_name {
                    type Target = #field_ty;

                    fn deref(&self) -> &Self::Target {
                        &self. #field_name
                    }
                }
            }
        },
    )
}

fn impl_into(input: &syn::DeriveInput, via: Option<&syn::Path>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field_ident = &field.ident;
    let field_ty = &field.ty;

    via.map_or_else(
        || {
            field_ident.as_ref().map_or_else(
                || {
                    quote! {
                        impl From<#struct_name> for #field_ty {
                            fn from(__: #struct_name) -> Self {
                                __.0
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl From<#struct_name> for #field_ty {
                            fn from(__: #struct_name) -> Self {
                                __. #field_name
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl From<#struct_name> for #via {
                    fn from(__: #struct_name) -> Self {
                        let de: &#via = &*__;
                        de.to_owned()
                    }
                }
            }
        },
    )
}

fn impl_from(input: &syn::DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field_ident = &field.ident;
    let field_ty = &field.ty;

    field_ident.as_ref().map_or_else(
        || {
            quote! {
                impl From<#field_ty> for #struct_name {
                    fn from(__: #field_ty) -> Self {
                        Self(__)
                    }
                }
            }
        },
        |field_name| {
            quote! {
                impl From<#field_ty> for #struct_name {
                    fn from(#field_name: #field_ty) -> Self {
                        Self{ #field_name }
                    }
                }
            }
        },
    )
}

fn impl_display(input: &syn::DeriveInput, via: Option<&syn::Path>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl std::fmt::Display for #struct_name {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                write!(f, "{}", self.0)
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl std::fmt::Display for #struct_name {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                write!(f, "{}", self. #field_name)
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl std::fmt::Display for #struct_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let de: &#via = self;
                        write!(f, "{}", de)
                    }
                }
            }
        },
    )
}

fn impl_eq(input: &syn::DeriveInput, via: Option<&syn::Path>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl std::cmp::PartialEq for #struct_name {
                            fn eq(&self, other: &Self) -> bool {
                                self.0.eq(&other.0)
                            }
                        }

                        impl std::cmp::Eq for #struct_name {}
                    }
                },
                |field_name| {
                    quote! {
                        impl std::cmp::PartialEq for #struct_name {
                            fn eq(&self, other: &Self) -> bool {
                                self.#field_name.eq(&other.#field_name)
                            }
                        }

                        impl std::cmp::Eq for #struct_name {}
                    }
                },
            )
        },
        |via| {
            quote! {
                impl std::cmp::PartialEq for #struct_name {
                    fn eq(&self, other: &Self) -> bool {
                        let left: &#via = &*self;
                        let right: &#via = &*other;
                        left.eq(right)
                    }
                }

                impl std::cmp::Eq for #struct_name {}
            }
        },
    )
}

fn impl_ord(input: &syn::DeriveInput, via: Option<&syn::Path>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl Ord for #struct_name {
                            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                                self.0.cmp(&other.0)
                            }
                        }

                        impl PartialOrd for #struct_name {
                            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                                self.0.partial_cmp(&other.0)
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl Ord for #struct_name {
                            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                                self.#field_name.cmp(&other.#field_name)
                            }
                        }

                        impl PartialOrd for #struct_name {
                            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                                self.#field_name.partial_cmp(&other.#field_name)
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl Ord for #struct_name {
                    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                        type De = <#via as std::ops::Deref>::Target;
                        let left: &De = &*self;
                        let right: &De = &*other;
                        left.cmp(right)
                    }
                }

                impl PartialOrd for #struct_name {
                    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                        let left: &#via = &*self;
                        let right: &#via = &*other;
                        left.partial_cmp(right)
                    }
                }
            }
        },
    )
}

fn impl_hash(input: &syn::DeriveInput, via: Option<&syn::Path>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl std::hash::Hash for #struct_name {
                            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                                self.0.hash(state);
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl std::hash::Hash for #struct_name {
                            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                                self.#field_name.hash(state);
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl std::hash::Hash for #struct_name {
                    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                        let de: &#via = &*self;
                        de.hash(state);
                    }
                }
            }
        },
    )
}

fn impl_serialize(input: &syn::DeriveInput, via: Option<&syn::Path>) -> TokenStream {
    let struct_name = &input.ident;
    let field = extract_single_field(input);
    let field = &field.ident;

    via.map_or_else(
        || {
            field.as_ref().map_or_else(
                || {
                    quote! {
                        impl serde::Serialize for #struct_name {
                            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                                self.0.serialize(serializer)
                            }
                        }
                    }
                },
                |field_name| {
                    quote! {
                        impl serde::Serialize for #struct_name {
                            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                                self.#field_name.serialize(serializer)
                            }
                        }
                    }
                },
            )
        },
        |via| {
            quote! {
                impl serde::Serialize for #struct_name {
                    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                        let de: &#via = &*self;
                        de.serialize(serializer)
                    }
                }
            }
        },
    )
}
