use std::path::PathBuf;

/// Steam install roots that exist, `PRYBAR_STEAM_DIR` first.
pub fn steam_roots() -> Vec<PathBuf> {
    let env = std::env::var_os("PRYBAR_STEAM_DIR").map(PathBuf::from);
    let found = env
        .into_iter()
        .chain(super::registry::steam_path())
        .chain(super::defaults::default_roots());
    dedup_dirs(found)
}

/// Every library folder of every root: the root itself plus the paths
/// listed in `steamapps/libraryfolders.vdf`.
pub fn steam_libraries() -> Vec<PathBuf> {
    let libs = steam_roots().into_iter().flat_map(|root| {
        let vdf = root.join("steamapps").join("libraryfolders.vdf");
        let listed = std::fs::read(vdf).map(|b| super::library_paths(&b));
        std::iter::once(root).chain(listed.unwrap_or_default())
    });
    dedup_dirs(libs)
}

fn dedup_dirs(paths: impl Iterator<Item = PathBuf>) -> Vec<PathBuf> {
    // Compare canonical forms: on Linux ~/.steam/steam is usually a
    // symlink to ~/.local/share/Steam. Keep the path as given.
    let mut seen: Vec<PathBuf> = Vec::new();
    let mut out: Vec<PathBuf> = Vec::new();
    for p in paths.filter(|p| p.is_dir()) {
        let key = std::fs::canonicalize(&p).unwrap_or_else(|_| p.clone());
        if !seen.contains(&key) {
            seen.push(key);
            out.push(p);
        }
    }
    out
}
