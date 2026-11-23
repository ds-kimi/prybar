//! The tree under construction: a stack of open tags, each with what it
//! holds so far, and the ways a tag leaves the stack.

use super::{Node, Tag};

pub struct Frame {
    /// `None` for the root, which is never closed.
    pub tag: Option<Tag>,
    pub value: Option<String>,
    /// The opening tag as written, given back if it is never closed.
    pub raw: String,
    pub kids: Vec<Node>,
}

pub struct Tree {
    pub stack: Vec<Frame>,
}

impl Tree {
    pub fn new() -> Tree {
        let root = Frame {
            tag: None,
            value: None,
            raw: String::new(),
            kids: Vec::new(),
        };
        Tree { stack: vec![root] }
    }

    /// Adds a node to the innermost open tag; text joins text before it.
    pub fn push(&mut self, node: Node) {
        let Some(top) = self.stack.last_mut() else {
            return;
        };
        match (top.kids.last_mut(), node) {
            (Some(Node::Text(a)), Node::Text(b)) => a.push_str(&b),
            (_, node) => top.kids.push(node),
        }
    }

    /// Takes the innermost tag off the stack. Closed, it becomes a node;
    /// left open, its opening tag goes back as text before what it held.
    /// List items need no closing tag, so they always count as closed.
    pub fn pop(&mut self, closed: bool) {
        if self.stack.len() < 2 {
            return;
        }
        let Some(f) = self.stack.pop() else { return };
        let Some(tag) = f.tag else { return };
        if closed || tag == Tag::Item {
            return self.push(Node::Tag(tag, f.value, f.kids));
        }
        self.push(Node::Text(f.raw));
        for kid in f.kids {
            self.push(kid);
        }
    }

    /// `[code]` and `[noparse]` hold text as written, tags included, up
    /// to their own closing tag; without one the rest is plain text.
    pub fn verbatim(
        &mut self,
        s: &str,
        at: usize,
        open: (Tag, Option<String>),
        raw: &str,
    ) -> usize {
        let close = format!("[/{}]", open.0.name());
        let rest = s.get(at..).unwrap_or("");
        let found = rest.match_indices("[/").find(|(i, _)| {
            rest.get(*i..*i + close.len())
                .is_some_and(|c| c.eq_ignore_ascii_case(&close))
        });
        let Some((i, _)) = found else {
            self.push(Node::Text(format!("{raw}{rest}")));
            return s.len();
        };
        let kids = vec![Node::Text(rest[..i].to_owned())];
        self.push(Node::Tag(open.0, open.1, kids));
        at + i + close.len()
    }
}
