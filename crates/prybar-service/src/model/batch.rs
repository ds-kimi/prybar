use super::target::targets;
use super::{load, write_model, DecompileOptions, DecompileReport, Failure, ModelReport};
use crate::source::{expand, Origin};
use crate::Result;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::Path;

/// Decompiles every model the arguments name, in parallel. A bad model
/// or an argument that names nothing lands in `failures` and never stops
/// the batch. Arguments are expanded one at a time so only one package
/// is held in memory.
pub fn decompile(
    args: &[String],
    opts: &DecompileOptions,
    progress: &(dyn Fn(&str) + Sync),
) -> Result<DecompileReport> {
    let mut report = DecompileReport::default();
    let mut taken = HashSet::new();
    for arg in args {
        let batch = match expand(arg, opts.recursive) {
            Ok(b) => b,
            // A usage mistake is the caller's to fix, not a bad model.
            Err(e) if e.exit_code() == crate::ExitCode::Usage => return Err(e),
            Err(e) => {
                report.failures.push(failure(arg, &e));
                continue;
            }
        };
        let origin = batch.package.as_ref().map_or(Origin::Disk, Origin::Package);
        targets(&batch, &opts.out, &mut taken)
            .par_iter()
            .map(|(stem, dir)| one(origin, stem, dir, opts.split_qc, progress))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|r| match r {
                Ok(m) => report.models.push(m),
                Err(f) => report.failures.push(f),
            });
    }
    Ok(report)
}

fn failure(input: &str, e: &crate::ServiceError) -> Failure {
    Failure {
        input: input.to_owned(),
        error: e.to_string(),
        code: e.exit_code() as u8,
    }
}

fn one(
    origin: Origin,
    stem: &str,
    out_dir: &Path,
    split_qc: bool,
    progress: &(dyn Fn(&str) + Sync),
) -> std::result::Result<ModelReport, Failure> {
    let input = format!("{stem}.mdl");
    progress(&input);
    let fail = |e: crate::ServiceError| failure(&input, &e);
    let mut ir = load(origin, stem).map_err(&fail)?;
    let files = write_model(&mut ir, out_dir, split_qc).map_err(&fail)?;
    Ok(ModelReport {
        input,
        out_dir: out_dir.to_path_buf(),
        files,
        notes: ir.notes,
    })
}
