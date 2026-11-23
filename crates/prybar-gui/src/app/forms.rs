//! Every text field of the app, made once so typing survives redraws
//! and screen changes.

use gpui::{App, AppContext as _, Entity, ScrollHandle};

use super::publish::PublishInputs;
use crate::designs::input::{any, Spec, TextInput};
use crate::state::checks;
use crate::state::AppState;
use crate::textures::view::TextureView;
use crate::viewport::view::Viewport;

#[derive(Clone)]
pub struct Forms {
    pub search: Entity<TextInput>,
    pub out: Entity<TextInput>,
    pub qc: Entity<TextInput>,
    /// The Workshop form.
    pub publish: PublishInputs,
    /// Settings: the edited game's folder and its studiomdl.
    pub game_dir: Entity<TextInput>,
    pub studiomdl: Entity<TextInput>,
    /// The compile log, so the view can keep its tail in sight.
    pub compile_log: ScrollHandle,
    /// The Workshop grid's width last frame, so its columns fit it.
    pub items_width: std::rc::Rc<std::cell::Cell<f32>>,
    /// The 3D preview, kept so its GPU thread and camera survive tabs.
    pub viewport: Entity<Viewport>,
    /// The texture screen, kept so its decoded image and pan survive tabs.
    pub textures: Entity<TextureView>,
}

pub(super) type Check = fn(&str) -> Option<&'static str>;

impl Forms {
    pub fn new(cx: &mut App) -> Forms {
        let out = std::env::temp_dir()
            .join("prybar-gui-out")
            .join("decompile")
            .display()
            .to_string();
        Forms {
            search: field(cx, "Filter files", "", false, any),
            out: field(cx, "/path/to/output", &out, false, checks::out_dir),
            qc: field(cx, "/path/to/model.qc", "", false, checks::qc),
            publish: PublishInputs::new(cx),
            game_dir: field(cx, "/path/to/garrysmod", "", false, checks::game_dir),
            studiomdl: field(
                cx,
                "/path/to/bin/studiomdl.exe",
                "",
                false,
                checks::existing,
            ),
            compile_log: ScrollHandle::new(),
            items_width: std::rc::Rc::default(),
            viewport: cx.new(|_| Viewport::default()),
            textures: cx.new(|_| TextureView::default()),
        }
    }

    /// Points the 3D view and the texture screen at the model the state
    /// wants; each loads only when its screen first shows that model.
    pub fn follow(&self, state: &Entity<AppState>, cx: &mut App) {
        let s = state.read(cx);
        let (model, textures) = (
            crate::designs::wanted(s),
            crate::designs::wanted_textures(s),
        );
        if let Some(path) = model {
            self.viewport.update(cx, |v, cx| v.show(Some(path), cx));
        }
        if let Some(path) = textures {
            self.textures.update(cx, |v, cx| v.show(Some(path), cx));
        }
    }
}

pub(super) fn field(
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
