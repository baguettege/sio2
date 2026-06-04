use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

use core::net::{
    Ipv4Addr, Ipv6Addr, IpAddr,
    SocketAddrV4, SocketAddrV6, SocketAddr,
};

macro_rules! impl_ipv_n_addr {
    ($($T:ty),* $(,)?) => {
        $(
            impl Encode for $T {
                #[inline]
                fn len(&self) -> usize {
                    self.octets().len()
                }

                #[cfg(feature = "alloc")]
                #[inline]
                fn encode(&self, buf: &mut BufMut) {
                    buf.put(self.octets());
                }

                #[cfg(not(feature = "alloc"))]
                #[inline]
                fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
                    buf.put(self.octets())?;
                    Ok(())
                }
            }

            impl Decode<'_> for $T{
                #[inline]
                fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
                    Ok(Self::from_octets(*buf.take_array()?))
                }
            }
        )*
    };
}

impl_ipv_n_addr!(Ipv4Addr, Ipv6Addr);

impl Encode for SocketAddrV4 {
    #[inline]
    fn len(&self) -> usize {
        self.ip().len() + self.port().len()
    }

    #[cfg(feature = "alloc")]
    fn encode(&self, buf: &mut BufMut) {
        buf
            .encode(self.ip())
            .encode(&self.port());
    }

    #[cfg(not(feature = "alloc"))]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        buf.encode(self.ip())?
            .encode(&self.port())?;
        Ok(())
    }
}

impl Decode<'_> for SocketAddrV4 {
    fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
        Ok(Self::new(
            buf.decode()?,
            buf.decode()?,
        ))
    }
}

impl Encode for SocketAddrV6 {
    #[inline]
    fn len(&self) -> usize {
        self.ip().len() +
            self.port().len() +
            self.flowinfo().len() +
            self.scope_id().len()
    }

    #[cfg(feature = "alloc")]
    fn encode(&self, buf: &mut BufMut) {
        buf
            .encode(self.ip())
            .encode(&self.port())
            .encode(&self.flowinfo())
            .encode(&self.scope_id());
    }

    #[cfg(not(feature = "alloc"))]
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        buf.encode(self.ip())?
            .encode(&self.port())?
            .encode(&self.flowinfo())?
            .encode(&self.scope_id())?;
        Ok(())
    }
}

impl Decode<'_> for SocketAddrV6 {
    fn decode(buf: &mut Buf<'_>) -> DecodeResult<Self> {
        Ok(Self::new(
            buf.decode()?,
            buf.decode()?,
            buf.decode()?,
            buf.decode()?,
        ))
    }
}

macro_rules! impl_addr {
    ($($T:ty),* $(,)?) => {
        $(
            impl Encode for $T {
                #[inline]
                fn len(&self) -> usize {
                    size_of::<u8>() + match self {
                        Self::V4(v) => v.len(),
                        Self::V6(v) => v.len(),
                    }
                }

                #[cfg(feature = "alloc")]
                fn encode(&self, buf: &mut BufMut) {
                    match self {
                        Self::V4(v) => buf
                            .encode(&0u8)
                            .encode(v),
                        Self::V6(v) => buf
                            .encode(&1u8)
                            .encode(v),
                    };
                }

                #[cfg(not(feature = "alloc"))]
                fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
                    match self {
                        Self::V4(v) => buf
                            .encode(&0u8)?
                            .encode(v)?,
                        Self::V6(v) => buf
                            .encode(&1u8)?
                            .encode(v)?,
                    };

                    Ok(())
                }
            }
        )*
    };
}

impl_addr!(IpAddr, SocketAddr);
