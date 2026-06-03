use crate::{Encode, BufMut, Decode, Buf, DecodeResult, DecodeError};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

use core::num::{
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128,
    NonZeroUsize, NonZeroIsize,
};

macro_rules! impl_ {
    ($($T:ty => $Int:ty),* $(,)?) => {
        $(
            impl Encode for $T {
                #[inline]
                fn len(&self) -> usize {
                    size_of::<$Int>()
                }

                #[cfg(feature = "alloc")]
                #[inline]
                fn encode(&self, buf: &mut BufMut) {
                    buf.encode(&self.get());
                }

                #[cfg(not(feature = "alloc"))]
                #[inline]
                fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
                    buf.encode(&self.get())?;
                    Ok(())
                }
            }

            impl Decode<'_> for $T {
                fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
                    let v: $Int = buf.decode()?;
                    Self::new(v).ok_or(DecodeError::InvalidNonZero)
                }
            }
        )*
    };
}

impl_!(
    NonZeroU8 => u8,
    NonZeroU16 => u16,
    NonZeroU32 => u32,
    NonZeroU64 => u64,
    NonZeroU128 => u128,

    NonZeroI8 => i8,
    NonZeroI16 => i16,
    NonZeroI32 => i32,
    NonZeroI64 => i64,
    NonZeroI128 => i128,

    NonZeroUsize => usize,
    NonZeroIsize => isize,
);
