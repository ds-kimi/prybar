//! A description as plain text, for agents and terminals that cannot
//! draw the markup: the words a reader sees, with list bullets, quote
//! marks and link targets kept so nothing meaningful is lost.

use super::{Node, Tag};

/// Plain text of `nodes`, with the tags used, each once, in order seen.
pub fn plain(nodes: &[Node]) -> (String, Vec<&'static str>) {
    let mut out = String::new();
    let mut tags = Vec::new();
    walk(nodes, &mut out, &mut tags);
    (out.trim().to_owned(), tags)
}

fn walk(nodes: &[Node], out: &mut String, tags: &mut Vec<&'static str>) {
    for n in nodes {
        match n {
            Node::Text(t) => out.push_str(t),
            Node::Rule => {
                note(tags, "hr");
                out.push_str("\n----\n");
            }
            Node::Tag(tag, value, kids) => {
                note(tags, tag.name());
                tagged(*tag, value.as_deref(), kids, out, tags);
            }
        }
    }
}

fn tagged(
    tag: Tag,
    value: Option<&str>,
    kids: &[Node],
    out: &mut String,
    tags: &mut Vec<&'static str>,
) {
    match tag {
        Tag::H1 | Tag::H2 | Tag::H3 | Tag::Table | Tag::Tr => out.push('\n'),
        Tag::Item => out.push_str("\n- "),
        Tag::Quote => out.push_str("\n> "),
        Tag::Td | Tag::Th => out.push_str(" | "),
        _ => {}
    }
    walk(kids, out, tags);
    match (tag, value) {
        (Tag::Url, Some(v)) => out.push_str(&format!(" ({v})")),
        (Tag::H1 | Tag::H2 | Tag::H3 | Tag::Quote | Tag::List | Tag::Olist, _) => out.push('\n'),
        _ => {}
    }
}

fn note(tags: &mut Vec<&'static str>, name: &'static str) {
    if !tags.contains(&name) {
        tags.push(name);
    }
}
