//! Round-trip harness: decompile real game models, rebuild them with the
//! game's own studiomdl and check the rebuild matches the original.
//!
//! `prybar-roundtrip [count per package] [report dir]`. Needs a local
//! Garry's Mod install (`PRYBAR_GMOD_DIR` or Steam detection); exits 0
//! with a skip message when there is none, since assets are never
//! committed.

mod case;
mod compare;
mod compile;
mod metrics;
mod report;
mod sample;

use case::Tools;
use prybar_service::package::Package;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

/// GMod's own content plus the mounted HL2 and CS:S packages, so the
/// sample covers several studiomdl generations.
const PACKAGES: [&str; 4] = [
    "garrysmod/garrysmod_dir.vpk",
    "sourceengine/hl2_misc_dir.vpk",
    "sourceengine/content_hl2_dir.vpk",
    "sourceengine/content_cstrike_dir.vpk",
];

fn main() -> std::process::ExitCode {
    let mut args = std::env::args().skip(1);
    let count = args.next().and_then(|a| a.parse().ok()).unwrap_or(25);
    let out = PathBuf::from(args.next().unwrap_or_else(|| "target/roundtrip".into()));
    let Some(root) = prybar_tools::paths::gmod_dir() else {
        eprintln!("skip: no Garry's Mod install (set PRYBAR_GMOD_DIR)");
        return std::process::ExitCode::SUCCESS;
    };
    let results = run_all(&root, count);
    if let Err(e) = report::write(&out, &results) {
        eprintln!("cannot write report: {e}");
        return std::process::ExitCode::FAILURE;
    }
    let pass = report::pass_count(&results);
    println!(
        "{pass} / {} pass, report in {}",
        results.len(),
        out.display()
    );
    // The plan's bar: 90% of the sample must survive the rebuild.
    let ok = pass * 10 >= results.len() * 9;
    std::process::ExitCode::from(if ok { 0 } else { 1 })
}

fn run_all(root: &Path, count: usize) -> Vec<case::CaseResult> {
    let studiomdl = root.join("bin").join("studiomdl.exe");
    let gameinfo = root.join("garrysmod").join("gameinfo.txt");
    let tools = Tools {
        studiomdl: &studiomdl,
        gameinfo: &gameinfo,
    };
    let mut all = Vec::new();
    for rel in PACKAGES {
        let Ok(pkg) = Package::open(&root.join(rel)) else {
            eprintln!("skip package {rel}: not found or unreadable");
            continue;
        };
        let stems = sample::sample(&pkg, count);
        let mut res: Vec<_> = stems
            .par_iter()
            .map(|s| case::run(&pkg, s, &tools))
            .collect();
        all.append(&mut res);
    }
    all
}
