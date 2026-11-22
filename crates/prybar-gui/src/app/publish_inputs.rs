//! The Workshop form's fields, kept together so the form reads them
//! in one place.

use gpui::{App, Entity};

use super::forms::field;
use crate::designs::input::TextInput;
use crate::state::checks;
use crate::state::tools::PublishForm;

#[derive(Clone)]
pub struct PublishInputs {
    pub item_id: Entity<TextInput>,
    pub title: Entity<TextInput>,
    pub description: Entity<TextInput>,
    pub tags: Entity<TextInput>,
    pub visibility: Entity<TextInput>,
    pub content: Entity<TextInput>,
    pub preview: Entity<TextInput>,
    pub change_note: Entity<TextInput>,
}

impl PublishInputs {
    pub fn new(cx: &mut App) -> Self {
        let about = "What it is and how to use it";
        PublishInputs {
            item_id: field(cx, "Empty for a new item", "", false, checks::item_id),
            title: field(cx, "Addon name", "", false, checks::title),
            description: field(cx, about, "", true, checks::description),
            tags: field(cx, "model, prop", "model", false, checks::tags),
            visibility: field(cx, "Private", "Private", false, checks::visibility),
            content: field(cx, "/path/to/addon.gma", "", false, checks::existing),
            preview: field(cx, "/path/to/preview.jpg", "", false, checks::preview),
            change_note: field(cx, "What changed", "", true, checks::description),
        }
    }

    pub fn all(&self) -> [&Entity<TextInput>; 8] {
        [
            &self.item_id,
            &self.title,
            &self.description,
            &self.tags,
            &self.visibility,
            &self.content,
            &self.preview,
            &self.change_note,
        ]
    }

    /// The inputs as plain text for the state to turn into a spec.
    pub fn form(&self, cx: &App) -> PublishForm {
        let text = |e: &Entity<TextInput>| e.read(cx).text().to_owned();
        PublishForm {
            item_id: text(&self.item_id),
            title: text(&self.title),
            description: text(&self.description),
            tags: text(&self.tags),
            visibility: text(&self.visibility),
            preview: text(&self.preview),
            content: text(&self.content),
            change_note: text(&self.change_note),
        }
    }
}
