use gpui::TestAppContext;

use super::open;
use crate::sample;
use crate::state::{Document, JobStatus, Screen};

#[gpui::test]
fn opens_model_and_decompiles(cx: &mut TestAppContext) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let sample = sample::write(tmp.path()).expect("sample written");
    let state = open(cx, sample.model);
    let out = tmp.path().join("out");
    state.update(cx, |s, cx| {
        assert_eq!(s.screen, Screen::Inspector);
        assert!(
            matches!(&s.documents[0], Document::Model { summary, .. } if summary.version == 49)
        );
        assert!(s.documents[0].companions().iter().filter(|c| c.1).count() >= 2);
        s.decompile_selected(out.clone(), cx);
    });
    cx.run_until_parked();
    state.read_with(cx, |s, _| {
        assert_eq!(s.last_decompile.as_ref().map(|r| r.models.len()), Some(1));
        assert!(s
            .jobs
            .iter()
            .all(|j| matches!(j.status, JobStatus::Done(_))));
    });
}

#[gpui::test]
fn opens_package_and_extracts(cx: &mut TestAppContext) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let sample = sample::write(tmp.path()).expect("sample written");
    let state = open(cx, sample.package);
    let out = tmp.path().join("x");
    state.update(cx, |s, cx| {
        assert_eq!(s.screen, Screen::Package);
        assert!(matches!(&s.documents[0], Document::Package { report, .. } if report.total == 3));
        s.decompile_selected(out.join("d"), cx);
        s.extract_selected(out.join("e"), cx);
    });
    cx.run_until_parked();
    assert!(out.join("e/models/sample.mdl").is_file());
    state.read_with(cx, |s, _| {
        assert_eq!(s.last_decompile.as_ref().map(|r| r.models.len()), Some(1))
    });
}

#[gpui::test]
fn bad_file_becomes_error_toast(cx: &mut TestAppContext) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let junk = tmp.path().join("junk.mdl");
    std::fs::write(&junk, b"IDST").expect("junk written");
    let state = open(cx, junk);
    state.read_with(cx, |s, _| {
        assert!(s.documents.is_empty());
        assert!(matches!(s.jobs[0].status, JobStatus::Failed(_)));
        assert_eq!(s.toasts.len(), 1);
    });
}

#[test]
fn companions_of_a_missing_package_are_all_missing() {
    let path = std::path::Path::new("/nowhere/pak_dir.vpk:models/a.mdl");
    let found = crate::state::companions::found(path, &[".vvd", ".phy"]);
    assert_eq!(found, vec![(".vvd", false), (".phy", false)]);
}
