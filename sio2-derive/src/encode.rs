use crate::ir::Input;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

/// Expands `input` into an implementation of [`Encode`] for the annotated type.
pub fn expand(input: Input) -> TokenStream {
    self::input(input)
}

fn input(input: Input) -> TokenStream {
    let krate = input
        .attrs
        .krate
        .clone()
        .unwrap_or_else(|| parse_quote!(::sio2));
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    #[cfg(feature = "alloc")]
    let imports = quote! {
        use #krate::__private::{
            Encode as __Encode,
            BufMut as __BufMut,
            size_of as __size_of,
        };
    };
    #[cfg(not(feature = "alloc"))]
    let imports = quote! {
        use #krate::__private::{
            Encode as __Encode,
            BufMut as __BufMut,
            EncodeResult as __EncodeResult,
            size_of as __size_of,
        };
    };

    let len = len::expand(&input);
    let encode = encode::expand(&input);

    let impl_ = quote! {
        impl #impl_generics __Encode for #ident #ty_generics #where_clause {
            #len
            #encode
        }
    };

    quote! {
        const _: () = {
            #imports
            #impl_
        };
    }
}

mod len {
    use crate::ir::{Data, Enum, Fields, Input, Struct, Variant, Variants};
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};

    /// Expands `input` into the [`Encode::len`] method.
    pub fn expand(input: &Input) -> TokenStream {
        let body = data(&input.data);

        quote! {
            fn len(&self) -> usize {
                #body
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
        let pattern = super::fields::pattern(&input.fields);
        let fields = fields(&input.fields);

        quote! {
            let Self #pattern = self;
            #fields
        }
    }

    fn enum_(input: &Enum) -> TokenStream {
        let variants = variants(&input.variants);

        quote! {
            __size_of::<u8>() + match self {
                #variants
            }
        }
    }

    fn variants(input: &Variants) -> TokenStream {
        input.0
            .iter()
            .map(variant)
            .collect()
    }

    fn variant(input: &Variant) -> TokenStream {
        let ident = &input.ident;
        let pattern = super::fields::pattern(&input.fields);
        let fields = fields(&input.fields);

        quote! {
            Self::#ident #pattern => {
                #fields
            }
        }
    }

    fn fields(input: &Fields) -> TokenStream {
        let fields = match input {
            Fields::Named(fields) => fields
                .iter()
                .map(|f| {
                    let mangled = format_ident!("__{}", f.ident);
                    quote! { + __Encode::len(#mangled) }
                })
                .collect(),
            Fields::Unnamed(fields) => fields
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let mangled = format_ident!("__self_{i}");
                    quote! { + __Encode::len(#mangled) }
                })
                .collect(),
            Fields::Unit => quote! {},
        };

        quote! { 0usize #fields }
    }
}

#[allow(clippy::module_inception)] // outer = trait, inner = method
mod encode {
    use crate::ir::{Data, Enum, Fields, Input, Struct, Variant, Variants};
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};

    #[cfg(feature = "alloc")]
    macro_rules! buf_encode {
        ($value:ident) => {
            quote! { __buf.encode(#$value); }
        };
        (&$value:ident) => {
            quote! { __buf.encode(&#$value); }
        };
    }

    #[cfg(not(feature = "alloc"))]
    macro_rules! buf_encode {
        ($value:ident) => {
            quote! { __buf.encode(#$value)?; }
        };
        (&$value:ident) => {
            quote! { __buf.encode(&#$value)?; }
        };
    }

    /// Expands `input` into the [`Encode::encode`] method.
    #[cfg(feature = "alloc")]
    pub fn expand(input: &Input) -> TokenStream {
        let body = data(&input.data);

        quote! {
            #[allow(unused_variables)]
            fn encode(&self, __buf: &mut __BufMut) {
                #body
            }
        }
    }

    /// Expands `input` into the [`Encode::encode`] method.
    #[cfg(not(feature = "alloc"))]
    pub fn expand(input: &Input) -> TokenStream {
        let body = data(&input.data);

        quote! {
            #[allow(unused_variables)]
            fn encode(&self, __buf: &mut __BufMut) -> __EncodeResult<()> {
                #body
                __EncodeResult::Ok(())
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
        let pattern = super::fields::pattern(&input.fields);
        let fields = fields(&input.fields);

        quote! {
            let Self #pattern = self;
            #fields
        }
    }

    fn enum_(input: &Enum) -> TokenStream {
        let variants = variants(&input.variants);

        quote! {
            match self {
                #variants
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
        let pattern = super::fields::pattern(&input.fields);
        let fields = fields(&input.fields);
        let discriminant = buf_encode!(&discriminant);

        quote! {
            Self::#ident #pattern => {
                #discriminant
                #fields
            }
        }
    }

    fn fields(input: &Fields) -> TokenStream {
        match input {
            Fields::Named(fields) => fields
                .iter()
                .map(|f| {
                    let mangled = format_ident!("__{}", f.ident);
                    buf_encode!(mangled)
                })
                .collect(),
            Fields::Unnamed(fields) => fields
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let mangled = format_ident!("__self_{i}");
                    buf_encode!(mangled)
                })
                .collect(),
            Fields::Unit => quote! {},
        }
    }
}

mod fields {
    use crate::ir::Fields;
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};

    /// Expands `input` into a struct/variant destructuring pattern.
    pub fn pattern(input: &Fields) -> TokenStream {
        match input {
            Fields::Named(fields) => {
                let fields = fields
                    .iter()
                    .map(|f| {
                        let ident = &f.ident;
                        let mangled = format_ident!("__{ident}");
                        quote! { #ident: #mangled }
                    });
                quote! { { #(#fields),* } }
            }
            Fields::Unnamed(fields) => {
                let fields = fields
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let mangled = format_ident!("__self_{i}");
                        quote! { #mangled }
                    });
                quote! { (#(#fields),*) }
            }
            Fields::Unit => quote! {},
        }
    }
}
