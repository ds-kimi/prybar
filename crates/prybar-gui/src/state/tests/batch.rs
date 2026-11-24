//! Folder batches: items listed, marked, and reasons kept.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use gpui::{AppContext as _, TestAppContext};
use prybar_service::tools::{
    qc_files, CompileBatchReport, CompileItem, CompileOptions, Game, ItemStatus,
};
use prybar_tools::compile::CompileReport;

use super::fake;
use crate::sample;
use crate::state::batch::ItemState;
use crate::state::{AppState, Tools};

/// Every QC compiles except `bad.qc`; a set cancel skips them all.
pub(crate) fn compile_batch(
    _: &Game,
    args: &[String],
    opts: &CompileOptions,
    progress: &(dyn Fn(&str) + Sync),
    cancel: &AtomicBool,
) -> Result<CompileBatchReport, String> {
    let mut report = CompileBatchReport::default();
    for qc in qc_files(args, opts.recursive).0 {
        let name = qc.display().to_string();
        progress(&name);
        let ok = !name.ends_with("bad.qc") && !cancel.load(Ordering::Relaxed);
        let r = CompileReport {
            success: ok,
            exit_code: Some(i32::from(!ok)),
            errors: Vec::new(),
            warnings: Vec::new(),
            outputs: vec!["/g/models/x.mdl".into()],
            log: String::new(),
        };
        report.items.push(CompileItem::from_report(name, r, false));
    }
    report.compiled = report
        .items
        .iter()
        .filter(|i| i.status == ItemStatus::Ok)
        .count();
    report.failed = report.items.len() - report.compiled;
    Ok(report)
}

fn with_fake_batch(cx: &mut TestAppContext) -> gpui::Entity<AppState> {
    cx.executor().allow_parking();
    let (mut backend, games) = (fake::backend(), fake::games(1));
    backend.compile_batch = compile_batch;
    let tools = Tools {
        backend,
        games,
        ..Default::default()
    };
    cx.new(|_| AppState {
        tools,
        ..AppState::default()
    })
}

#[gpui::test]
fn compiles_a_folder_item_by_item(cx: &mut TestAppContext) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let sub = tmp.path().join("sub");
    std::fs::create_dir_all(&sub).expect("dir");
    for p in [tmp.path().join("a.qc"), sub.join("bad.qc")] {
        std::fs::write(p, "$modelname x.mdl").expect("qc");
    }
    let state = with_fake_batch(cx);
    state.update(cx, |s, cx| s.compile_folder(tmp.path().into(), cx));
    cx.run_until_parked();
    state.read_with(cx, |s, _| {
        let run = &s.batch.compile;
        assert!(!run.running);
        assert_eq!((run.items.len(), run.finished()), (2, 2));
        let failed = run.items.iter().find(|i| i.key.ends_with("bad.qc"));
        assert!(matches!(
            failed.map(|i| &i.state),
            Some(ItemState::Failed(_))
        ));
        assert_eq!(run.out, Some(PathBuf::from("/g/models")));
    });
}

#[gpui::test]
fn decompiles_a_folder_and_cancels(cx: &mut TestAppContext) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let sample = sample::write(tmp.path()).expect("sample written");
    let dir = sample.model.parent().expect("parent").to_path_buf();
    cx.executor().allow_parking();
    let state = cx.new(|_| AppState::default());
    let out = tmp.path().join("out");
    state.update(cx, |s, cx| {
        s.batch.folder = Some(dir.clone());
        s.decompile_selected(out.clone(), cx);
    });
    cx.run_until_parked();
    state.read_with(cx, |s, _| {
        let run = &s.batch.decompile;
        assert!(!run.running && !run.items.is_empty());
        assert!(run.items.iter().all(|i| i.state == ItemState::Done));
    });
    // A cancel set before the run starts skips every model.
    state.update(cx, |s, cx| {
        s.decompile_selected(out.join("again"), cx);
        s.cancel_batch();
    });
    cx.run_until_parked();
    state.read_with(cx, |s, _| {
        let states = s.batch.decompile.items.iter().map(|i| &i.state);
        assert!(states.clone().all(|st| *st != ItemState::Running));
    });
}
