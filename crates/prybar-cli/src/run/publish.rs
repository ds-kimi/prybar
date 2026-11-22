use super::pretty;
use crate::cli::PublishArgs;
use crate::output::Output;
use prybar_service::{tools, Result, ServiceError};

pub fn publish(a: &PublishArgs, out: &Output) -> Result<u8> {
    if let Some(app_id) = a.status {
        out.emit(&tools::steam_status(app_id), pretty);
        return Ok(0);
    }
    if let Some(app_id) = a.list {
        let items = tools::workshop_list(app_id)?;
        out.emit(&items, pretty);
        return Ok(0);
    }
    let Some(path) = &a.spec else {
        return Err(ServiceError::Usage(
            "publish needs a spec file or --list".into(),
        ));
    };
    let spec = tools::load_spec(path)?;
    let report = tools::publish(&spec, a.dry_run, &mut |p| {
        out.progress(&format!("{}: {}/{}", p.stage, p.done, p.total));
    })?;
    out.emit(&report, pretty);
    Ok(0)
}
