//! Text content of each screen, read from the shared state.

use gpui::{div, prelude::*, AnyElement, Context};

use super::placeholder;
use crate::actions::MOD;
use crate::app::Root;
use crate::state::{AppState, Document, Screen};
use crate::theme::Palette;

/// Entries shown in the package list before "and N more".
const LIST_ROWS: usize = 20;

pub fn lines(state: &AppState) -> Vec<String> {
    let doc = state.selected.and_then(|i| state.documents.get(i));
    match (state.screen, doc) {
        (Screen::Home, _) => std::iter::once(format!("Drop a file here or press {MOD}-o."))
            .chain(
                state
                    .recent
                    .iter()
                    .map(|p| format!("Recent: {}", p.display())),
            )
            .collect(),
        (Screen::Inspector, Some(doc)) if matches!(doc, Document::Model { .. }) => model(doc),
        (Screen::Package, Some(Document::Package { report, .. })) => {
            package(report, state.search.as_deref())
        }
        (Screen::Decompile, _) => match &state.last_decompile {
            Some(r) => vec![format!(
                "{} model(s) written, {} failed",
                r.models.len(),
                r.failures.len()
            )],
            None => vec!["Decompile the selected file into a temp folder.".to_owned()],
        },
        (Screen::Settings, _) => vec!["Games config is edited here.".to_owned()],
        _ => vec!["Open a model or a package first.".to_owned()],
    }
}

fn model(doc: &Document) -> Vec<String> {
    let Document::Model { summary: s, .. } = doc else {
        return Vec::new();
    };
    let mut out = vec![
        format!("{} · version {}", s.name, s.version),
        format!(
            "{} bones · {} meshes · {} sequences",
            s.bones, s.meshes, s.sequences
        ),
        format!(
            "{} materials · {} flex controllers",
            s.materials.len(),
            s.flex_controllers
        ),
    ];
    let found = |ok: bool| if ok { "found" } else { "missing" };
    out.extend(
        doc.companions()
            .iter()
            .map(|(c, ok)| format!("{c}: {}", found(*ok))),
    );
    out
}

fn package(report: &prybar_service::ops::ListReport, search: Option<&str>) -> Vec<String> {
    let query = search.unwrap_or("").to_ascii_lowercase();
    let hits: Vec<_> = report
        .entries
        .iter()
        .filter(|e| e.path.to_ascii_lowercase().contains(&query))
        .collect();
    let mut out = vec![format!(
        "{:?} · {} entries · {} bytes",
        report.format, report.total, report.total_bytes
    )];
    out.extend(
        hits.iter()
            .take(LIST_ROWS)
            .map(|e| format!("{}  {} B", e.path, e.size)),
    );
    if hits.len() > LIST_ROWS {
        out.push(format!("and {} more", hits.len() - LIST_ROWS));
    }
    out
}

pub fn body(screen: Screen, lines: Vec<String>, p: Palette, cx: &mut Context<Root>) -> AnyElement {
    if let Some(ph) = screen.placeholder() {
        return placeholder::region(ph, p);
    }
    let run = matches!(screen, Screen::Decompile | Screen::Package)
        .then(|| placeholder::button(screen, p, cx));
    div()
        .flex()
        .flex_col()
        .gap_1()
        .children(lines.into_iter().map(|l| div().child(l)))
        .children(run)
        .into_any_element()
}
