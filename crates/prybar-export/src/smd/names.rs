use std::collections::HashSet;

/// Names often carry a source path (`folder/name.smd`) or a leading `@`
/// (studiomdl's mark for anims made inline in a `$sequence`); only a safe
/// stem is kept so every file lands in the output folder.
pub fn stem(raw: &str, fallback: &str) -> String {
    let base = raw.rsplit(['/', '\\']).next().unwrap_or(raw);
    let base = base.strip_suffix(".smd").unwrap_or(base);
    let clean: String = base
        .trim_start_matches('@')
        .chars()
        .map(|c| {
            if c.is_control() || ":*?\"<>|".contains(c) {
                '_'
            } else {
                c
            }
        })
        .collect();
    match clean.trim_matches('.') {
        "" => fallback.to_owned(),
        s => s.to_owned(),
    }
}

/// Stems are not unique across sub-models or anims, and a clash would
/// overwrite a file on disk.
pub fn unique(taken: &mut HashSet<String>, name: String) -> String {
    let stem = name.trim_end_matches(".smd").to_owned();
    let mut candidate = name;
    let mut n = 2;
    while !taken.insert(candidate.to_lowercase()) {
        candidate = format!("{stem}_{n}.smd");
        n += 1;
    }
    candidate
}
