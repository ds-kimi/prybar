use std::path::PathBuf;

/// The OS user config dir. Hand-rolled rather than the `dirs` crate,
/// whose `option-ext` dependency is MPL-2.0 and fails our license policy.
pub fn user_config_dir() -> Option<PathBuf> {
    let env = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty());
    if cfg!(windows) {
        return env("APPDATA").map(PathBuf::from);
    }
    let home = env("HOME").map(PathBuf::from);
    if cfg!(target_os = "macos") {
        return home.map(|h| h.join("Library").join("Application Support"));
    }
    env("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .or_else(|| home.map(|h| h.join(".config")))
}
