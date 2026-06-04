use crate::ir::*;
use proc_macro2::{Span, TokenStream};
use syn::{Result, Error};

pub fn parse(input: TokenStream) -> Result<Input> {
    let input: syn::DeriveInput = syn::parse2(input)?;
    Input::from_syn(input)
}

/// A type that can be created from a [`syn`] value.
trait FromSyn: Sized {
    type Syn;

    fn from_syn(syn: Self::Syn) -> Result<Self>;
}

impl FromSyn for Input {
    type Syn = syn::DeriveInput;

    fn from_syn(syn: Self::Syn) -> Result<Self> {
        Ok(Self {
            attrs: InputAttrs::from_syn(syn.attrs)?,
            ident: syn.ident,
            generics: syn.generics,
            data: Data::from_syn(syn.data)?,
        })
    }
}

impl FromSyn for InputAttrs {
    type Syn = Vec<syn::Attribute>;

    fn from_syn(syn: Self::Syn) -> Result<Self> {
        let mut krate = None;
        let mut borrow = None;

        for attr in syn {
            if !attr.path().is_ident("sio2") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                let ident = meta
                    .path
                    .get_ident()
                    .map(ToString::to_string);

                let content;
                match ident.as_deref() {
                    Some("crate") => {
                        syn::parenthesized!(content in meta.input);
                        krate = Some(content.parse()?);
                    }
                    Some("borrow") => {
                        syn::parenthesized!(content in meta.input);
                        borrow = Some(content.parse()?);
                    }
                    _ => {}
                }

                Ok(())
            })?;
        }

        Ok(Self { krate, borrow })
    }
}

impl FromSyn for Data {
    type Syn = syn::Data;

    fn from_syn(syn: Self::Syn) -> Result<Self> {
        match syn {
            syn::Data::Struct(v) => Ok(Self::Struct(Struct::from_syn(v)?)),
            syn::Data::Enum(v) => Ok(Self::Enum(Enum::from_syn(v)?)),
            syn::Data::Union(v) => Err(Error::new_spanned(
                v.union_token,
                "unions are not supported",
            )),
        }
    }
}

impl FromSyn for Struct {
    type Syn = syn::DataStruct;

    fn from_syn(syn: Self::Syn) -> Result<Self> {
        Ok(Self { fields: Fields::from_syn(syn.fields)? })
    }
}

impl FromSyn for Enum {
    type Syn = syn::DataEnum;

    fn from_syn(syn: Self::Syn) -> Result<Self> {
        let variants = Vec::from_iter(syn.variants);
        Ok(Self { variants: Variants::from_syn(variants)? })
    }
}

impl FromSyn for Variants {
    type Syn = Vec<syn::Variant>;

    fn from_syn(syn: Self::Syn) -> Result<Self> {
        let variants = syn.into_iter()
            .map(Variant::from_syn)
            .collect::<Result<Vec<_>>>()?;

        match variants.len() {
            0 => Err(Error::new(Span::call_site(), "enums must have at least 1 variant")),
            1..=255 => Ok(Self(variants)),
            _ => Err(Error::new(Span::call_site(), "enums cannot have more than 255 variants")),
        }
    }
}

impl FromSyn for Variant {
    type Syn = syn::Variant;

    fn from_syn(syn: Self::Syn) -> Result<Self> {
        Ok(Self {
            ident: syn.ident,
            fields: Fields::from_syn(syn.fields)?,
        })
    }
}

impl FromSyn for Fields {
    type Syn = syn::Fields;

    fn from_syn(syn: Self::Syn) -> Result<Self> {
        Ok(match syn {
            syn::Fields::Named(f) => Self::Named(f
                .named
                .into_iter()
                .map(NamedField::from_syn)
                .collect::<Result<_>>()?),
            syn::Fields::Unnamed(f) => Self::Unnamed(f
                .unnamed
                .into_iter()
                .map(UnnamedField::from_syn)
                .collect::<Result<_>>()?),
            syn::Fields::Unit => Self::Unit,
        })
    }
}

impl FromSyn for NamedField {
    type Syn = syn::Field;

    fn from_syn(syn: Self::Syn) -> Result<Self> {
        assert!(syn.ident.is_some(), "`NamedField::from_syn` called on unnamed field");
        Ok(Self { ident: syn.ident.unwrap() })
    }
}

impl FromSyn for UnnamedField {
    type Syn = syn::Field;

    fn from_syn(syn: Self::Syn) -> Result<Self> {
        assert!(syn.ident.is_none(), "`UnnamedField::from_syn` called on named field");
        Ok(Self)
    }
}
