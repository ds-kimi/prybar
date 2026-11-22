mod extract;
mod find;
mod plan;

pub use plan::{install_plan, Plan};

use crate::{Kind, UpdateError, Verified};
use std::path::Path;

/// What an install did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Applied {
    /// Files replaced; the new version runs from the next start.
    Installed(String),
    /// Nothing replaced; show this command or link to the user.
    Manual(String),
}

/// Installs a verified archive following `plan`. Only a `Verified` value is
/// accepted, so an unchecked download can never be installed.
pub fn apply(verified: &Verified, kind: Kind, plan: &Plan) -> Result<Applied, UpdateError> {
    let target = match plan {
        Plan::Manual(hint) => return Ok(Applied::Manual(hint.clone())),
        Plan::Binary(p) | Plan::Bundle(p) => p,
    };
    // Unpack next to the target so the final step is a same-volume rename.
    let parent = target
        .parent()
        .ok_or_else(|| UpdateError::NotInstallable("no parent dir".into()))?;
    let stage = tempfile::Builder::new()
        .prefix(".prybar-update")
        .tempdir_in(parent)?;
    extract::extract(&verified.archive, stage.path())?;
    let new = find::find(stage.path(), kind.file_name())?;
    match plan {
        Plan::Bundle(app) => find::swap_dir(&new, app)?,
        _ => replace_binary(&new, target)?,
    }
    Ok(Applied::Installed(verified.release.version.to_string()))
}

/// The running executable goes through `self_replace` (Windows cannot
/// overwrite a running file); any other path is a plain rename.
fn replace_binary(new: &Path, target: &Path) -> Result<(), UpdateError> {
    let current = std::env::current_exe()
        .ok()
        .and_then(|p| p.canonicalize().ok());
    let is_self = current.is_some() && current == target.canonicalize().ok();
    if is_self {
        self_replace::self_replace(new)?;
    } else {
        std::fs::rename(new, target)?;
    }
    Ok(())
}
