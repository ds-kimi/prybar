//! Search roots derived from the model path and the game folders.

use std::path::{Path, PathBuf};

/// The folder that holds `models/` above a loose model, as a game dir would.
pub fn tree_root(model: &Path) -> Option<PathBuf> {
    let at = model.ancestors().find(|a| {
        a.file_name()
            .is_some_and(|n| n.eq_ignore_ascii_case("models"))
    })?;
    at.parent().map(Path::to_path_buf)
}

/// `*_dir.vpk` in the game dir and, for Garry's Mod, the shared
/// `sourceengine` folder next to it where the HL2 content lives.
pub fn game_vpks(dir: &Path) -> impl Iterator<Item = PathBuf> {
    let shared = dir.parent().map(|p| p.join("sourceengine"));
    let dirs = std::iter::once(dir.to_path_buf()).chain(shared);
    let mut vpks: Vec<PathBuf> = dirs
        .filter_map(|d| std::fs::read_dir(d).ok())
        .flat_map(|rd| rd.filter_map(|e| e.ok().map(|e| e.path())))
        .filter(|p| {
            p.to_string_lossy()
                .to_ascii_lowercase()
                .ends_with("_dir.vpk")
        })
        .collect();
    vpks.sort();
    vpks.into_iter()
}
