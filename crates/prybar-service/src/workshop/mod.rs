//! Workshop helpers the publish screen uses, exposed to the CLI and MCP:
//! description and preview checks, the searchable item list and an
//! item's stats and version history.

mod check;
mod history;
mod items;

pub use check::{check_bbcode, check_preview, BbcodeReport, PreviewReport};
pub use history::{workshop_item, ChangeEntry, ItemReport, StatPoint};
pub use items::{workshop_items, ItemQuery, ItemsReport};

use std::path::PathBuf;

/// The config's folder, where the GUI also keeps stats snapshots and
/// changelog copies, so both front ends read and grow the same history.
pub fn data_dir() -> PathBuf {
    prybar_tools::config::config_path()
        .ok()
        .and_then(|p| Some(p.parent()?.to_path_buf()))
        .unwrap_or_else(|| std::env::temp_dir().join("prybar"))
}
