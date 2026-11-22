//! Package screen: facts and extract up top, live search, then a
//! folder tree of the entries.

use gpui::{div, prelude::*, AnyElement, Context};
use prybar_service::ops::ListReport;

use super::super::widgets::{self, Kind};
use crate::actions::{Search, MOD};
use crate::app::Root;
use crate::theme::prybar::Tokens;

/// Rows drawn before "and N more"; keeps huge packages responsive.
const MAX_ROWS: usize = 300;

pub fn view(r: &ListReport, search: Option<&str>, t: Tokens, cx: &Context<Root>) -> AnyElement {
    let query = search.unwrap_or("").to_ascii_lowercase();
    let hits: Vec<(&str, u64)> = r
        .entries
        .iter()
        .filter(|e| e.path.to_ascii_lowercase().contains(&query))
        .map(|e| (e.path.as_str(), e.size))
        .collect();
    let more = hits.len().saturating_sub(MAX_ROWS);
    let tail = (more > 0).then(|| div().text_color(t.muted).child(format!("and {more} more")));
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(header(r, t, cx))
        .child(search_bar(search, hits.len(), t))
        .child(tree(&hits, t))
        .children(tail)
        .into_any_element()
}

/// Output goes to a temp folder until a folder picker lands.
fn header(r: &ListReport, t: Tokens, cx: &Context<Root>) -> impl IntoElement {
    let out = std::env::temp_dir().join("prybar-gui-out").join("extract");
    let extract = widgets::button("pb-extract", "Extract all", Kind::Primary, true, t)
        .debug_selector(|| "pb-extract".into())
        .on_click(cx.listener(move |this, _, _, cx| {
            let out = out.clone();
            this.state.update(cx, |s, cx| s.extract_selected(out, cx))
        }));
    let facts = format!(
        "{} package, {} files, {} bytes",
        format!("{:?}", r.format).to_uppercase(),
        r.total,
        r.total_bytes
    );
    div()
        .flex()
        .items_center()
        .gap_3()
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap_1()
                .child(widgets::heading("Package"))
                .child(div().text_color(t.muted).child(facts)),
        )
        .child(extract)
}

/// Shows the query typed while search is open (`MOD-f`).
fn search_bar(search: Option<&str>, hits: usize, t: Tokens) -> AnyElement {
    let text = match search {
        Some(q) => format!("{q}▏  {hits} match"),
        None => format!("Filter files  {MOD}-f"),
    };
    widgets::ring(div().id("pb-search"), t)
        .debug_selector(|| "pb-search".into())
        .h_8()
        .px_2()
        .flex()
        .items_center()
        .rounded_md()
        .border_color(if search.is_some() { t.rust } else { t.border })
        .bg(t.surface)
        .text_color(if search.is_some() { t.text } else { t.muted })
        .cursor_text()
        .hover(move |st| st.border_color(t.muted))
        .on_click(|_, window, cx| window.dispatch_action(Box::new(Search), cx))
        .child(text)
        .into_any_element()
}

/// Sorted paths drawn as a tree: a folder line whenever the folder changes.
fn tree(hits: &[(&str, u64)], t: Tokens) -> AnyElement {
    let mut rows = Vec::new();
    let mut last_dir = None;
    for (ix, (path, size)) in hits.iter().take(MAX_ROWS).enumerate() {
        let (dir, name) = path.rsplit_once('/').unwrap_or(("", path));
        if last_dir != Some(dir) {
            rows.push(
                widgets::section(format!("{dir}/"), t)
                    .px_2()
                    .into_any_element(),
            );
            last_dir = Some(dir);
        }
        let size = div()
            .text_xs()
            .text_color(t.muted)
            .child(format!("{size} B"));
        let row = widgets::row(("pb-entry", ix), false, t)
            .pl_5()
            .child(div().flex_1().truncate().child(name.to_owned()))
            .child(size);
        rows.push(row.into_any_element());
    }
    let none = rows
        .is_empty()
        .then(|| div().text_color(t.muted).child("No file matches."));
    let list = div().flex().flex_col().children(rows);
    list.children(none).into_any_element()
}
