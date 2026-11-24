use super::expand::qc_files;
use super::report::{CompileBatchReport, CompileItem, CompileOptions, ItemStatus};
use super::studiomdl;
use crate::{ExitCode, Result, ServiceError};
use prybar_tools::runner::Runner;
use rayon::prelude::*;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

/// More parallel studiomdl runs than this only thrash the disk.
const MAX_JOBS: usize = 64;

/// Compiles every QC the arguments name with the game's studiomdl, a
/// few at a time. A failed QC or an argument that names nothing is an
/// item in the report and never stops the batch; an unknown game or a
/// missing studiomdl is the caller's mistake and fails the call.
/// Setting `cancel` kills running compiles and skips the rest.
pub fn compile_batch(
    game_name: &str,
    args: &[String],
    opts: &CompileOptions,
    progress: &(dyn Fn(&str) + Sync),
    cancel: &AtomicBool,
) -> Result<CompileBatchReport> {
    let (g, studiomdl) = studiomdl(game_name)?;
    let (files, bad) = qc_files(args, opts.recursive);
    let mut report = CompileBatchReport {
        items: bad.into_iter().map(CompileItem::bad_input).collect(),
        ..Default::default()
    };
    if !files.is_empty() {
        let tool = (studiomdl.as_path(), g.game_dir.as_path());
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(jobs(opts.jobs))
            .build()
            .map_err(|e| ServiceError::Tool(e.to_string()))?;
        let done: Vec<CompileItem> = pool.install(|| {
            files
                .par_iter()
                .map(|qc| one(tool, qc, progress, cancel))
                .collect()
        });
        report.items.extend(done);
    }
    tally(&mut report);
    Ok(report)
}

fn jobs(asked: Option<usize>) -> usize {
    let cores = std::thread::available_parallelism().map_or(2, |n| n.get());
    asked.unwrap_or(cores / 2).clamp(1, MAX_JOBS)
}

fn one(
    (studiomdl, game_dir): (&Path, &Path),
    qc: &Path,
    progress: &(dyn Fn(&str) + Sync),
    cancel: &AtomicBool,
) -> CompileItem {
    let name = qc.display().to_string();
    if cancel.load(Ordering::Relaxed) {
        let code = ExitCode::Partial as u8;
        return CompileItem::without_run(name, ItemStatus::Cancelled, code, "cancelled".into());
    }
    progress(&name);
    let runner = Runner::for_host();
    match prybar_tools::compile::compile_streamed(
        &runner,
        (studiomdl, game_dir, qc),
        &mut |_| {},
        cancel,
    ) {
        Ok(r) => CompileItem::from_report(name, r, cancel.load(Ordering::Relaxed)),
        Err(e) => CompileItem::bad_input((name, super::tool_error(e))),
    }
}

fn tally(r: &mut CompileBatchReport) {
    let count = |s| r.items.iter().filter(|i| i.status == s).count();
    (r.compiled, r.failed, r.cancelled) = (
        count(ItemStatus::Ok),
        count(ItemStatus::Failed),
        count(ItemStatus::Cancelled),
    );
}
