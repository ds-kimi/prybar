use crate::{Entry, Error, Gma, Result};
use prybar_io::safe_join;
use std::path::{Path, PathBuf};

/// Every entry is checked for traversal before anything is written, so a
/// hostile addon leaves no partial output behind.
pub fn extract_all(gma: &Gma, out: &Path) -> Result<usize> {
    for e in &gma.entries {
        safe_join(out, &e.path).ok_or_else(|| Error::UnsafePath(e.path.clone()))?;
    }
    for e in &gma.entries {
        extract_one(gma, e, out)?;
    }
    Ok(gma.entries.len())
}

pub fn extract_one(gma: &Gma, e: &Entry, out: &Path) -> Result<PathBuf> {
    let dest = safe_join(out, &e.path).ok_or_else(|| Error::UnsafePath(e.path.clone()))?;
    let data = gma.read(e)?;
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&dest, data)?;
    Ok(dest)
}
