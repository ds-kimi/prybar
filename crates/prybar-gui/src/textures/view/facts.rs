//! The lines under the preview: what the texture is, or why it is not.

use gpui::SharedString;

use super::TextureView;
use crate::textures::{Info, Slot};

pub fn found(i: &Info, v: &TextureView) -> Vec<SharedString> {
    let shown = v.shown.as_ref().map_or(String::new(), |s| {
        format!(" · showing {}×{}", s.width, s.height)
    });
    let size = format!(
        "{}×{} · {} · {} mips{shown}",
        i.width, i.height, i.format, i.mips
    );
    vec![size.into(), format!("From {}", i.location).into()]
}

/// The paths tried for a missing material or texture, and the roots.
pub fn missing(s: &Slot, v: &TextureView) -> Vec<SharedString> {
    let looked = v
        .set
        .as_ref()
        .map(|set| set.sources.describe())
        .unwrap_or_default();
    let base = s
        .report
        .base_texture
        .iter()
        .map(|b| format!("materials/{b}.vtf"));
    let tried: Vec<String> = s.report.tried.iter().cloned().chain(base).collect();
    let roots = if looked.is_empty() {
        "nothing to search yet: add a game in Settings".to_owned()
    } else {
        looked.join(", ")
    };
    vec![
        format!("Looked for {}", tried.join(", ")).into(),
        format!("In {roots}").into(),
    ]
}
