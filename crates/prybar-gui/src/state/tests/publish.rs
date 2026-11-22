use gpui::TestAppContext;

use super::tools::state;
use crate::state::tools::PublishForm;

#[gpui::test]
fn publish_dry_run_then_upload(cx: &mut TestAppContext) {
    let state = state(cx);
    let form = PublishForm {
        title: "Crate".into(),
        content: "/a.gma".into(),
        ..PublishForm::default()
    };
    for dry in [true, false] {
        state.update(cx, |s, cx| s.publish(form.clone(), dry, cx));
        cx.run_until_parked();
        state.read_with(cx, |s, _| {
            let run = &s.tools.publish;
            assert_eq!(run.progress.as_ref().map(|p| p.done), Some(1));
            let r = run.outcome.clone().expect("finished").expect("ok");
            assert_eq!((r.dry_run, r.item_id), (dry, (!dry).then_some(42)));
        });
    }
    let bad = PublishForm {
        item_id: "abc".into(),
        ..form
    };
    state.update(cx, |s, cx| s.publish(bad, true, cx));
    state.read_with(cx, |s, _| {
        assert!(matches!(s.tools.publish.outcome, Some(Err(_))))
    });
}
