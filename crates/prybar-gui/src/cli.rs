//! Command line: `prybar-gui [--sample] [--screen <name>] [files...]`.

use std::path::PathBuf;

use crate::state::Screen;

#[derive(Debug, Default, PartialEq)]
pub struct Launch {
    pub sample: bool,
    /// Screen to show once the files are open, by its bar label.
    pub screen: Option<Screen>,
    pub files: Vec<PathBuf>,
}

pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Launch, String> {
    let mut launch = Launch::default();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--sample" => launch.sample = true,
            "--screen" => {
                let name = args.next().ok_or("--screen needs a screen name")?;
                let screen = Screen::ALL
                    .into_iter()
                    .find(|s| s.label().eq_ignore_ascii_case(&name));
                launch.screen = Some(screen.ok_or(format!("unknown screen {name}"))?);
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
    let args = ["--sample", "--screen", "3d Preview", "a.mdl"].map(String::from);
    let launch = parse(args.into_iter()).expect("valid args");
    assert_eq!(launch.screen, Some(Screen::ModelPreview));
    assert!(parse(["--screen", "nope"].map(String::from).into_iter()).is_err());
    assert!(launch.sample);
    assert_eq!(launch.files, vec![PathBuf::from("a.mdl")]);
    assert!(
        parse(["--design".to_string()].into_iter()).is_err(),
        "removed flag"
    );
    assert!(parse(["--nope".to_string()].into_iter()).is_err());
}
