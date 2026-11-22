use crate::{Result, EMBEDDED};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

/// `name_dir.vpk` + index 3 -> `name_003.vpk`. `None` when the dir file
/// does not follow the multi-archive naming or the index is the embedded marker.
pub fn archive_path(dir: &Path, index: u16) -> Option<PathBuf> {
    if index >= EMBEDDED {
        return None;
    }
    let stem = dir.file_name()?.to_str()?.strip_suffix("_dir.vpk")?;
    Some(dir.with_file_name(format!("{stem}_{index:03}.vpk")))
}

/// Length is checked against the real file size before allocating so a
/// forged entry cannot request gigabytes.
pub(crate) fn read_range(path: &Path, offset: u32, len: u32) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let size = file.metadata()?.len();
    if u64::from(offset) + u64::from(len) > size {
        return Err(prybar_io::Error::OutOfBounds {
            offset: offset as usize,
            len: len as usize,
            size: usize::try_from(size).unwrap_or(usize::MAX),
        }
        .into());
    }
    file.seek(SeekFrom::Start(offset.into()))?;
    let mut out = vec![0; len as usize];
    file.read_exact(&mut out)?;
    Ok(out)
}
