use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListArgs {
    /// Absolute path of a `.vpk` (the `_dir.vpk` for multi-archive VPKs), `.gma`, `.apk` or `.hfs` package.
    pub package: String,
    /// Optional wildcard on the entry path, e.g. `models/*.mdl`. Paths use `/` and are lower-case for VPK.
    pub filter: Option<String>,
    /// Maximum entries returned (default 200, at most 5000). `total` always counts every match.
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnpackArgs {
    /// Absolute path of the package to extract.
    pub package: String,
    /// Absolute output folder; created if missing.
    pub out: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PackArgs {
    /// Absolute folder whose files go into the package.
    pub src: String,
    /// Absolute output file. Its extension picks the format: `.vpk` or `.gma`.
    pub out: String,
    /// VPK version, 1 or 2 (default 2). Ignored for GMA.
    pub vpk_version: Option<u32>,
    /// VPK only: split file data into `_NNN.vpk` archives of about this many bytes.
    pub chunk_size: Option<u64>,
}
