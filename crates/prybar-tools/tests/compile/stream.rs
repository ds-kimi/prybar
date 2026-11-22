//! Streaming through a fake studiomdl: `sh` stands in as the runner, so
//! the "tool" is a shell script and no game install is needed.
#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use prybar_tools::compile::{compile_streamed, written_files};
use prybar_tools::runner::Runner;

fn fake(dir: &Path, body: &str) -> PathBuf {
    let script = dir.join("studiomdl.sh");
    std::fs::write(&script, body).unwrap();
    std::fs::write(dir.join("a.qc"), "$modelname x.mdl\n").unwrap();
    script
}

#[test]
fn lines_arrive_in_order_and_errors_fail_the_report() {
    let tmp = tempfile::tempdir().unwrap();
    let body = "echo 'Processing a.qc'\necho 'WARNING: no weights'\n\
                echo 'writing /g/models/x.mdl:'\necho 'ERROR: bad' 1>&2\nexit 0\n";
    let script = fake(tmp.path(), body);
    let mut seen = Vec::new();
    let qc = tmp.path().join("a.qc");
    let runner = Runner::Wine("/bin/sh".into());
    let stop = AtomicBool::new(false);
    let report = compile_streamed(
        &runner,
        (&script, tmp.path(), &qc),
        &mut |l| seen.push(l.to_owned()),
        &stop,
    )
    .unwrap();
    assert_eq!(seen.len(), 4);
    assert!(seen.contains(&"ERROR: bad".to_owned()));
    assert!(!report.success);
    assert_eq!((report.errors.len(), report.warnings.len()), (1, 1));
    assert_eq!(written_files(&report.log), ["/g/models/x.mdl"]);
}

#[test]
fn cancel_kills_a_running_compile() {
    let tmp = tempfile::tempdir().unwrap();
    let script = fake(tmp.path(), "echo started\nexec sleep 30\n");
    let qc = tmp.path().join("a.qc");
    let stop = AtomicBool::new(false);
    let started = std::time::Instant::now();
    let report = compile_streamed(
        &Runner::Wine("/bin/sh".into()),
        (&script, tmp.path(), &qc),
        &mut |_| stop.store(true, Ordering::Relaxed),
        &stop,
    )
    .unwrap();
    assert!(!report.success);
    assert!(started.elapsed().as_secs() < 10);
}
