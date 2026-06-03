use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

use core::sync::atomic::{
    Ordering,
    AtomicU8, AtomicU16, AtomicU32, AtomicU64,
    AtomicI8, AtomicI16, AtomicI32, AtomicI64,
    AtomicUsize, AtomicIsize,
    AtomicBool,
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
                    buf.encode(&self.load(Ordering::Relaxed));
                }

                #[cfg(not(feature = "alloc"))]
                #[inline]
                fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
                    buf.encode(&self.load(Ordering::Relaxed))?;
                    Ok(())
                }
            }

            impl Decode<'_> for $T {
                #[inline]
                fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
                    Ok(Self::new(buf.decode()?))
                }
            }
        )*
    };
}

impl_!(
    AtomicU8 => u8,
    AtomicU16 => u16,
    AtomicU32 => u32,
    AtomicU64 => u64,

    AtomicI8 => i8,
    AtomicI16 => i16,
    AtomicI32 => i32,
    AtomicI64 => i64,

    AtomicUsize => usize,
    AtomicIsize => isize,

    AtomicBool => u8,
);
