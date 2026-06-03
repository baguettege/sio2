pub use self::error::{DecodeError, DecodeResult};

mod error {
    use core::str::Utf8Error;
    use core::ffi::FromBytesWithNulError;

    #[cfg(feature = "alloc")]
    use alloc::boxed::Box;

    /// The error type returned when decoding a value from a [`Buf`](super::Buf) fails.
    #[derive(Debug, thiserror::Error)]
    #[non_exhaustive]
    #[cfg_attr(not(feature = "alloc"), derive(Clone, Copy, PartialEq, Eq))]
    pub enum DecodeError {
        /// The buffer was unexpectedly exhausted.
        #[error("unexpected EOF")]
        UnexpectedEof,

        /// A byte representing a `bool` was not `0` or `1`.
        #[error("invalid bool byte")]
        InvalidBool,

        /// A 32-bit integer was outside the valid Unicode scalar range or fell
        /// into the surrogate gap.
        #[error("invalid char codepoint")]
        InvalidChar,

        /// A byte sequence was not valid UTF-8.
        #[error(transparent)]
        InvalidUtf8(#[from] Utf8Error),

        /// A [`NonZero`](core::num::NonZero) value was `0`.
        #[error("non-zero value was `0`")]
        InvalidNonZero,

        /// A [`CStr`](core::ffi::CStr) was not nul-terminated or contained an
        /// interior nul byte.
        #[error(transparent)]
        InvalidCStr(#[from] FromBytesWithNulError),

        /// An enum variant's discriminant was invalid.
        #[error("invalid discriminant `{0}`")]
        InvalidDiscriminant(u16),

        /// A custom, user-defined error.
        #[cfg(feature = "alloc")]
        #[error(transparent)]
        Other(#[from] Box<dyn core::error::Error + Send + Sync>),
    }

    pub type DecodeResult<T> = Result<T, DecodeError>;
}

/// A type that can be decoded from a [`Buf`].
pub trait Decode<'buf>: Sized {
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self>;
}

/// A read-only byte buffer for decoding values sequentially.
#[derive(Debug, Copy, Clone)]
pub struct Buf<'buf> {
    buf: &'buf [u8],
}

impl<'buf> Buf<'buf> {
    /// Constructs a [`Buf`] wrapping `buf`.
    #[inline]
    pub const fn new(buf: &'buf [u8]) -> Self {
        Self { buf }
    }

    /// Returns the remaining number of bytes that can be read from this buffer.
    #[inline]
    pub const fn remaining(&self) -> usize {
        self.buf.len()
    }

    /// Consumes and returns a borrowed byte slice of length `n` from this buffer.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::UnexpectedEof`] if there are less than `n` remaining
    /// bytes in this buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sio2_core as sio2;
    /// use sio2::{Buf, DecodeResult, DecodeError};
    ///
    /// let mut buf = Buf::new(&[0u8, 0u8, 1u8, 0u8]);
    /// assert!(matches!(buf.take(2), Ok(&[0u8, 0u8])));
    /// assert!(matches!(buf.take(2), Ok(&[1u8, 0u8])));
    /// assert!(matches!(buf.take(1), Err(DecodeError::UnexpectedEof)));
    /// ```
    #[inline]
    pub const fn take(&mut self, n: usize) -> DecodeResult<&'buf [u8]> {
        match self.buf.split_at_checked(n) {
            None => Err(DecodeError::UnexpectedEof),
            Some((head, tail)) => {
                self.buf = tail;
                Ok(head)
            }
        }
    }

    /// Consumes and returns a borrowed byte array of length `N` from this buffer.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError::UnexpectedEof`] for the same reasons as [`Self::take`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use sio2_core as sio2;
    /// use sio2::{Buf, DecodeResult, DecodeError};
    ///
    /// let mut buf = Buf::new(&[1u8, 2u8, 3u8, 4u8]);
    /// assert!(matches!(buf.take_array::<2>(), Ok(&[1u8, 2u8])));
    /// assert!(matches!(buf.take_array::<2>(), Ok(&[3u8, 4u8])));
    /// assert!(matches!(buf.take_array::<1>(), Err(DecodeError::UnexpectedEof)));
    /// ```
    #[inline]
    pub fn take_array<const N: usize>(&mut self) -> DecodeResult<&'buf [u8; N]> {
        // `take(N)` guarantees `len() == N`, so `try_into` is infallible.
        Ok(self.take(N)?.try_into().unwrap())
    }

    /// Decodes a value of type `D` from this buffer.
    ///
    /// # Errors
    ///
    /// Returns [`DecodeError`] if decoding fails or the buffer is unexpectedly
    /// exhausted.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sio2_core as sio2;
    /// use sio2::{Buf, DecodeError};
    ///
    /// let mut buf = Buf::new(&[0u8, 1u8, 2u8, 3u8, 4u8]);
    /// assert!(matches!(buf.decode::<u32>(), Ok(66_051)));
    /// assert!(matches!(buf.decode::<u8>(), Ok(4)));
    /// assert!(matches!(buf.decode::<u16>(), Err(DecodeError::UnexpectedEof)));
    /// ```
    #[inline]
    pub fn decode<D>(&mut self) -> DecodeResult<D>
    where
        D: Decode<'buf>,
    {
        D::decode(self)
    }
}
