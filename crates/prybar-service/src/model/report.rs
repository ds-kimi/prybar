use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct DecompileOptions {
    /// Each model gets its own `<out>/<model name>/` folder.
    pub out: PathBuf,
    pub recursive: bool,
    /// One `.qci` per QC section.
    pub split_qc: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModelReport {
    pub input: String,
    pub out_dir: PathBuf,
    pub files: usize,
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
}
