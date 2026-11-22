//! Command line: `prybar-gui [--sample] [--screen <name>] [--size <w>x<h>] [files...]`.

use std::path::PathBuf;

use crate::state::Screen;

#[derive(Debug, Default, PartialEq)]
pub struct Launch {
    pub sample: bool,
    /// Screen to show once the files are open, by its bar label.
    pub screen: Option<Screen>,
    /// Window size in points, to check layouts at fixed sizes.
    pub size: Option<(f32, f32)>,
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
            "--size" => {
                let v = args.next().ok_or("--size needs <w>x<h>")?;
                let (w, h) = v.split_once('x').ok_or("--size needs <w>x<h>")?;
                let num = |t: &str| {
                    t.parse::<f32>()
                        .ok()
                        .filter(|n| (400.0..=8000.0).contains(n))
                };
                launch.size = Some(num(w).zip(num(h)).ok_or(format!("bad size {v}"))?);
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
    let sized = parse(["--size", "900x600"].map(String::from).into_iter());
    assert_eq!(sized.expect("size").size, Some((900.0, 600.0)));
    assert!(parse(["--size", "9x"].map(String::from).into_iter()).is_err());
    assert!(parse(["--nope".to_string()].into_iter()).is_err());
}
