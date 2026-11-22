use super::Packer;

/// A Source game prybar can detect under `steamapps/common`.
#[derive(Debug, Clone, Copy)]
pub struct KnownGame {
    pub name: &'static str,
    pub install: &'static str,
    pub content: &'static str,
    pub packer: Packer,
}

const fn vpk(name: &'static str, install: &'static str, content: &'static str) -> KnownGame {
    KnownGame {
        name,
        install,
        content,
        packer: Packer::Vpk,
    }
}

/// Games that ship studiomdl/hlmv or are common compile targets. Only
/// Garry's Mod loads GMA addons; the rest take VPKs or loose files.
pub const KNOWN_GAMES: &[KnownGame] = &[
    KnownGame {
        name: "garrysmod",
        install: "GarrysMod",
        content: "garrysmod",
        packer: Packer::Gma,
    },
    vpk("tf2", "Team Fortress 2", "tf"),
    vpk("css", "Counter-Strike Source", "cstrike"),
    vpk("hl2", "Half-Life 2", "hl2"),
    vpk("l4d2", "Left 4 Dead 2", "left4dead2"),
    vpk("portal2", "Portal 2", "portal2"),
    vpk("sdk2013mp", "Source SDK Base 2013 Multiplayer", "hl2mp"),
];
