//! Upload progress as Steam reports it, then the outcome and the link.

use gpui::{div, prelude::*, px, relative, AnyElement, Context, Div};
use prybar_workshop::PublishReport;

use super::super::super::widgets::Tipped as _;
use super::super::super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;

/// Steam shows every item here; the agreement page unhides new ones.
const ITEM_URL: &str = "https://steamcommunity.com/sharedfiles/filedetails/?id=";
const AGREEMENT_URL: &str = "https://steamcommunity.com/sharedfiles/workshoplegalagreement";

pub fn view(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let run = &s.tools.publish;
    let body = div()
        .debug_selector(|| "pb-pub-result".into())
        .flex()
        .flex_col()
        .gap_2();
    if run.running {
        return body.child(progress(s, t)).into_any_element();
    }
    match &run.outcome {
        None => body.into_any_element(),
        Some(Err(why)) => body
            .text_color(t.danger)
            .child(why.clone())
            .into_any_element(),
        Some(Ok(r)) if r.dry_run => {
            let what = if r.created {
                "a new item"
            } else {
                "the update"
            };
            body.child(format!("Dry run passed: {what} is ready to upload."))
                .into_any_element()
        }
        Some(Ok(r)) => published(body, r, t, cx),
    }
}

/// The new item's id, its link, and the agreement note when Steam
/// keeps the item hidden.
fn published(body: Div, r: &PublishReport, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let Some(id) = r.item_id else {
        return body.child("Published.").into_any_element();
    };
    let url = format!("{ITEM_URL}{id}");
    let open = widgets::button("pb-pub-open", "Open in Steam", Kind::Secondary, true, t)
        .tip("Open the item's Workshop page in Steam", t)
        .on_click(cx.listener(move |_, _, _, cx| cx.open_url(&url)));
    let note =
        format!("It stays hidden until you accept the Workshop agreement once: {AGREEMENT_URL}");
    body.child(format!("Published item {id}."))
        .when(r.needs_legal_agreement, |el| {
            el.child(div().text_color(t.rust).child(note))
        })
        .child(div().flex().child(open))
        .into_any_element()
}

/// A bar once Steam knows the size, else the stage alone.
fn progress(s: &AppState, t: Tokens) -> impl IntoElement {
    let p = s.tools.publish.progress.as_ref();
    let stage = p.map_or_else(|| "Starting…".to_owned(), |p| p.stage.clone());
    let frac = p
        .filter(|p| p.total > 0)
        .map(|p| (p.done as f32 / p.total as f32).clamp(0., 1.));
    div()
        .flex()
        .flex_col()
        .gap_1()
        .child(div().text_color(t.muted).child(stage))
        .when_some(frac, |el, frac| {
            el.child(
                div()
                    .h(px(6.))
                    .w_full()
                    .rounded_full()
                    .bg(t.hover)
                    .child(div().h_full().w(relative(frac)).rounded_full().bg(t.rust)),
            )
        })
}
