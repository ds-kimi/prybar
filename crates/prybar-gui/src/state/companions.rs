//! Finds a model's companion files on disk or inside its package, so a
//! model opened from a VPK shows the same list as one on disk.

use std::path::{Path, PathBuf};

use prybar_service::package::Package;
use prybar_service::source::ModelRef;

/// (suffix, found) for every suffix, next to the `.mdl` at `path`.
pub fn found(path: &Path, suffixes: &[&'static str]) -> Vec<(&'static str, bool)> {
    let text = path.to_string_lossy();
    match ModelRef::parse(&text) {
        ModelRef::Path(p) => on_disk(&p, suffixes),
        ModelRef::Packed { package, inner } => packed(&package, &inner, suffixes),
    }
}

fn on_disk(path: &Path, suffixes: &[&'static str]) -> Vec<(&'static str, bool)> {
    let stem = path.with_extension("");
    let has = |s: &str| PathBuf::from(format!("{}{s}", stem.display())).is_file();
    suffixes.iter().map(|s| (*s, has(s))).collect()
}

/// Package paths are case-insensitive in the engine, so compare lowered.
fn packed(package: &Path, inner: &str, suffixes: &[&'static str]) -> Vec<(&'static str, bool)> {
    let lower = inner.to_ascii_lowercase();
    let stem = lower.strip_suffix(".mdl").unwrap_or(&lower);
    let names: Vec<String> = Package::open(package)
        .map(|p| p.entries())
        .unwrap_or_default()
        .into_iter()
        .map(|e| e.path.to_ascii_lowercase())
        .collect();
    let has = |s: &str| names.iter().any(|n| n == &format!("{stem}{s}"));
    suffixes.iter().map(|s| (*s, has(s))).collect()
}
