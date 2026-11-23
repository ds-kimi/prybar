//! The description editor: a Steam BBCode toolbar over a multi-line
//! field, and a preview drawn close to how the Workshop shows it.

mod ask;
mod images;
pub mod render;
mod tools;
mod view;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use gpui::{AppContext as _, Context, Entity, Subscription};

use super::input::{InputEvent, TextInput};
pub use images::Shot;
pub use tools::{Ask, Tool};

/// Downloads an image link into the cache and gives its file.
pub type Fetch = Arc<dyn Fn(&str) -> Result<PathBuf, String> + Send + Sync>;

pub struct BbEditor {
    pub input: Entity<TextInput>,
    /// The small prompt's field, for link, image and quote values.
    pub value: Entity<TextInput>,
    pub asking: Option<Ask>,
    pub preview: bool,
    shots: HashMap<String, Shot>,
    fetch: Fetch,
    /// Focus moves on the next draw, where a window is at hand.
    focus_value: bool,
    focus_input: bool,
    _subs: [Subscription; 2],
}

impl BbEditor {
    pub fn new(input: Entity<TextInput>, fetch: Fetch, cx: &mut Context<Self>) -> Self {
        let value = cx.new(|cx| TextInput::new(tools::value_spec(), cx));
        let keys = cx.subscribe(&input, |this, _, ev: &InputEvent, cx| {
            if let InputEvent::Format(key) = ev {
                this.shortcut(key, cx);
            }
        });
        let submit = cx.subscribe(&value, |this, _, ev: &InputEvent, cx| {
            if matches!(ev, InputEvent::Submit) {
                this.submit(cx);
            }
        });
        BbEditor {
            input,
            value,
            asking: None,
            preview: false,
            shots: HashMap::new(),
            fetch,
            focus_value: false,
            focus_input: false,
            _subs: [keys, submit],
        }
    }

    /// Where preview images come from; tests swap in a local one.
    #[cfg(test)]
    pub fn set_fetch(&mut self, fetch: Fetch) {
        self.fetch = fetch;
        self.shots.clear();
    }

    /// Mod-b, i and u wrap at once; mod-k asks for the link.
    fn shortcut(&mut self, key: &str, cx: &mut Context<Self>) {
        let tag = match key {
            "b" => "b",
            "i" => "i",
            "u" => "u",
            _ => return self.run(Tool::Ask(Ask::Link), cx),
        };
        let (open, close) = (format!("[{tag}]"), format!("[/{tag}]"));
        self.input.update(cx, |i, cx| i.surround(&open, &close, cx));
    }

    /// A toolbar button: tags go around the selection, or a prompt opens
    /// for the value the tag needs.
    pub fn run(&mut self, tool: Tool, cx: &mut Context<Self>) {
        match tool {
            Tool::Wrap(open, close) => {
                self.input.update(cx, |i, cx| i.surround(open, close, cx));
                self.focus_input = true;
            }
            Tool::Ask(ask) => self.ask(ask, cx),
        }
        cx.notify();
    }
}
