//! The selected item beside the grid: its numbers now, a chart of how
//! they moved, its linked folder and its version history.

mod chart;
mod history;
mod stats;
mod when;

pub use when::selected;

use gpui::{div, prelude::*, px, AnyElement, Context};
use prybar_workshop::ListedItem;

use super::super::super::widgets::Tipped as _;
use super::super::super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

const ITEM_URL: &str = "https://steamcommunity.com/sharedfiles/filedetails/?id=";

pub fn view(s: &AppState, it: &ListedItem, t: Tokens, cx: &Context<Root>) -> AnyElement {
    div()
        .debug_selector(|| "pb-details".into())
        .w(px(320.))
        .flex_none()
        .flex()
        .flex_col()
        .gap_3()
        .p_3()
        .rounded_lg()
        .border_1()
        .border_color(t.border)
        .bg(t.surface)
        .child(
            div()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .child(it.title.clone()),
        )
        .child(actions(it, t, cx))
        .child(stats::view(it, t))
        .child(chart::view(&s.tools.item.series, t))
        .child(folder(s, it, t, cx))
        .child(history::view(s.tools.item.history.get(&it.item_id), t))
        .into_any_element()
}

fn actions(it: &ListedItem, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let (edit_it, url) = (it.clone(), format!("{ITEM_URL}{}", it.item_id));
    let edit = widgets::button("pb-det-edit", "Edit", Kind::Primary, true, t)
        .debug_selector(|| "pb-det-edit".into())
        .tip("Change the title, text, tags, picture or content", t)
        .on_click(
            cx.listener(move |this, _, w, cx| this.open_publish(Some(edit_it.clone()), w, cx)),
        );
    let open = widgets::button("pb-det-open", "Open in Steam", Kind::Secondary, true, t)
        .tip("Open the item's Workshop page in Steam", t)
        .on_click(cx.listener(move |_, _, _, cx| cx.open_url(&url)));
    div().flex().gap_2().child(edit).child(open)
}

/// The addon folder the item is built from: link one, then send it.
fn folder(s: &AppState, it: &ListedItem, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let linked = s.tools.item.links.0.get(&it.item_id).cloned();
    let id = it.item_id;
    let pick = widgets::button("pb-det-link", "Link folder…", Kind::Secondary, true, t)
        .tip("Pick the local addon folder this item is built from", t)
        .on_click(cx.listener(move |this, _, _, cx| this.pick_link(id, cx)));
    let ready = linked.is_some() && !s.tools.publish.running;
    let (item, dir) = (it.clone(), linked.clone());
    let send = widgets::button("pb-det-update", "Update content", Kind::Secondary, ready, t)
        .debug_selector(|| "pb-det-update".into())
        .tip(
            "Pack the linked folder and send it to Steam; nothing else changes",
            t,
        )
        .when(ready, |el| {
            el.on_click(cx.listener(move |this, _, _, cx| {
                let (item, dir) = (item.clone(), dir.clone());
                this.state
                    .update(cx, |s, cx| dir.map(|d| s.update_content(&item, d, cx)));
            }))
        });
    let text = linked.map_or("No folder linked.".to_owned(), |d| d.display().to_string());
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(widgets::section("Addon folder", t).pt_0())
        .child(div().text_xs().text_color(t.muted).truncate().child(text))
        .child(div().flex().gap_2().child(pick).child(send))
}
