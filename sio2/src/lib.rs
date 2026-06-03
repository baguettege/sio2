//! A deterministic, zero-copy binary serialization library with optional
//! `alloc` and `std` support.
//!
//! # Determinism
//!
//! For the majority of types, encoding order is deterministic and yields a predictable,
//! reproducible binary layout.
//!
//! Encoding order is not deterministic across runs for:
//! - [`HashMap`](std::collections::HashMap) and [`HashSet`](std::collections::HashSet)
//!   due to random hashing
//! - [`BinaryHeap`](std::collections::BinaryHeap) due to insertion order history
//!
//! Round-trip correctness is guaranteed, but byte-level reproducibility is not
//! for these types.

#![cfg_attr(not(feature = "std"), no_std)]

pub use sio2_core::*;
