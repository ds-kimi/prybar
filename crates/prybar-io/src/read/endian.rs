use crate::{Primitive, Reader};

/// Widest primitive is 8 bytes; a stack buffer avoids an allocation per
/// swapped read.
const MAX: usize = 8;

impl<'a> Reader<'a> {
    /// Reader whose scalars are big-endian. Only scalar reads swap; raw
    /// `bytes` and strings are the same in both orders.
    pub fn big_endian(data: &'a [u8]) -> Self {
        Self { data, big: true }
    }

    pub fn is_big_endian(&self) -> bool {
        self.big
    }
}

/// Callers pass exactly `T::SIZE` bytes, as `Primitive::from_le` requires.
pub(crate) fn decode<T: Primitive>(bytes: &[u8], big: bool) -> T {
    if !big || T::SIZE > MAX {
        return T::from_le(bytes);
    }
    let mut buf = [0u8; MAX];
    buf[..T::SIZE].copy_from_slice(bytes);
    buf[..T::SIZE].reverse();
    T::from_le(&buf[..T::SIZE])
}
