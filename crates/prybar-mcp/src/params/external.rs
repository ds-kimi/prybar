use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CompileArgs {
    /// Name of a configured game (see games_list), case-insensitive, e.g. `GarrysMod`.
    pub game: String,
    /// Absolute path of one `.qc` file; the reply is then the full
    /// compile report with the studiomdl log.
    #[serde(default)]
    pub qc: Option<String>,
    /// Batch mode: absolute `.qc` files, folders (every `.qc` inside) or
    /// wildcards like `/abs/out/*.qc`. The reply is a per-QC report
    /// (status, errors, warnings, outputs) without the logs.
    #[serde(default)]
    pub inputs: Vec<String>,
    /// Batch mode: also search sub-folders of folder and wildcard inputs.
    #[serde(default)]
    pub recursive: bool,
    /// Batch mode: studiomdl runs at once (default half the CPU cores).
    #[serde(default)]
    pub jobs: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ViewArgs {
    /// Name of a configured game whose hlmv opens the model.
    pub game: String,
    /// Absolute path of the `.mdl` file to open.
    pub model: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PublishArgs {
    /// Absolute path of a JSON item spec: `app_id`, optional `item_id` (update instead of create), `title`, `description`, `tags`, `visibility` (`public`, `friendsonly`, `private`, `unlisted`; default `private`), `preview` (png/jpg resized to 512x512, or gif under 1 MB), `content` (Garry's Mod: an addon folder with addon.json or a .gma; Team Fortress 2: a folder with at least one .bsp map), `change_note`. Paths inside it must be absolute.
    pub spec: Option<String>,
    /// Check the spec and content without uploading anything.
    #[serde(default)]
    pub dry_run: bool,
    /// Must be `true` for a real (non dry-run) publish: it changes the user's public Steam profile. Ask the user first.
    #[serde(default)]
    pub confirm: bool,
    /// Instead of publishing, list the signed-in user's Workshop items for this Steam app id.
    pub list_app_id: Option<u32>,
    /// Workshop game: `garrysmod` (4000) or `tf2` (440), by name or app id. With `spec` it overrides the spec's `app_id`; without `spec` it lists the user's items for that game.
    pub game: Option<String>,
    /// Instead of publishing, list the supported Workshop games, which ones are installed on this machine, and why other Source games are not supported.
    #[serde(default)]
    pub list_games: bool,
}
