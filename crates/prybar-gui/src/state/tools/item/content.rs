use std::path::PathBuf;

use gpui::Context;
use prybar_workshop::spec::Visibility;
use prybar_workshop::ListedItem;

use crate::state::tools::PublishForm;
use crate::state::AppState;

impl AppState {
    /// "Update content": the linked folder goes out with every other
    /// field as listed, so the edit path sends the content alone.
    pub fn update_content(&mut self, item: &ListedItem, dir: PathBuf, cx: &mut Context<Self>) {
        let visibility = match item.visibility {
            Visibility::Public => "public",
            Visibility::FriendsOnly => "friends",
            Visibility::Unlisted => "unlisted",
            Visibility::Private => "private",
        };
        let form = PublishForm {
            item_id: item.item_id.to_string(),
            title: item.title.clone(),
            description: item.description.clone(),
            visibility: visibility.into(),
            content: dir.display().to_string(),
            change_note: "Content update".into(),
            editing: Some(item.clone()),
            ..PublishForm::default()
        };
        self.publish(form, false, cx);
    }
}
