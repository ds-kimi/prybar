use crate::qc::sanitize::ident;
use prybar_mdl::ModelIr;

/// Out-of-range controller indices print as `0` so the rule still parses.
pub fn controller(ir: &ModelIr, i: i32) -> String {
    let c = usize::try_from(i)
        .ok()
        .and_then(|i| ir.flex_controllers.get(i));
    c.map_or_else(|| "0".to_owned(), |c| ident(&c.name))
}

pub fn desc(ir: &ModelIr, i: i32) -> String {
    let d = usize::try_from(i).ok().and_then(|i| ir.flex_descs.get(i));
    d.map_or_else(|| format!("flex{i}"), |d| ident(d))
}
