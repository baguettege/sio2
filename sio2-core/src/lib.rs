#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod encode;
mod decode;
mod impls;

pub use encode::{BufMut, Encode};
#[cfg(feature = "alloc")]
pub use encode::{ToBytes, VecExt};
#[cfg(not(feature = "alloc"))]
pub use encode::{EncodeResult, Overflow};

pub use decode::{Buf, Decode, DecodeError, DecodeResult};
