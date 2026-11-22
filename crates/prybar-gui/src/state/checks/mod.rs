//! Rules for form text, with messages that say what to type instead.

mod publish;

pub use publish::{description, tags, title, visibility};

use std::path::Path;

/// Decompile output: the folder is created, so it only has to be a full path.
pub fn out_dir(text: &str) -> Option<&'static str> {
    if text.trim().is_empty() {
        return Some("Choose an output folder.");
    }
    (!Path::new(text.trim()).is_absolute()).then_some("Use a full path, like /tmp/prybar-out.")
}

/// Empty is fine until compile runs; anything typed must be a real QC.
pub fn qc(text: &str) -> Option<&'static str> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    if !text.to_ascii_lowercase().ends_with(".qc") {
        return Some("A QC file name ends in .qc.");
    }
    (!Path::new(text).is_file()).then_some("No file at this path.")
}

pub fn required(text: &str) -> Option<&'static str> {
    text.trim().is_empty().then_some("This can't be empty.")
}
