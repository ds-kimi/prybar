use std::path::Path;

use tempfile::TempDir;

use crate::WorkshopError;

/// Garry's Mod items are one `.gma` file; every other game uploads a folder.
pub const GMOD_APP_ID: u32 = 4000;

pub(super) fn check_content(app_id: u32, path: &Path) -> Result<(), WorkshopError> {
    let is_gma = path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("gma"));
    let ok = if app_id == GMOD_APP_ID {
        is_gma && path.is_file()
    } else {
        path.is_dir()
    };
    if !ok {
        let want = if app_id == GMOD_APP_ID {
            "an existing .gma file"
        } else {
            "an existing folder"
        };
        return Err(WorkshopError::invalid("content", format!("must be {want}")));
    }
    Ok(())
}

/// Steam uploads a folder, so a GMA is copied alone into a temp folder
/// (the guard deletes it once the upload finished). Folders pass as is.
pub fn stage_content(app_id: u32, path: &Path) -> Result<Option<TempDir>, WorkshopError> {
    if app_id != GMOD_APP_ID {
        return Ok(None);
    }
    let io = |p: &Path, source| WorkshopError::Io {
        path: p.to_path_buf(),
        source,
    };
    let dir = tempfile::tempdir().map_err(|e| io(path, e))?;
    let name = path.file_name().unwrap_or("addon.gma".as_ref());
    std::fs::copy(path, dir.path().join(name)).map_err(|e| io(path, e))?;
    Ok(Some(dir))
}
