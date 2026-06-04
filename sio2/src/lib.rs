//! A deterministic, zero-copy binary serialization library with optional
//! `alloc` and `std` support.
//!
//! # Determinism
//!
//! For the majority of types, encoding order is deterministic and yields a predictable,
//! reproducible binary layout.
//!
//! Encoding order is not deterministic for:
//! - [`HashMap`](std::collections::HashMap) and [`HashSet`](std::collections::HashSet)
//!   across runs due to random hashing
//! - [`BinaryHeap`](std::collections::BinaryHeap) due to non-deterministic internal
//!   ordering
//!
//! Round-trip correctness is guaranteed, but byte-level reproducibility is not.

#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
pub mod __private;

pub use sio2_core::*;
pub use sio2_derive::*;
