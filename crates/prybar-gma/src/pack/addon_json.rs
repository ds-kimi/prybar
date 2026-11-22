use crate::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// The `addon.json` gmad reads from the addon root.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct AddonJson {
    pub title: String,
    #[serde(default, rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,
    #[serde(default)]
    pub description: String,
}

#[derive(Serialize)]
struct StoredDescription<'a> {
    description: &'a str,
    #[serde(rename = "type")]
    kind: &'a str,
    tags: &'a [String],
}

impl AddonJson {
    pub fn load(path: &Path) -> Result<Self> {
        Ok(serde_json::from_slice(&std::fs::read(path)?)?)
    }

    /// gmod reads the addon type and tags back from this JSON blob.
    pub(crate) fn stored_description(&self) -> Result<String> {
        let d = StoredDescription {
            description: &self.description,
            kind: &self.kind,
            tags: &self.tags,
        };
        Ok(serde_json::to_string(&d)?)
    }
}
