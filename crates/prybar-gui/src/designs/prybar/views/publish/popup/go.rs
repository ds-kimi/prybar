use gpui::{div, prelude::*, Context};
use prybar_workshop::ListedItem;

use super::super::super::super::widgets::Tipped as _;
use super::super::super::super::widgets::{self, Kind};
use crate::app::{Forms, Root};
use crate::state::AppState;
use crate::theme::prybar::Tokens;

/// Both wait for a clean form and for the run in flight. An edit sends
/// only the fields that differ from the item as listed.
pub fn buttons(
    s: &AppState,
    f: &Forms,
    editing: bool,
    t: Tokens,
    cx: &Context<Root>,
) -> impl IntoElement {
    let clean = f.publish.all().iter().all(|i| i.read(cx).error().is_none());
    let ready = clean && !s.tools.publish.running;
    let main = if editing { SAVE } else { PUBLISH };
    div()
        .flex()
        .justify_end()
        .gap_2()
        .child(go(DRY, Kind::Secondary, true, ready, t, cx))
        .child(go(main, Kind::Primary, false, ready, t, cx))
}

type Label = (&'static str, &'static str, &'static str);
const DRY: Label = (
    "pb-pub-dry",
    "Dry run",
    "Check everything without uploading",
);
const SAVE: Label = (
    "pb-pub-go",
    "Save to Workshop",
    "Send only what you changed to Steam",
);
const PUBLISH: Label = (
    "pb-pub-go",
    "Publish",
    "Upload the new item to the Steam Workshop",
);

fn go(
    (id, label, tip): Label,
    kind: Kind,
    dry: bool,
    ready: bool,
    t: Tokens,
    cx: &Context<Root>,
) -> impl IntoElement {
    widgets::button(id, label, kind, ready, t)
        .debug_selector(move || id.into())
        .tip(tip, t)
        .when(ready, |el| {
            el.on_click(cx.listener(move |this, _, _, cx| {
                let mut form = this.forms.publish.form(cx);
                form.editing = this.editing.clone();
                this.state.update(cx, |s, cx| s.publish(form, dry, cx))
            }))
        })
}

/// The folder this item is built from, one click away; content sent
/// from a folder links it for next time.
pub fn link_hint(s: &AppState, it: &ListedItem, t: Tokens) -> impl IntoElement {
    let text = match s.tools.item.links.0.get(&it.item_id) {
        Some(dir) => format!(
            "Linked folder: {}. Pick it above to send new content.",
            dir.display()
        ),
        None => {
            "No folder linked yet. Content sent from a folder is linked to this item.".to_owned()
        }
    };
    div().text_xs().text_color(t.muted).child(text)
}
