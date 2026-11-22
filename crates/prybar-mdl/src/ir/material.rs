use serde::Serialize;

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Material {
    pub name: String,
    pub flags: i32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct IncludeModel {
    pub label: String,
    pub path: String,
}

/// An 8-bit paletted texture embedded in a GoldSrc model; Source models
/// reference VTF materials instead and never fill this.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Texture {
    pub name: String,
    pub flags: i32,
    pub width: u32,
    pub height: u32,
    /// One palette index per pixel, rows top to bottom.
    #[serde(skip)]
    pub pixels: Vec<u8>,
    /// 256 RGB triples.
    #[serde(skip)]
    pub palette: Vec<u8>,
}
