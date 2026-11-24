use super::pretty;
use crate::cli::{GamesArgs, ViewArgs};
use crate::output::Output;
use prybar_service::{tools, Result};

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
