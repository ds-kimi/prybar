use crate::format::archive_path;
use crate::{Error, Result, EMBEDDED};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Streams file data either into an in-memory blob (embedded) or into
/// numbered archives, so chunked packs never hold every file at once.
pub(crate) struct Placer<'a> {
    out: &'a Path,
    chunk: Option<u64>,
    next_index: u16,
    current: Option<BufWriter<File>>,
    current_len: u64,
    pub embedded: Vec<u8>,
}

impl<'a> Placer<'a> {
    pub fn new(out: &'a Path, chunk: Option<u64>) -> Self {
        Self {
            out,
            chunk,
            next_index: 0,
            current: None,
            current_len: 0,
            embedded: Vec::new(),
        }
    }

    /// Returns (archive index, offset) where `data` was stored.
    pub fn put(&mut self, data: &[u8]) -> Result<(u16, u32)> {
        let Some(chunk) = self.chunk else {
            let off = u32::try_from(self.embedded.len()).map_err(|_| Error::TooLarge)?;
            self.embedded.extend_from_slice(data);
            return Ok((EMBEDDED, off));
        };
        let len = data.len() as u64;
        if self.current.is_none() || (self.current_len > 0 && self.current_len + len > chunk) {
            self.roll()?;
        }
        let off = u32::try_from(self.current_len).map_err(|_| Error::TooLarge)?;
        if let Some(w) = self.current.as_mut() {
            w.write_all(data)?;
        }
        self.current_len += len;
        Ok((self.next_index - 1, off))
    }

    fn roll(&mut self) -> Result<()> {
        self.finish()?;
        let path = archive_path(self.out, self.next_index).ok_or(Error::TooLarge)?;
        self.current = Some(BufWriter::new(File::create(path)?));
        self.current_len = 0;
        self.next_index += 1;
        Ok(())
    }

    pub fn finish(&mut self) -> Result<()> {
        if let Some(mut w) = self.current.take() {
            w.flush()?;
        }
        Ok(())
    }
}
