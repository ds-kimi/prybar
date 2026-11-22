use std::path::{Path, PathBuf};

/// How an installed program can be updated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Plan {
    /// Replace this executable file.
    Binary(PathBuf),
    /// Swap this whole macOS `.app` bundle, keeping its signature intact.
    Bundle(PathBuf),
    /// Self-replace is not allowed; show this command or link instead.
    Manual(String),
}

/// Package managers own these paths; replacing their files behind their back
/// breaks their bookkeeping, so the user gets their command instead.
const MANAGED: &[(&str, &str)] = &[
    ("/Cellar/", "brew upgrade prybar"),
    ("/homebrew/", "brew upgrade prybar"),
    (
        "/.cargo/bin/",
        "cargo install --git https://github.com/ds-kimi/prybar prybar-cli prybar-mcp",
    ),
    ("/nix/store/", "update prybar with nix"),
    ("/snap/", "snap refresh prybar"),
    ("/usr/bin/", "update prybar with your package manager"),
];

/// Decides how to update the program at `exe`; `page` is the release link
/// used when it cannot be replaced in place.
pub fn install_plan(exe: &Path, page: &str) -> Plan {
    let shown = exe.to_string_lossy().replace('\\', "/");
    if let Some((_, cmd)) = MANAGED.iter().find(|(p, _)| shown.contains(p)) {
        return Plan::Manual((*cmd).to_string());
    }
    let target = bundle_of(exe)
        .map(Plan::Bundle)
        .unwrap_or_else(|| Plan::Binary(exe.to_path_buf()));
    let dir = match &target {
        Plan::Bundle(app) | Plan::Binary(app) => app.parent().map(Path::to_path_buf),
        Plan::Manual(_) => None,
    };
    match dir {
        Some(d) if writable(&d) => target,
        _ => Plan::Manual(format!("download the update from {page}")),
    }
}

/// `X.app/Contents/MacOS/bin` -> `X.app`.
fn bundle_of(exe: &Path) -> Option<PathBuf> {
    let macos = exe.parent()?;
    let contents = macos.parent()?;
    let app = contents.parent()?;
    let is_app = macos.file_name()? == "MacOS"
        && contents.file_name()? == "Contents"
        && app.extension()? == "app";
    is_app.then(|| app.to_path_buf())
}

/// Probes by creating a file: permission bits alone miss ACLs and
/// read-only mounts.
fn writable(dir: &Path) -> bool {
    tempfile::Builder::new()
        .prefix(".prybar-probe")
        .tempfile_in(dir)
        .is_ok()
}
