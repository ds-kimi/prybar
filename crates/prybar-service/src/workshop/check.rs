//! Checks that run before an upload and never contact Steam.

use std::path::Path;

use prybar_workshop::spec::DESCRIPTION_MAX;
use serde::Serialize;

use crate::{Result, ServiceError};

#[derive(Debug, Clone, Serialize)]
pub struct BbcodeReport {
    /// Characters, counted the way Steam's limit counts them.
    pub chars: usize,
    pub max_chars: usize,
    pub too_long: bool,
    /// Tags that parsed and will be drawn, each once.
    pub tags: Vec<&'static str>,
    /// What a reader sees, markup removed.
    pub plain: String,
}

/// Unknown or unclosed tags are not errors: Steam shows them as text,
/// and so does `plain`.
pub fn check_bbcode(text: &str) -> BbcodeReport {
    let (plain, tags) = prybar_workshop::bbcode::plain(&prybar_workshop::bbcode::parse(text));
    let chars = text.chars().count();
    BbcodeReport {
        chars,
        max_chars: DESCRIPTION_MAX,
        too_long: chars > DESCRIPTION_MAX,
        tags,
        plain,
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PreviewReport {
    pub width: u32,
    pub height: u32,
    pub bytes: u64,
    /// Steam gets a 512x512 center crop instead of the file as is.
    pub resized: bool,
    pub ok: bool,
    pub problem: Option<String>,
}

pub fn check_preview(path: &Path) -> Result<PreviewReport> {
    if !path.is_file() {
        return Err(ServiceError::NotFound(path.display().to_string()));
    }
    let info = prybar_workshop::preview::describe(path)
        .map_err(|e| ServiceError::invalid(path.display(), e))?;
    Ok(PreviewReport {
        width: info.width,
        height: info.height,
        bytes: info.bytes,
        resized: info.resized,
        ok: info.problem.is_none(),
        problem: info.problem,
    })
}
