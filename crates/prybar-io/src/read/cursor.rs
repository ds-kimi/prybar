use crate::{Primitive, Reader, Result};

/// Sequential reads over a `Reader`; tree-style formats walk forward and
/// would otherwise thread an offset through every call.
#[derive(Debug, Clone, Copy)]
pub struct Cursor<'a> {
    reader: Reader<'a>,
    pub pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(reader: Reader<'a>, pos: usize) -> Self {
        Self { reader, pos }
    }

    pub fn read<T: Primitive>(&mut self) -> Result<T> {
        let value = self.reader.read(self.pos)?;
        self.pos += T::SIZE;
        Ok(value)
    }

    pub fn bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        let raw = self.reader.bytes(self.pos, len)?;
        self.pos += len;
        Ok(raw)
    }

    pub fn cstr(&mut self, max: usize) -> Result<&'a str> {
        let s = self.reader.cstr(self.pos, max)?;
        self.pos += s.len() + 1;
        Ok(s)
    }
}
