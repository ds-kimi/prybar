//! First-launch analytics notice: a band under the bar, not a modal, so
//! the app stays usable. It lists exactly what is and is never sent, and
//! its two answers carry the same weight.

use gpui::{div, prelude::*, px, AnyElement, Context};

use super::super::widgets::Tipped as _;
use super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::AppState;
use crate::telemetry::PRIVACY_URL;
use crate::theme::prybar::Tokens;

const SENT: &str = "Sent: the app version, OS and CPU type each time the app starts, \
and crash reports with file paths, file names and user names removed.";
const NEVER: &str = "Never sent: your files or their names, paths, any ID for you or \
this computer. The collector does not store IP addresses.";

pub fn notice(s: &AppState, t: Tokens, cx: &Context<Root>) -> Option<AnyElement> {
    if s.privacy.notice_seen {
        return None;
    }
    let el = div()
        .id("pb-privacy-notice")
        .debug_selector(|| "pb-privacy-notice".into())
        .mx_6()
        .mt_3()
        .p_4()
        .max_w(px(760.))
        .flex()
        .flex_col()
        .gap_3()
        .rounded_lg()
        .border_1()
        .border_color(t.border)
        .bg(t.surface)
        .child(text(s, t))
        .child(answers(t, cx));
    Some(el.into_any_element())
}

fn text(s: &AppState, t: Tokens) -> impl IntoElement {
    let text = div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .child("Anonymous usage and crash reports are on"),
        )
        .child(div().text_color(t.muted).child(SENT))
        .child(div().text_color(t.muted).child(NEVER));
    let missing = "This build has no collector set up, so nothing is sent either way.";
    text.when(s.privacy.endpoint.is_empty(), |el| {
        el.child(div().text_color(t.muted).child(missing))
    })
}

const KEEP: &str = "Send anonymous crash reports and app starts, nothing else";
const OFF: &str = "Send nothing; you can change this in Settings";

fn answers(t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let answer = |id: &'static str, label: &'static str, keep: bool| {
        widgets::button(id, label, Kind::Secondary, true, t)
            .debug_selector(move || id.into())
            .tip(if keep { KEEP } else { OFF }, t)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.state.update(cx, |s, cx| s.answer_notice(keep, cx))
            }))
    };
    let details = widgets::button(
        "pb-privacy-details",
        "What exactly is sent",
        Kind::Secondary,
        true,
        t,
    )
    .tip("Open the privacy policy in the browser", t)
    .on_click(|_, _, cx| cx.open_url(PRIVACY_URL));
    div()
        .flex()
        .gap_2()
        .child(answer("pb-privacy-keep", "Keep on", true))
        .child(answer("pb-privacy-off", "Turn off", false))
        .child(details)
}
