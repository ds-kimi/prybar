use crate::{Entry, Error, Result, Vpk};
use prybar_io::safe_join;
use std::path::{Path, PathBuf};

/// Every entry is checked for traversal before anything is written, so a
/// hostile archive leaves no partial output behind.
pub fn extract_all(vpk: &Vpk, out: &Path) -> Result<usize> {
    for e in &vpk.entries {
        safe_join(out, &e.path).ok_or_else(|| Error::UnsafePath(e.path.clone()))?;
    }
    for e in &vpk.entries {
        extract_one(vpk, e, out)?;
    }
    Ok(vpk.entries.len())
}

pub fn extract_one(vpk: &Vpk, e: &Entry, out: &Path) -> Result<PathBuf> {
    let dest = safe_join(out, &e.path).ok_or_else(|| Error::UnsafePath(e.path.clone()))?;
    let data = vpk.read(e)?;
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&dest, data)?;
    Ok(dest)
}
