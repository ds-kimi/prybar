use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Package format the game loads addons from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Packer {
    Gma,
    Vpk,
}

/// One configured game. `game_dir` is the folder holding `gameinfo.txt`,
/// the value studiomdl expects after `-game`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub game_dir: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub studiomdl: Option<PathBuf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hlmv: Option<PathBuf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packer: Option<Packer>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub games: Vec<Game>,
}

impl Config {
    /// Game names are matched case-insensitively: agents and users type
    /// "garrysmod" as often as "GarrysMod".
    pub fn game(&self, name: &str) -> Option<&Game> {
        self.games
            .iter()
            .find(|g| g.name.eq_ignore_ascii_case(name))
    }

    /// Detected games are added unless a game of that name is already
    /// configured, so hand-edited paths survive a re-detection.
    pub fn merge(&mut self, found: Vec<Game>) {
        for game in found {
            if self.game(&game.name).is_none() {
                self.games.push(game);
            }
        }
    }
}
