use crate::{Encode, BufMut, Decode, Buf, DecodeResult, DecodeError};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

use core::cmp::Ordering::{self, *};

impl Encode for Ordering {
    #[inline]
    fn len(&self) -> usize {
        size_of::<u8>()
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(&match self {
            Less => 0u8,
            Equal => 1u8,
            Greater => 2u8,
        });
    }

    #[cfg(not(feature = "alloc"))]
    #[inline]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        buf.encode(&match self {
            Less => 0u8,
            Equal => 1u8,
            Greater => 2u8,
        })?;

        Ok(())
    }
}

impl Decode<'_> for Ordering {
    #[inline]
    fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
        match buf.decode::<u8>()? {
            0 => Ok(Less),
            1 => Ok(Equal),
            2 => Ok(Greater),
            d => Err(DecodeError::InvalidDiscriminant(d)),
        }
    }
}

