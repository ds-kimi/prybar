use prybar_gma::AddonJson;

/// The values Garry's Mod's own publisher accepts for `type` and `tags`;
/// anything else makes the item invisible in the in-game browser.
pub const ADDON_TYPES: &[&str] = &[
    "servercontent",
    "gamemode",
    "map",
    "weapon",
    "vehicle",
    "npc",
    "entity",
    "tool",
    "effects",
    "model",
];
pub const ADDON_TAGS: &[&str] = &[
    "fun", "roleplay", "scenic", "movie", "realism", "cartoon", "water", "comic", "build",
];
pub const MAX_ADDON_TAGS: usize = 2;

/// One sentence per problem, empty when the game will take it.
pub fn check_meta(json: &AddonJson) -> Vec<String> {
    let mut out = Vec::new();
    if json.title.trim().is_empty() {
        out.push("title is empty".to_owned());
    }
    if !ADDON_TYPES.contains(&json.kind.to_lowercase().as_str()) {
        let allowed = ADDON_TYPES.join(", ");
        out.push(format!("type `{}` is not one of: {allowed}", json.kind));
    }
    if json.tags.len() > MAX_ADDON_TAGS {
        out.push(format!(
            "at most {MAX_ADDON_TAGS} tags, found {}",
            json.tags.len()
        ));
    }
    for tag in &json.tags {
        if !ADDON_TAGS.contains(&tag.to_lowercase().as_str()) {
            let allowed = ADDON_TAGS.join(", ");
            out.push(format!("tag `{tag}` is not one of: {allowed}"));
        }
    }
    out
}

/// The Workshop tags the game filters on: `Addon`, the type, the tags.
pub fn gmod_tags(json: &AddonJson) -> Vec<String> {
    let mut tags = vec!["Addon".to_owned()];
    if !json.kind.is_empty() {
        tags.push(json.kind.to_lowercase());
    }
    tags.extend(json.tags.iter().map(|t| t.to_lowercase()));
    tags
}
