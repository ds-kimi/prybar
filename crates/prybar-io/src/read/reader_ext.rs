use crate::{Error, Primitive, Reader, Result};

impl<'a> Reader<'a> {
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Scans at most `max` bytes so a missing terminator cannot make us
    /// walk the whole file.
    pub fn cstr(&self, offset: usize, max: usize) -> Result<&'a str> {
        let avail = self.data.len().saturating_sub(offset).min(max);
        let window = self.bytes(offset, avail)?;
        let end = window
            .iter()
            .position(|&b| b == 0)
            .ok_or(Error::Unterminated { offset, max })?;
        std::str::from_utf8(&window[..end]).map_err(|_| Error::InvalidUtf8 { offset })
    }

    /// The cap rejects forged counts before any allocation happens.
    pub fn array<T: Primitive>(&self, offset: usize, count: usize, cap: usize) -> Result<Vec<T>> {
        if count > cap {
            return Err(Error::CountTooLarge { count, cap });
        }
        let len = count
            .checked_mul(T::SIZE)
            .ok_or(Error::CountTooLarge { count, cap })?;
        let raw = self.bytes(offset, len)?;
        Ok(raw
            .chunks_exact(T::SIZE)
            .map(|b| super::endian::decode(b, self.big))
            .collect())
    }
}
