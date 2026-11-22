use std::path::{Path, PathBuf};

use steamworks::{PublishedFileVisibility, UpdateHandle};

use crate::spec::{ItemSpec, Visibility};

/// Only the fields the spec sets; Steam keeps the rest on update.
pub fn apply(mut h: UpdateHandle, spec: &ItemSpec, content: Option<&Path>) -> UpdateHandle {
    if let Some(title) = &spec.title {
        h = h.title(title);
    }
    if let Some(text) = &spec.description {
        h = h.description(text);
    }
    if !spec.tags.is_empty() {
        h = h.tags(spec.tags.clone(), false);
    }
    if let Some(v) = spec.visibility {
        h = h.visibility(visibility(v));
    }
    if let Some(preview) = &spec.preview {
        h = h.preview_path(&absolute(preview));
    }
    if let Some(dir) = content {
        h = h.content_path(&absolute(dir));
    }
    h
}

fn visibility(v: Visibility) -> PublishedFileVisibility {
    match v {
        Visibility::Public => PublishedFileVisibility::Public,
        Visibility::FriendsOnly => PublishedFileVisibility::FriendsOnly,
        Visibility::Private => PublishedFileVisibility::Private,
        Visibility::Unlisted => PublishedFileVisibility::Unlisted,
    }
}

/// Steam requires absolute paths for content and preview.
fn absolute(p: &Path) -> PathBuf {
    std::path::absolute(p).unwrap_or_else(|_| p.to_path_buf())
}
