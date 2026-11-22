//! The real update calls, as plain function pointers so tests swap in a
//! fake release source without network or a real install.

use std::path::Path;
use std::process::Command;

use prybar_update::{
    apply, asset_for, check, download_verified, install_plan, target_triple, Applied, Config, Kind,
    Plan, Release, Status, UpdateError, Verified,
};

use super::VERSION;

pub type CheckFn = fn() -> Status;
/// Reports `(done, total)` bytes while downloading.
pub type DownloadFn = fn(&Release, &mut dyn FnMut(u64, Option<u64>)) -> Result<Verified, String>;
/// Installs the verified archive and starts the new copy; the caller quits.
pub type RestartFn = fn(&Verified) -> Result<(), String>;

#[derive(Clone, Copy)]
pub struct Backend {
    pub check: CheckFn,
    pub download: DownloadFn,
    pub restart: RestartFn,
}

impl Default for Backend {
    fn default() -> Self {
        Backend {
            check: || check(&Config::from_env(VERSION)),
            download,
            restart,
        }
    }
}

/// Refuses early when this copy cannot be replaced (package manager,
/// dev build, unsigned release), so no download is wasted.
fn download(r: &Release, progress: &mut dyn FnMut(u64, Option<u64>)) -> Result<Verified, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    if let Plan::Manual(hint) = install_plan(&exe, &r.page) {
        return Err(format!("This copy updates by hand: {hint}"));
    }
    let target = target_triple().ok_or("No update build for this platform")?;
    let asset = asset_for(r, target).ok_or_else(|| format!("No update build for {target}"))?;
    let cfg = Config::from_env(VERSION);
    download_verified(&cfg, r, asset, progress).map_err(|e| match e {
        UpdateError::Unsigned(_) => "This release is not signed, so it is not installed".into(),
        other => other.to_string(),
    })
}

/// A bundle reopens through `open -n` so macOS starts the new signature;
/// a plain binary is started directly.
fn restart(v: &Verified) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let plan = install_plan(&exe, &v.release.page);
    match apply(v, Kind::Gui, &plan).map_err(|e| e.to_string())? {
        Applied::Manual(hint) => return Err(format!("This copy updates by hand: {hint}")),
        Applied::Installed(_) => {}
    }
    let started = match &plan {
        Plan::Bundle(app) => Command::new("open").arg("-n").arg(app).spawn(),
        _ => Command::new(Path::new(&exe)).spawn(),
    };
    started
        .map(drop)
        .map_err(|e| format!("Updated, but could not restart: {e}"))
}
