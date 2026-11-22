use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Vpk,
    Gma,
    Apk,
    Hfs,
}

impl Format {
    /// Packages carry no shared magic, so the extension decides.
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_ascii_lowercase();
        Self::from_ext(&ext)
    }

    pub fn from_ext(ext: &str) -> Option<Self> {
        match ext {
            "vpk" => Some(Self::Vpk),
            "gma" => Some(Self::Gma),
            "apk" => Some(Self::Apk),
            "hfs" => Some(Self::Hfs),
            _ => None,
        }
    }
}
