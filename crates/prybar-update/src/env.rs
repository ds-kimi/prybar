use crate::{cached_status, Config, Status};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// The CLI and MCP check at most once in this window.
pub const CHECK_EVERY: Duration = Duration::from_secs(24 * 60 * 60);

impl Config {
    /// `Config::new` plus overrides from the environment:
    /// `PRYBAR_UPDATE_URL` (release API, for tests and mirrors) and
    /// `PRYBAR_UPDATE_CACHE` (cache dir).
    pub fn from_env(current: &str) -> Self {
        let mut cfg = Self::new(current);
        if let Some(url) = std::env::var_os("PRYBAR_UPDATE_URL") {
            cfg.api_url = url.to_string_lossy().into_owned();
        }
        if let Some(dir) = std::env::var_os("PRYBAR_UPDATE_CACHE") {
            cfg.cache_dir = Some(dir.into());
        }
        cfg
    }
}

/// Automatic checks are off on request (`PRYBAR_NO_UPDATE_CHECK=1`) and in
/// CI, where a notice is noise and the network may be metered.
pub fn checks_disabled() -> bool {
    let on = |k: &str| std::env::var_os(k).is_some_and(|v| !v.is_empty() && v != "0");
    on("PRYBAR_NO_UPDATE_CHECK") || on("CI")
}

/// The cached status when it is younger than `max_age`; `None` means a new
/// check is due.
pub fn fresh_status(cfg: &Config, max_age: Duration) -> Option<Status> {
    let (status, at) = cached_status(cfg)?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    (now.saturating_sub(at) < max_age.as_secs()).then_some(status)
}
