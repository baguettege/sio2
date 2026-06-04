#![doc(hidden)]

pub use crate::{
    Encode, Decode,
    BufMut, Buf,
    DecodeError, DecodeResult,
};
#[cfg(not(feature = "alloc"))]
pub use crate::EncodeResult;

pub use core::mem::size_of;
