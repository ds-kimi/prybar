//! A Steam stand-in for tests: scripted status and items, records every
//! upload, reports the same progress stages the real client does.

use std::sync::Mutex;

use super::Steam;
use crate::spec::ItemSpec;
use crate::{ListedItem, Progress, PublishReport, SteamStatus, WorkshopError};

pub struct MockSteam {
    pub status: SteamStatus,
    pub items: Vec<ListedItem>,
    /// Set to make every upload fail with this Steam message.
    pub fail: Option<String>,
    pub needs_legal_agreement: bool,
    /// Specs as `upload` received them, staged paths included.
    pub uploads: Mutex<Vec<ItemSpec>>,
}

impl MockSteam {
    pub fn ready() -> Self {
        MockSteam {
            status: SteamStatus::Ready {
                user: "tester".into(),
            },
            items: Vec::new(),
            fail: None,
            needs_legal_agreement: false,
            uploads: Mutex::new(Vec::new()),
        }
    }
}

impl Steam for MockSteam {
    fn status(&self, _app_id: u32) -> SteamStatus {
        self.status.clone()
    }

    fn list(&self, _app_id: u32) -> Result<Vec<ListedItem>, WorkshopError> {
        match &self.status {
            SteamStatus::Ready { .. } => Ok(self.items.clone()),
            other => Err(WorkshopError::Steam(format!("{other:?}"))),
        }
    }

    fn upload(
        &self,
        spec: &ItemSpec,
        progress: &mut dyn FnMut(&Progress),
    ) -> Result<PublishReport, WorkshopError> {
        if let Ok(mut all) = self.uploads.lock() {
            all.push(spec.clone());
        }
        for (stage, done) in [
            ("Preparing content", 0),
            ("Uploading content", 50),
            ("Committing changes", 100),
        ] {
            progress(&Progress {
                stage: stage.into(),
                done,
                total: 100,
            });
        }
        if let Some(why) = &self.fail {
            return Err(WorkshopError::Steam(why.clone()));
        }
        Ok(PublishReport {
            dry_run: false,
            item_id: Some(spec.item_id.unwrap_or(1)),
            created: spec.item_id.is_none(),
            needs_legal_agreement: self.needs_legal_agreement,
        })
    }
}
