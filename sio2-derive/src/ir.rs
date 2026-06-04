pub struct Input {
    pub attrs: InputAttrs,
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: Data,
}

pub struct InputAttrs {
    /// `#[sio2(crate(::silica))]`
    pub krate: Option<syn::Path>,
    /// `#[sio2(borrow('buf))]`
    pub borrow: Option<syn::Lifetime>,
}

pub enum Data {
    Struct(Struct),
    Enum(Enum),
}

pub struct Struct {
    pub fields: Fields,
}

pub struct Enum {
    pub variants: Variants,
}

/// Always contains between 1 and 255 (inclusive) variants.
pub struct Variants(pub Vec<Variant>);

pub struct Variant {
    pub ident: syn::Ident,
    pub fields: Fields,
}

pub enum Fields {
    Named(Vec<NamedField>),
    Unnamed(Vec<UnnamedField>),
    Unit,
}

pub struct NamedField {
    pub ident: syn::Ident,
}

pub struct UnnamedField;
