//! The last decompile: a bursting check mark and what came out.

use gpui::{div, prelude::*, AnyElement, Div};
use prybar_service::model::DecompileReport;

use super::super::motion;
use crate::state::AppState;
use crate::theme::Palette;

/// The burst id follows the job count so every new run replays it.
pub fn view(s: &AppState, p: Palette) -> AnyElement {
    let Some(r) = &s.last_decompile else {
        return div().into_any_element();
    };
    div()
        .p_6()
        .rounded_3xl()
        .bg(p.surface)
        .shadow_md()
        .flex()
        .items_center()
        .gap_4()
        .when(!r.models.is_empty(), |el| el.child(badge(s.jobs.len(), p)))
        .child(lines(r, p))
        .into_any_element()
}

/// A check mark with a ring bursting out of it.
fn badge(run: usize, p: Palette) -> impl IntoElement {
    let ring = div()
        .absolute()
        .rounded_full()
        .border_4()
        .border_color(p.accent);
    let check = div()
        .size_10()
        .rounded_full()
        .bg(p.accent)
        .flex()
        .items_center()
        .justify_center()
        .text_color(p.surface)
        .child("✓");
    div()
        .relative()
        .size_12()
        .flex()
        .items_center()
        .justify_center()
        .child(motion::burst(ring, ("cv-burst", run)))
        .child(check)
}

fn lines(r: &DecompileReport, p: Palette) -> Div {
    let ok = r.models.iter().map(|m| {
        let out = m.out_dir.display();
        div().child(format!("{} → {} files in {out}", m.input, m.files))
    });
    let bad = r.failures.iter().map(|f| {
        div()
            .text_color(p.danger)
            .child(format!("✗ {}: {}", f.input, f.error))
    });
    div()
        .flex()
        .flex_col()
        .gap_1()
        .text_sm()
        .children(ok)
        .children(bad)
}
