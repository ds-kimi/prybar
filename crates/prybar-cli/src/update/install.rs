use crate::output::Output;
use prybar_service::{Result, ServiceError};
use prybar_update::{
    apply, asset_for, check, download_verified, install_plan, target_triple, Applied, Config, Kind,
    Plan, Release, Status, UpdateError,
};
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
struct UpdateReport {
    current: String,
    latest: String,
    /// `up_to_date`, `installed` or `manual`.
    action: &'static str,
    /// The command or link to run for `manual`.
    detail: Option<String>,
}

/// `prybar self-update`: check, then install only a verified download, or
/// print what to run when the binary belongs to a package manager.
pub fn self_update(out: &Output) -> Result<u8> {
    let current = env!("CARGO_PKG_VERSION");
    let cfg = Config::from_env(current);
    let release = match check(&cfg) {
        Status::Available(r) => r,
        Status::UpToDate => return report(out, current, current, "up_to_date", None),
        Status::Unknown(why) => return Err(ServiceError::Tool(format!("update check: {why}"))),
    };
    let latest = release.version.to_string();
    let exe = std::env::current_exe().map_err(|e| ServiceError::io(Path::new("prybar"), e))?;
    match install(&cfg, &release, &exe, out).map_err(to_service)? {
        Applied::Installed(_) => report(out, current, &latest, "installed", None),
        Applied::Manual(hint) => report(out, current, &latest, "manual", Some(hint)),
    }
}

/// Replaces `prybar`, and `prybar-mcp` too when it sits next to it, from
/// the same verified archive so both stay on one version.
fn install(
    cfg: &Config,
    r: &Release,
    exe: &Path,
    out: &Output,
) -> std::result::Result<Applied, UpdateError> {
    let plan = install_plan(exe, &r.page);
    if let Plan::Manual(hint) = plan {
        return Ok(Applied::Manual(hint));
    }
    let target =
        target_triple().ok_or_else(|| UpdateError::MissingAsset("this platform".into()))?;
    let asset = asset_for(r, target).ok_or_else(|| UpdateError::MissingAsset(target.into()))?;
    out.progress(&format!("downloading {}", asset.name));
    let verified = match download_verified(cfg, r, asset, &mut |_, _| {}) {
        Err(UpdateError::Unsigned(page)) => {
            return Ok(Applied::Manual(format!("download it from {page}")))
        }
        other => other?,
    };
    let done = apply(&verified, Kind::Cli, &plan)?;
    let mcp = exe.with_file_name(Kind::Mcp.file_name());
    if mcp.is_file() {
        apply(&verified, Kind::Mcp, &install_plan(&mcp, &r.page))?;
    }
    Ok(done)
}

fn report(
    out: &Output,
    current: &str,
    latest: &str,
    action: &'static str,
    detail: Option<String>,
) -> Result<u8> {
    let r = UpdateReport {
        current: current.into(),
        latest: latest.into(),
        action,
        detail,
    };
    out.emit(&r, |r| match (r.action, &r.detail) {
        ("up_to_date", _) => format!("prybar {} is up to date\n", r.current),
        ("installed", _) => format!("prybar updated to {}\n", r.latest),
        (_, d) => format!(
            "prybar {} is available: {}\n",
            r.latest,
            d.as_deref().unwrap_or("")
        ),
    });
    Ok(0)
}

fn to_service(e: UpdateError) -> ServiceError {
    match e {
        UpdateError::Io(io) => ServiceError::io(Path::new("update"), io),
        UpdateError::BadSignature
        | UpdateError::BadChecksum(_)
        | UpdateError::BadArchive(_)
        | UpdateError::BadRelease(_) => ServiceError::invalid("update", e),
        other => ServiceError::Tool(other.to_string()),
    }
}
