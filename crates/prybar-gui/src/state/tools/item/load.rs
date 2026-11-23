use std::path::{Path, PathBuf};

use gpui::Context;
use prybar_workshop::ListedItem;

use crate::state::AppState;

pub fn stats_file(dir: &Path) -> PathBuf {
    dir.join("workshop-stats.tsv")
}

/// Days since 1970 in UTC, the unit snapshots are kept in.
pub fn today() -> u32 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    u32::try_from(secs / 86_400).unwrap_or(u32::MAX)
}

impl AppState {
    /// Saves today's totals off the UI thread; a failure only costs one
    /// point on the chart, so it stays silent.
    pub fn record_stats(&mut self, items: &[ListedItem], cx: &mut Context<Self>) {
        if items.is_empty() {
            return;
        }
        let file = stats_file(&(self.tools.backend.data_dir)());
        let items = items.to_vec();
        let work = cx.background_executor().spawn(async move {
            let _ = prybar_workshop::item::record(&file, &items, today());
        });
        work.detach();
    }
}
