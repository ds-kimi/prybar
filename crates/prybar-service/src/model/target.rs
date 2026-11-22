use crate::source::Batch;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Output folder per model. Package entries keep their inner path
/// (`models/props/crate`), files on disk use their name; clashes get a
/// `_2`, `_3`... suffix so two `crate.mdl` never overwrite each other.
pub fn targets(batch: &Batch, out: &Path, taken: &mut HashSet<String>) -> Vec<(String, PathBuf)> {
    batch
        .stems
        .iter()
        .map(|stem| {
            let rel = match batch.package {
                Some(_) => stem.as_str(),
                None => stem.rsplit(['/', '\\']).next().unwrap_or(stem),
            };
            let base = prybar_io::safe_join(out, rel).unwrap_or_else(|| out.join("model"));
            (stem.clone(), unique(taken, base))
        })
        .collect()
}

/// Keys are lower-cased: on Windows and macOS `Crate` and `crate` are
/// the same folder.
fn unique(taken: &mut HashSet<String>, base: PathBuf) -> PathBuf {
    let mut candidate = base.clone();
    let mut n = 2;
    while !taken.insert(candidate.display().to_string().to_lowercase()) {
        let mut name = base.file_name().unwrap_or_default().to_os_string();
        name.push(format!("_{n}"));
        candidate = base.with_file_name(name);
        n += 1;
    }
    candidate
}
