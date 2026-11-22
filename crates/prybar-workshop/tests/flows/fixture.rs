use std::path::{Path, PathBuf};

/// A small valid addon folder; `extra` files are added as given.
pub fn addon(dir: &Path, json: &str, extra: &[&str]) -> PathBuf {
    let root = dir.join("my_addon");
    let files = ["lua/autorun/hello.lua"].iter().chain(extra);
    for rel in files {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, b"print(1)").unwrap();
    }
    std::fs::write(root.join("addon.json"), json).unwrap();
    root
}

pub const GOOD: &str = r#"{"title":"Hello","type":"tool","tags":["fun","build"]}"#;

/// A plain PNG of the given size, written to `dir`.
pub fn png(dir: &Path, w: u32, h: u32) -> PathBuf {
    let path = dir.join(format!("p{w}x{h}.png"));
    image::RgbImage::from_pixel(w, h, image::Rgb([200, 80, 20]))
        .save(&path)
        .unwrap();
    path
}
