//! The one listed item the fake Steam returns, and what the app keeps
//! about it.

use std::path::PathBuf;

use prybar_workshop::changelog::{parse, Log};
use prybar_workshop::http::Failure;
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

thread_local! {
    /// Changelog requests made on this test thread.
    pub(crate) static ASKED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// One change note for item 7, Steam busy for item 8 (30 s), any
/// other id is "offline".
pub(crate) fn history(id: u64) -> Result<Log, Failure> {
    ASKED.with(|c| c.set(c.get() + 1));
    let fail = |status, retry_after, message: &str| Failure {
        status,
        retry_after,
        message: message.into(),
    };
    match id {
        7 => Ok(Log {
            changes: parse("changelog headline\">Update: 1 Jan</div><p id=\"1700000000\">First</p>"),
            age: 0,
            stale: false,
        }),
        8 => Err(fail(Some(429), Some(30), "Steam answered 429")),
        _ => Err(fail(None, None, "offline")),
    }
}

/// Per test thread, so parallel tests never share a file.
pub(crate) fn data_dir() -> PathBuf {
    let id = format!("{:?}", std::thread::current().id());
    std::env::temp_dir().join(format!("prybar-gui-test-{}", id.replace(['(', ')'], "")))
}
