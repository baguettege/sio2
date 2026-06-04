#![cfg(feature = "alloc")]

use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
use alloc::borrow::{Cow, ToOwned};

impl<E> Encode for Cow<'_, E>
where
    E: Encode + ToOwned + ?Sized,
{
    #[inline]
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    #[inline]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(self.as_ref());
    }
}

impl<'buf, D> Decode<'buf> for Cow<'buf, D>
where
    D: ToOwned + ?Sized,
    &'buf D: Decode<'buf>,
{
    #[inline]
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        Ok(Self::Borrowed(buf.decode::<&D>()?))
    }
}
