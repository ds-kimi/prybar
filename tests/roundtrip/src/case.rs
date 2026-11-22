use crate::compare::diff;
use crate::metrics::Metrics;
use prybar_service::model::{load, write_model};
use prybar_service::package::Package;
use prybar_service::source::Origin;
use serde::Serialize;
use std::path::{Path, PathBuf};

/// Where one model ended: `stage` is the step that stopped it.
#[derive(Debug, Serialize)]
pub struct CaseResult {
    pub model: String,
    pub stage: &'static str,
    pub diffs: Vec<String>,
}

pub struct Tools<'a> {
    pub studiomdl: &'a Path,
    pub gameinfo: &'a Path,
}

/// Decompile, rebuild in a private game dir, re-parse, compare. Each
/// case owns its temp dir so cases can run in parallel.
pub fn run(pkg: &Package, stem: &str, tools: &Tools) -> CaseResult {
    let fail = |stage, why: String| CaseResult {
        model: stem.to_owned(),
        stage,
        diffs: vec![why],
    };
    let mut ir = match load(Origin::Package(pkg), stem) {
        Ok(ir) => ir,
        Err(e) => return fail("parse", e.to_string()),
    };
    let original = Metrics::of(&ir);
    let tmp = match tempfile::tempdir() {
        Ok(t) => t,
        Err(e) => return fail("io", e.to_string()),
    };
    let rebuilt = match rebuild(&mut ir, tmp.path(), tools) {
        Ok(m) => m,
        Err((stage, why)) => return fail(stage, why),
    };
    let diffs = diff(&original, &rebuilt);
    let stage = if diffs.is_empty() { "pass" } else { "compare" };
    CaseResult {
        model: stem.to_owned(),
        stage,
        diffs,
    }
}

type StageErr = (&'static str, String);

fn rebuild(ir: &mut prybar_mdl::ModelIr, tmp: &Path, tools: &Tools) -> Result<Metrics, StageErr> {
    let (dec, game) = (tmp.join("dec"), tmp.join("game"));
    write_model(ir, &dec, false).map_err(|e| ("decompile", e.to_string()))?;
    std::fs::create_dir_all(&game).map_err(|e| ("io", e.to_string()))?;
    std::fs::copy(tools.gameinfo, game.join("gameinfo.txt")).map_err(|e| ("io", e.to_string()))?;
    let qc = qc_in(&dec).ok_or(("decompile", "no .qc written".to_owned()))?;
    crate::compile::compile(tools.studiomdl, &game, &qc).map_err(|e| ("compile", e))?;
    let name = ir.name.replace('\\', "/");
    let stem = game.join("models").join(name.trim_end_matches(".mdl"));
    let stem = stem.to_string_lossy().into_owned();
    let back = load(Origin::Disk, &stem).map_err(|e| ("reparse", e.to_string()))?;
    Ok(Metrics::of(&back))
}

fn qc_in(dir: &Path) -> Option<PathBuf> {
    std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .find(|p| p.extension().is_some_and(|x| x == "qc"))
}
