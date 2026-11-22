use crate::package::Package;
use crate::Result;
use std::path::Path;

/// Reads a model's companions (`.vvd`, `.dx90.vtx`, `.phy`, `.ani`,
/// GoldSrc `T.mdl`) from wherever the `.mdl` itself lives.
#[derive(Debug, Clone, Copy)]
pub enum Origin<'a> {
    Disk,
    Package(&'a Package),
}

impl Origin<'_> {
    /// `stem` is the model path without `.mdl`; a missing companion is
    /// `Ok(None)`, a broken one is an error.
    pub fn get(&self, stem: &str, suffix: &str) -> Result<Option<Vec<u8>>> {
        let name = format!("{stem}{suffix}");
        match self {
            Self::Disk => disk(&name).or_else(|| upper(stem, suffix)).transpose(),
            Self::Package(p) => p.read(&name),
        }
    }
}

/// Linux file systems are case-sensitive and old models ship as
/// `FOO.MDL` / `FOO.VVD`; elsewhere the first open already matched.
fn upper(stem: &str, suffix: &str) -> Option<Result<Vec<u8>>> {
    let upper = suffix.to_uppercase();
    let case_sensitive = !cfg!(any(windows, target_os = "macos"));
    (case_sensitive && upper != suffix)
        .then(|| disk(&format!("{stem}{upper}")))
        .flatten()
}

/// `None` when the file does not exist.
fn disk(name: &str) -> Option<Result<Vec<u8>>> {
    match std::fs::read(Path::new(name)) {
        Ok(d) => Some(Ok(d)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => Some(Err(crate::ServiceError::io(Path::new(name), e))),
    }
}
