//! Window-wide subscriptions the root sets up once.

use gpui::{Context, Subscription};

use super::{Forms, Root};
use crate::designs::input::InputEvent;
use crate::state::Screen;
use crate::viewport::view::Full;

/// The file filter feeds the state as typed.
pub(super) fn search(forms: &Forms, cx: &mut Context<Root>) -> Subscription {
    cx.subscribe(&forms.search, |this, input, _: &InputEvent, cx| {
        let query = input.read(cx).text().to_owned();
        this.state.update(cx, |s, cx| {
            s.search = Some(query);
            cx.notify();
        })
    })
}

/// The viewport asks for full size; the 3D preview screen is that mode,
/// so the same key goes back to the model screen.
pub(super) fn full_view(forms: &Forms, cx: &mut Context<Root>) -> Subscription {
    cx.subscribe(&forms.viewport, |this, _, _: &Full, cx| {
        this.state.update(cx, |s, cx| {
            let to = match s.screen {
                Screen::ModelPreview => Screen::Inspector,
                _ => Screen::ModelPreview,
            };
            s.navigate(to, cx)
        })
    })
}
