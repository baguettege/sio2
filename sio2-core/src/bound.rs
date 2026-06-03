use crate::{Encode, BufMut, Decode, Buf, DecodeResult, DecodeError};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

use core::ops::Bound::{self, *};

impl<E> Encode for Bound<E>
where
    E: Encode,
{
    #[inline]
    fn len(&self) -> usize {
        size_of::<u8>() + match self {
            Included(v) | Excluded(v) => v.len(),
            Unbounded => 0usize,
        }
    }

    #[cfg(feature = "alloc")]
    fn encode(&self, buf: &mut BufMut) {
        match self {
            Included(v) => buf
                .encode(&0u8)
                .encode(v),
            Excluded(v) => buf
                .encode(&1u8)
                .encode(v),
            Unbounded => buf
                .encode(&2u8),
        };
    }

    #[cfg(not(feature = "alloc"))]
    fn encode(&self, buf: &mut BufMut) -> crate::encode::error::EncodeResult<()> {
        match self {
            Included(v) => buf
                .encode(&0u8)?
                .encode(v)?,
            Excluded(v) => buf
                .encode(&1u8)?
                .encode(v)?,
            Unbounded => buf
                .encode(&2u8)?,
        };

        Ok(())
    }
}

impl<'buf, D> Decode<'buf> for Bound<D>
where
    D: Decode<'buf>,
{
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        match buf.decode::<u8>()? {
            0 => Ok(Included(buf.decode()?)),
            1 => Ok(Excluded(buf.decode()?)),
            2 => Ok(Unbounded),
            d => Err(DecodeError::InvalidDiscriminant(d as u16)),
        }
    }
}
