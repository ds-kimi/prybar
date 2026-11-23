use std::path::Path;

use super::{PREVIEW_MAX, PREVIEW_SIDE, SOURCE_MAX};

/// What the previewer says about a picked image before any upload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewInfo {
    pub width: u32,
    pub height: u32,
    pub bytes: u64,
    /// Steam gets a 512x512 center crop instead of the file as is.
    pub resized: bool,
    /// Why it cannot be sent; `None` when it can.
    pub problem: Option<String>,
}

/// Reads only the header, so it is cheap enough to run on each pick.
pub fn describe(path: &Path) -> Result<PreviewInfo, String> {
    let bytes = std::fs::metadata(path).map_err(|e| e.to_string())?.len();
    let (width, height) = image::image_dimensions(path).map_err(|e| e.to_string())?;
    let gif = path
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("gif"));
    let square = width == PREVIEW_SIDE && height == PREVIEW_SIDE;
    let problem = if bytes > SOURCE_MAX {
        Some("larger than 32 MB, pick a smaller image".to_owned())
    } else if gif && bytes >= PREVIEW_MAX {
        Some("a GIF must be under 1 MB: it is sent as is to keep it animated".to_owned())
    } else {
        None
    };
    Ok(PreviewInfo {
        width,
        height,
        bytes,
        resized: !gif && (!square || bytes >= PREVIEW_MAX),
        problem,
    })
}
