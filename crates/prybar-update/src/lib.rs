//! Update checks for prybar: find the latest GitHub release, download the
//! asset for this platform, verify it against `SHA256SUMS` and its minisign
//! signature, then replace the installed binary or app bundle.
//!
//! Checks never fail loudly: a network problem reads as "status unknown" so
//! the tools keep working offline.

mod apply;
mod check;
mod download;
mod env;
mod error;
mod key;
mod net;
mod release;
mod verify;

use std::path::PathBuf;
use std::time::Duration;

pub use apply::{apply, install_plan, Applied, Plan};
pub use check::{cached_status, check, Status};
pub use download::{download_verified, Verified};
pub use env::{checks_disabled, fresh_status, CHECK_EVERY};
pub use error::UpdateError;
pub use key::embedded_key;
pub use release::{asset_for, target_triple, Asset, Kind, Release};
pub use verify::{parse_sums, verify_signature};

/// Where the latest release is published.
pub const LATEST_URL: &str = "https://api.github.com/repos/ds-kimi/prybar/releases/latest";

/// Settings shared by every step; tests point `api_url` at a local server.
#[derive(Debug, Clone)]
pub struct Config {
    pub api_url: String,
    /// Holds the ETag cache; `None` disables caching.
    pub cache_dir: Option<PathBuf>,
    pub current: semver::Version,
    pub timeout: Duration,
    /// Minisign public key (the `.pub` file text). `None` means updates can
    /// only be offered as a link, never installed.
    pub public_key: Option<String>,
}

impl Config {
    /// Defaults for a shipped binary of version `current`.
    pub fn new(current: &str) -> Self {
        Self {
            api_url: LATEST_URL.to_string(),
            cache_dir: dirs::cache_dir().map(|d| d.join("prybar")),
            current: semver::Version::parse(current).unwrap_or(semver::Version::new(0, 0, 0)),
            timeout: Duration::from_secs(5),
            public_key: embedded_key().map(str::to_string),
        }
    }
}
