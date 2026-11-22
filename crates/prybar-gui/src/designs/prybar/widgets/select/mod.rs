//! A fixed-choice field: a button showing the current choice that opens
//! a short list. Arrow keys move, Enter picks, Escape closes and typing a
//! letter jumps to the next choice starting with it.

mod keys;
mod view;

use gpui::{App, AppContext as _, Context, Entity, FocusHandle, SharedString};

/// One entry: what it says and what it means, for its tooltip.
#[derive(Clone)]
pub struct Choice {
    pub label: SharedString,
    pub tip: SharedString,
}

pub struct Select {
    id: SharedString,
    choices: Vec<Choice>,
    current: usize,
    /// The highlighted entry while the list is open.
    hi: usize,
    open: bool,
    focus: FocusHandle,
}

impl Select {
    /// `choices` must not be empty; an empty list yields a select that
    /// shows nothing and never opens rather than a panic.
    pub fn new(id: &str, choices: Vec<Choice>, current: usize, cx: &mut App) -> Entity<Select> {
        let current = current.min(choices.len().saturating_sub(1));
        cx.new(|cx| Select {
            id: SharedString::from(id.to_owned()),
            choices,
            current,
            hi: current,
            open: false,
            focus: cx.focus_handle(),
        })
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn label(&self) -> SharedString {
        self.choices
            .get(self.current)
            .map(|c| c.label.clone())
            .unwrap_or_default()
    }

    /// Picks by index and closes the list; readers ask [`Select::current`].
    pub fn pick(&mut self, ix: usize, cx: &mut Context<Self>) {
        self.open = false;
        if ix < self.choices.len() {
            self.current = ix;
        }
        cx.notify();
    }
}
