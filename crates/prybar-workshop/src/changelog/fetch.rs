use std::path::Path;
use std::time::{Duration, SystemTime};

use super::{parse, Change};
use crate::http::{Failure, Gate};

/// Changelog pages are a few tens of KB; this leaves room for long ones.
const MAX_PAGE: u64 = 2 << 20;
/// A day old is fresh enough for history that changes on each update.
const FRESH: Duration = Duration::from_secs(24 * 3600);
/// Steam rate-limits this page hard: one request at a time, app-wide.
static GATE: Gate = Gate::new(1);

/// The history and how old the copy is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Log {
    pub changes: Vec<Change>,
    /// Seconds since this copy was read from Steam.
    pub age: u64,
    /// Steam did not answer, so an older copy is shown.
    pub stale: bool,
}

/// The item's history, from the cache in `dir` when it is less than a
/// day old, else from its Workshop changelog page.
pub fn cached(item_id: u64, dir: &Path) -> Result<Log, Failure> {
    let url = format!("https://steamcommunity.com/sharedfiles/filedetails/changelog/{item_id}");
    cached_at(&url, &dir.join(format!("{item_id}.html")))
}

/// [`cached`] for any page and cache file. A failed fetch falls back to
/// an older cache, marked stale, before giving up.
pub fn cached_at(url: &str, file: &Path) -> Result<Log, Failure> {
    let local = |message: String| Failure {
        status: None,
        retry_after: None,
        message,
    };
    if age(file).is_some_and(|a| a < FRESH) {
        return read(file, false).map_err(local);
    }
    match crate::http::get(url, MAX_PAGE, &GATE) {
        Ok(bytes) => {
            // Saving is a cache: a full disk must not hide the history.
            if let Some(dir) = file.parent() {
                let part = file.with_extension("part");
                let saved =
                    std::fs::create_dir_all(dir).and_then(|()| std::fs::write(&part, &bytes));
                let _ = saved.and_then(|()| std::fs::rename(&part, file));
            }
            let changes = parse(&String::from_utf8_lossy(&bytes));
            Ok(Log {
                changes,
                age: 0,
                stale: false,
            })
        }
        Err(e) => read(file, true).map_err(|_| e),
    }
}

fn age(file: &Path) -> Option<Duration> {
    let when = std::fs::metadata(file).and_then(|m| m.modified()).ok()?;
    SystemTime::now().duration_since(when).ok()
}

fn read(file: &Path, stale: bool) -> Result<Log, String> {
    let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
    let age = age(file).map_or(0, |a| a.as_secs());
    let changes = parse(&String::from_utf8_lossy(&bytes));
    Ok(Log {
        changes,
        age,
        stale,
    })
}
