use super::placer::Placer;
use crate::{Entry, Error, Result};
use std::path::{Path, PathBuf};

/// Returns the entries plus the embedded data blob (empty when chunked).
pub(crate) fn place(
    files: &[(String, PathBuf)],
    out: &Path,
    chunk: Option<u64>,
) -> Result<(Vec<Entry>, Vec<u8>)> {
    let mut p = Placer::new(out, chunk);
    let mut entries = Vec::with_capacity(files.len());
    for (rel, path) in files {
        entries.push(place_one(&mut p, rel, path)?);
    }
    p.finish()?;
    Ok((entries, p.embedded))
}

fn place_one(p: &mut Placer, rel: &str, path: &Path) -> Result<Entry> {
    let data = std::fs::read(path)?;
    let length = u32::try_from(data.len()).map_err(|_| Error::TooLarge)?;
    let (archive_index, offset) = p.put(&data)?;
    Ok(Entry {
        path: rel.to_owned(),
        crc: crc32fast::hash(&data),
        preload: Vec::new(),
        archive_index,
        offset,
        length,
    })
}
