use crate::EMBEDDED;

/// One file in the directory tree. `path` is the full `/`-separated name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub path: String,
    pub crc: u32,
    pub preload: Vec<u8>,
    pub archive_index: u16,
    pub offset: u32,
    pub length: u32,
}

impl Entry {
    pub fn size(&self) -> u64 {
        self.preload.len() as u64 + u64::from(self.length)
    }

    pub fn is_embedded(&self) -> bool {
        self.archive_index == EMBEDDED
    }
}

/// The tree stores a single space for "no directory" and "no extension".
pub(crate) fn join_path(ext: &str, dir: &str, name: &str) -> String {
    let mut out = String::new();
    if dir != " " {
        out.push_str(dir);
        out.push('/');
    }
    out.push_str(name);
    if ext != " " {
        out.push('.');
        out.push_str(ext);
    }
    out
}

/// Inverse of `join_path`: returns (ext, dir, name). Dotfiles keep the
/// whole name, since an empty name would read as the end-of-names marker.
pub(crate) fn split_path(path: &str) -> (&str, &str, &str) {
    let (dir, file) = match path.rsplit_once('/') {
        Some((d, f)) if !d.is_empty() => (d, f),
        Some((_, f)) => (" ", f),
        None => (" ", path),
    };
    match file.rsplit_once('.') {
        Some((name, ext)) if !ext.is_empty() && !name.is_empty() => (ext, dir, name),
        _ => (" ", dir, file),
    }
}
