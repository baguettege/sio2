use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

macro_rules! impl_number {
    ($($T:ty),* $(,)?) => {
        $(
            #[cfg(feature = "alloc")]
            impl Encode for $T {
                #[inline]
                fn len(&self) -> usize {
                    size_of::<Self>()
                }

                #[inline]
                fn encode(&self, buf: &mut BufMut) {
                    buf.put(self.to_be_bytes());
                }
            }

            #[cfg(not(feature = "alloc"))]
            impl Encode for $T {
                #[inline]
                fn len(&self) -> usize {
                    size_of::<Self>()
                }

                #[inline]
                fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
                    buf.put(self.to_be_bytes())?;
                    Ok(())
                }
            }

            impl Decode<'_> for $T {
                #[inline]
                fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
                    Ok(Self::from_be_bytes(*buf.take_array()?))
                }
            }
        )*
    };

    ($($T:ty as $To:ty),* $(,)?) => {
        $(
            #[cfg(feature = "alloc")]
            impl Encode for $T {
                #[inline]
                fn len(&self) -> usize {
                    size_of::<$To>()
                }

                #[inline]
                fn encode(&self, buf: &mut BufMut) {
                    buf.encode(&(*self as $To));
                }
            }

            #[cfg(not(feature = "alloc"))]
            impl Encode for $T {
                #[inline]
                fn len(&self) -> usize {
                    size_of::<$To>()
                }

                #[inline]
                fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
                    buf.encode(&(*self as $To))?;
                    Ok(())
                }
            }

            impl Decode<'_> for $T {
                #[inline]
                fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
                    Ok(buf.decode::<$To>()? as Self)
                }
            }
        )*
    };
}

impl_number!(
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
);
impl_number!(
    usize as u64,
    isize as i64,
);


