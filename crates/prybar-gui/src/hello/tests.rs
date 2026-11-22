use gpui::{AppContext, Modifiers, TestAppContext, VisualTestContext};

use super::Hello;

#[gpui::test]
fn click_increments(cx: &mut TestAppContext) {
    let window = cx.update(|cx| {
        cx.open_window(Default::default(), |_, cx| cx.new(|_| Hello::default()))
            .expect("test window opens")
    });
    let mut cx = VisualTestContext::from_window(window.into(), cx);
    let view = window.root(&mut cx).expect("root view");
    cx.run_until_parked();
    let bounds = cx.debug_bounds("hello").expect("button was drawn");
    cx.simulate_click(bounds.center(), Modifiers::none());
    assert_eq!(view.read_with(&cx, |v, _| v.clicks), 1);
}
