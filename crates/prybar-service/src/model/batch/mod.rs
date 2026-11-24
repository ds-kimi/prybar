mod include;
mod run;
mod target;

use super::{load, write_model, DecompileOptions, DecompileReport, Failure, ModelReport};
use crate::source::{expand, Batch, Origin};
use crate::Result;
use include::{next_jobs, Job};
use rayon::prelude::*;
use run::{first_jobs, record, Run};

/// Includes of includes are followed this deep; `done` already stops
/// cycles, this only bounds hostile chains.
const MAX_INCLUDE_DEPTH: usize = 16;

/// Decompiles every model the arguments name, in parallel, into a tree
/// that mirrors each input root, plus the `$includemodel`s they need.
/// A bad model or an argument that names nothing lands in `failures`
/// and never stops the batch. Arguments are expanded one at a time so
/// only one package is held in memory.
pub fn decompile(
    args: &[String],
    opts: &DecompileOptions,
    progress: &(dyn Fn(&str) + Sync),
) -> Result<DecompileReport> {
    let mut run = Run::default();
    for arg in args {
        let batch = match expand(arg, opts.recursive) {
            Ok(b) => b,
            // A usage mistake is the caller's to fix, not a bad model.
            Err(e) if e.exit_code() == crate::ExitCode::Usage => return Err(e),
            Err(e) => {
                run.report.failures.push(failure(arg, &e));
                continue;
            }
        };
        let jobs = first_jobs(&batch, opts, &mut run);
        run_batch(&batch, jobs, opts, &mut run, progress);
    }
    run.report.warnings = run.placed.warnings;
    Ok(run.report)
}

fn run_batch(
    batch: &Batch,
    mut jobs: Vec<Job>,
    opts: &DecompileOptions,
    run: &mut Run,
    progress: &(dyn Fn(&str) + Sync),
) {
    let origin = batch.package.as_ref().map_or(Origin::Disk, Origin::Package);
    for _ in 0..MAX_INCLUDE_DEPTH {
        let results: Vec<_> = jobs
            .par_iter()
            .map(|j| one(origin, j, opts.split_qc, progress))
            .collect();
        let found = record(run, &jobs, results);
        if !opts.with_includes || found.is_empty() {
            return;
        }
        let (at, to) = (
            (origin, batch.root.as_str()),
            (opts.out.as_path(), opts.flat),
        );
        jobs = next_jobs(at, found, to, &mut run.placed, &mut run.done);
    }
    for j in jobs {
        let w = format!("{}.mdl: include chain too deep, skipped", j.stem);
        run.placed.warnings.push(w);
    }
}

fn failure(input: &str, e: &crate::ServiceError) -> Failure {
    Failure {
        input: input.to_owned(),
        error: e.to_string(),
        code: e.exit_code() as u8,
    }
}

/// Writes one model and returns the `$includemodel` names it carries.
fn one(
    origin: Origin,
    job: &Job,
    split_qc: bool,
    progress: &(dyn Fn(&str) + Sync),
) -> std::result::Result<(ModelReport, Vec<String>), Failure> {
    let input = format!("{}.mdl", job.stem);
    progress(&input);
    let fail = |e: crate::ServiceError| failure(&input, &e);
    let mut ir = load(origin, &job.stem).map_err(&fail)?;
    let files = write_model(&mut ir, &job.dir, split_qc).map_err(&fail)?;
    let includes = ir.include_models.iter().map(|m| m.path.clone()).collect();
    let report = ModelReport {
        input,
        out_dir: job.dir.clone(),
        files,
        included_from: job.from.clone(),
        notes: ir.notes,
    };
    Ok((report, includes))
}
