use std::path::{Path, PathBuf};

use super::{Game, KnownGame, KNOWN_GAMES};
use crate::paths::find_game_dir;

/// Every known game installed in a Steam library on this machine.
pub fn detect_games() -> Vec<Game> {
    KNOWN_GAMES
        .iter()
        .filter_map(|k| game_from_install(k, &find_game_dir(k.install)?))
        .collect()
}

/// Builds the config entry from an install root; `None` when the
/// content folder has no `gameinfo.txt` (not a usable Source game).
pub fn game_from_install(known: &KnownGame, root: &Path) -> Option<Game> {
    let game_dir = root.join(known.content);
    if !game_dir.join("gameinfo.txt").is_file() {
        return None;
    }
    Some(Game {
        name: known.name.to_string(),
        game_dir,
        studiomdl: find_tool(root, "studiomdl"),
        hlmv: find_tool(root, "hlmv"),
        packer: Some(known.packer),
    })
}

/// 32-bit `bin/` first: every SDK-era game has it, while `bin/win64`
/// only exists on the x64 GMod branch and behaves the same.
fn find_tool(root: &Path, stem: &str) -> Option<PathBuf> {
    let exe = format!("{stem}.exe");
    let bin = root.join("bin");
    [bin.join(&exe), bin.join("win64").join(&exe)]
        .into_iter()
        .find(|p| p.is_file())
}
