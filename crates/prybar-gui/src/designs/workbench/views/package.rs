//! Package screen: a folder tree of entries, live search, extract.

use gpui::{div, prelude::*, AnyElement, Context};
use prybar_service::ops::ListReport;

use super::super::widgets;
use crate::actions::{Search, MOD};
use crate::app::Root;
use crate::theme::Palette;

/// Rows drawn before "and N more"; keeps huge packages responsive.
const MAX_ROWS: usize = 300;

pub fn view(
    report: &ListReport,
    search: Option<&str>,
    p: Palette,
    cx: &Context<Root>,
) -> AnyElement {
    let query = search.unwrap_or("").to_ascii_lowercase();
    let hits: Vec<(&str, u64)> = report
        .entries
        .iter()
        .filter(|e| e.path.to_ascii_lowercase().contains(&query))
        .map(|e| (e.path.as_str(), e.size))
        .collect();
    let more = hits.len().saturating_sub(MAX_ROWS);
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(header(report, p, cx))
        .child(search_bar(search, hits.len(), p))
        .child(tree(&hits, p))
        .children((more > 0).then(|| div().text_color(p.muted).child(format!("and {more} more"))))
        .into_any_element()
}

/// Package facts and the extract button; output goes to a temp folder.
fn header(report: &ListReport, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let out = std::env::temp_dir().join("prybar-gui-out").join("extract");
    let extract = widgets::button("wb-extract", "Extract all", true, true, p)
        .debug_selector(|| "wb-extract".into())
        .on_click(cx.listener(move |this, _, _, cx| {
            let out = out.clone();
            this.state.update(cx, |s, cx| s.extract_selected(out, cx))
        }));
    let facts = format!(
        "{:?} · {} entries · {} B",
        report.format, report.total, report.total_bytes
    );
    div()
        .flex()
        .items_center()
        .gap_2()
        .child(div().text_lg().flex_1().child(facts))
        .child(extract)
}

/// Shows the query typed while search is open (`MOD-f`).
fn search_bar(search: Option<&str>, hits: usize, p: Palette) -> AnyElement {
    let text = match search {
        Some(q) => format!("Search: {q}▏  ({hits} match)"),
        None => format!("Press {MOD}-f to filter"),
    };
    widgets::ring(
        div()
            .id("wb-search")
            .debug_selector(|| "wb-search".into())
            .h_7()
            .px_2()
            .flex()
            .items_center()
            .rounded_md()
            .border_1()
            .border_color(if search.is_some() { p.accent } else { p.border })
            .bg(p.bg)
            .text_color(if search.is_some() { p.text } else { p.muted })
            .cursor_text()
            .hover(move |st| st.border_color(p.muted))
            .on_click(|_, window, cx| window.dispatch_action(Box::new(Search), cx))
            .child(text),
        p,
    )
    .into_any_element()
}

/// Sorted paths drawn as a tree: a folder header whenever the folder changes.
fn tree(hits: &[(&str, u64)], p: Palette) -> AnyElement {
    let mut rows = Vec::new();
    let mut last_dir = "";
    for (ix, (path, size)) in hits.iter().take(MAX_ROWS).enumerate() {
        let (dir, name) = path.rsplit_once('/').unwrap_or(("", path));
        if dir != last_dir {
            rows.push(
                div()
                    .pt_1()
                    .px_2()
                    .text_color(p.muted)
                    .child(format!("▾ {dir}/"))
                    .into_any_element(),
            );
            last_dir = dir;
        }
        let row = widgets::row(("wb-entry", ix), false, p)
            .pl_6()
            .child(div().flex_1().child(name.to_owned()))
            .child(
                div()
                    .text_xs()
                    .text_color(p.muted)
                    .child(format!("{size} B")),
            );
        rows.push(row.into_any_element());
    }
    div().flex().flex_col().children(rows).into_any_element()
}
