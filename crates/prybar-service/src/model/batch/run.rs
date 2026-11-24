use super::include::Job;
use super::target::{key, place, Placed};
use crate::model::{DecompileOptions, DecompileReport, Failure, ModelReport};
use crate::source::{Batch, Origin};
use std::collections::HashSet;

/// State shared by every argument of one decompile call, so output
/// folders and included models are unique across the whole run.
#[derive(Default)]
pub struct Run {
    pub report: DecompileReport,
    pub placed: Placed,
    pub done: HashSet<String>,
}

/// One job per model of the batch that this run has not seen yet; a
/// model already written (named twice, or pulled in as an include) is a
/// warning so the caller knows where it went.
pub fn first_jobs(batch: &Batch, opts: &DecompileOptions, run: &mut Run) -> Vec<Job> {
    let origin = batch.package.as_ref().map_or(Origin::Disk, Origin::Package);
    let mut jobs = Vec::new();
    for stem in &batch.stems {
        if !run.done.insert(key(origin, stem)) {
            let w = format!("{stem}.mdl: already decompiled in this run, skipped");
            run.placed.warnings.push(w);
            continue;
        }
        let name = stem.rsplit(['/', '\\']).next().unwrap_or(stem);
        let at = (batch.root.as_str(), stem.as_str(), name);
        let dir = place(&opts.out, opts.flat, at, &mut run.placed);
        let stem = stem.clone();
        jobs.push(Job {
            stem,
            dir,
            from: None,
        });
    }
    jobs
}

type Outcome = Result<(ModelReport, Vec<String>), Failure>;

/// Files each outcome in the report and returns, per written model, the
/// `$includemodel` names it carries. An included model that fails is a
/// warning, like one that is missing: every model the caller named may
/// still be complete, so it must not turn the exit code into 7.
pub fn record(run: &mut Run, jobs: &[Job], results: Vec<Outcome>) -> Vec<(String, Vec<String>)> {
    let mut found = Vec::new();
    for (job, r) in jobs.iter().zip(results) {
        match r {
            Ok((m, inc)) => {
                found.push((job.stem.clone(), inc));
                run.report.models.push(m);
            }
            Err(f) => match &job.from {
                Some(from) => {
                    let w = format!("{from}: included {} failed: {}", f.input, f.error);
                    run.placed.warnings.push(w);
                }
                None => run.report.failures.push(f),
            },
        }
    }
    found
}
