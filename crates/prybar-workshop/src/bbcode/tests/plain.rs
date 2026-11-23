use crate::bbcode::{parse, plain};

#[test]
fn keeps_words_bullets_and_link_targets() {
    let (text, tags) = plain(&parse(
        "[h1]Crate[/h1][b]Big[/b] box\n[list][*]one[*]two[/list][url=https://x.y]site[/url]",
    ));
    assert!(text.starts_with("Crate"));
    assert!(text.contains("- one"));
    assert!(text.contains("site (https://x.y)"));
    assert_eq!(tags, ["h1", "b", "list", "*", "url"]);
}

#[test]
fn broken_markup_stays_text() {
    let (text, tags) = plain(&parse("[b]open [nope]x[/nope]"));
    assert!(text.contains("[nope]x[/nope]"));
    assert!(text.starts_with("[b]open"));
    assert!(tags.is_empty());
}
