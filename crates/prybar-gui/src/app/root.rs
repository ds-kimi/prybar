//! The window's root view: draws the design and owns the window-wide
//! handlers.

use gpui::{
    div, prelude::*, App, Context, Entity, ExternalPaths, FocusHandle, Focusable, Subscription,
    Window,
};

use super::Forms;
use crate::designs::input::InputEvent;
use crate::designs::modal::Modal;
use crate::state::{AppState, Screen};
use crate::theme::prybar::{tokens, Light};
use crate::viewport::view::Full;

pub struct Root {
    pub state: Entity<AppState>,
    pub forms: Forms,
    pub(super) focus: FocusHandle,
    /// The open popup and the subscription that drops it when closed.
    pub(super) modal: Option<(Entity<Modal>, Subscription)>,
    _subs: Vec<Subscription>,
}

impl Root {
    /// Takes focus at once so key bindings work before any click, and
    /// takes it back whenever it is lost.
    pub fn new(state: Entity<AppState>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus = cx.focus_handle();
        focus.focus(window, cx);
        let _observe = cx.observe(&state, |this: &mut Self, state, cx| {
            this.forms.follow(&state, cx);
            cx.notify()
        });
        // A clicked row that a screen change removes takes focus with it;
        // without this, keys would reach nothing until the next click.
        let _refocus = cx.on_focus_lost(window, |this, window, cx| this.focus.focus(window, cx));
        let forms = Forms::new(cx);
        let _search = cx.subscribe(&forms.search, |this, input, _: &InputEvent, cx| {
            let query = input.read(cx).text().to_owned();
            this.state.update(cx, |s, cx| {
                s.search = Some(query);
                cx.notify();
            })
        });
        let mut _subs = forms.publish.watch(&state, cx);
        _subs.extend([_observe, _refocus, _search, full_view(&forms, cx)]);
        // The bar's account badge needs Steam's answer on every screen.
        state.update(cx, |s, cx| s.refresh_steam(cx));
        Root {
            state,
            forms,
            focus,
            modal: None,
            _subs,
        }
    }
}

impl Render for Root {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let light = self.state.read(cx).light;
        cx.set_global(Light(light));
        let t = tokens(light);
        let root = div()
            .id("root")
            .track_focus(&self.focus)
            .size_full()
            .relative()
            .bg(t.bg)
            .text_color(t.text)
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _, cx| {
                this.open_paths(paths.paths().to_vec(), cx)
            }))
            .drag_over::<ExternalPaths>(move |s, _, _, _| s.border_2().border_color(t.rust));
        Self::ui_actions(Self::navigation_actions(root, cx), cx)
            .child(crate::designs::render(
                self.state.clone(),
                &self.forms.clone(),
                cx,
            ))
            .children(self.modal.as_ref().map(|m| m.0.clone()))
    }
}

impl Focusable for Root {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus.clone()
    }
}

/// The viewport asks for full size; the 3D preview screen is that mode,
/// so the same key goes back to the model screen.
fn full_view(forms: &Forms, cx: &mut Context<Root>) -> Subscription {
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
