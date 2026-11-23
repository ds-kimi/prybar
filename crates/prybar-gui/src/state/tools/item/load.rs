use std::path::{Path, PathBuf};

use gpui::Context;
use prybar_workshop::ListedItem;

use super::History;
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

    /// Once per run per item; the backend's disk cache spans runs.
    pub(super) fn fetch_history(&mut self, id: u64, cx: &mut Context<Self>) {
        if self.tools.item.history.contains_key(&id) {
            return;
        }
        self.tools.item.history.insert(id, History::Loading);
        let fetch = self.tools.backend.history;
        let work = cx.background_executor().spawn(async move { fetch(id) });
        cx.spawn(async move |this, cx| {
            let got = match work.await {
                Ok(all) => History::Ready(all),
                Err(why) => History::Failed(why),
            };
            let _ = this.update(cx, |s, cx| {
                s.tools.item.history.insert(id, got);
                cx.notify();
            });
        })
        .detach();
    }
}
