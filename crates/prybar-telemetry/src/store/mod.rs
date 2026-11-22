//! On-disk state: `outbox/` holds events not sent yet, `sent.jsonl` logs
//! every event queued or sent, with time and endpoint, for the user.

mod log;
mod outbox;
#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub use log::Entry;

/// A crash loop must not fill the disk; past this, new events are dropped.
pub const MAX_QUEUED: usize = 20;

#[derive(Clone, Debug)]
pub struct Store {
    pub dir: PathBuf,
}

impl Store {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub fn log_path(&self) -> PathBuf {
        self.dir.join("sent.jsonl")
    }

    fn outbox(&self) -> PathBuf {
        self.dir.join("outbox")
    }
}

/// File names and log times both come from here.
pub(crate) fn now_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos())
}
