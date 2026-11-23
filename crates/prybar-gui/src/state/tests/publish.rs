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

/// An edit opened on a listed item sends only the fields that differ.
#[test]
fn edit_form_sends_only_changes() {
    let item = super::fake::item();
    let form = PublishForm {
        item_id: "7".into(),
        title: "Crate pack 2".into(),
        tags: "addon".into(),
        visibility: "public".into(),
        editing: Some(item),
        ..PublishForm::default()
    };
    let spec = form.spec().expect("valid");
    assert_eq!(spec.title.as_deref(), Some("Crate pack 2"));
    assert!(spec.visibility.is_none() && spec.tags.is_empty() && spec.content.is_none());
    let new = PublishForm {
        editing: None,
        ..form
    };
    assert!(
        new.spec().expect("valid").visibility.is_some(),
        "a plain form sends all"
    );
}
