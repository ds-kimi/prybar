//! Action handlers for opening files, going back and search.

use std::path::PathBuf;

use gpui::{
    Context, Div, Focusable as _, InteractiveElement as _, PathPromptOptions, Stateful, Window,
};

use super::root::Root;
use crate::actions::{Back, Open, Search};

impl Root {
    pub fn open_paths(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        self.state.update(cx, |state, cx| {
            for path in paths {
                state.open(path, cx);
            }
        });
    }

    /// The platform dialog answers later; a cancel or a dialog error
    /// leaves things as they were.
    fn prompt_open(&mut self, cx: &mut Context<Self>) {
        let options = PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: Some("Open".into()),
        };
        let answer = cx.prompt_for_paths(options);
        cx.spawn(async move |this, cx| {
            if let Ok(Ok(Some(paths))) = answer.await {
                let _ = this.update(cx, |this, cx| this.open_paths(paths, cx));
            }
        })
        .detach();
    }

    pub(super) fn navigation_actions(el: Stateful<Div>, cx: &mut Context<Self>) -> Stateful<Div> {
        el.on_action(cx.listener(|this, _: &Open, _, cx| this.prompt_open(cx)))
            .on_action(cx.listener(|this, _: &Back, window, cx| {
                if this.state.read(cx).search.is_some() {
                    this.close_search(window, cx);
                }
                this.state.update(cx, |s, cx| s.back(cx))
            }))
            .on_action(cx.listener(|this, _: &Search, window, cx| {
                if this.state.read(cx).search.is_some() {
                    return this.close_search(window, cx);
                }
                this.state.update(cx, |s, cx| {
                    s.search = Some(String::new());
                    cx.notify();
                });
                this.forms.search.focus_handle(cx).focus(window, cx);
            }))
    }

    /// Search closes empty and hands keys back to the window.
    fn close_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.forms.search.update(cx, |i, cx| i.set_text("", cx));
        self.state.update(cx, |s, cx| {
            s.search = None;
            cx.notify();
        });
        self.focus.focus(window, cx);
    }
}
