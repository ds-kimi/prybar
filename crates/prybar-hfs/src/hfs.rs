use crate::format::{eocd, record};
use crate::{Error, Result};
use prybar_io::Reader;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// `/`-separated, as decoded; `.comp` entries keep their suffix.
    pub path: String,
    pub offset: usize,
    /// Bytes stored in the package.
    pub stored_size: u32,
    /// Size once decompressed, as declared by the directory.
    pub size: u32,
}

impl Entry {
    pub fn is_compressed(&self) -> bool {
        self.path.ends_with(".comp")
    }
}

#[derive(Debug)]
pub struct Hfs {
    data: Vec<u8>,
    pub entries: Vec<Entry>,
}

impl Hfs {
    pub fn open(path: &Path) -> Result<Self> {
        Self::from_bytes(std::fs::read(path)?)
    }

    /// The count is checked against what the directory could hold before
    /// any allocation, and records are walked forward only, so forged
    /// counts or offsets can neither exhaust memory nor loop.
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let r = Reader::new(&data);
        let end = eocd::find(&data)?;
        let cap = data.len().saturating_sub(end.offset) / record::CENTRAL_LEN;
        if end.count > cap {
            let (count, cap) = (end.count, cap);
            return Err(prybar_io::Error::CountTooLarge { count, cap }.into());
        }
        let mut entries = Vec::with_capacity(end.count);
        let mut at = end.offset;
        for _ in 0..end.count {
            let c = record::central(&r, at)?;
            let offset = record::data_offset(&r, c.local)?;
            entries.push(Entry {
                path: c.name,
                offset,
                stored_size: c.stored_size,
                size: c.size,
            });
            at = c.next;
        }
        Ok(Self { data, entries })
    }

    /// Stored bytes; `.comp` entries are refused rather than returned raw
    /// so callers never mistake compressed bytes for the real file.
    pub fn read(&self, e: &Entry) -> Result<&[u8]> {
        if e.is_compressed() {
            return Err(Error::Unsupported(e.path.clone()));
        }
        let r = Reader::new(&self.data);
        Ok(r.bytes(e.offset, e.stored_size as usize)?)
    }
}
