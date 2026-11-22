use std::path::PathBuf;

/// Folder name under `steamapps/common`, same on every OS.
const GMOD: &str = "GarrysMod";

/// `steamapps/common/<name>` in the first Steam library that has it.
pub fn find_game_dir(name: &str) -> Option<PathBuf> {
    super::steam_libraries()
        .into_iter()
        .map(|lib| lib.join("steamapps").join("common").join(name))
        .find(|dir| dir.is_dir())
}

/// Garry's Mod install: `PRYBAR_GMOD_DIR`, else Steam detection.
pub fn gmod_dir() -> Option<PathBuf> {
    match std::env::var_os("PRYBAR_GMOD_DIR") {
        Some(dir) => Some(PathBuf::from(dir)).filter(|d| d.is_dir()),
        None => find_game_dir(GMOD),
    }
}

/// The `garrysmod` content folder (holds `garrysmod_dir.vpk`). On macOS
/// it may sit inside the signed app bundle instead of the install root;
/// that layout is unverified on a real Mac, so both are probed.
pub fn gmod_content_dir() -> Option<PathBuf> {
    let root = gmod_dir()?;
    let bundle = ["GarrysMod_Signed.app", "Contents", "MacOS", "garrysmod"]
        .iter()
        .fold(root.clone(), |p, s| p.join(s));
    [root.join("garrysmod"), bundle]
        .into_iter()
        .find(|d| d.is_dir())
}
