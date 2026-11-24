//! Where the QCs come from: one file or a folder, and whether the
//! folder's subfolders count.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::super::widgets::Tipped as _;
use super::super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let choose = widgets::button("pb-qc-pick", "Choose…", Kind::Secondary, true, t)
        .tip("Pick the .qc file to compile", t)
        .on_click(cx.listener(|this, _, _, cx| this.pick_into(this.forms.qc.clone(), false, cx)));
    let folder = widgets::button("pb-qc-folder", "Folder…", Kind::Secondary, true, t)
        .debug_selector(|| "pb-qc-folder".into())
        .tip(
            "Pick a folder: every .qc in it compiles, a few at a time",
            t,
        )
        .on_click(cx.listener(|this, _, _, cx| this.pick_into(this.forms.qc.clone(), true, cx)));
    let field = widgets::field("QC file or folder", &f.qc, t)
        .flex_1()
        .debug_selector(|| "pb-qc".into());
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .items_end()
                .gap_2()
                .child(field)
                .child(choose)
                .child(folder),
        )
        .child(subfolders(s, t, cx))
        .into_any_element()
}

fn subfolders(s: &AppState, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let on = !s.batch.compile_shallow;
    widgets::switch("pb-qc-recursive", "Include subfolders", on, t)
        .debug_selector(|| "pb-qc-recursive".into())
        .tip("For a folder, also compile the QCs in folders inside it", t)
        .on_click(cx.listener(|this, _, _, cx| {
            this.state.update(cx, |s, cx| {
                s.batch.compile_shallow = !s.batch.compile_shallow;
                cx.notify();
            })
        }))
}
