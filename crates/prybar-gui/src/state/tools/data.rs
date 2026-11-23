//! Where the app keeps what it learns about Workshop items.

use std::path::PathBuf;

/// Previews live next to the config, under `previews/`.
pub fn default_thumb(url: &str) -> Result<PathBuf, String> {
    prybar_workshop::thumb::cached(url, &data_dir().join("previews"))
}

/// The config's folder, where everything the app keeps about items goes.
pub fn data_dir() -> PathBuf {
    prybar_tools::config::config_path()
        .ok()
        .and_then(|p| Some(p.parent()?.to_path_buf()))
        .unwrap_or_else(|| std::env::temp_dir().join("prybar"))
}
