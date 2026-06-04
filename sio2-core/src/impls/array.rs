use crate::{Encode, BufMut, Decode, Buf, DecodeResult};
#[cfg(not(feature = "alloc"))]
use crate::EncodeResult;

use core::mem::MaybeUninit;

impl<const N: usize, E> Encode for [E; N]
where
    E: Encode,
{
    fn len(&self) -> usize {
        self.iter()
            .map(Encode::len)
            .sum()
    }

    #[cfg(feature = "alloc")]
    fn encode(&self, buf: &mut BufMut) {
        for item in self {
            buf.encode(item);
        }
    }

    #[cfg(not(feature = "alloc"))]
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

        for slot in array.iter_mut() {
            match buf.decode::<D>() {
                Ok(item) => {
                    slot.write(item);
                    initialized += 1;
                }
                Err(e) => {
                    for slot in array.iter_mut().take(initialized) {
                        // SAFETY: Only the first `initialized` slots have been written
                        // to because `initialized` is only incremented after a successful
                        // write, therefore `take(initialized)` visits only initialized slots.
                        unsafe { slot.assume_init_drop() }
                    }

                    return Err(e);
                }
            }
        }

        // SAFETY: Reaching this point means the loop did not exit early, so all
        // slots in `array` have been written and initialized.
        Ok(array.map(|item| unsafe { item.assume_init() }))
    }
}

impl<'buf, const N: usize> Decode<'buf> for &'buf [u8; N] {
    fn decode(buf: &mut Buf<'buf>) -> DecodeResult<Self> {
        buf.take_array::<N>()
    }
}
