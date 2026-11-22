use std::path::Path;

use tempfile::TempDir;

use crate::addon::gmod_tags;
use crate::preview::prepare_preview;
use crate::spec::{stage_content, ItemSpec};
use crate::WorkshopError;

/// The spec as Steam gets it: an addon folder packed to a GMA, the
/// preview fitted, tags filled. The temp dirs must outlive the upload.
pub fn stage(spec: &ItemSpec) -> Result<(ItemSpec, [Option<TempDir>; 2]), WorkshopError> {
    let work = TempDir::new().map_err(|e| WorkshopError::io(&std::env::temp_dir(), e))?;
    let mut ready = spec.clone();
    let staged = match &spec.content {
        Some(c) => stage_content(spec.app_id, c)?,
        None => None,
    };
    ready.content = staged
        .as_ref()
        .map(|d| d.path().to_path_buf())
        .or(ready.content);
    if ready.tags.is_empty() {
        ready.tags = spec.content.as_deref().map(folder_tags).unwrap_or_default();
    }
    if let Some(p) = &spec.preview {
        ready.preview = Some(prepare_preview(p, work.path())?);
    }
    Ok((ready, [Some(work), staged]))
}

/// Garry's Mod reads type and tags from the Workshop tags, so an addon
/// folder fills them from its `addon.json` when none were given.
fn folder_tags(content: &Path) -> Vec<String> {
    content
        .is_dir()
        .then(|| prybar_gma::AddonJson::load(&content.join("addon.json")).ok())
        .flatten()
        .map(|json| gmod_tags(&json))
        .unwrap_or_default()
}
