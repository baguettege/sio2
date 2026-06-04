use crate::ir::Input;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

/// Expands `input` into an implementation of [`Decode`] for the annotated type.
pub fn expand(input: Input) -> TokenStream {
    self::input(input)
}

struct Ctx {
    borrow: syn::Lifetime,
}

fn input(input: Input) -> TokenStream {
    let krate = input
        .attrs
        .krate
        .clone()
        .unwrap_or_else(|| parse_quote!(::sio2));
    let borrow = input
        .attrs
        .borrow
        .clone()
        .unwrap_or_else(|| parse_quote!('_));
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let imports = quote! {
        #[allow(unused_imports)]
        use #krate::__private::{
            Decode as __Decode,
            Buf as __Buf,
            DecodeResult as __DecodeResult,
            DecodeError as __DecodeError,
        };
    };

    let ctx = Ctx { borrow: borrow.clone() };
    let decode = decode::expand(&input, &ctx);

    let impl_ = quote! {
        impl #impl_generics __Decode<#borrow> for #ident #ty_generics #where_clause {
            #decode
        }
    };

    quote! {
        const _: () = {
            #imports
            #impl_
        };
    }
}

#[allow(clippy::module_inception)] // outer = trait, inner = method
mod decode {
    use super::Ctx;
    use crate::ir::{Data, Enum, Input, Struct, Variant, Variants};
    use proc_macro2::TokenStream;
    use quote::quote;

    /// Expands `input` into the [`Decode::decode`] method.
    pub fn expand(input: &Input, ctx: &Ctx) -> TokenStream {
        let borrow = &ctx.borrow;
        let body = data(&input.data);

        quote! {
            fn decode(__buf: &mut __Buf<#borrow>) -> __DecodeResult<Self> {
                __DecodeResult::Ok({ #body })
            }
        }
    }

    fn data(input: &Data) -> TokenStream {
        match input {
            Data::Struct(v) => struct_(v),
            Data::Enum(v) => enum_(v),
        }
    }

    fn struct_(input: &Struct) -> TokenStream {
        let constructor = super::fields::constructor(&input.fields);
        quote! { Self #constructor }
    }

    fn enum_(input: &Enum) -> TokenStream {
        let variants = variants(&input.variants);

        quote! {
            match __buf.decode::<u8>()? {
                #variants
                d => return __DecodeResult::Err(
                    __DecodeError::InvalidDiscriminant(d),
                ),
            }
        }
    }

    fn variants(input: &Variants) -> TokenStream {
        input.0
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let discriminant: u8 = i
                    .try_into()
                    .expect("variant count exceeds 255");
                variant(v, discriminant)
            })
            .collect()
    }

    fn variant(input: &Variant, discriminant: u8) -> TokenStream {
        let ident = &input.ident;
        let constructor = super::fields::constructor(&input.fields);

        quote! {
            #discriminant => {
                Self::#ident #constructor
            }
        }
    }
}

mod fields {
    use crate::ir::Fields;
    use proc_macro2::TokenStream;
    use quote::quote;

    /// Expands `input` into a struct/variant constructor pattern.
    pub fn constructor(input: &Fields) -> TokenStream {
        match input {
            Fields::Named(fields) => {
                let fields = fields
                    .iter()
                    .map(|f| {
                        let ident = &f.ident;
                        quote! { #ident: __buf.decode()? }
                    });
                quote! { { #(#fields),* } }
            }
            Fields::Unnamed(fields) => {
                let fields = fields
                    .iter()
                    .map(|_| quote! { __buf.decode()? });
                quote! { (#(#fields),*) }
            }
            Fields::Unit => quote! {},
        }
    }
}
