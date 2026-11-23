use crate::{Error, Primitive, Result};

/// Random-access view over an untrusted byte buffer.
#[derive(Debug, Clone, Copy)]
pub struct Reader<'a> {
    pub(crate) data: &'a [u8],
    /// Console builds (Xbox 360, PS3) store every scalar byte-swapped.
    pub(crate) big: bool,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, big: false }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Source offsets are often relative or forged, so the end is
    /// computed with checked math to reject wrap-around.
    pub fn bytes(&self, offset: usize, len: usize) -> Result<&'a [u8]> {
        let size = self.data.len();
        let oob = Error::OutOfBounds { offset, len, size };
        let end = offset.checked_add(len).ok_or_else(|| oob.clone())?;
        self.data.get(offset..end).ok_or(oob)
    }

    pub fn read<T: Primitive>(&self, offset: usize) -> Result<T> {
        self.bytes(offset, T::SIZE)
            .map(|b| super::endian::decode(b, self.big))
    }
}
