//! The list choices kept in `gui.txt`, as `items_*=` lines.

use prybar_workshop::catalog::Sort;

use super::Catalog;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemPrefs {
    pub sort: Sort,
    pub descending: bool,
    pub grid: bool,
}

impl ItemPrefs {
    /// Steam's own order: last updated first, as a grid.
    pub const STEAM: ItemPrefs = ItemPrefs {
        sort: Sort::Updated,
        descending: true,
        grid: true,
    };

    /// Unknown or missing lines keep the default.
    pub fn parse(text: &str) -> ItemPrefs {
        let mut p = ItemPrefs::STEAM;
        for (key, value) in text.lines().filter_map(|l| l.trim().split_once('=')) {
            match key {
                "items_sort" => {
                    p.sort = Sort::ALL
                        .into_iter()
                        .find(|s| s.key() == value)
                        .unwrap_or(p.sort)
                }
                "items_order" => p.descending = value != "asc",
                "items_view" => p.grid = value != "list",
                _ => {}
            }
        }
        p
    }
}

impl Catalog {
    pub fn prefs(&self) -> ItemPrefs {
        ItemPrefs {
            sort: self.query.sort,
            descending: self.query.descending,
            grid: self.grid,
        }
    }

    pub fn set_prefs(&mut self, p: ItemPrefs) {
        (self.query.sort, self.query.descending, self.grid) = (p.sort, p.descending, p.grid);
    }
}

/// The lines [`ItemPrefs::parse`] reads back.
pub fn lines(p: ItemPrefs) -> String {
    let order = if p.descending { "desc" } else { "asc" };
    let view = if p.grid { "grid" } else { "list" };
    format!(
        "items_sort={}\nitems_order={order}\nitems_view={view}\n",
        p.sort.key()
    )
}
