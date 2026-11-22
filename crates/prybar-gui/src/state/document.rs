//! A file the user opened, already read by prybar-service.

use std::path::{Path, PathBuf};

use prybar_mdl::Summary;
use prybar_service::ops::ListReport;

pub enum Document {
    Model { path: PathBuf, summary: Summary },
    Package { path: PathBuf, report: ListReport },
}

/// Companion files a model needs next to it, by extension suffix.
const COMPANIONS: [&str; 4] = [".vvd", ".dx90.vtx", ".phy", ".ani"];

impl Document {
    /// Reads a model or a package. Runs off the UI thread: big packages
    /// take a while and a bad file must only produce an error message.
    pub fn load(path: &Path) -> Result<Document, String> {
        let is_model = path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| e.eq_ignore_ascii_case("mdl"));
        let text = path.to_string_lossy();
        if is_model {
            let summary = prybar_service::model::inspect(&text).map_err(|e| e.to_string())?;
            return Ok(Document::Model {
                path: path.to_owned(),
                summary,
            });
        }
        let report = prybar_service::ops::list(path, None, None).map_err(|e| e.to_string())?;
        Ok(Document::Package {
            path: path.to_owned(),
            report,
        })
    }

    pub fn path(&self) -> &Path {
        match self {
            Document::Model { path, .. } | Document::Package { path, .. } => path,
        }
    }

    pub fn title(&self) -> String {
        let path = self.path();
        path.file_name().map_or_else(
            || path.display().to_string(),
            |n| n.to_string_lossy().into_owned(),
        )
    }

    /// Which companions sit next to a model, as (suffix, found).
    pub fn companions(&self) -> Vec<(&'static str, bool)> {
        let Document::Model { path, .. } = self else {
            return Vec::new();
        };
        let stem = path.with_extension("");
        COMPANIONS
            .iter()
            .map(|s| {
                (
                    *s,
                    PathBuf::from(format!("{}{s}", stem.display())).is_file(),
                )
            })
            .collect()
    }
}
