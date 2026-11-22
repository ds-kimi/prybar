//! Update notice, `prybar version` and `prybar self-update`.

mod install;
mod notice;

pub use install::self_update;
pub use notice::Notice;

use crate::cli::VersionArgs;
use crate::output::Output;
use prybar_service::Result;
use prybar_update::{check, Config, Status};
use serde::Serialize;

#[derive(Serialize)]
struct VersionReport {
    current: String,
    /// `null` when not asked or when GitHub could not be reached.
    latest: Option<String>,
    update_available: bool,
}

/// `prybar version [--check]`. An unreachable GitHub is not an error: the
/// report just has no `latest`.
pub fn version(a: &VersionArgs, out: &Output) -> Result<u8> {
    let current = env!("CARGO_PKG_VERSION").to_string();
    let status = if a.check {
        check(&Config::from_env(&current))
    } else {
        Status::Unknown("not checked".into())
    };
    let latest = match &status {
        Status::Available(r) => Some(r.version.to_string()),
        Status::UpToDate => Some(current.clone()),
        Status::Unknown(_) => None,
    };
    let report = VersionReport {
        update_available: matches!(status, Status::Available(_)),
        current,
        latest,
    };
    out.emit(&report, |r| {
        let mut s = format!("prybar {}\n", r.current);
        if let Some(line) = notice::message(&status, &r.current) {
            s = s + &line + "\n";
        } else if a.check && r.latest.is_none() {
            s.push_str("could not check for updates\n");
        }
        s
    });
    Ok(0)
}
