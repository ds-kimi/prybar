//! Action handlers for opening files, going back and search.

use std::path::PathBuf;

use gpui::{
    Context, Div, Focusable as _, InteractiveElement as _, PathPromptOptions, Stateful, Window,
};

use super::root::Root;
use crate::actions::{Back, Open, Search};
use crate::state::Screen;

impl Root {
    /// Files open; a folder becomes the batch input of the compile
    /// screen when it is showing, else of the decompile screen. A `.qc`
    /// dropped on the compile screen fills its field.
    pub fn open_paths(&mut self, paths: Vec<PathBuf>, cx: &mut Context<Self>) {
        let compiling = self.state.read(cx).screen == Screen::Compile;
        for path in paths {
            let qc = path
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("qc"));
            if compiling && (qc || path.is_dir()) {
                let text = path.display().to_string();
                self.forms.qc.update(cx, |i, cx| i.set_text(&text, cx));
            } else if path.is_dir() {
                self.state.update(cx, |s, cx| {
                    s.batch.folder = Some(path);
                    s.navigate(Screen::Decompile, cx);
                });
            } else {
                self.state.update(cx, |s, cx| s.open(path, cx));
            }
        }
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
