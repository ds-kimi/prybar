//! Reading a description into nodes with a stack of open tags.

use super::lex::{next, Tok};
use super::tree::{Frame, Tree};
use super::{Node, Tag};

/// Deeper nesting than this is text, so hostile input cannot make the
/// tree, or the views drawn from it, recurse without end.
pub(super) const MAX_DEPTH: usize = 24;

/// The description as nodes. Never fails: what is not valid markup
/// stays as the text it was.
pub fn parse(s: &str) -> Vec<Node> {
    let mut tree = Tree::new();
    let mut at = 0;
    while let Some((tok, end)) = next(s, at) {
        let raw = s.get(at..end).unwrap_or("");
        at = end;
        match tok {
            Tok::Text(t) => tree.push(Node::Text(t.to_owned())),
            Tok::Close(tag) => tree.close(tag, raw),
            Tok::Open(tag, value) => at = tree.open(s, at, (tag, value), raw),
        }
    }
    while tree.stack.len() > 1 {
        tree.pop(false);
    }
    tree.stack.pop().map(|f| f.kids).unwrap_or_default()
}

impl Tree {
    /// Opens a tag; returns where reading goes on, which is past the
    /// closing tag for tags that hold raw text.
    fn open(&mut self, s: &str, at: usize, open: (Tag, Option<String>), raw: &str) -> usize {
        match open.0 {
            Tag::Rule => self.push(Node::Rule),
            Tag::Item => self.item(raw),
            Tag::Code | Tag::Noparse => return self.verbatim(s, at, open, raw),
            _ if self.stack.len() > MAX_DEPTH => self.push(Node::Text(raw.to_owned())),
            tag => self.stack.push(Frame {
                tag: Some(tag),
                value: open.1,
                raw: raw.to_owned(),
                kids: Vec::new(),
            }),
        }
        at
    }

    /// Closes the nearest open tag of this kind; tags opened inside it
    /// and left open turn back into text. A closing tag with nothing to
    /// close is text.
    fn close(&mut self, tag: Tag, raw: &str) {
        let Some(at) = self.stack.iter().rposition(|f| f.tag == Some(tag)) else {
            return self.push(Node::Text(raw.to_owned()));
        };
        while self.stack.len() > at + 1 {
            self.pop(false);
        }
        self.pop(true);
    }
}
