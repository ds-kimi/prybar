//! Files baked into the binary, so a moved or bundled app never misses
//! its logo or icons.

use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

pub struct Assets;

const FILES: [(&str, &[u8]); 3] = [
    ("prybar.png", include_bytes!("../../assets/prybar.png")),
    ("icons/sun.svg", include_bytes!("../../assets/sun.svg")),
    ("icons/moon.svg", include_bytes!("../../assets/moon.svg")),
];

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let found = FILES.iter().find(|(name, _)| *name == path);
        Ok(found.map(|(_, bytes)| Cow::Borrowed(*bytes)))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let names = FILES.iter().map(|(name, _)| *name);
        Ok(names
            .filter(|n| n.starts_with(path))
            .map(SharedString::from)
            .collect())
    }
}
