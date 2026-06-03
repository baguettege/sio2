use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

use core::marker::PhantomData;

impl<E> Encode for PhantomData<E> {
    #[inline(always)]
    fn len(&self) -> usize {
        0
    }

    #[cfg(feature = "alloc")]
    #[inline(always)]
    fn encode(&self, _: &mut BufMut) {}

    #[cfg(not(feature = "alloc"))]
    #[inline(always)]
    fn encode(&self, _: &mut BufMut) -> EncodeResult<()> {
        Ok(())
    }
}

impl<D> Decode<'_> for PhantomData<D> {
    #[inline(always)]
    fn decode(_: &mut Buf<'_>) -> DecodeResult<Self> {
        Ok(Self)
    }
}
