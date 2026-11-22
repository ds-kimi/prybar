use std::path::Path;

use super::content::check_content;
use super::text::{check_tags, check_text, DESCRIPTION_MAX, TITLE_MAX};
use super::ItemSpec;
use crate::WorkshopError;

/// Steam rejects previews of 1 MB or more.
const PREVIEW_MAX: u64 = 1024 * 1024;

/// Every check Steam would otherwise fail on mid-upload.
pub fn validate(spec: &ItemSpec) -> Result<(), WorkshopError> {
    if spec.app_id == 0 {
        return Err(WorkshopError::invalid("app_id", "must be set"));
    }
    if spec.item_id.is_none() && spec.content.is_none() {
        return Err(WorkshopError::invalid(
            "content",
            "required to create an item",
        ));
    }
    match spec.title.as_deref() {
        Some(t) if t.trim().is_empty() => return Err(WorkshopError::invalid("title", "empty")),
        None if spec.item_id.is_none() => return Err(WorkshopError::invalid("title", "required")),
        Some(t) => check_text("title", t, TITLE_MAX)?,
        None => {}
    }
    let long = [
        ("description", &spec.description),
        ("change_note", &spec.change_note),
    ];
    for (field, text) in long {
        text.as_deref()
            .map_or(Ok(()), |t| check_text(field, t, DESCRIPTION_MAX))?;
    }
    check_tags(&spec.tags)?;
    spec.preview.as_deref().map_or(Ok(()), check_preview)?;
    spec.content
        .as_deref()
        .map_or(Ok(()), |c| check_content(spec.app_id, c))
}

fn check_preview(path: &Path) -> Result<(), WorkshopError> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let ok = ["png", "jpg", "jpeg", "gif"]
        .iter()
        .any(|e| ext.eq_ignore_ascii_case(e));
    if !ok {
        return Err(WorkshopError::invalid("preview", "must be png, jpg or gif"));
    }
    let meta = std::fs::metadata(path).map_err(|source| WorkshopError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    if !meta.is_file() || meta.len() >= PREVIEW_MAX {
        return Err(WorkshopError::invalid(
            "preview",
            "must be a file under 1 MB",
        ));
    }
    Ok(())
}
