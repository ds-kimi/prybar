//! Body group, skin and LOD pickers in the top-left corner. Each chip
//! cycles to the next variant: models rarely have more than a handful.

use gpui::{div, prelude::*, Context};
use prybar_mdl::ir::ModelIr;

use super::chip;
use crate::theme::prybar::Tokens;
use crate::viewport::mesh::{lods, Pick};
use crate::viewport::view::Viewport;

/// Selector, label and the pick a click switches to.
type Choice = (String, String, Pick);

pub fn variants(v: &Viewport, t: &Tokens, cx: &mut Context<Viewport>) -> impl IntoElement {
    let row = div().absolute().top_2().left_2().flex().flex_wrap().gap_1();
    let Some(ir) = v.ir.as_deref() else {
        return row;
    };
    let mut choices = bodies(ir, &v.pick);
    choices.extend(others(ir, &v.pick));
    row.children(choices.into_iter().map(|(id, label, next)| {
        chip(id, label, false, t, cx, move |v, cx| {
            v.repick(next.clone(), cx)
        })
    }))
}

fn bodies(ir: &ModelIr, pick: &Pick) -> Vec<Choice> {
    // Sub-model names are often full paths; the last piece is enough.
    // A `blank` body choice has no name.
    let short = |n: &str| match n.rsplit(['/', '\\']).next().unwrap_or(n) {
        "" => "none".to_string(),
        n => n.replace(".smd", ""),
    };
    let parts = ir.body_parts.iter().enumerate();
    let parts = parts.filter(|(_, p)| p.models.len() > 1);
    parts
        .map(|(i, part)| {
            let n = part.models.len();
            let at = pick.body.get(i).copied().unwrap_or(0).min(n - 1);
            let label = format!("{}: {}", part.name, short(&part.models[at].name));
            (
                format!("pb-vp-body-{i}"),
                label,
                with_body(pick, i, (at + 1) % n),
            )
        })
        .collect()
}

fn others(ir: &ModelIr, pick: &Pick) -> Vec<Choice> {
    let mut out = Vec::new();
    let skins = ir.skins.len();
    if skins > 1 {
        let skin = (pick.skin + 1) % skins;
        let label = format!("Skin {}/{skins}", pick.skin + 1);
        out.push((
            "pb-vp-skin".into(),
            label,
            Pick {
                skin,
                ..pick.clone()
            },
        ));
    }
    let levels = lods(ir);
    if levels > 1 {
        let lod = (pick.lod + 1) % levels;
        let label = format!("LOD {}", pick.lod);
        out.push((
            "pb-vp-lod".into(),
            label,
            Pick {
                lod,
                ..pick.clone()
            },
        ));
    }
    out
}

fn with_body(pick: &Pick, part: usize, model: usize) -> Pick {
    let mut next = pick.clone();
    if next.body.len() <= part {
        next.body.resize(part + 1, 0);
    }
    next.body[part] = model;
    next
}
