use super::pretty;
use crate::cli::PublishArgs;
use crate::output::Output;
use prybar_service::{tools, Result, ServiceError};

pub fn publish(a: &PublishArgs, out: &Output) -> Result<u8> {
    if a.games {
        out.emit(&tools::workshop_games(), pretty);
        return Ok(0);
    }
    if let Some(game) = &a.status {
        out.emit(&tools::steam_status(tools::workshop_app_id(game)?), pretty);
        return Ok(0);
    }
    if let Some(game) = &a.list {
        let items = tools::workshop_list(tools::workshop_app_id(game)?)?;
        out.emit(&items, pretty);
        return Ok(0);
    }
    let Some(path) = &a.spec else {
        return Err(ServiceError::Usage(
            "publish needs a spec file or --list".into(),
        ));
    };
    let mut spec = tools::load_spec(path)?;
    if let Some(game) = &a.game {
        spec.app_id = tools::workshop_app_id(game)?;
    }
    let report = tools::publish(&spec, a.dry_run, &mut |p| {
        out.progress(&format!("{}: {}/{}", p.stage, p.done, p.total));
    })?;
    out.emit(&report, pretty);
    Ok(0)
}
