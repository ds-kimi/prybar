//! Workshop publish form: new item or update by id, then progress and
//! the item's link.

mod addon;
mod fields;
mod items;
mod result;
mod steam;

use gpui::{div, prelude::*, px, AnyElement, Context};

use super::super::widgets::Tipped as _;
use super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    div()
        .max_w(px(880.))
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("Publish"))
        .child(
            div()
                .text_color(t.muted)
                .child("Upload a Garry's Mod addon folder or .gma to the Steam Workshop. Dry run checks it without uploading."),
        )
        .child(steam::view(s, t, cx))
        .child(items::view(s, f, t, cx))
        .child(form(s, f, t, cx))
        .into_any_element()
}

/// The fields, kept to reading width under the wider item grid.
fn form(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    div()
        .max_w(px(520.))
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::section("Item", t))
        .child(widgets::field("Item id", &f.publish.item_id, t))
        .child(
            widgets::field("Title", &f.publish.title, t).debug_selector(|| "pb-pub-title".into()),
        )
        .child(widgets::field("Description", &f.publish.description, t))
        .child(widgets::field("Tags", &f.publish.tags, t))
        .child(fields::visibility(f, t))
        .child(fields::files(s, f, t, cx))
        .child(widgets::field("Change note", &f.publish.change_note, t))
        .child(buttons(s, f, t, cx))
        .child(result::view(s, t, cx))
}

/// Both wait for a clean form and for the run in flight.
fn buttons(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let clean = f.publish.all().iter().all(|i| i.read(cx).error().is_none());
    let ready = clean && !s.tools.publish.running;
    let go = |id: &'static str, label: &'static str, kind, dry: bool| {
        widgets::button(id, label, kind, ready, t)
            .debug_selector(move || id.into())
            .tip(
                if dry {
                    "Check everything without uploading"
                } else {
                    "Upload to the Steam Workshop"
                },
                t,
            )
            .when(ready, |el| {
                el.on_click(cx.listener(move |this, _, _, cx| {
                    let form = this.forms.publish.form(cx);
                    this.state.update(cx, |s, cx| s.publish(form, dry, cx))
                }))
            })
    };
    div()
        .flex()
        .gap_2()
        .child(go("pb-pub-dry", "Dry run", Kind::Secondary, true))
        .child(go("pb-pub-go", "Publish", Kind::Primary, false))
}
