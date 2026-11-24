use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Output folders already handed out in this run, and the warnings
/// raised while placing models.
#[derive(Debug, Default)]
pub struct Placed {
    pub taken: HashSet<String>,
    pub warnings: Vec<String>,
}

/// Output folder of one model. The default mirrors the input tree
/// relative to `root` (Crowbar behaviour), so same-named models in
/// different folders never meet. `flat` puts every model in
/// `<out>/<name>/`; a clash there gets a `_2`, `_3`... suffix and a
/// warning, never an overwrite. `fallback` is used when the stem is not
/// under `root` (an `$includemodel` found elsewhere).
pub fn place(
    out: &Path,
    flat: bool,
    (root, stem, fallback): (&str, &str, &str),
    placed: &mut Placed,
) -> PathBuf {
    let name = stem.rsplit(['/', '\\']).next().unwrap_or(stem);
    let rel = match flat {
        true => name,
        false => relative(root, stem).unwrap_or(fallback),
    };
    let base = prybar_io::safe_join(out, rel).unwrap_or_else(|| out.join("model"));
    let dir = unique(&mut placed.taken, base.clone());
    if dir != base {
        let (b, d) = (base.display(), dir.display());
        placed
            .warnings
            .push(format!("{stem}.mdl: {b} is already used, wrote to {d}"));
    }
    dir
}

/// `stem` minus the `root` folder, matched case-insensitively because
/// package refs are typed by hand and packages ignore case.
fn relative<'a>(root: &str, stem: &'a str) -> Option<&'a str> {
    let root = root.trim_end_matches(['/', '\\']);
    if root.is_empty() {
        return Some(stem);
    }
    let head = stem.get(..root.len())?;
    let rest = stem.get(root.len()..)?;
    let rest = rest.strip_prefix(['/', '\\'])?;
    head.eq_ignore_ascii_case(root).then_some(rest)
}

/// Keys are lower-cased: on Windows and macOS `Crate` and `crate` are
/// the same folder.
fn unique(taken: &mut HashSet<String>, base: PathBuf) -> PathBuf {
    let mut candidate = base.clone();
    let mut n = 2;
    while !taken.insert(candidate.display().to_string().to_lowercase()) {
        let mut name = base.file_name().unwrap_or_default().to_os_string();
        name.push(format!("_{n}"));
        candidate = base.with_file_name(name);
        n += 1;
    }
    candidate
}

/// Identity of a model across a run, so an `$includemodel` reached by
/// two routes (`./a` and `a`, `A` and `a`) is decompiled once.
pub fn key(stem: &str) -> String {
    let real = std::fs::canonicalize(format!("{stem}.mdl")).ok();
    let real = real.as_ref().and_then(|p| p.to_str()).unwrap_or(stem);
    real.replace('\\', "/").to_lowercase()
}
