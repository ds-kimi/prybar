use crate::{Apk, Entry, Error, Result};
use prybar_io::safe_join;
use std::path::{Path, PathBuf};

/// Every entry is checked for traversal before anything is written, so a
/// hostile package leaves no partial output behind.
pub fn extract_all(apk: &Apk, out: &Path) -> Result<usize> {
    for e in &apk.entries {
        safe_join(out, &e.path).ok_or_else(|| Error::UnsafePath(e.path.clone()))?;
    }
    for e in &apk.entries {
        extract_one(apk, e, out)?;
    }
    Ok(apk.entries.len())
}

pub fn extract_one(apk: &Apk, e: &Entry, out: &Path) -> Result<PathBuf> {
    let dest = safe_join(out, &e.path).ok_or_else(|| Error::UnsafePath(e.path.clone()))?;
    let data = apk.read(e)?;
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&dest, data)?;
    Ok(dest)
}
