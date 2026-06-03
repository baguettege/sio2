use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

use core::cell::{Cell, RefCell};

impl<E> Encode for Cell<E>
where
    E: Encode + Copy,
{
    #[inline]
    fn len(&self) -> usize {
        self.get().len()
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(&self.get());
    }

    #[cfg(not(feature = "alloc"))]
    #[inline]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        buf.encode(&self.get())?;
        Ok(())
    }
}

impl<'buf, D> Decode<'buf> for Cell<D>
where
    D: Decode<'buf>,
{
    #[inline]
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        Ok(Self::new(buf.decode()?))
    }
}

impl<E> Encode for RefCell<E>
where
    E: Encode,
{
    #[inline]
    fn len(&self) -> usize {
        self.borrow().len()
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(&*self.borrow());
    }

    #[cfg(not(feature = "alloc"))]
    #[inline]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        buf.encode(&*self.borrow())?;
        Ok(())
    }
}

impl<'buf, D> Decode<'buf> for RefCell<D>
where
    D: Decode<'buf>,
{
    #[inline]
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        Ok(Self::new(buf.decode()?))
    }
}
