use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

use core::mem::MaybeUninit;

#[cfg(feature = "alloc")]
impl<const N: usize, E> Encode for [E; N]
where
    E: Encode,
{
    fn len(&self) -> usize {
        self.iter()
            .map(Encode::len)
            .sum()
    }

    fn encode(&self, buf: &mut BufMut) {
        for item in self {
            buf.encode(item);
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<const N: usize, E> Encode for [E; N]
where
    E: Encode,
{
    fn len(&self) -> usize {
        self.iter()
            .map(Encode::len)
            .sum()
    }

    fn encode(&self, buf: &mut BufMut) -> EncodeResult<()> {
        for item in self {
            buf.encode(item)?;
        }

        Ok(())
    }
}

impl<'buf, const N: usize, D> Decode<'buf> for [D; N]
where
    D: Decode<'buf>,
{
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        // SAFETY: `MaybeUninit<D>` is always valid regardless of its contents,
        // so an array of `MaybeUninit<D>` is valid even when uninitialized.
        let mut array: [MaybeUninit<D>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut initialized: usize = 0;

        for i in 0..N {
            match buf.decode::<D>() {
                Ok(item) => {
                    array[i].write(item);
                    initialized += 1;
                }
                Err(e) => {
                    for j in 0..initialized {
                        // SAFETY: The item at index `j` has been initialized since
                        // `j < initialized`, and `initialized` is only incremented
                        // after a successful write.
                        unsafe { array[j].assume_init_drop() }
                    }

                    return Err(e);
                }
            }
        }

        // SAFETY: The loop initializes indices `0..N` and exits early on error,
        // guaranteeing that all items in `array` are initialized.
        Ok(array.map(|item| unsafe { item.assume_init() }))
    }
}

impl<'buf, const N: usize> Decode<'buf> for &'buf [u8; N] {
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        buf.take_array::<N>()
    }
}
