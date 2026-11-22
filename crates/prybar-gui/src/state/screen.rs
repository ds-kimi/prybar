//! Screens the app offers.

/// One navigable screen. Designs lay them out differently, but all of
/// them exist in every design so the comparison is about design only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Home,
    Inspector,
    Package,
    Decompile,
    Compile,
    Publish,
    Settings,
    ModelPreview,
    TexturePreview,
}

impl Screen {
    pub const ALL: [Screen; 9] = [
        Self::Home,
        Self::Inspector,
        Self::Package,
        Self::Decompile,
        Self::Compile,
        Self::Publish,
        Self::Settings,
        Self::ModelPreview,
        Self::TexturePreview,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Home => "Open",
            Self::Inspector => "Model",
            Self::Package => "Package",
            Self::Decompile => "Decompile",
            Self::Compile => "Compile",
            Self::Publish => "Publish",
            Self::Settings => "Settings",
            Self::ModelPreview => "3D preview",
            Self::TexturePreview => "Textures",
        }
    }
}
