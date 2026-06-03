use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

macro_rules! impl_ {
    ($($T:ident),* $(,)?) => {
        impl<$($T,)*> Encode for ($($T,)*)
        where
            $($T: Encode,)*
        {
            #[inline]
            fn len(&self) -> usize {
                #[allow(unused_mut)]
                let mut len = 0usize;

                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $( len += $T.len(); )*

                len
            }

            #[cfg(feature = "alloc")]
            #[allow(unused_variables)]
            fn encode(&self, buf: &mut BufMut) {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $( buf.encode($T); )*
            }

            #[cfg(not(feature = "alloc"))]
            #[allow(unused_variables)]
            fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $( buf.encode($T)?; )*
                Ok(())
            }
        }

        impl<'buf, $($T,)*> Decode<'buf> for ($($T,)*)
        where
            $($T: Decode<'buf>,)*
        {
            #[allow(unused_variables)]
            #[allow(non_snake_case)]
            fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
                $( let $T: $T = buf.decode()?; )*
                Ok(($($T,)*))
            }
        }
    };
}

impl_!();
impl_!(A);
impl_!(A, B);
impl_!(A, B, C);
impl_!(A, B, C, D);
impl_!(A, B, C, D, E);
impl_!(A, B, C, D, E, F);
impl_!(A, B, C, D, E, F, G);
impl_!(A, B, C, D, E, F, G, H);
impl_!(A, B, C, D, E, F, G, H, I);
impl_!(A, B, C, D, E, F, G, H, I, J);
impl_!(A, B, C, D, E, F, G, H, I, J, K);
impl_!(A, B, C, D, E, F, G, H, I, J, K, L);
