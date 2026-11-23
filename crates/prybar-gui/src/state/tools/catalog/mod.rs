//! The "my items" list: the query the toolbar sets, list or grid, and
//! the preview images fetched for the items that scrolled into view.

mod prefs;

pub use prefs::{lines as prefs_lines, ItemPrefs};

use std::collections::HashMap;
use std::path::PathBuf;

use gpui::Context;
use prybar_workshop::catalog::Query;

use crate::state::AppState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Thumb {
    Loading,
    Ready(PathBuf),
    /// No preview, or it failed: the card keeps its placeholder.
    Missing,
}

pub struct Catalog {
    pub query: Query,
    pub grid: bool,
    pub thumbs: HashMap<u64, Thumb>,
}

impl Default for Catalog {
    fn default() -> Self {
        let mut c = Catalog {
            query: Query::default(),
            grid: true,
            thumbs: HashMap::new(),
        };
        c.set_prefs(ItemPrefs::STEAM);
        c
    }
}

impl AppState {
    /// Fetches the previews of items now on screen, each once per run;
    /// the disk cache makes later runs skip the network.
    pub fn want_thumbs(&mut self, wanted: Vec<(u64, Option<String>)>, cx: &mut Context<Self>) {
        let fetch = self.tools.backend.thumb;
        for (id, url) in wanted {
            let thumbs = &mut self.tools.steam.catalog.thumbs;
            if thumbs.contains_key(&id) {
                continue;
            }
            let Some(url) = url else {
                thumbs.insert(id, Thumb::Missing);
                continue;
            };
            thumbs.insert(id, Thumb::Loading);
            let work = cx.background_executor().spawn(async move { fetch(&url) });
            cx.spawn(async move |this, cx| {
                let got = work.await.map_or(Thumb::Missing, Thumb::Ready);
                let _ = this.update(cx, |s, cx| {
                    s.tools.steam.catalog.thumbs.insert(id, got);
                    cx.notify();
                });
            })
            .detach();
        }
        cx.notify();
    }

    /// Changes the list's look; sort, order and layout are saved.
    pub fn set_items(&mut self, change: impl FnOnce(&mut Catalog), cx: &mut Context<Self>) {
        let before = self.tools.steam.catalog.prefs();
        change(&mut self.tools.steam.catalog);
        if self.tools.steam.catalog.prefs() != before {
            self.save_prefs(cx);
        }
        cx.notify();
    }
}

/// Previews live next to the config, under `previews/`.
pub fn default_thumb(url: &str) -> Result<PathBuf, String> {
    let dir = prybar_tools::config::config_path()
        .ok()
        .and_then(|p| Some(p.parent()?.join("previews")))
        .unwrap_or_else(|| std::env::temp_dir().join("prybar-previews"));
    prybar_workshop::thumb::cached(url, &dir)
}
