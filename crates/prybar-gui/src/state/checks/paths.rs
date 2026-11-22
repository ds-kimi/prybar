//! Rules for the path and id fields of publish and settings.

use std::path::Path;

/// Empty means a new item; otherwise the number from the item's link.
pub fn item_id(text: &str) -> Option<&'static str> {
    let text = text.trim();
    (!text.is_empty() && text.parse::<u64>().is_err())
        .then_some("Use the number from the item's Workshop link, or leave it empty.")
}

/// Optional; Steam takes PNG, JPG or GIF under 1 MB.
pub fn preview(text: &str) -> Option<&'static str> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    let lower = text.to_ascii_lowercase();
    if ![".png", ".jpg", ".jpeg", ".gif"]
        .iter()
        .any(|e| lower.ends_with(e))
    {
        return Some("Use a PNG, JPG or GIF image.");
    }
    existing(text)
}

/// Empty is allowed (the action says what it needs); typed must exist.
pub fn existing(text: &str) -> Option<&'static str> {
    let text = text.trim();
    (!text.is_empty() && !Path::new(text).exists()).then_some("Nothing at this path.")
}

/// The folder that holds gameinfo.txt.
pub fn game_dir(text: &str) -> Option<&'static str> {
    let path = Path::new(text.trim());
    if !path.is_absolute() {
        return Some("Use a full path, like /…/GarrysMod/garrysmod.");
    }
    (!path.join("gameinfo.txt").is_file()).then_some("No gameinfo.txt in this folder.")
}
