use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct ListArgs {
    /// .vpk (the `_dir.vpk` of a split set), .gma, .apk or .hfs.
    pub package: PathBuf,
    /// Only entries matching this wildcard (`materials/*.vmt`).
    #[arg(long)]
    pub filter: Option<String>,
    /// Print at most this many entries; the total is still reported.
    #[arg(long)]
    pub limit: Option<usize>,
}

#[derive(Debug, Args)]
pub struct UnpackArgs {
    pub package: PathBuf,
    /// Destination folder.
    #[arg(long, short)]
    pub out: PathBuf,
}

#[derive(Debug, Args)]
pub struct PackArgs {
    /// Folder whose content becomes the package root.
    pub src: PathBuf,
    /// Output .vpk or .gma; the extension picks the format. A GMA needs
    /// an addon.json in `src`.
    #[arg(long, short)]
    pub out: PathBuf,
    /// VPK version, 1 or 2.
    #[arg(long, default_value_t = 2)]
    pub vpk_version: u32,
    /// Split VPK data into `_NNN.vpk` archives of about this many bytes
    /// (`out` must end in `_dir.vpk`).
    #[arg(long)]
    pub chunk_size: Option<u64>,
}
