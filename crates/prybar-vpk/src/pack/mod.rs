mod place;
mod placer;

use crate::format::{archive_path, write_header, write_tree};
use crate::{Error, Result};
use place::place;
use prybar_io::Writer;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackOptions {
    /// 1 or 2.
    pub version: u32,
    /// `None` embeds all data in `out`; `Some(n)` splits it into
    /// `name_NNN.vpk` archives of about `n` bytes and needs `out` to end in `_dir.vpk`.
    pub chunk_size: Option<u64>,
}

/// Packs every file under `src`; returns the number of files.
pub fn pack(src: &Path, out: &Path, opts: &PackOptions) -> Result<usize> {
    if !matches!(opts.version, 1 | 2) {
        return Err(Error::BadVersion(opts.version));
    }
    if opts.chunk_size.is_some() && archive_path(out, 0).is_none() {
        return Err(Error::BadOutputName);
    }
    let files = collect(src)?;
    let (entries, embedded) = place(&files, out, opts.chunk_size)?;
    let tree = write_tree(&entries)?;
    let tree_size = u32::try_from(tree.len()).map_err(|_| Error::TooLarge)?;
    let data_size = u32::try_from(embedded.len()).map_err(|_| Error::TooLarge)?;
    let mut w = Writer::new();
    write_header(&mut w, opts.version, tree_size, data_size);
    w.put_bytes(&tree).put_bytes(&embedded);
    std::fs::write(out, w.into_inner())?;
    Ok(entries.len())
}

/// Sorted so output is deterministic across filesystems.
fn collect(src: &Path) -> Result<Vec<(String, PathBuf)>> {
    let mut out = Vec::new();
    for item in WalkDir::new(src).sort_by_file_name() {
        let item = item?;
        if !item.file_type().is_file() {
            continue;
        }
        let rel = item.path().strip_prefix(src).unwrap_or(item.path());
        let rel = rel.to_str().ok_or_else(|| Error::NonUtf8(rel.into()))?;
        out.push((rel.replace('\\', "/"), item.path().to_path_buf()));
    }
    Ok(out)
}
