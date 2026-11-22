//! Command line: `prybar-gui [--design <name>] [--sample] [files...]`.

use std::path::PathBuf;

use crate::state::DesignKind;

#[derive(Debug, Default, PartialEq)]
pub struct Launch {
    pub design: Option<DesignKind>,
    pub sample: bool,
    pub files: Vec<PathBuf>,
}

pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Launch, String> {
    let mut launch = Launch::default();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--sample" => launch.sample = true,
            "--design" => {
                let name = args
                    .next()
                    .ok_or("--design needs workbench, canvas or console")?;
                let design = DesignKind::parse(&name).ok_or(format!("unknown design {name}"))?;
                launch.design = Some(design);
            }
            _ if arg.starts_with("--") => return Err(format!("unknown flag {arg}")),
            _ => launch.files.push(arg.into()),
        }
    }
    Ok(launch)
}

#[cfg(test)]
#[test]
fn parses_flags_and_files() {
    let args = ["--design", "Console", "--sample", "a.mdl"].map(String::from);
    let launch = parse(args.into_iter()).expect("valid args");
    assert_eq!(launch.design, Some(DesignKind::Console));
    assert!(launch.sample);
    assert_eq!(launch.files, vec![PathBuf::from("a.mdl")]);
    assert!(parse(["--design".to_string()].into_iter()).is_err());
    assert!(parse(["--nope".to_string()].into_iter()).is_err());
}
