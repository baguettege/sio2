use crate::{Encode, BufMut};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

macro_rules! impl_ {
    ($($T:ty),* $(,)?) => {
        $(
            impl<E> Encode for $T
            where
                E: Encode,
            {
                fn len(&self) -> usize {
                    E::len(*self)
                }

                #[cfg(feature = "alloc")]
                fn encode(&self, buf: &mut BufMut) {
                    E::encode(*self, buf)
                }
                
                #[cfg(not(feature = "alloc"))]
                fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
                    E::encode(*self, buf)
                }
            }
        )*
    };
}

impl_!(&E, &mut E);
