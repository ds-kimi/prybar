use crate::Primitive;

/// Growable little-endian output buffer.
#[derive(Debug, Default, Clone)]
pub struct Writer {
    pub(crate) buf: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put<T: Primitive>(&mut self, value: T) -> &mut Self {
        value.to_le(&mut self.buf);
        self
    }

    pub fn put_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.buf.extend_from_slice(bytes);
        self
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buf
    }
}
