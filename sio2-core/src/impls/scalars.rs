use crate::{Encode, BufMut, Decode, Buf, DecodeResult, DecodeError};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

#[cfg(feature = "alloc")]
impl Encode for bool {
    #[inline]
    fn len(&self) -> usize {
        size_of::<u8>()
    }

    #[inline]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(&(*self as u8));
    }
}

#[cfg(not(feature = "alloc"))]
impl Encode for bool {
    #[inline]
    fn len(&self) -> usize {
        size_of::<u8>()
    }

    #[inline]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        buf.encode(&(*self as u8))?;
        Ok(())
    }
}

impl Decode<'_> for bool {
    fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
        match buf.decode::<u8>()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::InvalidBool),
        }
    }
}

#[cfg(feature = "alloc")]
impl Encode for char {
    #[inline]
    fn len(&self) -> usize {
        size_of::<u32>()
    }

    #[inline]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(&(*self as u32));
    }
}

#[cfg(not(feature = "alloc"))]
impl Encode for char {
    #[inline]
    fn len(&self) -> usize {
        size_of::<u32>()
    }

    #[inline]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        buf.encode(&(*self as u32))?;
        Ok(())
    }
}

impl Decode<'_> for char {
    fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
        let codepoint: u32 = buf.decode()?;
        char::from_u32(codepoint).ok_or(DecodeError::InvalidChar)
    }
}
