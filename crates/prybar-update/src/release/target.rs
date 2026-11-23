use super::{Asset, Release};

/// Which program is being updated; picks the file inside the archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Cli,
    Mcp,
    /// The GUI; on macOS this is the whole `Prybar.app` bundle, elsewhere a
    /// plain `prybar-gui` binary.
    Gui,
}

impl Kind {
    /// File name of the program inside a release archive.
    pub fn file_name(self) -> &'static str {
        let os = std::env::consts::OS;
        match (self, os) {
            (Kind::Cli, "windows") => "prybar.exe",
            (Kind::Cli, _) => "prybar",
            (Kind::Mcp, "windows") => "prybar-mcp.exe",
            (Kind::Mcp, _) => "prybar-mcp",
            (Kind::Gui, "macos") => "Prybar.app",
            (Kind::Gui, "windows") => "prybar-gui.exe",
            // Linux ships a plain binary next to the CLI.
            (Kind::Gui, _) => "prybar-gui",
        }
    }
}

/// The release target this binary was built for, as named in release.yml.
pub fn target_triple() -> Option<&'static str> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    match (os, arch) {
        ("windows", "x86_64") => Some("x86_64-pc-windows-msvc"),
        ("linux", "x86_64") => Some("x86_64-unknown-linux-gnu"),
        ("linux", "aarch64") => Some("aarch64-unknown-linux-gnu"),
        ("macos", "x86_64") => Some("x86_64-apple-darwin"),
        ("macos", "aarch64") => Some("aarch64-apple-darwin"),
        _ => None,
    }
}

/// The archive for `target` in `release`: `prybar-<tag>-<target>.<ext>`.
pub fn asset_for<'a>(release: &'a Release, target: &str) -> Option<&'a Asset> {
    let stem = format!("prybar-{}-{target}", release.tag);
    ["zip", "tar.gz"]
        .iter()
        .find_map(|ext| release.asset(&format!("{stem}.{ext}")))
}
