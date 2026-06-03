#![cfg(feature = "alloc")]

use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;

macro_rules! impl_ {
    ($($T:ident),* $(,)?) => {
        $(
            impl<E> Encode for $T<E>
            where
                E: Encode + ?Sized,
            {
                #[inline]
                fn len(&self) -> usize {
                    E::len(self.as_ref())
                }
                
                #[inline]
                fn encode(&self, buf: &mut BufMut) {
                    E::encode(self.as_ref(), buf)
                }
            }
        
            impl<'buf, D> Decode<'buf> for $T<D>
            where
                D: Decode<'buf>,
            {
                fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
                    buf.decode::<D>().map(Self::new)
                }
            }
        )*
    };
}

impl_!(Box, Rc, Arc);
