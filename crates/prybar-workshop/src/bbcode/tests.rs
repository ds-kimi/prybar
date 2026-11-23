use super::{parse, Node, Tag};

fn text(s: &str) -> Node {
    Node::Text(s.into())
}

fn tag(t: Tag, value: Option<&str>, kids: Vec<Node>) -> Node {
    Node::Tag(t, value.map(Into::into), kids)
}

/// A description like a real one: bold, a rule, a list, code, and a
/// badge image inside a link.
#[test]
fn real_description() {
    let s = "[b]DSKCloak[/b] hides your addon.\n[hr]\n[list]\n[*]Fast\n[*]Safe[/list]\n\
             [code]dsk --in [b]x[/b][/code]\n\
             [url=https://example.com/dsk][img]https://example.com/badge.svg[/img][/url]";
    let got = parse(s);
    assert_eq!(got[0], tag(Tag::Bold, None, vec![text("DSKCloak")]));
    assert_eq!(got[2], Node::Rule);
    let items = vec![
        text("\n"),
        tag(Tag::Item, None, vec![text("Fast\n")]),
        tag(Tag::Item, None, vec![text("Safe")]),
    ];
    assert_eq!(got[4], tag(Tag::List, None, items));
    assert_eq!(
        got[6],
        tag(Tag::Code, None, vec![text("dsk --in [b]x[/b]")])
    );
    let img = tag(Tag::Img, None, vec![text("https://example.com/badge.svg")]);
    let link = tag(Tag::Url, Some("https://example.com/dsk"), vec![img]);
    assert_eq!(got[8], link);
}

/// Values, options and any case are read; unknown and broken tags
/// stay as text.
#[test]
fn values_and_broken_tags() {
    let got = parse("[QUOTE=\"Ann\"]hi[/quote][table noborder=1][/table]");
    assert_eq!(got[0], tag(Tag::Quote, Some("Ann"), vec![text("hi")]));
    assert_eq!(got[1], tag(Tag::Table, Some("noborder=1"), vec![]));
    assert_eq!(
        parse("[color=red]x[/color]"),
        vec![text("[color=red]x[/color]")]
    );
    assert_eq!(parse("a[/b]"), vec![text("a[/b]")]);
    let open = parse("[b]bold [i]both[/b] after");
    let kept = vec![text("bold [i]both")];
    assert_eq!(open, vec![tag(Tag::Bold, None, kept), text(" after")]);
    assert_eq!(parse("[u]never closed"), vec![text("[u]never closed")]);
    assert_eq!(parse("[*]no list"), vec![text("[*]no list")]);
    assert_eq!(parse("[code]open"), vec![text("[code]open")]);
}

/// Deep nesting stops at the cap and huge input stays linear.
#[test]
fn hostile_input() {
    let deep = "[quote]".repeat(10_000) + "x";
    fn depth(n: &[Node]) -> usize {
        n.iter()
            .map(|n| match n {
                Node::Tag(_, _, k) => 1 + depth(k),
                _ => 0,
            })
            .max()
            .unwrap_or(0)
    }
    assert!(depth(&parse(&deep)) <= 1, "unclosed tags turn into text");
    let closed = "[b]".repeat(1000) + &"[/b]".repeat(1000);
    assert!(depth(&parse(&closed)) <= 25);
    let huge = "[".repeat(200_000) + &"[code]".repeat(50_000);
    let start = std::time::Instant::now();
    let _ = parse(&huge);
    assert!(start.elapsed().as_secs() < 5);
}

/// Random mixes of tag pieces and odd chars never panic, and text with
/// no tag in it comes back unchanged.
#[test]
fn fuzz_smoke() {
    let parts = [
        "[",
        "]",
        "/",
        "=",
        "[b]",
        "[/b]",
        "[*]",
        "[list]",
        "[/list]",
        "[code]",
        "[/code]",
        "[url=",
        "é",
        "\n",
        "[table",
        " ",
        "[hr]",
        "[/",
        "x",
        "[noparse]",
        "🙂",
    ];
    let mut seed = 0x2545_f491_u64;
    for _ in 0..3000 {
        let mut s = String::new();
        for _ in 0..(seed % 40) {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            s.push_str(parts[(seed % parts.len() as u64) as usize]);
        }
        let _ = parse(&s);
    }
    assert_eq!(parse("a [ b ] c"), vec![text("a [ b ] c")]);
}
