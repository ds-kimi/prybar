//! Screens every design offers, and the regions that are later work.

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

/// A feature that is not built yet; designs draw it as a styled,
/// animated region carrying this label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placeholder {
    ModelViewport,
    TextureView,
    CompileLog,
    PublishProgress,
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

    pub fn placeholder(self) -> Option<Placeholder> {
        match self {
            Self::Compile => Some(Placeholder::CompileLog),
            Self::Publish => Some(Placeholder::PublishProgress),
            Self::ModelPreview => Some(Placeholder::ModelViewport),
            Self::TexturePreview => Some(Placeholder::TextureView),
            _ => None,
        }
    }
}

impl Placeholder {
    pub fn label(self) -> &'static str {
        match self {
            Self::ModelViewport => "3D preview here",
            Self::TextureView => "Texture preview here",
            Self::CompileLog => "Compile log stream here",
            Self::PublishProgress => "Workshop upload progress here",
        }
    }
}
