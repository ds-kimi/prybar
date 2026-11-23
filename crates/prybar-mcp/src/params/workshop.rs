use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GameArgs {
    /// Workshop game: `garrysmod` (4000) or `tf2` (440), by name or app id.
    pub game: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ItemsArgs {
    /// Workshop game: `garrysmod` (4000) or `tf2` (440), by name or app id.
    pub game: String,
    /// Case-insensitive text matched against title, tags and item id. Empty = all.
    #[serde(default)]
    pub search: String,
    /// One of `subscribed`, `rated`, `favorited`, `updated` (default), `newest`, `title`. Numbers sort largest first, `title` A to Z.
    pub sort: Option<String>,
    /// Reverse the order: smallest first, or Z to A for `title`.
    #[serde(default)]
    pub reverse: bool,
    /// Only items with this visibility: `public`, `friendsonly`, `private` or `unlisted`.
    pub visibility: Option<String>,
    /// Only items carrying this tag (any case); the reply's `tags` lists those in use.
    pub tag: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ItemArgs {
    /// Workshop game: `garrysmod` (4000) or `tf2` (440), by name or app id.
    pub game: String,
    /// Published file id of one of the signed-in user's items.
    pub item_id: u64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PreviewArgs {
    /// Absolute path of a PNG, JPEG or GIF preview image.
    pub image: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BbcodeArgs {
    /// Description text in Steam BBCode: [h1] [h2] [h3] [b] [i] [u] [strike] [spoiler] [noparse] [hr] [url=…] [img] [list] [olist] [*] [quote=…] [code] [table] [tr] [th] [td]. At most 7999 characters.
    pub text: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TextureArgs {
    /// Absolute path of a `.vtf` file, or with `material` a model (`.mdl` path or `<package>:models/x.mdl`) whose folder tree, package and configured games are searched like the engine does.
    pub input: String,
    /// Material name as inspect_model lists it, e.g. `models/props/crate01`. Omit to decode `input` as a `.vtf` file.
    pub material: Option<String>,
    /// Mip level, 0 = full size; clamped to the smallest mip.
    #[serde(default)]
    pub level: u8,
    /// Absolute path of the PNG file to write (overwritten).
    pub out: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PrivacyArgs {
    /// Change the desktop app's anonymous analytics: `true` on, `false` off (also drops anything queued). Omit to only read.
    pub set: Option<bool>,
    /// Empty the local log of sent data.
    #[serde(default)]
    pub clear_log: bool,
    /// Must be `true` when `set` or `clear_log` is given: it changes the user's settings. Ask the user first.
    #[serde(default)]
    pub confirm: bool,
}
