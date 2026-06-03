use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

impl<E> Encode for [E]
where
    E: Encode,
{
    fn len(&self) -> usize {
        self.iter()
            .map(Encode::len)
            .sum()
    }

    #[cfg(feature = "alloc")]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(&self.len());
        for item in self {
            buf.encode(item);
        }
    }

    #[cfg(not(feature = "alloc"))]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        buf.encode(&self.len())?;
        for item in self {
            buf.encode(item)?;
        }

        Ok(())
    }
}

impl<'buf> Decode<'buf> for &'buf [u8] {
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        let len: usize = buf.decode()?;
        buf.take(len)
    }
}

#[cfg(feature = "alloc")]
impl<'buf, D> Decode<'buf> for Box<[D]>
where
    D: Decode<'buf>,
{
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        buf.decode::<Vec<D>>().map(Vec::into_boxed_slice)
    }
}
