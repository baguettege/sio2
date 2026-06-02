#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod encode;
mod decode;
mod impls;

pub use encode::{Encode, BufMut};
#[cfg(feature = "alloc")]
pub use encode::{VecExt, ToBytes};
#[cfg(not(feature = "alloc"))]
pub use encode::{Overflow, EncodeResult};

pub use decode::{DecodeResult, DecodeError, Decode, Buf};
