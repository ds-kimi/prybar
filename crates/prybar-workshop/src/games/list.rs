use serde::Serialize;

/// What one Workshop item of the game holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Content {
    /// One `.gma`, packed from an addon folder with `addon.json`.
    Gma,
    /// A folder with at least one compiled map (`.bsp`).
    Maps,
}

/// A game prybar can publish to.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct WorkshopGame {
    /// Preset name, the same as in `prybar games`.
    pub id: &'static str,
    pub name: &'static str,
    pub app_id: u32,
    /// Folder under `steamapps/common`, used to tell if it is installed.
    pub install: &'static str,
    pub content: Content,
    /// Tags the game's Workshop filters on; others are accepted but
    /// do not show up in its filters.
    pub tags: &'static [&'static str],
    /// One line on what the content must be, shown before a publish.
    pub rules: &'static str,
}

/// A game prybar knows but cannot publish to, with the reason.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Unsupported {
    pub id: &'static str,
    pub name: &'static str,
    pub app_id: u32,
    pub reason: &'static str,
}

pub const WORKSHOP_GAMES: &[WorkshopGame] = &[
    WorkshopGame {
        id: "garrysmod",
        name: "Garry's Mod",
        app_id: 4000,
        install: "GarrysMod",
        content: Content::Gma,
        tags: &[
            "Addon", "gamemode", "map", "weapon", "vehicle", "npc", "entity", "tool", "effects",
            "model", "servercontent", "fun", "roleplay", "scenic", "movie", "realism", "cartoon",
            "water", "comic", "build",
        ],
        rules: "an addon folder with addon.json (whitelisted files only) or a .gma",
    },
    WorkshopGame {
        id: "tf2",
        name: "Team Fortress 2",
        app_id: 440,
        install: "Team Fortress 2",
        content: Content::Maps,
        tags: &["Maps"],
        rules: "a folder with at least one compiled map (.bsp); cosmetics go through the in-game importer",
    },
];

pub const UNSUPPORTED: &[Unsupported] = &[
    Unsupported {
        id: "l4d2",
        name: "Left 4 Dead 2",
        app_id: 550,
        reason: "uses the legacy single-file Workshop; publish with Valve's L4D2 Workshop Publisher",
    },
    Unsupported {
        id: "portal2",
        name: "Portal 2",
        app_id: 620,
        reason: "uses the legacy Workshop; publish from the Puzzle Maker or Portal 2 Authoring Tools",
    },
    Unsupported {
        id: "css",
        name: "Counter-Strike: Source",
        app_id: 240,
        reason: "has no Steam Workshop",
    },
    Unsupported {
        id: "hl2",
        name: "Half-Life 2",
        app_id: 220,
        reason: "has no Steam Workshop",
    },
    Unsupported {
        id: "sdk2013mp",
        name: "Source SDK Base 2013 Multiplayer",
        app_id: 243750,
        reason: "has no Steam Workshop",
    },
];
