use crate::{Encode, BufMut, Decode, Buf, DecodeResult, DecodeError};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

impl<T, E> Encode for Result<T, E>
where
    T: Encode,
    E: Encode,
{
    #[inline]
    fn len(&self) -> usize {
        size_of::<u8>() + match self {
            Ok(v) => v.len(),
            Err(e) => e.len(),
        }
    }

    #[cfg(feature = "alloc")]
    fn encode(&self, buf: &mut BufMut) {
        match self {
            Ok(v) => buf
                .encode(&0u8)
                .encode(v),
            Err(e) => buf
                .encode(&1u8)
                .encode(e),
        };
    }

    #[cfg(not(feature = "alloc"))]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        match self {
            Ok(v) => buf
                .encode(&0u8)?
                .encode(v)?,
            Err(e) => buf
                .encode(&1u8)?
                .encode(e)?,
        };

        Ok(())
    }
}

impl<'buf, T, E> Decode<'buf> for Result<T, E>
where
    T: Decode<'buf>,
    E: Decode<'buf>,
{
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        match buf.decode::<u8>()? {
            0 => Ok(Ok(buf.decode::<T>()?)),
            1 => Ok(Err(buf.decode::<E>()?)),
            d => Err(DecodeError::InvalidDiscriminant(d)),
        }
    }
}
