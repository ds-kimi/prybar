//! The status line and auto-update switch of Settings › Updates.

use std::time::SystemTime;

use gpui::{prelude::*, Context};

use super::super::super::widgets;
use crate::app::Root;
use crate::state::AppState;
use crate::theme::prybar::Tokens;
use crate::update::{Phase, Updates};

/// One line that says where things stand and when it was last asked.
pub fn status(u: &Updates) -> String {
    let latest = u.release.as_ref().map(|r| format!("v{}", r.version));
    let what = match (&u.phase, latest) {
        (Phase::Idle, _) => "Not checked yet".to_string(),
        (Phase::Checking, _) => "Checking…".into(),
        (Phase::UpToDate, _) => "Up to date".into(),
        (Phase::Unknown(why), _) => format!("Could not check: {why}"),
        (Phase::Downloading(_), Some(v)) => format!("Downloading {v}"),
        (Phase::Ready, Some(v)) => format!("{v} is ready, restart to update"),
        (Phase::Failed(_), Some(v)) => format!("{v} could not be installed"),
        (_, v) => format!("{} is available", v.unwrap_or_default()),
    };
    let ago = u
        .checked_at
        .and_then(|at| SystemTime::now().duration_since(at).ok())
        .map(|d| match d.as_secs() / 60 {
            0 => " · checked just now".to_string(),
            m if m < 120 => format!(" · checked {m} min ago"),
            m => format!(" · checked {} h ago", m / 60),
        });
    what + &ago.unwrap_or_default()
}

pub fn auto_switch(s: &AppState, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    widgets::switch(
        "pb-auto-update",
        "Download updates automatically",
        s.updates.auto,
        t,
    )
    .debug_selector(|| "pb-auto-update".into())
    .on_click(cx.listener(|this, _, _, cx| {
        this.state
            .update(cx, |s, cx| s.set_auto_update(!s.updates.auto, cx))
    }))
}
