//! A deterministic, zero-copy binary serialization library with optional `alloc`
//! and `std` support.
//!
//! # Features
//!
//! - encoding via the [`Encode`] trait and [`BufMut`] buffer
//! - decoding via the [`Decode`] trait and [`Buf`] buffer
//! - proc macro derives for the above traits
//! - full `#[no_std]` support
//!
//! # `#[no_std]`
//!
//! To disable [`std`], disable `default-features` in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sio2 = { version = "0.1.0", default-features = false }
//! ```
//!
//! The decoding API remains entirely identical, however, the encoding API changes,
//! since [`Encode::encode`] is fallible due to potential overflow.
//!
//! # `alloc`
//!
//! By enabling the `alloc` feature:
//!
//! ```toml
//! [dependencies]
//! sio2 = { version = "0.1.0", default-features = false, features = ["alloc"] }
//! ```
//!
//! The default encoding API is restored, alongside many implementations
//! for [`Encode`] and [`Decode`] for common `alloc` types.
//!
//! # `std`
//!
//! The `std` feature is enabled by default. It automatically enables `alloc`,
//! alongside `std` specific implementations as well, such as
//! [`HashMap`](std::collections::HashMap) and [`HashSet`](std::collections::HashSet).
//!
//! # `indexmap` and `hashbrown`
//!
//! The features `indexmap` and `hashbrown` can be optionally enabled for the [`Encode`]
//!  and [`Decode`] implementations of [`IndexMap`](indexmap::IndexMap),
//! [`IndexSet`](indexmap::IndexSet), [`hashbrown::HashMap`], and [`hashbrown::HashSet`].
//!
//! # Determinism
//!
//! For the majority of types, encoding order is deterministic and yields a predictable,
//! reproducible binary layout. This remains true for any generated implementations by
//! the proc macros, so long as the contained fields also uphold this.
//!
//! Encoding order is not deterministic for:
//! - [`HashMap`](std::collections::HashMap) and [`HashSet`](std::collections::HashSet)
//!   across runs due to random hashing
//! - [`BinaryHeap`](std::collections::BinaryHeap) due to non-deterministic internal
//!   ordering
//!
//! Round-trip correctness is guaranteed, but byte-level reproducibility is not.
//!
//! # Deriving
//!
//! Proc macro derives are provided for [`Encode`] and [`Decode`], with both `#[no_std]`
//! and `alloc` support, as well as preserving zero-copy decoding:
//!
//! ```
//! use sio2::{Encode, Decode};
//! use std::borrow::Cow;
//!
//! #[derive(Encode, Decode)]
//! #[sio2(borrow('buf))]
//! enum Complex<'buf> {
//!     Named { bytes: &'buf [u8] },
//!     Unnamed(Cow<'buf, str>),
//!     Unit,
//! }
//! ```
//!
//! See the documentation for these derives for additional examples and information.
//!
//! # Performance
//!
//! Blanket implementations favor simplicity over specialization. For example, decoding a
//! [`Vec<u8>`] runs through the blanket implementation for [`Vec<T>`], decoding each byte
//! individually rather than using a single memcpy.
//!
//! For byte-heavy/performance heavy use cases, prefer more manual usage, such as
//! `buf.decode::<&[u8]>()?.to_vec()` over `buf.decode::<Vec<u8>>()?`.
//!
//! Specialized fast-path proc macro attributes may be added in the future.
//!
//! # Endian-ness
//!
//! All implementations for numbers use big-endian.
//!
//! # [`Encode::len`]
//!
//! This method allows for the encoded length of values to be known before encoding,
//! which supports determinism and allows for pre-allocation, protocol framing,
//! easier debugging, and assertions.
//!
//! As a result of this, implementors must follow its invariant: The value returned
//! **must** always be equal to the number of bytes encoded in [`Encode::encode`]. Breaking
//! this invariant produces unpredictable behavior.

#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub mod __private;

pub use sio2_core::*;
pub use sio2_derive::*;
