//! The app's one design: views, shell, widgets and motion.

pub(crate) mod prybar;

pub use prybar::{
    bbcode, input, modal, publish_footer, publish_popup, render, select, wanted, wanted_textures,
    Tipped,
};
