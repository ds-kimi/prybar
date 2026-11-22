use crate::{Error, Result};
use prybar_io::{Cursor, Reader};
use std::path::Path;

pub const APK_ID: u32 = 0x2357;
const MAX_NAME: usize = 4096;
/// Smallest possible record: five u32 fields plus an empty name's NUL.
const MIN_RECORD: usize = 21;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// `/`-separated; the format stores `\`.
    pub path: String,
    pub offset: u32,
    pub size: u32,
}

#[derive(Debug)]
pub struct Apk {
    data: Vec<u8>,
    pub entries: Vec<Entry>,
}

impl Apk {
    pub fn open(path: &Path) -> Result<Self> {
        Self::from_bytes(std::fs::read(path)?)
    }

    /// The count is checked against what the directory could hold before
    /// any allocation, so a forged count cannot exhaust memory.
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let r = Reader::new(&data);
        if r.read::<u32>(0)? != APK_ID {
            return Err(Error::BadMagic);
        }
        let count = r.read::<u32>(8)? as usize;
        let dir = r.read::<u32>(12)? as usize;
        let cap = data.len().saturating_sub(dir) / MIN_RECORD;
        if count > cap {
            return Err(prybar_io::Error::CountTooLarge { count, cap }.into());
        }
        let mut c = Cursor::new(r, dir);
        let entries = (0..count)
            .map(|_| parse_entry(&mut c))
            .collect::<Result<_>>()?;
        Ok(Self { data, entries })
    }

    pub fn read(&self, e: &Entry) -> Result<&[u8]> {
        let r = Reader::new(&self.data);
        Ok(r.bytes(e.offset as usize, e.size as usize)?)
    }
}

/// Records are read back to back; the stored next-entry offset is
/// redundant and ignored so a forged one cannot create a loop.
fn parse_entry(c: &mut Cursor) -> Result<Entry> {
    let name_size = c.read::<u32>()? as usize;
    let path = c.cstr(name_size.saturating_add(1).min(MAX_NAME))?;
    let path = path.replace('\\', "/");
    let offset = c.read::<u32>()?;
    let size = c.read::<u32>()?;
    c.read::<u32>()?;
    c.read::<u32>()?;
    Ok(Entry { path, offset, size })
}
