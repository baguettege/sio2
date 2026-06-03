use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

#[cfg(feature = "alloc")]
impl Encode for str {
    #[inline]
    fn len(&self) -> usize {
        Encode::len(&self.len()) + self.len()
    }

    #[inline]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(self.as_bytes());
    }
}

#[cfg(not(feature = "alloc"))]
impl Encode for str {
    #[inline]
    fn len(&self) -> usize {
        Encode::len(&self.len()) + self.len()
    }

    #[inline]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        buf.encode(self.as_bytes())?;
        Ok(())
    }
}

impl<'buf> Decode<'buf> for &'buf str {
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        let bytes: &[u8] = buf.decode()?;
        Ok(str::from_utf8(bytes)?)
    }
}

#[cfg(feature = "alloc")]
impl Encode for String {
    #[inline]
    fn len(&self) -> usize {
        Encode::len(self.as_str())
    }

    #[inline]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(self.as_str());
    }
}

#[cfg(feature = "alloc")]
impl Decode<'_> for String {
    fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
        buf.decode::<&str>().map(str::to_string)
    }
}
