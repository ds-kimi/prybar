//! The window's root view: picks the chooser or a design and owns the
//! handlers every design shares.

use gpui::{
    div, prelude::*, App, Context, Entity, ExternalPaths, FocusHandle, Focusable, Subscription,
    Window,
};

use crate::actions::CONSOLE;
use crate::state::{AppState, DesignKind};
use crate::theme;

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

    fn body(&self, cx: &mut Context<Self>) -> gpui::AnyElement {
        match self.state.read(cx).design {
            None => crate::chooser::render(cx).into_any_element(),
            Some(kind) => crate::designs::render(kind, self.state.clone(), cx),
        }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let palette = theme::current(self.state.read(cx));
        let console = self.state.read(cx).design == Some(DesignKind::Console);
        let root = div()
            .id("root")
            .when(console, |el| el.key_context(CONSOLE))
            .track_focus(&self.focus)
            .size_full()
            .bg(palette.bg)
            .text_color(palette.text)
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                this.open_paths(paths.paths().to_vec(), cx)
            }))
            .drag_over::<ExternalPaths>(move |s, _, _, _| {
                s.border_2().border_color(palette.accent)
            });
        let root = Self::ui_actions(Self::navigation_actions(root, cx), cx);
        let root = Self::console_actions(root, cx);
        Self::design_actions(root, cx).child(self.body(cx))
    }
}

impl Focusable for Root {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}
