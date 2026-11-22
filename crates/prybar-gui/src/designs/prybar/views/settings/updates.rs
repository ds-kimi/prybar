//! Settings › Updates: version, last check, "Check now", the auto-update
//! switch, release notes, and the way out when installing fails.

use gpui::{div, prelude::*, AnyElement, Context};

use super::super::super::widgets::{self, Kind};
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;
use crate::update::{Phase, VERSION};

use super::status::{auto_switch, status};

pub fn section(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(widgets::section("Updates", t))
        .child(widgets::readonly("Version", format!("v{VERSION}"), t))
        .child(widgets::readonly("Status", status(&s.updates), t))
        .child(controls(s, t, cx))
        .children(failure(s, t, cx))
        .children(notes(s, t))
        .into_any_element()
}

/// "Check now" above the auto-update switch.
fn controls(s: &AppState, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let checking = s.updates.phase == Phase::Checking;
    let check = widgets::button(
        "pb-check-update",
        if checking { "Checking…" } else { "Check now" },
        Kind::Secondary,
        !checking,
        t,
    )
    .debug_selector(|| "pb-check-update".into())
    .when(!checking, |el| {
        el.on_click(
            cx.listener(|this, _, _, cx| this.state.update(cx, |s, cx| s.check_updates(true, cx))),
        )
    });
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().flex().child(check))
        .child(auto_switch(s, t, cx))
}

/// The exact reason plus a link to the release page, so a failed install
/// never leaves the user without a way to update.
fn failure(s: &AppState, t: Tokens, cx: &Context<Root>) -> Option<AnyElement> {
    let Phase::Failed(why) = &s.updates.phase else {
        return None;
    };
    let link = s.updates.link();
    let open = widgets::button(
        "pb-update-link",
        "Open download page",
        Kind::Secondary,
        true,
        t,
    )
    .debug_selector(|| "pb-update-link".into())
    .on_click(cx.listener(move |_, _, _, cx| cx.open_url(&link)));
    let el = div()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().text_sm().text_color(t.danger).child(why.clone()))
        .child(div().flex().child(open));
    Some(el.into_any_element())
}

fn notes(s: &AppState, t: Tokens) -> Option<AnyElement> {
    let r = s.updates.release.as_ref()?;
    let text = if r.notes.trim().is_empty() {
        "No release notes.".to_string()
    } else {
        r.notes.trim().to_string()
    };
    let el = div()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_xs()
                .text_color(t.muted)
                .child(format!("What's new in v{}", r.version)),
        )
        .child(div().text_sm().text_color(t.text).child(text));
    Some(el.into_any_element())
}
