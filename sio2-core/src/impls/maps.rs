#![cfg(feature = "alloc")]

use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
use alloc::collections::BTreeMap;

#[cfg(any(feature = "std", feature = "indexmap"))]
use core::hash::Hash;
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "indexmap")]
use indexmap::IndexMap;

macro_rules! impl_encode_map {
    ($(#[$meta:meta])? $T:ty) => {
        $(#[$meta])?
        impl<K, V> Encode for $T
        where
            K: Encode,
            V: Encode,
        {
            fn len(&self) -> usize {
                let items: usize = self
                    .iter()
                    .map(|(k, v)| Encode::len(k) + Encode::len(v))
                    .sum();
                Encode::len(&self.len()) + items
            }

            fn encode(&self, buf: &mut BufMut) {
                buf.encode(&self.len());
                for (k, v) in self {
                    buf.encode(k)
                        .encode(v);
                }
            }
        }
    };
}

macro_rules! impl_decode_map {
    ($(#[$meta:meta])? $T:ty where $($bound:tt)*) => {
        $(#[$meta])?
        impl<'buf, K, V> Decode<'buf> for $T
        where
            K: Decode<'buf>,
            V: Decode<'buf>,
            $($bound)*
        {
            fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
                let len: usize = buf.decode()?;
                (0..len).map(|_| Ok((buf.decode()?, buf.decode()?))).collect()
            }
        }
    };
}

impl_encode_map!(BTreeMap<K, V>);
impl_encode_map!(#[cfg(feature = "std")] HashMap<K, V>);
impl_encode_map!(#[cfg(feature = "indexmap")] IndexMap<K, V>);

impl_decode_map!(BTreeMap<K, V> where K: Ord);
impl_decode_map!(#[cfg(feature = "std")] HashMap<K, V> where K: Hash + Eq);
impl_decode_map!(#[cfg(feature = "indexmap")] IndexMap<K, V> where K: Hash + Eq);
