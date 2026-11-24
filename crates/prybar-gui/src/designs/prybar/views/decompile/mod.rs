//! Decompile screen: a model, a package or a whole folder in, QC and
//! SMD files out, with the item list while it runs.

mod options;

use gpui::{div, prelude::*, px, AnyElement, Context};

use super::super::widgets::Tipped as _;
use super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::state::AppState;
use crate::theme::prybar::Tokens;

const NOTHING: &str = "Open a model or package, or choose a folder";

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let doc = s.selected.and_then(|i| s.documents.get(i));
    let input = match (&s.batch.folder, doc) {
        (Some(dir), _) => Some(dir.display().to_string()),
        (None, d) => d.map(|d| d.title()),
    };
    let ready = input.is_some() && f.out.read(cx).error().is_none();
    div()
        .flex()
        .flex_col()
        .gap_3()
        .max_w(px(560.))
        .child(widgets::heading("Decompile"))
        .child(div().text_color(t.muted).child(
            "Turn models back into QC and SMD files: one model, a package, or a folder (drop it here).",
        ))
        .child(source(s, input, t, cx))
        .child(widgets::field("Output folder", &f.out, t).debug_selector(|| "pb-out".into()))
        .child(options::view(s, t, cx))
        .child(div().flex().child(run(s, ready, t, cx)))
        .child(super::batch::view("pb-dec", &s.batch.decompile, t, cx))
        .child(warnings(s, t))
        .into_any_element()
}

/// What will be decompiled, a folder picker, and a way back to the open
/// file once a folder was chosen.
fn source(s: &AppState, input: Option<String>, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let label = if s.batch.folder.is_some() {
        "Input folder"
    } else {
        "Input"
    };
    let pick = widgets::button("pb-dec-pick", "Folder…", Kind::Secondary, true, t)
        .debug_selector(|| "pb-dec-pick".into())
        .tip("Decompile every model in a folder", t)
        .on_click(cx.listener(|this, _, _, cx| this.pick_batch_folder(cx)));
    let clear = s.batch.folder.is_some().then(|| {
        widgets::button("pb-dec-clear", "Clear", Kind::Secondary, true, t)
            .tip("Forget the folder and decompile the open file again", t)
            .on_click(cx.listener(|this, _, _, cx| {
                this.state
                    .update(cx, |s, cx| (s.batch.folder = None, cx.notify()).1)
            }))
    });
    let value = input.unwrap_or_else(|| NOTHING.into());
    div()
        .flex()
        .items_end()
        .gap_2()
        .child(widgets::readonly(label, value, t).flex_1().min_w_0())
        .child(pick)
        .children(clear)
        .into_any_element()
}

/// Decompile, or Cancel while a run is going.
fn run(s: &AppState, ready: bool, t: Tokens, cx: &Context<Root>) -> AnyElement {
    if s.batch.decompile.running {
        return widgets::button("pb-dec-cancel", "Cancel", Kind::Secondary, true, t)
            .debug_selector(|| "pb-dec-cancel".into())
            .tip(
                "Skip the models not started yet; the ones being written finish",
                t,
            )
            .on_click(cx.listener(|this, _, _, cx| this.state.update(cx, |s, _| s.cancel_batch())))
            .into_any_element();
    }
    widgets::button("pb-decompile", "Decompile", Kind::Primary, ready, t)
        .debug_selector(|| "pb-decompile".into())
        .tip(
            "Write the QC, SMD and animation files to the output folder",
            t,
        )
        .when(ready, |el| {
            el.on_click(cx.listener(|this, _, _, cx| {
                let out = this.forms.out.read(cx).text().trim().into();
                this.state.update(cx, |s, cx| s.decompile_selected(out, cx))
            }))
        })
        .into_any_element()
}

/// Renamed folders and `$includemodel`s not found: the output is usable
/// but not exactly what was asked.
fn warnings(s: &AppState, t: Tokens) -> AnyElement {
    let list = s.last_decompile.iter().flat_map(|r| r.warnings.iter());
    let rows: Vec<_> = list
        .map(|w| div().text_color(t.muted).child(w.clone()))
        .collect();
    if rows.is_empty() || s.batch.decompile.running {
        return div().into_any_element();
    }
    div()
        .debug_selector(|| "pb-result".into())
        .flex()
        .flex_col()
        .gap_1()
        .child(widgets::section("Warnings", t))
        .children(rows)
        .into_any_element()
}
