mod target;

pub use target::{asset_for, target_triple, Kind};

use crate::UpdateError;
use semver::Version;
use serde::Deserialize;

/// One downloadable file of a release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    pub name: String,
    pub url: String,
    pub size: u64,
}

/// A published, non-draft, non-prerelease GitHub release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub tag: String,
    pub version: Version,
    pub page: String,
    pub assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct RawAsset {
    name: String,
    browser_download_url: String,
    #[serde(default)]
    size: u64,
}

#[derive(Deserialize)]
struct RawRelease {
    tag_name: String,
    #[serde(default)]
    html_url: String,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    prerelease: bool,
    #[serde(default)]
    assets: Vec<RawAsset>,
}

impl Release {
    /// Parses the GitHub API answer; drafts and prereleases are refused so
    /// a mislabeled release never reaches users.
    pub fn parse(body: &[u8]) -> Result<Self, UpdateError> {
        let raw: RawRelease =
            serde_json::from_slice(body).map_err(|e| UpdateError::BadRelease(e.to_string()))?;
        if raw.draft || raw.prerelease {
            return Err(UpdateError::BadRelease("draft or prerelease".into()));
        }
        let version = Version::parse(raw.tag_name.trim_start_matches('v'))
            .map_err(|e| UpdateError::BadRelease(format!("tag {}: {e}", raw.tag_name)))?;
        if !version.pre.is_empty() {
            return Err(UpdateError::BadRelease("prerelease tag".into()));
        }
        let assets = raw
            .assets
            .into_iter()
            .map(|a| Asset {
                name: a.name,
                url: a.browser_download_url,
                size: a.size,
            })
            .collect();
        Ok(Self {
            tag: raw.tag_name,
            version,
            page: raw.html_url,
            assets,
        })
    }

    /// Finds an asset by exact name.
    pub fn asset(&self, name: &str) -> Option<&Asset> {
        self.assets.iter().find(|a| a.name == name)
    }
}
