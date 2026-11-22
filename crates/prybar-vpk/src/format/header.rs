use crate::{Error, Result};
use prybar_io::{Reader, Writer};

const SIGNATURE: u32 = 0x55aa_1234;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub version: u32,
    pub tree_size: u32,
    /// Size of the fixed header, i.e. where the tree starts.
    pub header_len: usize,
}

impl Header {
    /// Embedded file data starts right after the tree.
    pub fn data_base(&self) -> usize {
        self.header_len + self.tree_size as usize
    }
}

pub(crate) fn parse(r: &Reader) -> Result<Header> {
    if r.read::<u32>(0)? != SIGNATURE {
        return Err(Error::BadSignature);
    }
    let version = r.read::<u32>(4)?;
    let header_len = match version {
        1 => 12,
        2 => 28,
        v => return Err(Error::BadVersion(v)),
    };
    let tree_size = r.read::<u32>(8)?;
    Ok(Header {
        version,
        tree_size,
        header_len,
    })
}

/// v2 MD5 and signature sections are left empty; Valve tools accept that.
pub(crate) fn write(w: &mut Writer, version: u32, tree_size: u32, data_size: u32) {
    w.put(SIGNATURE).put(version).put(tree_size);
    if version == 2 {
        w.put(data_size).put(0u32).put(0u32).put(0u32);
    }
}
