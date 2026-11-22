use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// The last release answer and when it was fetched, so callers can rate
/// limit checks and GitHub can answer 304.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cached {
    pub etag: Option<String>,
    pub body: String,
    /// Seconds since the Unix epoch.
    pub checked_at: u64,
}

fn file(dir: &Path) -> PathBuf {
    dir.join("update-check.json")
}

/// Reads the cache; a missing or corrupt file is just "no cache".
pub fn load(dir: Option<&Path>) -> Option<Cached> {
    let text = std::fs::read_to_string(file(dir?)).ok()?;
    serde_json::from_str(&text).ok()
}

/// Writes the cache, ignoring failures: a read-only config dir must not
/// break the check itself.
pub fn store(dir: Option<&Path>, etag: Option<String>, body: &[u8]) {
    let Some(dir) = dir else { return };
    let checked_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let entry = Cached {
        etag,
        body: String::from_utf8_lossy(body).into_owned(),
        checked_at,
    };
    if let Ok(text) = serde_json::to_string(&entry) {
        let _ = std::fs::create_dir_all(dir);
        let _ = std::fs::write(file(dir), text);
    }
}
