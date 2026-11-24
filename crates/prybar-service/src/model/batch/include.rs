use super::target::{key, place, Placed};
use crate::source::Origin;
use std::collections::HashSet;
use std::path::Path;

/// One model to decompile. `from` is set for a model pulled in by
/// another one's `$includemodel`.
#[derive(Debug, Clone)]
pub struct Job {
    pub stem: String,
    pub dir: std::path::PathBuf,
    pub from: Option<String>,
}

/// The `$includemodel` name as a game-relative stem
/// (`models/humans/male_shared`), or `None` when it could escape.
pub fn game_stem(name: &str) -> Option<String> {
    let name = name.replace('\\', "/");
    let name = name.trim_start_matches('/');
    if name.split('/').any(|p| p == ".." || p.contains(':')) || name.is_empty() {
        return None;
    }
    let stem = crate::source::mdl_stem(name).unwrap_or(name);
    Some(stem.to_owned())
}

/// Finds the stem of an included model: inside the same package, or on
/// disk under any folder above the model (the game folder that the
/// `models/...` name is relative to), the input root, or next to the
/// model itself. First hit wins.
pub fn resolve(origin: Origin, root: &str, stem: &str, game: &str) -> Option<String> {
    let file = game.rsplit('/').next().unwrap_or(game);
    let model_dir = Path::new(stem).parent().unwrap_or(Path::new(""));
    if let Origin::Package(p) = origin {
        let dir = model_dir.to_str().unwrap_or("").replace('\\', "/");
        let near = format!("{dir}/{file}");
        let found = [game, near.trim_start_matches('/')]
            .into_iter()
            .find(|c| p.contains(&format!("{c}.mdl")));
        return found.map(str::to_owned);
    }
    let mut dirs: Vec<&Path> = model_dir.ancestors().collect();
    dirs.push(Path::new(root));
    let near = model_dir.join(file);
    dirs.iter()
        .map(|d| d.join(game))
        .chain(std::iter::once(near))
        .find(|c| with_mdl(c).is_file())
        .and_then(|c| c.to_str().map(str::to_owned))
}

fn with_mdl(stem: &Path) -> std::path::PathBuf {
    let mut s = stem.as_os_str().to_os_string();
    s.push(".mdl");
    s.into()
}

/// The models that the ones just written include and that this run has
/// not decompiled yet. Unresolved names are warnings: the including
/// model is still complete, only its shared animations are missing.
pub fn next_jobs(
    (origin, root): (Origin, &str),
    found: Vec<(String, Vec<String>)>,
    (out, flat): (&Path, bool),
    placed: &mut Placed,
    done: &mut HashSet<String>,
) -> Vec<Job> {
    let mut jobs = Vec::new();
    for (stem, name) in found
        .iter()
        .flat_map(|(s, n)| n.iter().map(move |n| (s, n)))
    {
        let hit = game_stem(name).and_then(|g| Some((resolve(origin, root, stem, &g)?, g)));
        let Some((hit, game)) = hit else {
            let w = format!("{stem}.mdl: $includemodel {name} not found");
            placed.warnings.push(w);
            continue;
        };
        if done.insert(key(origin, &hit)) {
            let dir = place(out, flat, (root, &hit, &game), placed);
            let from = Some(format!("{stem}.mdl"));
            jobs.push(Job {
                stem: hit,
                dir,
                from,
            });
        }
    }
    jobs
}
