use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

use core::ffi::CStr;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::ffi::CString;

impl Encode for str {
    #[inline]
    fn len(&self) -> usize {
        Encode::len(&self.len()) + self.len()
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(self.as_bytes());
    }

    #[cfg(not(feature = "alloc"))]
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
        Ok(Self::from(buf.decode::<&str>()?))
    }
}

impl Encode for CStr {
    #[inline]
    fn len(&self) -> usize {
        self.to_bytes_with_nul().len()
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(self.to_bytes_with_nul());
    }

    #[cfg(not(feature = "alloc"))]
    #[inline]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        buf.encode(self.to_bytes_with_nul())?;
        Ok(())
    }
}

impl<'buf> Decode<'buf> for &'buf CStr {
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        let bytes: &[u8] = buf.decode()?;
        Ok(CStr::from_bytes_with_nul(bytes)?)
    }
}

#[cfg(feature = "alloc")]
impl Decode<'_> for Box<CStr> {
    fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
        Ok(buf.decode::<CString>()?.into_boxed_c_str())
    }
}

#[cfg(feature = "alloc")]
impl Encode for CString {
    #[inline]
    fn len(&self) -> usize {
        self.as_c_str().len()
    }

    #[inline]
    fn encode(&self, buf: &mut BufMut) {
        buf.encode(self.as_c_str());
    }
}

#[cfg(feature = "alloc")]
impl Decode<'_> for CString {
    fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
        Ok(buf.decode::<&CStr>()?.to_owned())
    }
}
