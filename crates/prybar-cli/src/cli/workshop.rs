use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct WorkshopArgs {
    #[command(subcommand)]
    pub command: WorkshopCommand,
}

#[derive(Debug, Subcommand)]
pub enum WorkshopCommand {
    /// Your items for a game, searched, filtered and sorted like the
    /// app's list. Also records today's stats point.
    Items(ItemsArgs),
    /// One of your items: fields, stats, daily stats points and its
    /// version history from the Workshop changelog page.
    Item(ItemArgs),
    /// Check a preview image: size, whether Steam gets a 512x512 crop,
    /// and why it would be refused (over 32 MB, GIF of 1 MB or more).
    Preview(PreviewArgs),
    /// Check a Steam BBCode description: length against the 7999-char
    /// limit, tags used, and the plain text a reader sees.
    Bbcode(BbcodeArgs),
}

#[derive(Debug, Args)]
pub struct ItemsArgs {
    /// Workshop game: `garrysmod`, `tf2` or its app id.
    pub game: String,
    /// Case-insensitive match on title, tags and item id.
    #[arg(long, short, default_value = "")]
    pub search: String,
    /// subscribed, rated, favorited, updated (default), newest or title.
    #[arg(long)]
    pub sort: Option<String>,
    /// Reverse the order: smallest first, or Z to A for `title`.
    #[arg(long)]
    pub reverse: bool,
    /// public, friendsonly, private or unlisted.
    #[arg(long)]
    pub visibility: Option<String>,
    /// Only items carrying this tag (any case).
    #[arg(long)]
    pub tag: Option<String>,
}

#[derive(Debug, Args)]
pub struct ItemArgs {
    /// Workshop game: `garrysmod`, `tf2` or its app id.
    pub game: String,
    /// Published file id.
    pub item_id: u64,
}

#[derive(Debug, Args)]
pub struct PreviewArgs {
    /// PNG, JPEG or GIF file.
    pub image: PathBuf,
}

#[derive(Debug, Args)]
pub struct BbcodeArgs {
    /// Text file holding the description, or `-` for stdin.
    pub file: PathBuf,
}

#[derive(Debug, Args)]
pub struct TextureArgs {
    /// A `.vtf` file, or with `--material` a model (`.mdl` or
    /// `pak.vpk:models/x.mdl`) whose tree and games are searched.
    pub input: String,
    /// Material name as the model lists it, e.g. `models/props/crate01`.
    #[arg(long, short)]
    pub material: Option<String>,
    /// Mip level, 0 = full size (clamped to the smallest).
    #[arg(long, default_value_t = 0)]
    pub level: u8,
    /// PNG file to write.
    #[arg(long, short)]
    pub out: PathBuf,
}

#[derive(Debug, Args)]
pub struct PrivacyArgs {
    /// Turn the desktop app's anonymous analytics on.
    #[arg(long, conflicts_with = "off")]
    pub on: bool,
    /// Turn them off and drop anything still queued.
    #[arg(long)]
    pub off: bool,
    /// Empty the local log of sent data.
    #[arg(long)]
    pub clear_log: bool,
}
