use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct DecompileOptions {
    /// Models land in `<out>/<path relative to the input root>/`.
    pub out: PathBuf,
    pub recursive: bool,
    /// One `.qci` per QC section.
    pub split_qc: bool,
    /// `<out>/<model name>/` for every model instead of the mirrored tree.
    pub flat: bool,
    /// Also decompile the models named by `$includemodel` (animations,
    /// gestures, postures), once each.
    pub with_includes: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelReport {
    pub input: String,
    pub out_dir: PathBuf,
    pub files: usize,
    /// The model whose `$includemodel` pulled this one in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_from: Option<String>,
    /// What could not be recovered; the files written are still usable.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Failure {
    pub input: String,
    pub error: String,
    /// Exit code this error alone would give.
    pub code: u8,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct DecompileReport {
    pub models: Vec<ModelReport>,
    pub failures: Vec<Failure>,
    /// Renamed output folders and `$includemodel`s that were not found.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
}
