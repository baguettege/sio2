use proc_macro::TokenStream;

mod ir;
mod parse;
mod encode;
mod decode;

/// Generates an [`Encode`] implementation for the annotated struct or enum.
///
/// # Examples
///
/// ```ignore
/// # use sio2_derive as sio2;
/// use sio2::{Encode, ToBytes};
///
/// #[derive(Encode)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// let point = Point { x: 1, y: 2 };
/// assert_eq!(point.len(), size_of::<i32>() * 2);
/// assert_eq!(point.to_bytes(), vec![0, 0, 0, 1, 0, 0, 0, 2]);
/// ```
///
/// # Attributes
///
/// - `#[sio2(crate(...))]` overrides the crate path referenced by the generated implementation
///
/// ```ignore
/// use sio2 as silica;
/// use silica::Encode;
///
/// #[derive(Encode)]
/// #[sio2(crate(silica))]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
/// ```
#[proc_macro_derive(Encode, attributes(sio2))]
pub fn encode(input: TokenStream) -> TokenStream {
    derive(input, encode::expand)
}

/// Generates a [`Decode`] implementation for the annotated struct or enum.
///
/// # Examples
///
/// ```ignore
/// use sio2::{Decode, Buf};
///
/// #[derive(Decode)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// let bytes = [0, 0, 0, 1, 0, 0, 0, 2];
/// assert!(matches!(
///     Buf::new(&bytes).decode::<Point>(),
///     Ok(Point { x: 1, y: 2 }),
/// ))
/// ```
///
/// # Attributes
///
/// - `#[sio2(crate(...))]` overrides the crate path referenced by the generated implementation
/// - `#[sio2(borrow('...))]` sets the input-buffer lifetime used in the generated
///   `Decode<'...>` implementation to allow for decoded fields to borrow from the buffer
///
/// ```ignore
/// use sio2 as silica;
/// use silica::Decode;
///
/// #[derive(Decode)]
/// #[sio2(crate(silica), borrow('a))]
/// struct Foo<'a> {
///     bytes: &'a [u8],
/// }
/// ```
#[proc_macro_derive(Decode, attributes(sio2))]
pub fn decode(input: TokenStream) -> TokenStream {
    derive(input, decode::expand)
}

fn derive<F>(input: TokenStream, expand: F) -> TokenStream
where
    F: FnOnce(ir::Input) -> proc_macro2::TokenStream,
{
    parse::parse(input.into())
        .map(expand)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
