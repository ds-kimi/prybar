//! Parser tests: real descriptions, broken markup and hostile input.

mod cases;
mod hostile;
mod plain;

use super::Node;
use super::Tag;

fn text(s: &str) -> Node {
    Node::Text(s.into())
}

fn tag(t: Tag, value: Option<&str>, kids: Vec<Node>) -> Node {
    Node::Tag(t, value.map(Into::into), kids)
}

/// How many tags deep the tree goes.
fn depth(n: &[Node]) -> usize {
    n.iter()
        .map(|n| match n {
            Node::Tag(_, _, k) => 1 + depth(k),
            _ => 0,
        })
        .max()
        .unwrap_or(0)
}
