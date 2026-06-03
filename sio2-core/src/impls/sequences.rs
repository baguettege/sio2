#![cfg(feature = "alloc")]

use crate::{Encode, BufMut, Decode, Buf, DecodeResult};

use alloc::vec::Vec;
use alloc::collections::{VecDeque, LinkedList, BinaryHeap, BTreeSet};

#[cfg(any(feature = "std", feature = "indexmap"))]
use core::hash::Hash;
#[cfg(feature = "std")]
use std::collections::HashSet;
#[cfg(feature = "indexmap")]
use indexmap::IndexSet;

macro_rules! impl_encode {
    ($(#[$meta:meta])? $T:ty) => {
        $(#[$meta])?
        impl<E> Encode for $T
        where
            E: Encode,
        {
            fn len(&self) -> usize {
                let items: usize = self
                    .iter()
                    .map(Encode::len)
                    .sum();
                Encode::len(&self.len()) + items
            }

            fn encode(&self, buf: &mut BufMut) {
                buf.encode(&self.len());
                for item in self {
                    buf.encode(item);
                }
            }
        }
    };
}

macro_rules! impl_decode {
    ($(#[$meta:meta])? $T:ty $(where $($bound:tt)*)?) => {
        $(#[$meta])?
        impl<'buf, D> Decode<'buf> for $T
        where
            D: Decode<'buf>,
            $($($bound)*)?
        {
            fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
                let len: usize = buf.decode()?;
                (0..len).map(|_| buf.decode()).collect()
            }
        }
    };
}

impl_encode!(Vec<E>);
impl_encode!(VecDeque<E>);
impl_encode!(LinkedList<E>);
impl_encode!(BTreeSet<E>);
impl_encode!(BinaryHeap<E>);
impl_encode!(#[cfg(feature = "std")] HashSet<E>);
impl_encode!(#[cfg(feature = "indexmap")] IndexSet<E>);
impl_encode!(#[cfg(feature = "hashbrown")] hashbrown::HashSet<E>);

impl_decode!(Vec<D>);
impl_decode!(VecDeque<D>);
impl_decode!(LinkedList<D>);
impl_decode!(BTreeSet<D> where D: Ord);
impl_decode!(BinaryHeap<D> where D: Ord);
impl_decode!(#[cfg(feature = "std")] HashSet<D> where D: Hash + Eq);
impl_decode!(#[cfg(feature = "indexmap")] IndexSet<D> where D: Hash + Eq);
impl_decode!(#[cfg(feature = "hashbrown")] hashbrown::HashSet<D> where D: Hash + Eq);
