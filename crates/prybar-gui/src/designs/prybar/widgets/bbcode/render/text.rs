//! Small questions about nodes the drawing asks.

use prybar_workshop::bbcode::{Node, Tag};

/// All the text under nodes, tags dropped: a link's address, an image's.
pub fn text_of(nodes: &[Node]) -> String {
    let mut out = String::new();
    for n in nodes {
        match n {
            Node::Text(s) => out.push_str(s),
            Node::Tag(_, _, kids) => out.push_str(&text_of(kids)),
            Node::Rule => {}
        }
    }
    out
}

/// Whether a node flows inside a paragraph. A link around an image or
/// a block is drawn as a clickable block instead.
pub fn is_inline(n: &Node) -> bool {
    match n {
        Node::Text(_) => true,
        Node::Rule => false,
        Node::Tag(Tag::Url, _, kids) => kids.iter().all(is_inline),
        Node::Tag(tag, _, kids) => {
            matches!(
                tag,
                Tag::Bold | Tag::Italic | Tag::Underline | Tag::Strike | Tag::Noparse
            ) && kids.iter().all(is_inline)
        }
    }
}
