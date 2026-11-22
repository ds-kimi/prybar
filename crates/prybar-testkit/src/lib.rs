//! Shared helpers for tests that read a local game install.

use std::path::PathBuf;

/// `garrysmod_dir.vpk` of the local Garry's Mod install. Game assets are
/// never committed, so callers skip when this is `None`.
pub fn gmod_dir_vpk() -> Option<PathBuf> {
    let path = prybar_tools::paths::gmod_content_dir()?.join("garrysmod_dir.vpk");
    path.is_file().then_some(path)
}
