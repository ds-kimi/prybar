use crate::{Entry, Error, Hfs, Result};
use prybar_io::safe_join;
use std::path::{Path, PathBuf};

/// Every entry is checked for traversal and bounds before anything is
/// written, so a hostile package leaves no partial output behind. `.comp`
/// entries and ZIP-style directory records are skipped, not fatal, so one
/// package still yields its plain files; the return value counts only the
/// files written.
pub fn extract_all(hfs: &Hfs, out: &Path) -> Result<usize> {
    for e in &hfs.entries {
        safe_join(out, &e.path).ok_or_else(|| Error::UnsafePath(e.path.clone()))?;
    }
    let plain: Vec<&Entry> = hfs.entries.iter().filter(|e| is_file(e)).collect();
    for e in &plain {
        hfs.read(e)?;
    }
    for e in &plain {
        extract_one(hfs, e, out)?;
    }
    Ok(plain.len())
}

pub fn extract_one(hfs: &Hfs, e: &Entry, out: &Path) -> Result<PathBuf> {
    let dest = safe_join(out, &e.path).ok_or_else(|| Error::UnsafePath(e.path.clone()))?;
    let data = hfs.read(e)?;
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&dest, data)?;
    Ok(dest)
}

fn is_file(e: &Entry) -> bool {
    !e.is_compressed() && !e.path.ends_with('/')
}
