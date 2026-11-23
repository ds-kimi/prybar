use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use super::{parse, Change};

/// Changelog pages are a few tens of KB; this leaves room for long ones.
const MAX_PAGE: u64 = 2 << 20;
/// A day old is fresh enough for history that changes on each update.
const FRESH: Duration = Duration::from_secs(24 * 3600);

/// The item's history, from the cache in `dir` when it is less than a
/// day old, else from the page (then cached). A failed fetch falls back
/// to an older cache before giving up.
pub fn cached(item_id: u64, dir: &Path) -> Result<Vec<Change>, String> {
    let file = dir.join(format!("{item_id}.html"));
    if fresh(&file) {
        return read(&file);
    }
    let url = format!("https://steamcommunity.com/sharedfiles/filedetails/changelog/{item_id}");
    match crate::thumb::download(&url, MAX_PAGE) {
        Ok(bytes) => {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
            let part: PathBuf = file.with_extension("part");
            std::fs::write(&part, &bytes).map_err(|e| e.to_string())?;
            std::fs::rename(&part, &file).map_err(|e| e.to_string())?;
            Ok(parse(&String::from_utf8_lossy(&bytes)))
        }
        Err(e) if file.is_file() => read(&file).map_err(|_| e),
        Err(e) => Err(e),
    }
}

fn fresh(file: &Path) -> bool {
    let age = std::fs::metadata(file)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| SystemTime::now().duration_since(t).ok());
    age.is_some_and(|a| a < FRESH)
}

fn read(file: &Path) -> Result<Vec<Change>, String> {
    let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
    Ok(parse(&String::from_utf8_lossy(&bytes)))
}
