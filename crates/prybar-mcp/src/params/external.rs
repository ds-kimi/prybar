use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CompileArgs {
    /// Name of a configured game (see games_list), case-insensitive, e.g. `GarrysMod`.
    pub game: String,
    /// Absolute path of the `.qc` file to compile with the game's studiomdl.
    pub qc: String,
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
    /// Absolute path of a JSON item spec: `app_id`, optional `item_id` (update instead of create), `title`, `description`, `tags`, `visibility` (`public`, `friendsonly`, `private`, `unlisted`; default `private`), `preview`, `content`, `change_note`. Paths inside it must be absolute.
    pub spec: Option<String>,
    /// Check the spec and content without uploading anything.
    #[serde(default)]
    pub dry_run: bool,
    /// Instead of publishing, list the signed-in user's Workshop items for this Steam app id.
    pub list_app_id: Option<u32>,
}
