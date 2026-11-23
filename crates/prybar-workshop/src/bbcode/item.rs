//! List items, the one tag Steam closes by itself.

use super::parse::MAX_DEPTH;
use super::tree::{Frame, Tree};
use super::{Node, Tag};

impl Tree {
    /// `[*]` ends the item before it and starts one, only inside a list.
    pub(super) fn item(&mut self, raw: &str) {
        if self.top() == Some(Tag::Item) {
            self.pop(true);
        }
        if !matches!(self.top(), Some(Tag::List | Tag::Olist)) || self.stack.len() > MAX_DEPTH {
            return self.push(Node::Text(raw.to_owned()));
        }
        self.stack.push(Frame {
            tag: Some(Tag::Item),
            value: None,
            raw: raw.to_owned(),
            kids: Vec::new(),
        });
    }

    pub(super) fn top(&self) -> Option<Tag> {
        self.stack.last().and_then(|f| f.tag)
    }
}
