use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    FriendsOnly,
    /// Default so a mistaken first publish is never world-visible.
    #[default]
    Private,
    Unlisted,
}

/// A create (no `item_id`) or update (`item_id` set). On update, `None`
/// fields keep their current Steam value.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemSpec {
    pub app_id: u32,
    pub item_id: Option<u64>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub visibility: Option<Visibility>,
    pub preview: Option<PathBuf>,
    /// A folder, or for Garry's Mod a single `.gma` file.
    pub content: Option<PathBuf>,
    pub change_note: Option<String>,
}
