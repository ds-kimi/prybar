//! `$model` blocks: flex shapes, controllers, rules and eyeballs.

mod expr;
mod expr_ops;
mod eyes;
mod flexfile;
mod localvar;
mod lookup;

use super::fmt::{num, quote};
use super::sanitize::ident;
use super::Names;
use prybar_mdl::ir::SubModel;
use prybar_mdl::ModelIr;
use std::fmt::Write;

pub fn model_block(
    s: &mut String,
    ir: &ModelIr,
    names: &Names,
    (part, model): (usize, &SubModel),
    file: &str,
) {
    let name = ir.body_parts.get(part).map_or("", |p| p.name.as_str());
    let _ = writeln!(s, "$model {} {} {{", quote(name), quote(file));
    let vta = names.vta(part, 0);
    if let Some(vta) = &vta {
        flexfile::write(s, ir, model, vta);
    }
    eyes::eyeballs(s, ir, model);
    if let Some(vta) = &vta {
        eyes::eyelids(s, ir, model, vta);
    }
    if first_flexed(ir) == Some(part) {
        controllers(s, ir);
    }
    s.push_str("}\n");
}

/// Controllers and rules are model-wide; repeating them in every `$model`
/// would declare them twice.
fn first_flexed(ir: &ModelIr) -> Option<usize> {
    ir.body_parts
        .iter()
        .position(|p| match p.models.as_slice() {
            [m] => super::model::is_flexed(m) && !m.lods.is_empty(),
            _ => false,
        })
}

fn controllers(s: &mut String, ir: &ModelIr) {
    for c in &ir.flex_controllers {
        let kind = if c.kind.is_empty() {
            "default"
        } else {
            &c.kind
        };
        let (min, max) = (num(c.min), num(c.max));
        let _ = writeln!(
            s,
            "  flexcontroller {} range {min} {max} {}",
            ident(kind),
            ident(&c.name)
        );
    }
    localvar::write(s, ir);
    for rule in &ir.flex_rules {
        let target = lookup::desc(ir, rule.flex);
        let _ = writeln!(s, "  %{target} = {}", expr::infix(ir, &rule.ops));
    }
}
