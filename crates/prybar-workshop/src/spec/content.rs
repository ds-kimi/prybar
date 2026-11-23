use std::path::Path;

use tempfile::TempDir;

use crate::addon::check_addon;
use crate::WorkshopError;

/// Garry's Mod items are one `.gma` file; every other game uploads a folder.
pub const GMOD_APP_ID: u32 = 4000;

/// Garry's Mod takes a `.gma` or an addon folder (packed later); other
/// games take a folder as is.
pub(super) fn check_content(app_id: u32, path: &Path) -> Result<(), WorkshopError> {
    let is_gma = path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("gma"));
    if app_id == GMOD_APP_ID && path.is_dir() {
        return check_folder(path);
    }
    if app_id == GMOD_APP_ID && !(is_gma && path.is_file()) {
        let want = "an existing .gma file or addon folder";
        return Err(WorkshopError::invalid("content", format!("must be {want}")));
    }
    if app_id != GMOD_APP_ID && !path.is_dir() {
        return Err(WorkshopError::invalid(
            "content",
            "must be an existing folder",
        ));
    }
    crate::games::check_game_content(app_id, path)
}

/// Lists the first problems in the error so a CLI user sees what to fix.
fn check_folder(dir: &Path) -> Result<(), WorkshopError> {
    let check = check_addon(dir)?;
    if check.ok() {
        return Ok(());
    }
    let files = check
        .rejected
        .iter()
        .map(|r| format!("{} ({})", r.path, r.reason));
    let all: Vec<String> = check.problems.iter().cloned().chain(files).collect();
    let more = all.len().saturating_sub(10);
    let mut text = all.into_iter().take(10).collect::<Vec<_>>().join("; ");
    if more > 0 {
        text.push_str(&format!("; and {more} more"));
    }
    Err(WorkshopError::invalid("content", text))
}

/// Steam uploads a folder holding the one GMA, so a `.gma` is copied and
/// an addon folder packed into a temp folder (the guard deletes it once
/// the upload finished). Other games' folders pass as is.
pub fn stage_content(app_id: u32, path: &Path) -> Result<Option<TempDir>, WorkshopError> {
    if app_id != GMOD_APP_ID {
        return Ok(None);
    }
    let io = |p: &Path, source| WorkshopError::Io {
        path: p.to_path_buf(),
        source,
    };
    let dir = tempfile::tempdir().map_err(|e| io(path, e))?;
    if path.is_dir() {
        prybar_gma::pack(path, &dir.path().join("addon.gma"))
            .map_err(|e| WorkshopError::invalid("content", e.to_string()))?;
        return Ok(Some(dir));
    }
    let name = path.file_name().unwrap_or("addon.gma".as_ref());
    std::fs::copy(path, dir.path().join(name)).map_err(|e| io(path, e))?;
    Ok(Some(dir))
}
