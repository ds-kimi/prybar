use super::{classify, Kind, Package};
use crate::Result;
use std::path::Path;

impl Package {
    /// Entry names are matched case-insensitively, as the engine does.
    /// `Ok(None)` means no such entry.
    pub fn read(&self, path: &str) -> Result<Option<Vec<u8>>> {
        let Some(&i) = self.index.get(&path.to_ascii_lowercase()) else {
            return Ok(None);
        };
        let at = self.path.as_path();
        let data = match &self.kind {
            Kind::Vpk(p) => p.read(&p.entries[i]).map_err(|e| classify::vpk(at, e))?,
            Kind::Gma(p) => p
                .read(&p.entries[i])
                .map_err(|e| classify::gma(at, e))?
                .to_vec(),
            Kind::Apk(p) => p
                .read(&p.entries[i])
                .map_err(|e| classify::apk(at, e))?
                .to_vec(),
            Kind::Hfs(p) => p
                .read(&p.entries[i])
                .map_err(|e| classify::hfs(at, e))?
                .to_vec(),
        };
        Ok(Some(data))
    }

    /// Writes every entry under `out`; returns the file count.
    pub fn extract_all(&self, out: &Path) -> Result<usize> {
        match &self.kind {
            Kind::Vpk(p) => prybar_vpk::extract_all(p, out).map_err(|e| classify::vpk(out, e)),
            Kind::Gma(p) => prybar_gma::extract_all(p, out).map_err(|e| classify::gma(out, e)),
            Kind::Apk(p) => prybar_apk::extract_all(p, out).map_err(|e| classify::apk(out, e)),
            Kind::Hfs(p) => prybar_hfs::extract_all(p, out).map_err(|e| classify::hfs(out, e)),
        }
    }
}
