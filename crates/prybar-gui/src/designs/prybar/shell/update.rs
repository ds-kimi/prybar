//! The green update button: shown only while there is good news to act
//! on, so a current install keeps a quiet bar.

use gpui::{div, prelude::*, AnyElement, Context, FontWeight, SharedString};

use super::super::widgets::{ring, Tipped};
use crate::app::Root;
use crate::state::{AppState, Screen};
use crate::theme::prybar::Tokens;
use crate::update::Phase;

/// Longest release notes shown in the hover card; Settings has them all.
const NOTES_PREVIEW: usize = 400;

pub fn button(s: &AppState, t: Tokens, cx: &Context<Root>) -> Option<AnyElement> {
    let label = label(s)?;
    let tip = tip_text(s);
    let el = div()
        .id("pb-update")
        .debug_selector(|| "pb-update".into())
        .aria_label(SharedString::from(label.clone()))
        .h_8()
        .px_3()
        .mr_1()
        .flex()
        .items_center()
        .rounded_md()
        .bg(t.success)
        .text_color(t.on_success)
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer()
        .hover(|st| st.opacity(0.88))
        .active(|st| st.opacity(0.74))
        .tip(tip, t)
        .on_click(cx.listener(|this, _, _, cx| this.state.update(cx, click)))
        .child(label);
    Some(ring(el, t).into_any_element())
}

fn label(s: &AppState) -> Option<String> {
    let version = s.updates.release.as_ref()?.version.to_string();
    Some(match &s.updates.phase {
        Phase::Available => format!("Update to v{version}"),
        Phase::Downloading(Some(p)) => format!("Downloading… {:.0}%", p * 100.),
        Phase::Downloading(None) => "Downloading…".into(),
        Phase::Ready => "Restart to update".into(),
        _ => return None,
    })
}

/// Ready restarts; anything else opens Settings › Updates, where the full
/// notes and progress are, and starts the download if it waits on a click.
fn click(s: &mut AppState, cx: &mut Context<AppState>) {
    match s.updates.phase {
        Phase::Ready => s.restart_to_update(cx),
        Phase::Available => {
            s.start_update(cx);
            s.navigate(Screen::Settings, cx);
        }
        _ => s.navigate(Screen::Settings, cx),
    }
}

fn tip_text(s: &AppState) -> SharedString {
    let notes = s.updates.release.as_ref().map_or("", |r| r.notes.trim());
    if notes.is_empty() {
        return "No release notes. Click for details.".into();
    }
    let mut cut: String = notes.chars().take(NOTES_PREVIEW).collect();
    if cut.len() < notes.len() {
        cut.push('…');
    }
    cut.into()
}
