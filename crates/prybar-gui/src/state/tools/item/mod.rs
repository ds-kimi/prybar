//! The selected item's details: its stats over time from the local
//! snapshots, its version history from the Workshop page, and the addon
//! folder linked to it.

mod content;
mod load;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use gpui::Context;
use prybar_workshop::changelog::Change;
use prybar_workshop::item::{Links, Point};

use crate::state::AppState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum History {
    Loading,
    Ready(Vec<Change>),
    Failed(String),
}

#[derive(Default)]
pub struct ItemView {
    pub selected: Option<u64>,
    /// Subscribers and favorites per day for `selected`, oldest first.
    pub series: Vec<Point>,
    pub history: HashMap<u64, History>,
    pub links: Links,
    links_read: bool,
}

/// Stats, links and the changelog cache all live in this folder.
pub fn links_file(dir: &Path) -> PathBuf {
    dir.join("workshop-links.txt")
}

impl AppState {
    /// Shows the item's details; a second click on it hides them.
    pub fn select_item(&mut self, id: u64, cx: &mut Context<Self>) {
        let v = &mut self.tools.item;
        if v.selected == Some(id) {
            v.selected = None;
            return cx.notify();
        }
        v.selected = Some(id);
        let dir = (self.tools.backend.data_dir)();
        v.series = prybar_workshop::item::series(&load::stats_file(&dir), id);
        self.fetch_history(id, cx);
        cx.notify();
    }

    /// Remembers `dir` as the item's source, so Update content is one click.
    pub fn link_folder(&mut self, id: u64, dir: PathBuf, cx: &mut Context<Self>) {
        let data = (self.tools.backend.data_dir)();
        let links = self.links();
        if links.0.get(&id) == Some(&dir) {
            return;
        }
        links.0.insert(id, dir);
        if let Err(e) = links.save(&links_file(&data)) {
            let why = format!("Could not save the folder link: {e}");
            self.toast(crate::state::ToastKind::Error, why, cx);
        }
        cx.notify();
    }

    /// Read on first use rather than at start, when no screen needs them.
    pub fn links(&mut self) -> &mut Links {
        let v = &mut self.tools.item;
        if !std::mem::replace(&mut v.links_read, true) {
            v.links = Links::load(&links_file(&(self.tools.backend.data_dir)()));
        }
        &mut self.tools.item.links
    }
}
