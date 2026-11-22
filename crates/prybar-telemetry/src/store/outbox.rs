use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{Entry, Store, MAX_QUEUED};
use crate::Event;

/// Keeps two events queued in the same nanosecond apart.
static SEQ: AtomicU64 = AtomicU64::new(0);

impl Store {
    /// Writes the event for a later send and logs it as "queued".
    pub fn queue(&self, event: &Event) -> io::Result<()> {
        let dir = self.outbox();
        std::fs::create_dir_all(&dir)?;
        if std::fs::read_dir(&dir)?.count() >= MAX_QUEUED {
            return Ok(());
        }
        let name = format!(
            "{}-{}.json",
            super::now_nanos(),
            SEQ.fetch_add(1, Ordering::Relaxed)
        );
        let body = serde_json::to_vec(event).map_err(io::Error::other)?;
        std::fs::write(dir.join(name), body)?;
        self.append(&Entry::new("", "queued", event.clone()))
    }

    /// Queued events oldest first; unreadable or foreign files are removed
    /// so they can never be sent.
    pub fn pending(&self) -> Vec<(PathBuf, Event)> {
        let Ok(read) = std::fs::read_dir(self.outbox()) else {
            return Vec::new();
        };
        let mut paths: Vec<PathBuf> = read.flatten().map(|e| e.path()).collect();
        paths.sort();
        paths
            .into_iter()
            .filter_map(|p| {
                let ev = std::fs::read(&p)
                    .ok()
                    .and_then(|b| serde_json::from_slice::<Event>(&b).ok())
                    .filter(Event::valid);
                if ev.is_none() {
                    let _ = std::fs::remove_file(&p);
                }
                Some((p, ev?))
            })
            .collect()
    }

    /// Turning analytics off deletes what was waiting, crash reports included.
    pub fn drop_queue(&self) -> io::Result<()> {
        match std::fs::remove_dir_all(self.outbox()) {
            Err(e) if e.kind() != io::ErrorKind::NotFound => Err(e),
            _ => Ok(()),
        }
    }
}
