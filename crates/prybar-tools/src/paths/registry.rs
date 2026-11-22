use std::path::PathBuf;

/// `HKCU\Software\Valve\Steam\SteamPath`, the Steam client's own record.
#[cfg(windows)]
pub fn steam_path() -> Option<PathBuf> {
    let key = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let steam = key.open_subkey(r"Software\Valve\Steam").ok()?;
    let path: String = steam.get_value("SteamPath").ok()?;
    Some(PathBuf::from(path))
}

#[cfg(not(windows))]
pub fn steam_path() -> Option<PathBuf> {
    None
}
