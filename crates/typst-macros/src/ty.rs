use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, Result, Token};

use crate::util::{
    determine_name_and_title, documentation, foundations, has_attr, kw, parse_flag,
    parse_string, parse_string_array, BareType,
};

/// Expand the `#[ty]` macro.
pub fn ty(stream: TokenStream, mut item: syn::Item) -> Result<TokenStream> {
    let bare: BareType;

    let meta: Meta = syn::parse2(stream)?;
    let (ident, attrs, keep) = match &mut item {
        // A struct. This is the most common case, which profits the most from
        // the macro. It implements various traits, generates accessor methods
        // for fields, etc.
        syn::Item::Struct(item) => {
            process_struct(item);
            (&item.ident, &item.attrs, true)
        }

        // An enum. Always opaque because Typst doesn't have them.
        // They cannot have an auto-generated constructor, Repr, etc.
        syn::Item::Enum(item) => (&item.ident, &item.attrs, true),

        // A bare type like `type f64`. This is only used for a few primitives.
        syn::Item::Verbatim(item) => {
            bare = syn::parse2(item.clone())?;
            (&bare.ident, &bare.attrs, false)
        }

        _ => bail!(item, "invalid type item"),
    };

    let ty = parse(meta, ident.clone(), attrs)?;
    Ok(create(&ty, keep.then_some(&item)))
}

/// Rewrites a struct definition.
fn process_struct(item: &mut syn::ItemStruct) {
    for field in &mut item.fields {
        for s in [
            "required",
            "ghost",
            "positional",
            "variadic",
            "parse",
            "external",
            "resolve",
            "fold",
            "borrowed",
            "internal",
            "default",
        ] {
            let _ = has_attr(&mut field.attrs, s);
        }
    }
}

/// Holds all relevant parsed data about a type.
struct Type {
    meta: Meta,
    ident: Ident,
    name: String,
    long: String,
    title: String,
    docs: String,
}

/// The `..` in `#[ty(..)]`.
struct Meta {
    scope: bool,
    cast: bool,
    name: Option<String>,
    title: Option<String>,
    keywords: Vec<String>,
    scope: bool,
    cast: bool,
    capabilities: Vec<Ident>,
}

impl Meta {
    fn can(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }
}

impl Parse for Meta {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            scope: parse_flag::<kw::scope>(input)?,
            cast: parse_flag::<kw::cast>(input)?,
            name: parse_string::<kw::name>(input)?,
            title: parse_string::<kw::title>(input)?,
            keywords: parse_string_array::<kw::keywords>(input)?,
            scope: parse_flag::<kw::scope>(input)?,
            cast: parse_flag::<kw::cast>(input)?,
            capabilities: Punctuated::<Ident, Token![,]>::parse_terminated(input)?
                .into_iter()
                .collect(),
        })
    }
}

/// Parse details about the type from its definition.
fn parse(meta: Meta, ident: Ident, attrs: &[Attribute]) -> Result<Type> {
    let docs = documentation(attrs);
    let (name, title) =
        determine_name_and_title(meta.name.clone(), meta.title.clone(), &ident, None)?;
    let long = title.to_lowercase();
    Ok(Type { meta, ident, name, long, title, docs })
}

/// Produce the output of the macro.
fn create(ty: &Type, item: Option<&syn::Item>) -> TokenStream {
    let Type { ident, .. } = ty;

    let native_type = create_native_type_impl(ty);
    let cast = create_cast_impl(ty);
    let repr = create_repr_impl(ty);
    let construct = create_construct_impl(ty);
    let set = create_set_impl(ty);
    let locatable = create_locatable_impl(ty);

    let hack = item.map(|_|quote!{
        impl #ident {
            pub fn span(&self) -> ::typst::syntax::Span { todo!() }
            pub fn spanned(self, _span: ::typst::syntax::Span) -> Self { todo!() }
            pub fn location(&self) -> ::std::option::Option<::typst::introspection::Location> { todo!() }
            pub fn set_location(&self, _loc: ::typst::introspection::Location) { todo!() }
            pub fn pack(self) -> #foundations::Value { todo!() }
        }
    });

    quote! {
        #item
        #hack
        #native_type
        #cast
        #repr
        #construct
        #set
        #locatable
    }
}

/// Creates the `NativeType` implementation for the type.
///
/// This trait is always implemented automatically.
fn create_native_type_impl(ty: &Type) -> TokenStream {
    let Type { ident, name, long, title, docs, meta, .. } = ty;
    let Meta { keywords, .. } = meta;

    let constructor = if ty.meta.scope {
        quote! { <#ident as #foundations::NativeScope>::constructor() }
    } else {
        quote! { None }
    };

    let scope = if ty.meta.scope {
        quote! { <#ident as #foundations::NativeScope>::scope() }
    } else {
        quote! { #foundations::Scope::new() }
    };

    let cast = (!meta.cast).then(|| {
        quote! {
            #foundations::cast! { type #ident, }
        }
    });

    let data = quote! {
        #foundations::NativeTypeData {
            name: #name,
            long_name: #long,
            title: #title,
            docs: #docs,
            keywords: &[#(#keywords),*],
            constructor: #foundations::Lazy::new(|| #constructor),
            scope: #foundations::Lazy::new(|| #scope),
        }
    };

    quote! {
        impl #foundations::NativeType for #ident {
            const NAME: &'static str = #name;

            fn data() -> &'static #foundations::NativeTypeData {
                static DATA: #foundations::NativeTypeData = #data;
                &DATA
            }
        }
    }
}

/// Creates the `Reflect`, `IntoValue`, and `FromValue` implementations for
/// the type. Returns `None` if `#[ty(cast)]` is specified because the
/// implementations are provided manually or via the `cast!` macro.
fn create_cast_impl(ty: &Type) -> Option<TokenStream> {
    if ty.meta.cast {
        return None;
    }

    let Type { ident, .. } = ty;
    Some(quote! {
        impl #foundations::Reflect for #ident {
            fn input() -> #foundations::CastInfo {
                todo!()
            }

            fn output() -> #foundations::CastInfo {
                todo!()
            }

            fn castable(value: &#foundations::Value) -> bool {
                todo!()
            }
        }

        impl #foundations::IntoValue for #ident {
            fn into_value(self) -> #foundations::Value {
                todo!()
            }
        }

        impl #foundations::FromValue for #ident {
            fn from_value(value: #foundations::Value) -> ::typst::diag::StrResult<Self> {
                todo!()
            }
        }
    })
}

/// Creates the `Repr` implementation for the type. Returns `None` if
/// `#[ty(Repr)]` is specified because the trait is implemented manually.
fn create_repr_impl(ty: &Type) -> Option<TokenStream> {
    if ty.meta.can("Repr") {
        return None;
    }

    let Type { ident, .. } = ty;
    Some(quote! {
        impl #foundations::Repr for #ident {
            fn repr(&self) -> ::ecow::EcoString {
                todo!()
            }
        }
    })
}

/// Creates the `Construct` implementation for the type. Returns `None` if
/// `#[ty(Construct)]` or `#[ty(!Construct)]` is specified because the
/// constructor is implemeneted manually or is absent.
fn create_construct_impl(ty: &Type) -> Option<TokenStream> {
    if ty.meta.can("Construct") {
        return None;
    }

    let Type { ident, .. } = ty;
    Some(quote! {
        impl #foundations::Construct for #ident {
            fn construct(
                engine: &mut ::typst::engine::Engine,
                args: &mut #foundations::Args,
            ) -> ::typst::diag::SourceResult<#foundations::Value> {
                todo!()
            }
        }
    })
}

/// Create the `Set` implementation for the type.
///
/// This trait is always implemented automatically.
fn create_set_impl(ty: &Type) -> TokenStream {
    let Type { ident, .. } = ty;
    quote! {
        impl #foundations::Set for #ident {
            fn set(
                engine: &mut ::typst::engine::Engine,
                args: &mut #foundations::Args,
            ) -> ::typst::diag::SourceResult<#foundations::Styles> {
                Ok(#foundations::Styles::new())
            }
        }
    }
}

/// Create the `Locatable` implementation for the type.
///
/// This trait is only implemented if `#[ty(Locatable)]` is specified.
fn create_locatable_impl(ty: &Type) -> Option<TokenStream> {
    if !ty.meta.can("Locatable") {
        return None;
    }

    let Type { ident, .. } = ty;
    Some(quote! {
        impl ::typst::introspection::Locatable for #ident {}
    })
}
