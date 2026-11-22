use prybar_tools::compile::parse_log;

#[test]
fn splits_errors_and_warnings_by_line_start() {
    let log = "Processing tri.qc\n  WARNING: bone has no weights\nno errors yet\n\
               ERROR: c:/x/tri.qc(3): - bad command\nError: second\nERRORS not a tag\nWARNING\n";
    let (errors, warnings) = parse_log(log);
    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0].line, 4);
    assert_eq!(errors[1].text, "Error: second");
    assert_eq!(warnings.len(), 2);
    assert_eq!(warnings[0].text, "WARNING: bone has no weights");
}

#[test]
fn hostile_bytes_do_not_panic() {
    let log = String::from_utf8_lossy(&[0xff, b'E', 0xc3, b'\n', b'E', b'R']).into_owned();
    let (errors, warnings) = parse_log(&log);
    assert!(errors.is_empty() && warnings.is_empty());
}

#[test]
fn relative_paths_become_absolute_for_the_child() {
    let runner = prybar_tools::runner::Runner::Native;
    let qc = std::path::Path::new("models").join("foo.qc");
    let cmd = prybar_tools::compile::compile_command(
        &runner,
        std::path::Path::new("studiomdl.exe"),
        std::path::Path::new("game"),
        &qc,
    );
    assert!(std::path::Path::new(cmd.get_program()).is_absolute());
    let args: Vec<_> = cmd.get_args().collect();
    assert!(std::path::Path::new(args[1]).is_absolute());
    let last = args.last().unwrap();
    assert!(std::path::Path::new(last).is_absolute());
}
