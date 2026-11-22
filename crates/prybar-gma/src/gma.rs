use crate::format::{parse_header, parse_table, Entry, Header};
use crate::{Error, Result};
use prybar_io::{Cursor, Reader};
use std::path::Path;

/// A fully loaded addon; GMAs are small enough to keep in memory.
#[derive(Debug)]
pub struct Gma {
    data: Vec<u8>,
    data_start: usize,
    pub header: Header,
    pub entries: Vec<Entry>,
}

impl Gma {
    pub fn open(path: &Path) -> Result<Self> {
        Self::from_bytes(std::fs::read(path)?)
    }

    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let mut c = Cursor::new(Reader::new(&data), 0);
        let header = parse_header(&mut c)?;
        let entries = parse_table(&mut c)?;
        let data_start = c.pos;
        Ok(Self {
            data,
            data_start,
            header,
            entries,
        })
    }

    /// Returns the entry's bytes, verified against the stored CRC.
    pub fn read(&self, e: &Entry) -> Result<&[u8]> {
        let bad = || Error::BadSize(e.path.clone());
        let offset = usize::try_from(e.offset).map_err(|_| bad())?;
        let size = usize::try_from(e.size).map_err(|_| bad())?;
        let start = self.data_start.checked_add(offset).ok_or_else(bad)?;
        let bytes = Reader::new(&self.data).bytes(start, size)?;
        if crc32fast::hash(bytes) != e.crc {
            return Err(Error::Crc(e.path.clone()));
        }
        Ok(bytes)
    }
}
