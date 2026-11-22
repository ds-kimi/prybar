//! Action handlers shared by the chooser and every design.

use std::path::PathBuf;

use gpui::{Context, Div, InteractiveElement as _, PathPromptOptions, Stateful};

use super::root::Root;
use crate::actions::{Back, Open, Search, ShowCanvas, ShowChooser, ShowConsole, ShowWorkbench};
use crate::state::DesignKind;

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
            .on_action(
                cx.listener(|this, _: &Back, _, cx| this.state.update(cx, |s, cx| s.back(cx))),
            )
            .on_action(cx.listener(|this, _: &Search, _, cx| {
                this.state.update(cx, |s, cx| {
                    s.search = if s.search.is_some() {
                        None
                    } else {
                        Some(String::new())
                    };
                    cx.notify();
                })
            }))
    }

    pub(super) fn design_actions(el: Stateful<Div>, cx: &mut Context<Self>) -> Stateful<Div> {
        let set = |design: Option<DesignKind>| {
            move |this: &mut Root, cx: &mut Context<Root>| {
                this.state.update(cx, |s, cx| s.set_design(design, cx))
            }
        };
        let (chooser, bench, canvas, console) = (
            set(None),
            set(Some(DesignKind::Workbench)),
            set(Some(DesignKind::Canvas)),
            set(Some(DesignKind::Console)),
        );
        el.on_action(cx.listener(move |this, _: &ShowChooser, _, cx| chooser(this, cx)))
            .on_action(cx.listener(move |this, _: &ShowWorkbench, _, cx| bench(this, cx)))
            .on_action(cx.listener(move |this, _: &ShowCanvas, _, cx| canvas(this, cx)))
            .on_action(cx.listener(move |this, _: &ShowConsole, _, cx| console(this, cx)))
    }
}
