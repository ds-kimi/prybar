//! Every text field of the app, made once so typing survives redraws
//! and screen changes.

use gpui::{App, AppContext as _, Entity};

use crate::designs::input::{any, Spec, TextInput};
use crate::state::checks;
use crate::viewport::view::Viewport;

#[derive(Clone)]
pub struct Forms {
    pub search: Entity<TextInput>,
    pub out: Entity<TextInput>,
    pub qc: Entity<TextInput>,
    pub title: Entity<TextInput>,
    pub description: Entity<TextInput>,
    pub tags: Entity<TextInput>,
    pub visibility: Entity<TextInput>,
    pub config: Entity<TextInput>,
    /// The 3D preview, kept so its GPU thread and camera survive tabs.
    pub viewport: Entity<Viewport>,
}

type Check = fn(&str) -> Option<&'static str>;

impl Forms {
    pub fn new(cx: &mut App) -> Forms {
        let out = std::env::temp_dir()
            .join("prybar-gui-out")
            .join("decompile")
            .display()
            .to_string();
        let config = prybar_tools::config::config_path()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        Forms {
            search: field(cx, "Filter files", "", false, any),
            out: field(cx, "/path/to/output", &out, false, checks::out_dir),
            qc: field(cx, "/path/to/model.qc", "", false, checks::qc),
            title: field(cx, "Addon name", "", false, checks::title),
            description: field(
                cx,
                "What it is and how to use it",
                "",
                true,
                checks::description,
            ),
            tags: field(cx, "model, prop", "model", false, checks::tags),
            visibility: field(cx, "Private", "Private", false, checks::visibility),
            config: field(cx, "/path/to/games.toml", &config, false, checks::required),
            viewport: cx.new(|_| Viewport::default()),
        }
    }
}

fn field(
    cx: &mut App,
    placeholder: &'static str,
    text: &str,
    multiline: bool,
    check: Check,
) -> Entity<TextInput> {
    let spec = Spec {
        placeholder,
        multiline,
        text: text.to_owned(),
        check,
    };
    cx.new(|cx| TextInput::new(spec, cx))
}
