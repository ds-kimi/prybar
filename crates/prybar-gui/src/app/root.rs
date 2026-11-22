//! The window's root view: draws the design and owns the window-wide
//! handlers.

use gpui::{
    div, prelude::*, App, Context, Entity, ExternalPaths, FocusHandle, Focusable, Subscription,
    Window,
};

use crate::state::AppState;
use crate::theme::prybar::tokens;

pub struct Root {
    pub state: Entity<AppState>,
    focus: FocusHandle,
    _observe: Subscription,
    _refocus: Subscription,
}

impl Root {
    /// Takes focus at once so key bindings work before any click, and
    /// takes it back whenever it is lost.
    pub fn new(state: Entity<AppState>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus = cx.focus_handle();
        focus.focus(window, cx);
        let _observe = cx.observe(&state, |_, _, cx| cx.notify());
        // A clicked row that a screen change removes takes focus with it;
        // without this, keys would reach nothing until the next click.
        let _refocus = cx.on_focus_lost(window, |this, window, cx| this.focus.focus(window, cx));
        Root {
            state,
            focus,
            _observe,
            _refocus,
        }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = tokens(self.state.read(cx).light);
        let root = div()
            .id("root")
            .track_focus(&self.focus)
            .size_full()
            .bg(t.bg)
            .text_color(t.text)
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                this.open_paths(paths.paths().to_vec(), cx)
            }))
            .drag_over::<ExternalPaths>(move |s, _, _, _| s.border_2().border_color(t.rust));
        Self::ui_actions(Self::navigation_actions(root, cx), cx)
            .child(crate::designs::render(self.state.clone(), cx))
    }
}

impl Focusable for Root {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}
