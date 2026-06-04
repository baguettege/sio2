use crate::{Encode, BufMut};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

macro_rules! impl_ {
    ($($T:ty),* $(,)?) => {
        $(
            impl<E> Encode for $T
            where
                E: Encode + ?Sized,
            {
                #[inline]
                fn len(&self) -> usize {
                    (**self).len()
                }

                #[cfg(feature = "alloc")]
                #[inline]
                fn encode(&self, buf: &mut BufMut) {
                    buf.encode(*self);
                }

                #[cfg(not(feature = "alloc"))]
                #[inline]
                fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
                    buf.encode(*self)?;
                    Ok(())
                }
            }
        )*
    };
}

impl_!(&E, &mut E);
