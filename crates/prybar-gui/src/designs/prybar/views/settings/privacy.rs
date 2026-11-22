//! Settings › Privacy: the switch, where things stand, and the log of
//! every payload queued or sent.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::super::widgets::Tipped as _;
use super::super::super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::AppState;
use crate::telemetry::PRIVACY_URL;
use crate::theme::prybar::Tokens;

pub fn section(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let p = &s.privacy;
    let switch = widgets::switch(
        "pb-telemetry",
        "Send anonymous usage and crash reports",
        p.on,
        t,
    )
    .debug_selector(|| "pb-telemetry".into())
    .tip(
        "Send anonymous crash reports and app starts; every payload is logged below",
        t,
    )
    .on_click(cx.listener(|this, _, _, cx| {
        this.state
            .update(cx, |s, cx| s.set_telemetry(!s.privacy.on, cx))
    }));
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(widgets::section("Privacy", t))
        .child(switch)
        .child(widgets::readonly("Status", p.status(), t))
        .child(buttons(s, t, cx))
        .child(super::sent::table(s, t, cx))
        .into_any_element()
}

fn buttons(s: &AppState, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let open = open_log(s, t);
    let empty = s.privacy.log.is_empty();
    let clear = widgets::button(
        "pb-telemetry-clear",
        "Clear log",
        Kind::Secondary,
        !empty,
        t,
    )
    .debug_selector(|| "pb-telemetry-clear".into())
    .tip("Delete the local log of what was sent", t)
    .when(!empty, |el| {
        el.on_click(
            cx.listener(|this, _, _, cx| this.state.update(cx, |s, cx| s.clear_sent_log(cx))),
        )
    });
    let policy = widgets::button(
        "pb-telemetry-policy",
        "Privacy policy",
        Kind::Secondary,
        true,
        t,
    )
    .tip("Open the privacy policy in the browser", t)
    .on_click(|_, _, cx| cx.open_url(PRIVACY_URL));
    div().flex().gap_2().child(open).child(clear).child(policy)
}

fn open_log(s: &AppState, t: Tokens) -> impl IntoElement {
    let path = s.privacy.store.as_ref().map(|st| st.log_path());
    let has_log = path.as_ref().is_some_and(|p| p.exists());
    widgets::button(
        "pb-telemetry-open",
        "Open log file",
        Kind::Secondary,
        has_log,
        t,
    )
    .tip("Open the log of every payload sent, as plain JSON lines", t)
    .when_some(path.filter(|_| has_log), |el, p| {
        el.on_click(move |_, _, cx| cx.open_with_system(&p))
    })
}
