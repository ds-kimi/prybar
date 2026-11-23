use std::path::PathBuf;

use crate::spec::{ItemSpec, Visibility};
use crate::ListedItem;

/// The edit popup's fields. `preview` and `content` are set only when
/// the user picked a new image or wants the content re-sent.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Draft {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub visibility: Visibility,
    pub preview: Option<PathBuf>,
    pub content: Option<PathBuf>,
    pub change_note: String,
}

impl Draft {
    /// The popup starts from what Steam has now.
    pub fn from_item(item: &ListedItem) -> Self {
        Draft {
            title: item.title.clone(),
            description: item.description.clone(),
            tags: item.tags.clone(),
            visibility: item.visibility,
            ..Draft::default()
        }
    }
}

/// An update spec carrying only what differs from `item`, so an edit of
/// the title never re-sends a description someone changed on the web.
pub fn changes(app_id: u32, item: &ListedItem, d: &Draft) -> ItemSpec {
    let differs = |new: &str, old: &str| (new.trim() != old.trim()).then(|| new.trim().to_owned());
    let note = d.change_note.trim();
    ItemSpec {
        app_id,
        item_id: Some(item.item_id),
        title: differs(&d.title, &item.title),
        description: differs(&d.description, &item.description),
        tags: if same_tags(&d.tags, &item.tags) {
            Vec::new()
        } else {
            d.tags.clone()
        },
        visibility: (d.visibility != item.visibility).then_some(d.visibility),
        preview: d.preview.clone(),
        content: d.content.clone(),
        change_note: (!note.is_empty()).then(|| note.to_owned()),
    }
}

/// Steam stores tags without order or case.
fn same_tags(a: &[String], b: &[String]) -> bool {
    let norm = |t: &[String]| {
        let mut v: Vec<String> = t.iter().map(|s| s.trim().to_lowercase()).collect();
        v.sort();
        v.dedup();
        v
    };
    norm(a) == norm(b)
}
