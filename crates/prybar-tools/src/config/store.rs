use std::path::{Path, PathBuf};

use super::Config;
use crate::ToolError;

/// A real config is a few KB; anything bigger is not ours.
const MAX_BYTES: u64 = 1 << 20;

/// `PRYBAR_CONFIG` wins so tests and CI never touch the user's file;
/// otherwise `<user config dir>/prybar/config.toml` for this OS.
pub fn config_path() -> Result<PathBuf, ToolError> {
    if let Some(path) = std::env::var_os("PRYBAR_CONFIG") {
        return Ok(PathBuf::from(path));
    }
    let dir = crate::paths::user_config_dir().ok_or(ToolError::NoConfigDir)?;
    Ok(dir.join("prybar").join("config.toml"))
}

/// A missing file is an empty config, not an error: first run.
pub fn load(path: &Path) -> Result<Config, ToolError> {
    let meta = match std::fs::metadata(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Config::default()),
        other => other.map_err(|e| ToolError::io(path, e))?,
    };
    if meta.len() > MAX_BYTES {
        return Err(ToolError::config(path, "file larger than 1 MiB"));
    }
    let text = std::fs::read_to_string(path).map_err(|e| ToolError::io(path, e))?;
    toml::from_str(&text).map_err(|e| ToolError::config(path, &e.to_string()))
}

pub fn load_default() -> Result<Config, ToolError> {
    load(&config_path()?)
}

pub fn save(path: &Path, config: &Config) -> Result<(), ToolError> {
    let text =
        toml::to_string_pretty(config).map_err(|e| ToolError::config(path, &e.to_string()))?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| ToolError::io(dir, e))?;
    }
    std::fs::write(path, text).map_err(|e| ToolError::io(path, e))
}
