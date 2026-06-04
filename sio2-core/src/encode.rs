#[cfg(not(feature = "alloc"))]
pub use self::error::{Overflow, EncodeResult};

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(not(feature = "alloc"))]
mod error {
    /// The error type returned when a [`BufMut`](super::BufMut) does not have
    /// enough capacity to store additional bytes.
    #[derive(thiserror::Error, Debug, Clone, Copy)]
    #[error("buffer overflow")]
    pub struct Overflow;

    pub type EncodeResult<T> = Result<T, Overflow>;
}

/// A type that can be deterministically encoded into a [`BufMut`].
///
/// # Examples
///
/// ```
/// use sio2::{Encode, BufMut};
///
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// impl Encode for Point {
///     fn len(&self) -> usize {
///         self.x.len() + self.y.len()
///     }
///
///     fn encode(&self, buf: &mut BufMut) {
///         buf
///             .encode(&self.x)
///             .encode(&self.y);
///     }
/// }
/// ```
#[cfg(feature = "alloc")]
#[allow(clippy::len_without_is_empty)]
pub trait Encode {
    /// Returns the exact number of bytes that [`Self::encode`] will write.
    fn len(&self) -> usize;

    /// Encodes `self` into `buf`.
    fn encode(&self, buf: &mut BufMut);
}

/// A type that can be deterministically encoded into a [`BufMut`].
///
/// # Examples
///
/// ```
/// use sio2::{Encode, BufMut, EncodeResult};
///
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// impl Encode for Point {
///     fn len(&self) -> usize {
///         self.x.len() + self.y.len()
///     }
///
///     fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
///         buf
///             .encode(&self.x)?
///             .encode(&self.y)?;
///         Ok(())
///     }
/// }
/// ```
#[cfg(not(feature = "alloc"))]
#[allow(clippy::len_without_is_empty)]
pub trait Encode {
    /// Returns the exact number of bytes that [`Self::encode`] will write.
    fn len(&self) -> usize;

    /// Encodes `self` into `buf`.
    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()>;
}

/// A growable byte buffer for encoding values sequentially.
#[cfg(feature = "alloc")]
#[derive(Debug)]
pub struct BufMut<'a> {
    buf: &'a mut Vec<u8>,
}

#[cfg(feature = "alloc")]
impl<'a> BufMut<'a> {
    /// Constructs a new [`BufMut`] wrapping `buf`.
    #[inline]
    pub const fn new(buf: &'a mut Vec<u8>) -> Self {
        Self { buf }
    }

    /// Returns the number of bytes written to this buffer.
    #[inline]
    pub const fn len(&self) -> usize {
        Vec::len(self.buf)
    }

    /// Returns `true` if no bytes have been written to this buffer.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pushes `bytes` into this buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use sio2::BufMut;
    ///
    /// let mut bytes = Vec::new();
    /// let mut buf = BufMut::new(&mut bytes);
    ///
    /// buf.put([0u8; 4]);
    /// assert_eq!(buf.len(), 4);
    /// ```
    #[inline]
    pub fn put<T>(&mut self, bytes: T) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        self.buf.extend_from_slice(bytes.as_ref());
        self
    }

    /// Encodes `value` into this buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use sio2::BufMut;
    ///
    /// let mut bytes = Vec::new();
    /// let mut buf = BufMut::new(&mut bytes);
    ///
    /// buf.encode(&123u64);
    /// assert_eq!(buf.len(), 8);
    /// ```
    #[inline]
    pub fn encode<E>(&mut self, value: &E) -> &mut Self
    where
        E: Encode + ?Sized,
    {
        value.encode(self);
        self
    }
}

/// A fixed-size byte buffer for encoding values sequentially.
#[cfg(not(feature = "alloc"))]
#[derive(Debug)]
pub struct BufMut<'a> {
    buf: &'a mut [u8],
    /// The current offset into `buf`, always `<= buf.len()`.
    offset: usize,
}

#[cfg(not(feature = "alloc"))]
impl<'a> BufMut<'a> {
    /// Constructs a new [`BufMut`] wrapping `buf`.
    #[inline]
    pub const fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, offset: 0 }
    }

    /// Returns the capacity of this buffer.
    #[inline]
    pub const fn capacity(&self) -> usize {
        <[u8]>::len(self.buf)
    }

    /// Returns the number of bytes written to this buffer.
    #[inline]
    pub const fn len(&self) -> usize {
        self.offset
    }

    /// Returns `true` if no bytes have been written to this buffer.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the remaining number of bytes that can be written to this buffer.
    #[inline]
    pub const fn remaining(&self) -> usize {
        debug_assert!(self.capacity() >= self.len(), "capacity must be >= len",);
        self.capacity() - self.len()
    }

    /// Pushes `bytes` into this buffer.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] if the length of `bytes` exceeds the buffer's
    /// remaining space.
    ///
    /// # Examples
    ///
    /// ```
    /// use sio2::{BufMut, Overflow};
    ///
    /// let mut bytes = [0u8; 8];
    /// let mut buf = BufMut::new(&mut bytes);
    ///
    /// assert_eq!(buf.put([0u8; 4]), Ok(()));
    /// assert_eq!(buf.remaining(), 4);
    /// assert_eq!(buf.put([0u8; 5]), Err(Overflow))
    /// ```
    pub fn put<T>(&mut self, bytes: T) -> EncodeResult<&mut Self>
    where
        T: AsRef<[u8]>,
    {
        let bytes = bytes.as_ref();
        let len = bytes.len();

        if self.remaining() >= len {
            self.buf[self.offset..self.offset + len].copy_from_slice(bytes);
            self.offset += len;
            Ok(self)
        } else {
            Err(Overflow)
        }
    }

    /// Encodes `value` into this buffer.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] for the same reasons as [`Self::put`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sio2::{BufMut, Overflow};
    ///
    /// let mut bytes = [0u8; 8];
    /// let mut buf = BufMut::new(&mut bytes);
    ///
    /// assert_eq!(buf.encode(&123u64), Ok(()));
    /// assert_eq!(buf.remaining(), 0);
    /// assert_eq!(buf.encode(&0u8), Err(Overflow));
    /// ```
    #[inline]
    pub fn encode<E>(&mut self, value: &E) -> EncodeResult<&mut Self>
    where
        E: Encode + ?Sized,
    {
        value.encode(self)?;
        Ok(self)
    }

    /// Consumes this buffer, returning an immutable slice of the bytes written so far.
    #[inline]
    pub fn into_filled(self) -> &'a [u8] {
        &self.buf[..self.offset]
    }
}

#[cfg(feature = "alloc")]
#[doc(hidden)]
mod private {
    use super::Encode;
    use alloc::vec::Vec;

    // Multiple `Sealed` traits are used since a blanket impl over `E: Encode`
    // can conflict with a potential `Encode` impl for `Vec<u8>`.

    pub trait VecExtSealed {}
    pub trait ToBytesSealed {}

    impl VecExtSealed for Vec<u8> {}
    impl<E> ToBytesSealed for E
    where
        E: Encode + ?Sized,
    {}
}

/// Extension trait providing convenient methods for encoding values into a [`Vec<u8>`].
///
/// This trait is sealed and cannot be implemented outside of this crate.
///
/// # Examples
///
/// ```
/// use sio2::VecExt;
///
/// let mut buf: Vec<u8> = Vec::new();
///
/// buf.put([0u8; 4]);
/// buf.encode(&0u32);
///
/// assert_eq!(buf.len(), 8);
/// ```
#[cfg(feature = "alloc")]
pub trait VecExt: private::VecExtSealed {
    /// Pushes `bytes` into `self`
    fn put<T>(&mut self, bytes: T) -> &mut Self
    where
        T: AsRef<[u8]>;

    /// Encodes `value` into `self`.
    fn encode<E>(&mut self, value: &E) -> &mut Self
    where
        E: Encode + ?Sized;
}

#[cfg(feature = "alloc")]
impl VecExt for Vec<u8> {
    #[inline]
    fn put<T>(&mut self, bytes: T) -> &mut Self
    where
        T: AsRef<[u8]>
    {
        BufMut::new(self).put(bytes);
        self
    }

    #[inline]
    fn encode<E>(&mut self, value: &E) -> &mut Self
    where
        E: Encode + ?Sized
    {
        BufMut::new(self).encode(value);
        self
    }
}

/// Extension trait providing a convenience method to convert [`Encode`]
/// types into a [`Vec<u8>`].
///
/// This trait is sealed and cannot be implemented outside of this crate.
///
/// # Examples
///
/// ```
/// use sio2::ToBytes;
///
/// assert_eq!(256u32.to_bytes(), vec![0, 0, 1, 0]);
/// ```
#[cfg(feature = "alloc")]
pub trait ToBytes: private::ToBytesSealed {
    /// Converts `self` into a [`Vec<u8>`].
    fn to_bytes(&self) -> Vec<u8>;
}

#[cfg(feature = "alloc")]
impl<E> ToBytes for E
where
    E: Encode + ?Sized,
{
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len());
        VecExt::encode(&mut buf, self);
        buf
    }
}
