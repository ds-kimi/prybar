use crate::UpdateError;
use std::path::{Path, PathBuf};

/// Deepest a release archive nests its files (`prybar-vX-target/file`).
const MAX_DEPTH: usize = 3;

/// Finds the entry named `name` in the unpacked archive.
pub fn find(root: &Path, name: &str) -> Result<PathBuf, UpdateError> {
    search(root, name, 0).ok_or_else(|| UpdateError::MissingAsset(format!("{name} in archive")))
}

fn search(dir: &Path, name: &str, depth: usize) -> Option<PathBuf> {
    let entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .collect();
    if let Some(hit) = entries
        .iter()
        .find(|p| p.file_name().is_some_and(|n| n == name))
    {
        return Some(hit.clone());
    }
    if depth >= MAX_DEPTH {
        return None;
    }
    entries
        .iter()
        .filter(|p| p.is_dir())
        .find_map(|p| search(p, name, depth + 1))
}

/// Swaps a whole directory (an `.app` bundle) so its signature stays
/// valid; the old one is restored if the second rename fails.
pub fn swap_dir(new: &Path, target: &Path) -> Result<(), UpdateError> {
    let old = target.with_extension("app.old-update");
    let _ = std::fs::remove_dir_all(&old);
    std::fs::rename(target, &old)?;
    if let Err(e) = std::fs::rename(new, target) {
        let _ = std::fs::rename(&old, target);
        return Err(e.into());
    }
    let _ = std::fs::remove_dir_all(&old);
    Ok(())
}
