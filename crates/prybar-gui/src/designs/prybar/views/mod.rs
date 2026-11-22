//! The area under the bar: one view per screen. This first cut gives
//! each screen its title, purpose and main action; the full screens
//! follow on the same frame.

use gpui::{div, prelude::*, AnyElement, Context, FontWeight};

use super::motion;
use super::widgets::{self, Kind};
use crate::actions::Open;
use crate::app::Root;
use crate::state::{AppState, Screen};
use crate::theme::prybar::Tokens;

pub fn screen(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let ix = Screen::ALL.iter().position(|x| *x == s.screen).unwrap_or(0);
    let title = div()
        .text_base()
        .font_weight(FontWeight::SEMIBOLD)
        .child(s.screen.label());
    let body = div()
        .id("pb-view")
        .debug_selector(|| "pb-view".into())
        .size_full()
        .px_6()
        .py_5()
        .flex()
        .flex_col()
        .gap_3()
        .overflow_y_scroll()
        .child(title)
        .child(div().text_color(t.muted).child(purpose(s.screen)))
        .child(action(s, t, cx));
    motion::enter(div().flex_1().min_h_0().child(body), ("pb-view-in", ix))
}

fn purpose(screen: Screen) -> &'static str {
    match screen {
        Screen::Home => "Open or drop a model (.mdl) or a package (VPK, GMA, APK, HFS).",
        Screen::Inspector => "Version, bones, meshes, sequences and materials of the model.",
        Screen::Package => "Browse, search and extract the files of a package.",
        Screen::Decompile => "Turn the model back into QC and SMD files.",
        Screen::Compile => "Build a QC with the game's studiomdl.",
        Screen::Publish => "Upload an addon to the Steam Workshop.",
        Screen::Settings => "Games prybar knows and where their tools live.",
        Screen::ModelPreview => "Look at the model in 3D.",
        Screen::TexturePreview => "Look at the model's textures.",
    }
}

fn action(s: &AppState, t: Tokens, cx: &Context<Root>) -> AnyElement {
    match s.screen {
        Screen::Home => widgets::button("pb-open", "Open file", Kind::Primary, true, t)
            .debug_selector(|| "pb-open".into())
            .on_click(cx.listener(|_, _, window, cx| window.dispatch_action(Box::new(Open), cx)))
            .into_any_element(),
        Screen::Decompile => widgets::switch("pb-split", "One .qci per QC section", s.split_qc, t)
            .debug_selector(|| "pb-split".into())
            .on_click(cx.listener(|this, _, _, cx| {
                this.state.update(cx, |s, cx| {
                    s.split_qc = !s.split_qc;
                    cx.notify();
                })
            }))
            .into_any_element(),
        Screen::Settings => {
            widgets::field("pb-config", "Games config", config_path(), t).into_any_element()
        }
        _ => div().into_any_element(),
    }
}

fn config_path() -> String {
    prybar_tools::config::config_path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "No config folder on this system".into())
}
