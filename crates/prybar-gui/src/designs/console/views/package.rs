//! Package: the bar's query filters the file list live; one path per row.

use gpui::{div, prelude::*, AnyElement, Context};
use prybar_service::ops::ListReport;

use super::super::widgets;
use super::out_dir;
use crate::app::Root;
use crate::theme::Palette;

/// Rows drawn before "and N more"; keeps huge packages responsive.
const MAX_FILES: usize = 300;

pub fn view(r: &ListReport, query: Option<&str>, p: Palette, cx: &Context<Root>) -> AnyElement {
    let q = query.unwrap_or("").trim().to_ascii_lowercase();
    let hits = hits(r, &q);
    let more = hits.len().saturating_sub(MAX_FILES);
    let info = format!(
        "{:?} · {} files · {} bytes",
        r.format, r.total, r.total_bytes
    );
    let found = match q.is_empty() {
        true => "type to filter".to_owned(),
        false => format!("{} match \"{q}\"", hits.len()),
    };
    let rows = hits
        .into_iter()
        .take(MAX_FILES)
        .map(|f| div().child(f.to_owned()));
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(widgets::heading("package", info, p))
        .child(extract(found, p, cx))
        .child(div().flex().flex_col().children(rows))
        .children((more > 0).then(|| div().text_color(p.muted).child(format!("… {more} more"))))
        .into_any_element()
}

fn extract(found: String, p: Palette, cx: &Context<Root>) -> impl IntoElement {
    let run = widgets::action("cs-extract", "extract all", "extract", true, p).on_click(
        cx.listener(|this, _, _, cx| {
            this.state
                .update(cx, |s, cx| s.extract_selected(out_dir("extract"), cx))
        }),
    );
    div()
        .flex()
        .gap_3()
        .child(run)
        .child(div().text_color(p.muted).child(found))
}

fn hits<'a>(r: &'a ListReport, q: &str) -> Vec<&'a str> {
    r.entries
        .iter()
        .map(|e| e.path.as_str())
        .filter(|path| path.to_ascii_lowercase().contains(q))
        .collect()
}
