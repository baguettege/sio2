use crate::{Encode, BufMut, Decode, Buf, DecodeResult, DecodeError};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

impl<E> Encode for Option<E>
where
    E: Encode,
{
    #[inline]
    fn len(&self) -> usize {
        size_of::<u8>() + match self {
            None => 0,
            Some(v) => v.len(),
        }
    }

    #[cfg(feature = "alloc")]
    fn encode(&self, buf: &mut BufMut) {
        match self {
            None => buf
                .encode(&0u8),
            Some(v) => buf
                .encode(&1u8)
                .encode(v),
        };
    }

    #[cfg(not(feature = "alloc"))]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        match self {
            None => buf
                .encode(&0u8)?,
            Some(v) => buf
                .encode(&1u8)?
                .encode(v)?,
        };

        Ok(())
    }
}

impl<'buf, D> Decode<'buf> for Option<D>
where
    D: Decode<'buf>,
{
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        match buf.decode::<u8>()? {
            0 => Ok(None),
            1 => Ok(Some(buf.decode::<D>()?)),
            d => Err(DecodeError::InvalidDiscriminant(d)),
        }
    }
}
