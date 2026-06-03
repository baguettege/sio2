use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

use core::num::{Wrapping, Saturating};
use core::cmp::Reverse;

macro_rules! impl_newtypes {
    ($($T:ident),* $(,)?) => {
        $(
            impl<E> Encode for $T<E>
            where
                E: Encode,
            {
                #[inline]
                fn len(&self) -> usize {
                    self.0.len()
                }

                #[cfg(feature = "alloc")]
                #[inline]
                fn encode(&self, buf: &mut BufMut) {
                    buf.encode(&self.0);
                }

                #[cfg(not(feature = "alloc"))]
                #[inline]
                fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
                    buf.encode(&self.0)?;
                    Ok(())
                }
            }

            impl<'buf, D> Decode<'buf> for $T<D>
            where
                D: Decode<'buf>,
            {
                #[inline]
                fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
                    Ok(Self(buf.decode::<D>()?))
                }
            }
        )*
    };
}

impl_newtypes!(Wrapping, Saturating, Reverse);
