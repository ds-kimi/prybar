//! Command line: `prybar-gui [--sample] [files...]`.

use std::path::PathBuf;

#[derive(Debug, Default, PartialEq)]
pub struct Launch {
    pub sample: bool,
    pub files: Vec<PathBuf>,
}

pub fn parse(args: impl Iterator<Item = String>) -> Result<Launch, String> {
    let mut launch = Launch::default();
    for arg in args {
        match arg.as_str() {
            "--sample" => launch.sample = true,
            _ if arg.starts_with("--") => return Err(format!("unknown flag {arg}")),
            _ => launch.files.push(arg.into()),
        }
    }
    Ok(launch)
}

#[cfg(test)]
#[test]
fn parses_flags_and_files() {
    let args = ["--sample", "a.mdl"].map(String::from);
    let launch = parse(args.into_iter()).expect("valid args");
    assert!(launch.sample);
    assert_eq!(launch.files, vec![PathBuf::from("a.mdl")]);
    assert!(
        parse(["--design".to_string()].into_iter()).is_err(),
        "removed flag"
    );
    assert!(parse(["--nope".to_string()].into_iter()).is_err());
}
