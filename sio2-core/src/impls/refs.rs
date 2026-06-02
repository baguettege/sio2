use crate::{Encode, BufMut};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

macro_rules! impl_ref {
    ($($T:ty),* $(,)?) => {
        $(
            #[cfg(feature = "alloc")]
            impl<E> Encode for $T
            where
                E: Encode,
            {
                fn len(&self) -> usize {
                    E::len(*self)
                }

                fn encode(&self, buf: &mut BufMut) {
                    E::encode(*self, buf)
                }
            }

            #[cfg(not(feature = "alloc"))]
            impl<E> Encode for $T
            where
                E: Encode,
            {
                fn len(&self) -> usize {
                    E::len(*self)
                }

                fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
                    E::encode(*self, buf)
                }
            }
        )*
    };
}

impl_ref!(&E, &mut E);
