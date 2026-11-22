use crate::format::{archive_path, parse_header, parse_tree, read_range, Header};
use crate::{Entry, Error, Result};
use prybar_io::Reader;
use std::path::{Path, PathBuf};

/// An opened directory file. Archive files are only touched on `read`.
#[derive(Debug)]
pub struct Vpk {
    dir_path: PathBuf,
    data: Vec<u8>,
    pub header: Header,
    pub entries: Vec<Entry>,
}

impl Vpk {
    pub fn open(path: &Path) -> Result<Self> {
        let mut vpk = Self::from_bytes(std::fs::read(path)?)?;
        vpk.dir_path = path.to_path_buf();
        Ok(vpk)
    }

    /// Parses an in-memory dir file; entries stored in numbered archives
    /// resolve next to an empty path, so only embedded data is readable.
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let r = Reader::new(&data);
        let header = parse_header(&r)?;
        let tree = r.bytes(0, header.data_base())?;
        let entries = parse_tree(Reader::new(tree), header.header_len)?;
        Ok(Self {
            dir_path: PathBuf::new(),
            data,
            header,
            entries,
        })
    }

    /// Returns preload + archive bytes, verified against the stored CRC.
    pub fn read(&self, e: &Entry) -> Result<Vec<u8>> {
        let mut out = e.preload.clone();
        if e.length > 0 {
            out.extend_from_slice(&self.archive_bytes(e)?);
        }
        if crc32fast::hash(&out) != e.crc {
            return Err(Error::Crc(e.path.clone()));
        }
        Ok(out)
    }

    fn archive_bytes(&self, e: &Entry) -> Result<Vec<u8>> {
        if e.is_embedded() {
            let r = Reader::new(&self.data);
            let base = self.header.data_base().saturating_add(e.offset as usize);
            return Ok(r.bytes(base, e.length as usize)?.to_vec());
        }
        let path = archive_path(&self.dir_path, e.archive_index)
            .ok_or_else(|| Error::UnsafePath(e.path.clone()))?;
        read_range(&path, e.offset, e.length)
    }
}
