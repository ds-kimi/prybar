//! Folder batches on the decompile and compile screens: the items found,
//! the state of each, the cancel flag and where the output went.

mod compile;
mod decompile;
mod host;
mod stream;

pub use host::{compile_batch, precheck};

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemState {
    Queued,
    Running,
    Done,
    /// The reason, shown on hover.
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct BatchItem {
    /// The name the service reports progress with: `stem.mdl` for a
    /// decompile, the QC path for a compile.
    pub key: String,
    pub state: ItemState,
}

#[derive(Default)]
pub struct BatchRun {
    pub items: Vec<BatchItem>,
    pub running: bool,
    pub cancel: Arc<AtomicBool>,
    /// Folder "Open output" shows once something was written.
    pub out: Option<PathBuf>,
}

/// What the background work tells the screen while it runs.
pub enum Msg {
    Found(Vec<String>),
    Started(String),
}

/// Options and runs of both screens. Flags are named so that `false`,
/// the default, is the usual choice: recursive, mirrored tree, includes.
#[derive(Default)]
pub struct Batches {
    /// A dropped or picked folder; decompiled instead of the open file.
    pub folder: Option<PathBuf>,
    pub flat: bool,
    pub skip_includes: bool,
    pub shallow: bool,
    pub compile_shallow: bool,
    pub decompile: BatchRun,
    pub compile: BatchRun,
}

impl BatchRun {
    fn started(out: Option<PathBuf>) -> Self {
        BatchRun {
            running: true,
            out,
            ..Default::default()
        }
    }

    /// Adds the keys not listed yet as queued.
    fn list(&mut self, keys: Vec<String>) {
        for key in keys {
            if !self.items.iter().any(|i| i.key == key) {
                let state = ItemState::Queued;
                self.items.push(BatchItem { key, state });
            }
        }
    }

    /// Sets one item's state; an unknown key (an `$includemodel` pulled
    /// in) is appended. A late "running" never undoes a final state.
    fn mark(&mut self, key: &str, state: ItemState) {
        let Some(item) = self.items.iter_mut().find(|i| i.key == key) else {
            let key = key.to_owned();
            return self.items.push(BatchItem { key, state });
        };
        if state != ItemState::Running || item.state == ItemState::Queued {
            item.state = state;
        }
    }

    /// Items that reached a final state, for the progress bar.
    pub fn finished(&self) -> usize {
        let open = |s: &ItemState| matches!(s, ItemState::Queued | ItemState::Running);
        self.items.iter().filter(|i| !open(&i.state)).count()
    }
}
