#![cfg(feature = "alloc")]

use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
use alloc::vec::Vec;

impl<E> Encode for Vec<E>
where
    E: Encode,
{
    fn len(&self) -> usize {
        let items: usize = self
            .iter()
            .map(Encode::len)
            .sum();
        Encode::len(&self.len()) + items
    }

    fn encode(&self, buf: &mut BufMut) {
        buf.encode(&self.len());
        for item in self {
            buf.encode(item);
        }
    }
}

impl<'buf, D> Decode<'buf> for Vec<D>
where
    D: Decode<'buf>,
{
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        let len: usize = buf.decode()?;
        (0..len).map(|_| buf.decode()).collect()
    }
}
