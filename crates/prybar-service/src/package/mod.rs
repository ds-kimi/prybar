//! One handle over the four package formats.

pub(crate) mod classify;
mod format;
mod open;
mod read;
mod walk;

pub use format::Format;

use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) enum Kind {
    Vpk(prybar_vpk::Vpk),
    Gma(prybar_gma::Gma),
    Apk(prybar_apk::Apk),
    Hfs(prybar_hfs::Hfs),
}

/// An opened package plus a lower-cased name index: decompiling from a
/// game VPK looks up ~8 companions per model among tens of thousands of
/// entries.
#[derive(Debug)]
pub struct Package {
    kind: Kind,
    index: HashMap<String, usize>,
    /// Read errors name the package, not the entry inside it.
    path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EntryInfo {
    pub path: String,
    pub size: u64,
}

impl Package {
    pub fn open(path: &Path) -> crate::Result<Self> {
        let kind = open::open(path)?;
        let mut index = HashMap::new();
        for (i, e) in entries(&kind).into_iter().enumerate() {
            index.entry(e.path.to_ascii_lowercase()).or_insert(i);
        }
        Ok(Self {
            kind,
            index,
            path: path.to_path_buf(),
        })
    }

    pub fn format(&self) -> Format {
        match self.kind {
            Kind::Vpk(_) => Format::Vpk,
            Kind::Gma(_) => Format::Gma,
            Kind::Apk(_) => Format::Apk,
            Kind::Hfs(_) => Format::Hfs,
        }
    }

    pub fn entries(&self) -> Vec<EntryInfo> {
        entries(&self.kind)
    }
}

fn entries(kind: &Kind) -> Vec<EntryInfo> {
    let info = |path: &str, size: u64| EntryInfo {
        path: path.to_owned(),
        size,
    };
    match kind {
        Kind::Vpk(p) => p.entries.iter().map(|e| info(&e.path, e.size())).collect(),
        Kind::Gma(p) => p.entries.iter().map(|e| info(&e.path, e.size)).collect(),
        Kind::Apk(p) => p
            .entries
            .iter()
            .map(|e| info(&e.path, e.size.into()))
            .collect(),
        Kind::Hfs(p) => p
            .entries
            .iter()
            .map(|e| info(&e.path, e.size.into()))
            .collect(),
    }
}
