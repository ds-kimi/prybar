use crate::Writer;

impl Writer {
    pub fn position(&self) -> usize {
        self.buf.len()
    }

    pub fn put_cstr(&mut self, s: &str) -> &mut Self {
        self.buf.extend_from_slice(s.as_bytes());
        self.buf.push(0);
        self
    }

    /// An `align` of 0 is treated as 1 to avoid a division by zero.
    pub fn align(&mut self, align: usize) -> &mut Self {
        let align = align.max(1);
        let rem = self.buf.len() % align;
        if rem != 0 {
            self.buf.resize(self.buf.len() + align - rem, 0);
        }
        self
    }

    /// Back-patches offsets that are only known after later data is written.
    pub fn patch_u32(&mut self, at: usize, value: u32) -> bool {
        let Some(end) = at.checked_add(4) else {
            return false;
        };
        match self.buf.get_mut(at..end) {
            Some(slot) => {
                slot.copy_from_slice(&value.to_le_bytes());
                true
            }
            None => false,
        }
    }
}
