//! A model's materials and base textures, looked up once per model and
//! shared by the 3D view and the texture screen.

mod chain;
pub mod view;

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use prybar_mdl::ir::ModelIr;
use prybar_service::texture::{self, MaterialReport, Sources};
use prybar_vtf::Image;

/// What the file says about one base texture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Info {
    pub width: u32,
    pub height: u32,
    pub format: &'static str,
    pub mips: u8,
    pub location: String,
}

/// One material slot of the model, in model order.
pub struct Slot {
    pub report: MaterialReport,
    pub info: Option<Info>,
    /// Why the texture could not be shown, when it could not.
    pub error: Option<String>,
}

impl Slot {
    /// One line for the list: the base texture, or that it is missing.
    pub fn summary(&self) -> String {
        match (&self.report.base_texture, &self.info) {
            (Some(b), Some(_)) => b.clone(),
            (_, None) => "Missing".into(),
            (None, Some(_)) => String::new(),
        }
    }
}

pub struct TextureSet {
    pub path: PathBuf,
    pub slots: Vec<Slot>,
    /// Mips per slot for the GPU, largest first, at most
    /// `chain::GPU_SIDE` wide; empty where the texture is missing.
    pub gpu: Arc<Vec<Vec<Image>>>,
    pub sources: Sources,
}

/// The last model's set: the 3D view and the texture screen ask for the
/// same one, and opening every game VPK twice would double the wait.
static LAST: Mutex<Option<Arc<TextureSet>>> = Mutex::new(None);

/// Blocking: call it off the UI thread. The lock is held while loading
/// so a second caller waits for the first and reuses its result.
pub fn load(path: &Path, ir: &ModelIr) -> Arc<TextureSet> {
    let mut last = LAST.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(set) = last.as_ref().filter(|s| s.path == path) {
        return set.clone();
    }
    let games = game_dirs();
    let sources = Sources::for_model(&path.to_string_lossy(), &games);
    let (slots, gpu) = texture::materials(ir, &sources)
        .into_iter()
        .map(|report| chain::slot(report, &sources))
        .unzip();
    let (path, gpu) = (path.to_owned(), Arc::new(gpu));
    let set = Arc::new(TextureSet {
        path,
        slots,
        gpu,
        sources,
    });
    *last = Some(set.clone());
    set
}

/// Game folders from the config; none when it cannot be read, so a
/// broken config only means fewer textures found.
fn game_dirs() -> Vec<PathBuf> {
    prybar_tools::config::load_default()
        .map(|c| c.games.into_iter().map(|g| g.game_dir).collect())
        .unwrap_or_default()
}
