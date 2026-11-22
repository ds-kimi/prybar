use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GamesArgs {
    /// Also add games found in the local Steam libraries.
    #[serde(default)]
    pub detect: bool,
    /// Save the merged list to the config file. Hand-edited games are kept.
    #[serde(default)]
    pub save: bool,
}

/// Unknown fields are refused so an agent passing `studiomdl` learns it
/// is not settable here instead of seeing it silently dropped.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ConfigSetArgs {
    /// Game name, matched case-insensitively. An existing game of that name is replaced; its studiomdl and hlmv paths are kept.
    pub name: String,
    /// Remove the game instead of setting it; the other fields are then ignored.
    #[serde(default)]
    pub remove: bool,
    /// Absolute folder holding the game's `gameinfo.txt` (required unless `remove`).
    pub game_dir: Option<String>,
    /// Addon package format the game loads: `gma` or `vpk`.
    pub packer: Option<String>,
}
