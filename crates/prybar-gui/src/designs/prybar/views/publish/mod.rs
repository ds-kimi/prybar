//! Workshop publish form: new item or update by id, then progress and
//! the item's link.

mod result;

use gpui::{div, prelude::*, px, AnyElement, Context, Entity};

use super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::designs::input::TextInput;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

pub fn view(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> AnyElement {
    div()
        .max_w(px(520.))
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("Publish"))
        .child(
            div()
                .text_color(t.muted)
                .child("Upload a Garry's Mod addon to the Steam Workshop. Dry run checks it without uploading."),
        )
        .child(widgets::field("Item id", &f.publish.item_id, t))
        .child(widgets::field("Title", &f.publish.title, t).debug_selector(|| "pb-pub-title".into()))
        .child(widgets::field("Description", &f.publish.description, t))
        .child(widgets::field("Tags", &f.publish.tags, t))
        .child(widgets::field("Visibility", &f.publish.visibility, t))
        .child(picked("Addon (.gma)", "pb-pub-content", &f.publish.content, t, cx))
        .child(picked("Preview image", "pb-pub-preview", &f.publish.preview, t, cx))
        .child(widgets::field("Change note", &f.publish.change_note, t))
        .child(buttons(s, f, t, cx))
        .child(result::view(s, t, cx))
        .into_any_element()
}

/// A path field with a Choose… button that fills it.
fn picked(
    label: &'static str,
    id: &'static str,
    input: &Entity<TextInput>,
    t: Tokens,
    cx: &Context<Root>,
) -> impl IntoElement {
    let target = input.clone();
    let choose = widgets::button(id, "Choose…", Kind::Secondary, true, t)
        .on_click(cx.listener(move |this, _, _, cx| this.pick_into(target.clone(), false, cx)));
    div()
        .flex()
        .items_end()
        .gap_2()
        .child(widgets::field(label, input, t).flex_1())
        .child(choose)
}

/// Both wait for a clean form and for the run in flight.
fn buttons(s: &AppState, f: &Forms, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let clean = f.publish.all().iter().all(|i| i.read(cx).error().is_none());
    let ready = clean && !s.tools.publish.running;
    let go = |id: &'static str, label: &'static str, kind, dry: bool| {
        widgets::button(id, label, kind, ready, t)
            .debug_selector(move || id.into())
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
