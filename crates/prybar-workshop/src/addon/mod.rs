//! An addon folder as Garry's Mod sees it: `addon.json` values the game
//! accepts and every file checked against its whitelist, so all problems
//! show at once before anything is packed.

mod meta;

use std::path::Path;

use serde::Serialize;

pub use meta::{check_meta, gmod_tags, ADDON_TAGS, ADDON_TYPES, MAX_ADDON_TAGS};

use crate::WorkshopError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Rejected {
    pub path: String,
    pub reason: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct AddonCheck {
    pub title: String,
    pub kind: String,
    pub tags: Vec<String>,
    /// Files that will be packed.
    pub files: usize,
    pub rejected: Vec<Rejected>,
    /// `addon.json` problems, one sentence each.
    pub problems: Vec<String>,
}

impl AddonCheck {
    pub fn ok(&self) -> bool {
        self.rejected.is_empty() && self.problems.is_empty() && self.files > 0
    }
}

/// Reads and checks `dir`; only a missing or unreadable `addon.json`
/// is an error, everything else lands in the report.
pub fn check_addon(dir: &Path) -> Result<AddonCheck, WorkshopError> {
    let json_path = dir.join("addon.json");
    let gma = |e: prybar_gma::Error| WorkshopError::invalid("addon.json", e.to_string());
    if !json_path.is_file() {
        return Err(WorkshopError::invalid(
            "content",
            "the folder has no addon.json at its root",
        ));
    }
    let json = prybar_gma::AddonJson::load(&json_path).map_err(gma)?;
    let scan = prybar_gma::scan(dir).map_err(gma)?;
    let rejected = rejected(&scan);
    let mut problems = check_meta(&json);
    if scan.is_empty() {
        problems.push("the folder has no files to pack besides addon.json".into());
    }
    Ok(AddonCheck {
        title: json.title.clone(),
        kind: json.kind.to_lowercase(),
        tags: json.tags.iter().map(|t| t.to_lowercase()).collect(),
        files: scan.len() - rejected.len(),
        rejected,
        problems,
    })
}

fn rejected(scan: &[(String, Option<&'static str>)]) -> Vec<Rejected> {
    let bad = scan.iter().filter_map(|(path, why)| why.map(|r| (path, r)));
    bad.map(|(path, reason)| Rejected {
        path: path.clone(),
        reason: reason.into(),
    })
    .collect()
}
