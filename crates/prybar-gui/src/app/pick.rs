//! "Choose…" buttons: a platform file dialog that fills a text field.

use gpui::{Context, Entity, PathPromptOptions};

use super::root::Root;
use crate::designs::input::TextInput;

impl Root {
    /// A cancel or a dialog error leaves the field as it was.
    pub fn pick_into(&mut self, input: Entity<TextInput>, folder: bool, cx: &mut Context<Self>) {
        let options = PathPromptOptions {
            files: !folder,
            directories: folder,
            multiple: false,
            prompt: Some("Choose".into()),
        };
        let answer = cx.prompt_for_paths(options);
        cx.spawn(async move |_, cx| {
            if let Ok(Ok(Some(paths))) = answer.await {
                if let Some(path) = paths.into_iter().next() {
                    let text = path.display().to_string();
                    input.update(cx, |i, cx| i.set_text(&text, cx));
                }
            }
        })
        .detach();
    }
}

impl Root {
    /// Links a picked folder to the item; a cancel changes nothing.
    pub fn pick_link(&mut self, id: u64, cx: &mut Context<Self>) {
        let answer = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Link".into()),
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = answer.await else {
                return;
            };
            let Some(dir) = paths.into_iter().next() else {
                return;
            };
            let _ = this.update(cx, |r, cx| {
                r.state.update(cx, |s, cx| s.link_folder(id, dir, cx))
            });
        })
        .detach();
    }
}
