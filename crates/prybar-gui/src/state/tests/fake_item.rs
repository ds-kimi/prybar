//! The one listed item the fake Steam returns, and what the app keeps
//! about it.

use std::path::PathBuf;

use prybar_workshop::changelog::{parse, Change};
use prybar_workshop::spec::Visibility;
use prybar_workshop::ListedItem;

pub(crate) fn item() -> ListedItem {
    ListedItem {
        item_id: 7,
        title: "Crate pack".into(),
        tags: vec!["Addon".into()],
        time_updated: 1_700_000_000,
        time_created: 1_600_000_000,
        visibility: Visibility::Public,
        subscriptions: 12,
        favorites: 3,
        votes_up: 9,
        votes_down: 1,
        preview_url: None,
        ..Default::default()
    }
}

/// One change note for the listed item; any other id is "offline".
pub(crate) fn history(id: u64) -> Result<Vec<Change>, String> {
    match id {
        7 => Ok(parse(
            "changelog headline\">Update: 1 Jan</div><p id=\"1700000000\">First</p>",
        )),
        _ => Err("offline".into()),
    }
}

/// Per test thread, so parallel tests never share a file.
pub(crate) fn data_dir() -> PathBuf {
    let id = format!("{:?}", std::thread::current().id());
    std::env::temp_dir().join(format!("prybar-gui-test-{}", id.replace(['(', ')'], "")))
}
