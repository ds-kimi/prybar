//! Steam only reports current totals, so each listing appends one row
//! per item per day to a local file; the chart is drawn from it.

use std::path::Path;

use crate::ListedItem;

/// Rows older than this many are dropped so the file stays small.
const MAX_ROWS: usize = 200_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    /// Days since 1970-01-01 (UTC).
    pub day: u32,
    pub subscriptions: u64,
    pub favorites: u64,
}

/// Adds today's totals for `items`, replacing a row of the same day.
pub fn record(file: &Path, items: &[ListedItem], day: u32) -> std::io::Result<()> {
    let text = std::fs::read_to_string(file).unwrap_or_default();
    let ids: std::collections::HashSet<u64> = items.iter().map(|i| i.item_id).collect();
    let mut rows: Vec<String> = text
        .lines()
        .filter(|l| parse(l).is_none_or(|(id, p)| !(p.day == day && ids.contains(&id))))
        .map(str::to_owned)
        .collect();
    for i in items {
        rows.push(format!(
            "{}\t{day}\t{}\t{}",
            i.item_id, i.subscriptions, i.favorites
        ));
    }
    let skip = rows.len().saturating_sub(MAX_ROWS);
    let out: String = rows[skip..].iter().map(|r| format!("{r}\n")).collect();
    if let Some(dir) = file.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let part = file.with_extension("part");
    std::fs::write(&part, out)?;
    std::fs::rename(part, file)
}

/// One item's points, oldest first; bad rows are skipped.
pub fn series(file: &Path, item_id: u64) -> Vec<Point> {
    let text = std::fs::read_to_string(file).unwrap_or_default();
    let mut pts: Vec<Point> = text
        .lines()
        .filter_map(parse)
        .filter(|(id, _)| *id == item_id)
        .map(|(_, p)| p)
        .collect();
    pts.sort_by_key(|p| p.day);
    pts.dedup_by_key(|p| p.day);
    pts
}

fn parse(line: &str) -> Option<(u64, Point)> {
    let mut f = line.split('\t').map(|s| s.trim().parse::<u64>().ok());
    let id = f.next()??;
    let day = u32::try_from(f.next()??).ok()?;
    let point = Point {
        day,
        subscriptions: f.next()??,
        favorites: f.next()??,
    };
    Some((id, point))
}
