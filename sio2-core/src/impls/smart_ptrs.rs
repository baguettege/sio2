#![cfg(feature = "alloc")]

use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;

macro_rules! impl_smart_ptr {
    ($($T:ident),* $(,)?) => {
        $(
            impl<E> Encode for $T<E>
            where
                E: Encode + ?Sized,
            {
                fn len(&self) -> usize {
                    E::len(self.as_ref())
                }
                
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

impl_smart_ptr!(Box, Rc, Arc);
