use clap::Args;
use std::path::PathBuf;

const MODEL_REF: &str = "A .mdl path, or a model inside a package: `pak_dir.vpk:models/foo.mdl`";

#[derive(Debug, Args)]
pub struct InspectArgs {
    #[arg(help = MODEL_REF)]
    pub model: String,
}

#[derive(Debug, Args)]
pub struct DecompileArgs {
    /// .mdl files, folders, wildcards (`models/*.mdl`) or package refs
    /// (`pak_dir.vpk:models/props`, `addon.gma:models/*.mdl`).
    #[arg(required = true)]
    pub inputs: Vec<String>,
    /// Output folder; each model gets its own subfolder.
    #[arg(long, short)]
    pub out: PathBuf,
    /// Descend into subfolders of folder and wildcard inputs.
    #[arg(long, short)]
    pub recursive: bool,
    /// Write one .qci per QC section instead of a single QC.
    #[arg(long)]
    pub split_qc: bool,
}
