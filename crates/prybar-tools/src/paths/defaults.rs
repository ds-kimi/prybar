use std::path::{Path, PathBuf};

/// Usual Steam install roots for this OS, existing or not.
pub fn default_roots() -> Vec<PathBuf> {
    // Steam itself relies on $HOME on Unix. A missing home yields no
    // roots rather than cwd-relative guesses.
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let mut out: Vec<PathBuf> = home.map(unix_roots).unwrap_or_default();
    // Env var instead of a literal drive so non-C: installs still resolve.
    if let Some(pf) = std::env::var_os("ProgramFiles(x86)").filter(|_| cfg!(windows)) {
        out.push(Path::new(&pf).join("Steam"));
    }
    out
}

fn unix_roots(home: PathBuf) -> Vec<PathBuf> {
    let rel: &[&[&str]] = if cfg!(windows) {
        &[]
    } else if cfg!(target_os = "macos") {
        &[&["Library", "Application Support", "Steam"]]
    } else {
        &[
            &[".steam", "steam"],
            &[".local", "share", "Steam"],
            &[
                ".var",
                "app",
                "com.valvesoftware.Steam",
                ".local",
                "share",
                "Steam",
            ],
        ]
    };
    rel.iter()
        .map(|parts| parts.iter().fold(home.clone(), |p, s| p.join(s)))
        .collect()
}
