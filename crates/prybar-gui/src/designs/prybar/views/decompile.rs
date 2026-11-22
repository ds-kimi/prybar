//! Decompile screen: input, output, the one option, run, last result.

use gpui::{div, prelude::*, px, AnyElement, Context};

use super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let target = s
        .selected
        .and_then(|i| s.documents.get(i))
        .map(|d| d.title());
    let input = target
        .clone()
        .unwrap_or_else(|| "Open a model or package first".into());
    div()
        .flex()
        .flex_col()
        .gap_3()
        .max_w(px(520.))
        .child(widgets::heading("Decompile"))
        .child(
            div()
                .text_color(t.muted)
                .child("Turn a model back into QC and SMD files."),
        )
        .child(widgets::readonly("Input", input, t))
        .child(widgets::field("Output folder", &f.out, t).debug_selector(|| "pb-out".into()))
        .child(run(
            s,
            target.is_some() && f.out.read(cx).error().is_none(),
            t,
            cx,
        ))
        .child(result(s, t))
        .into_any_element()
}

/// The option, then the button: disabled with no file or no usable
/// output folder, and while a decompile already runs.
fn run(s: &AppState, has_target: bool, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let split = widgets::switch("pb-split", "One .qci file per QC section", s.split_qc, t)
        .debug_selector(|| "pb-split".into())
        .on_click(cx.listener(|this, _, _, cx| {
            this.state.update(cx, |s, cx| {
                s.split_qc = !s.split_qc;
                cx.notify();
            })
        }));
    let busy = s.is_running("Decompile");
    let ready = has_target && !busy;
    let label = if busy { "Decompiling…" } else { "Decompile" };
    let go = widgets::button("pb-decompile", label, Kind::Primary, ready, t)
        .debug_selector(|| "pb-decompile".into())
        .when(ready, |el| {
            el.on_click(cx.listener(|this, _, _, cx| {
                let out = this.forms.out.read(cx).text().trim().into();
                this.state.update(cx, |s, cx| s.decompile_selected(out, cx))
            }))
        });
    div()
        .flex()
        .flex_col()
        .items_start()
        .gap_3()
        .child(split)
        .child(go)
        .into_any_element()
}

fn result(s: &AppState, t: Tokens) -> AnyElement {
    let Some(r) = &s.last_decompile else {
        return div().into_any_element();
    };
    let ok = r.models.iter().map(|m| {
        div().child(format!(
            "{}: {} files in {}",
            m.input,
            m.files,
            m.out_dir.display()
        ))
    });
    let bad = r.failures.iter().map(|f| {
        div()
            .text_color(t.danger)
            .child(format!("Failed {}: {}", f.input, f.error))
    });
    div()
        .debug_selector(|| "pb-result".into())
        .flex()
        .flex_col()
        .gap_1()
        .child(widgets::section("Last run", t))
        .children(ok)
        .children(bad)
        .into_any_element()
}
