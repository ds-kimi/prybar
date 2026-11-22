use std::path::PathBuf;

/// A model argument as typed by the user or agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelRef {
    Path(PathBuf),
    /// `inner` is `/`-separated; it may name one model, a folder or a
    /// wildcard pattern.
    Packed {
        package: PathBuf,
        inner: String,
    },
}

const PACKAGE_EXTS: [&str; 4] = [".vpk:", ".gma:", ".apk:", ".hfs:"];

impl ModelRef {
    /// Splits at the first `<package ext>:`; a bare `:` cannot be used
    /// because Windows paths start with `C:`.
    pub fn parse(arg: &str) -> Self {
        let lower = arg.to_ascii_lowercase();
        let at = PACKAGE_EXTS
            .iter()
            .filter_map(|ext| lower.find(ext).map(|i| i + ext.len() - 1))
            .min();
        match at {
            Some(i) => Self::Packed {
                package: PathBuf::from(&arg[..i]),
                inner: arg[i + 1..].replace('\\', "/"),
            },
            None => Self::Path(PathBuf::from(arg)),
        }
    }
}
