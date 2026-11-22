//! Where textures are searched, in engine order.

use crate::package::Package;
use crate::source::ModelRef;
use crate::Result;
use std::path::PathBuf;

enum Root {
    Folder(PathBuf),
    Package(Box<Package>),
}

/// Ordered search roots; each is a game-style tree holding `materials/`.
pub struct Sources {
    roots: Vec<Root>,
}

/// A file read from one root, with a label saying which.
pub struct Found {
    pub bytes: Vec<u8>,
    pub location: String,
}

impl Sources {
    /// The model's own tree (disk folder above `models/`, or its package),
    /// then every configured game folder and its VPKs. Packages that fail to
    /// open are skipped: one broken VPK should not hide the others.
    pub fn for_model(arg: &str, game_dirs: &[PathBuf]) -> Sources {
        let mut roots = Vec::new();
        match ModelRef::parse(arg) {
            ModelRef::Path(p) => roots.extend(super::roots::tree_root(&p).map(Root::Folder)),
            ModelRef::Packed { package, .. } => roots.extend(
                Package::open(&package)
                    .ok()
                    .map(|p| Root::Package(Box::new(p))),
            ),
        }
        for dir in game_dirs {
            roots.push(Root::Folder(dir.clone()));
            roots.extend(
                super::roots::game_vpks(dir)
                    .filter_map(|p| Package::open(&p).ok())
                    .map(|p| Root::Package(Box::new(p))),
            );
        }
        Sources { roots }
    }

    /// First root holding `path` (already normalized, `/`-separated).
    pub fn read(&self, path: &str) -> Result<Option<Found>> {
        for root in &self.roots {
            let (bytes, location) = match root {
                Root::Folder(dir) => match std::fs::read(dir.join(path)) {
                    Ok(b) => (b, dir.display().to_string()),
                    Err(_) => continue,
                },
                Root::Package(p) => match p.read(path)? {
                    Some(b) => (b, p.path().display().to_string()),
                    None => continue,
                },
            };
            return Ok(Some(Found { bytes, location }));
        }
        Ok(None)
    }

    /// Location of `path` without reading it, for the materials list.
    pub fn locate(&self, path: &str) -> Option<String> {
        self.roots.iter().find_map(|root| match root {
            Root::Folder(dir) => dir.join(path).is_file().then(|| dir.display().to_string()),
            Root::Package(p) => p.contains(path).then(|| p.path().display().to_string()),
        })
    }

    /// Labels of every root, for "looked in" messages.
    pub fn describe(&self) -> Vec<String> {
        let label = |r: &Root| match r {
            Root::Folder(d) => d.display().to_string(),
            Root::Package(p) => p.path().display().to_string(),
        };
        self.roots.iter().map(label).collect()
    }
}
