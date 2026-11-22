use std::io::{self, Write as _};

use serde::{Deserialize, Serialize};

use super::Store;
use crate::Event;

/// One line of `sent.jsonl`: the exact payload and what happened to it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    /// Unix seconds.
    pub time: u64,
    /// Empty while only queued.
    pub endpoint: String,
    /// `queued`, `sent` or `failed: <reason>`.
    pub status: String,
    pub payload: Event,
}

impl Entry {
    pub fn new(endpoint: &str, status: &str, payload: Event) -> Self {
        Self {
            time: (super::now_nanos() / 1_000_000_000) as u64,
            endpoint: endpoint.into(),
            status: status.into(),
            payload,
        }
    }
}

impl Store {
    pub fn append(&self, entry: &Entry) -> io::Result<()> {
        std::fs::create_dir_all(&self.dir)?;
        let mut line = serde_json::to_vec(entry).map_err(io::Error::other)?;
        line.push(b'\n');
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.log_path())?;
        f.write_all(&line)
    }

    /// Every readable line, oldest first; a damaged line is skipped, not fatal.
    pub fn entries(&self) -> Vec<Entry> {
        std::fs::read_to_string(self.log_path())
            .unwrap_or_default()
            .lines()
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect()
    }

    pub fn clear_log(&self) -> io::Result<()> {
        match std::fs::remove_file(self.log_path()) {
            Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e),
            _ => Ok(()),
        }
    }
}
