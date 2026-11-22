use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct CompileArgs {
    pub qc: PathBuf,
    /// Configured game name (see `prybar games`).
    #[arg(long, short)]
    pub game: String,
}

#[derive(Debug, Args)]
pub struct ViewArgs {
    pub model: PathBuf,
    /// Configured game name (see `prybar games`).
    #[arg(long, short)]
    pub game: String,
}

#[derive(Debug, Args)]
pub struct PublishArgs {
    /// Item spec JSON: app_id, item_id (update) or none (create), title,
    /// description, tags, visibility, preview, content, change_note.
    #[arg(required_unless_present = "list")]
    pub spec: Option<PathBuf>,
    /// Validate only; never contacts Steam.
    #[arg(long)]
    pub dry_run: bool,
    /// List your published items for this Steam app id instead.
    #[arg(long, value_name = "APP_ID", conflicts_with = "spec")]
    pub list: Option<u32>,
}

#[derive(Debug, Args)]
pub struct GamesArgs {
    /// Add games found in Steam libraries.
    #[arg(long)]
    pub detect: bool,
    /// Write the result to the config file.
    #[arg(long)]
    pub save: bool,
}
