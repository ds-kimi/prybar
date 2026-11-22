use crate::package::{EntryInfo, Format, Package};
use crate::source::matches;
use crate::Result;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ListReport {
    pub format: Format,
    /// Entries matching the filter, before `limit`.
    pub total: usize,
    pub total_bytes: u64,
    pub entries: Vec<EntryInfo>,
    /// `entries` was cut at `limit`; `total` still counts everything.
    pub truncated: bool,
}

/// Entries sorted by path, optionally filtered by a wildcard and capped:
/// a game VPK holds tens of thousands of files.
pub fn list(path: &Path, filter: Option<&str>, limit: Option<usize>) -> Result<ListReport> {
    let pkg = Package::open(path)?;
    let mut entries: Vec<EntryInfo> = pkg
        .entries()
        .into_iter()
        .filter(|e| filter.is_none_or(|f| matches(f, &e.path)))
        .collect();
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    let total = entries.len();
    // Sizes come from the package header and may be forged near u64::MAX.
    let total_bytes = entries
        .iter()
        .fold(0u64, |acc, e| acc.saturating_add(e.size));
    let truncated = limit.is_some_and(|n| n < total);
    entries.truncate(limit.unwrap_or(usize::MAX));
    Ok(ListReport {
        format: pkg.format(),
        total,
        total_bytes,
        entries,
        truncated,
    })
}
