use super::pretty;
use crate::cli::{CompileArgs, GamesArgs, ViewArgs};
use crate::output::Output;
use prybar_service::{tools, ExitCode, Result};

/// A compile that ran but failed exits 6 with the report on stdout, so
/// the errors are not lost behind a bare error line.
pub fn compile(a: &CompileArgs, out: &Output) -> Result<u8> {
    out.progress(&format!("compiling {}", a.qc.display()));
    let report = tools::compile(&a.game, &a.qc)?;
    out.emit(&report, |r| {
        let mut s = r.log.clone();
        s.push_str(if r.success {
            "\ncompile ok\n"
        } else {
            "\ncompile FAILED\n"
        });
        s
    });
    Ok(if report.success {
        0
    } else {
        ExitCode::Tool as u8
    })
}

pub fn view(a: &ViewArgs, out: &Output) -> Result<u8> {
    let report = tools::view(&a.game, &a.model)?;
    out.emit(&report, |r| format!("hlmv started (pid {})\n", r.pid));
    Ok(0)
}

pub fn games(a: &GamesArgs, out: &Output) -> Result<u8> {
    let report = tools::games(a.detect, a.save)?;
    out.emit(&report, pretty);
    Ok(0)
}
