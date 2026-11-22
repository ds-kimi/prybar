use crate::WorkshopError;

/// Steam's own buffer sizes (`k_cchPublishedDocumentTitleMax` and
/// friends), in bytes including the NUL, hence the minus one.
pub const TITLE_MAX: usize = 128;
pub const DESCRIPTION_MAX: usize = 7999;
pub const TAG_MAX: usize = 255;
pub const TAGS_MAX: usize = 32;

/// NUL would truncate the C string Steam receives; an empty title shows
/// as a blank item.
pub fn check_text(field: &'static str, text: &str, max: usize) -> Result<(), WorkshopError> {
    if text.len() > max {
        return Err(WorkshopError::invalid(field, format!("over {max} bytes")));
    }
    if text.contains('\0') {
        return Err(WorkshopError::invalid(field, "contains NUL"));
    }
    Ok(())
}

/// Steam stores tags comma-joined, so a comma would split one tag in two.
pub fn check_tags(tags: &[String]) -> Result<(), WorkshopError> {
    if tags.len() > TAGS_MAX {
        return Err(WorkshopError::invalid("tags", format!("over {TAGS_MAX}")));
    }
    for tag in tags {
        check_text("tags", tag, TAG_MAX)?;
        if tag.trim().is_empty() || tag.contains(',') {
            return Err(WorkshopError::invalid("tags", format!("bad tag `{tag}`")));
        }
    }
    Ok(())
}
