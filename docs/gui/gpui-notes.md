# GPUI notes

Shared reading for the GUI lots. Paths starting with `zed/` point into the
reference checkout at `~/Documents/prybar-ref-zed` (read only, never copied
into this repo).

## Dependency

- `gpui` and `gpui_platform` as git dependencies pinned to Zed commit
  `d31d1074a07b7bbaa43a52fef7847a00dd491baf` (see `crates/prybar-gui/Cargo.toml`).
  crates.io has `gpui 0.2.2` but not `gpui_platform`, which now owns the
  platform backends, so the published crate does not match current docs.
- `gpui_platform` features: `font-kit` (macOS glyphs; without it text is
  invisible), `wayland` + `x11` (Linux windowing), `runtime_shaders` (macOS:
  compile Metal shaders at launch, so building does not need
  `xcodebuild -downloadComponent MetalToolchain`).
- Tests: dev-dependency `gpui` with `test-support`.
- `gpui-component` (longbridge) not adopted: it pins its own gpui revision,
  which would fight our pin, and its look would pull the three designs
  together. The only missing primitive is a text input; build a small one
  modeled on `zed/crates/gpui/examples/input.rs` (read, do not copy).

## App and windows

- `gpui_platform::application().run(|cx: &mut App| ...)`; open with
  `cx.open_window(WindowOptions { window_bounds: Some(WindowBounds::Windowed(
  Bounds::centered(None, size(px(w), px(h)), cx))), ..Default::default() },
  |window, cx| cx.new(|cx| View::new(cx)))`. Examples: `zed/crates/gpui/examples/window.rs`,
  `window_positioning.rs`, `on_window_close_quit.rs`.
- Views are entities implementing `Render`; reusable stateless pieces
  implement `RenderOnce` + `#[derive(IntoElement)]` (pattern:
  `zed/crates/ui/src/components/chip.rs`).
- `cx.notify()` after mutating state, or nothing redraws.
- Shared state: an `Entity<State>` held by views; `cx.observe(&entity, ..)` or
  `cx.subscribe` with `EventEmitter`. See `examples/ownership_post.rs`.

## Styling and interaction states

Tailwind-like builders on `div()` (`zed/crates/gpui/src/styled.rs`):
`flex`, `flex_col`, `gap_2`, `px_4`, `rounded_md`, `bg(rgb(..))`,
`text_color`, `border_1`, `shadow_md`, `opacity`, `size_full`.

- Interactive elements need `.id("..")` (makes it `Stateful<Div>`).
- `.hover(|s| ..)`, `.active(|s| ..)` (pressed), `.focus(|s| ..)`,
  `.focus_visible(|s| ..)` (keyboard focus only) — `zed/crates/gpui/src/elements/div.rs`
  around lines 866, 1320, 1342, 1608. Example `focus_visible.rs`, `tab_stop.rs`.
- `.group("card")` on a parent + `.group_hover("card", |s| ..)` on children.
- `.cursor_pointer()`, `.tooltip(|w, cx| view)`, `.on_hover(|hovered, w, cx| ..)`.
- Disabled: no built-in; skip listeners and apply a muted style ourselves.
- Focus: `cx.focus_handle()`, `.track_focus(&handle)`, `.tab_index(n)`,
  `handle.focus(window, cx)`; implement `Focusable` on views.

## Animation

`zed/crates/gpui/src/elements/animation.rs`, example `examples/animation.rs`.

- `el.with_animation(id, Animation::new(Duration).with_easing(ease_in_out), |el, t| ..)`
  where `t` goes 0..1. `.repeat()` loops. Easings: `linear`, `quadratic`,
  `ease_in_out`, `ease_out_quint()`, `bounce(f)`, `pulsating_between(a, b)`.
- The id must change to replay an animation (e.g. `("fade", self.clicks)`);
  the same id keeps its finished state.
- Springs: `el.with_spring(id, SpringAnimation::new(SpringConfig::new(stiffness,
  damping, mass)).to(value), |el, v| el.left(v))`; retargeting keeps velocity,
  good for selection highlights and sliding panels. `AnimationPhase` lets one
  spring drive several properties.
- Hover/active changes are instant; for animated hover, track hover with
  `on_hover` into state and drive a spring.

## Actions and keys

- `actions!(prybar, [Open, Back, Search]);`
- `cx.bind_keys([KeyBinding::new("cmd-o", Open, Some("Context"))])`;
  the element sets `.key_context("Context")` and `.on_action(cx.listener(Self::open))`.
  Example `examples/testing.rs`. `secondary-` prefix = cmd on macOS, ctrl elsewhere.

## Lists, scroll, images

- `uniform_list(id, count, |range, window, cx| Vec<Element>)` for long equal-height
  lists (package trees); `list(ListState, ..)` for variable heights.
  Examples `uniform_list.rs`, `list_example.rs`, `data_table.rs`, `tree.rs`.
- Scroll: `.overflow_y_scroll()` + `.track_scroll(&ScrollHandle)` (`scrollable.rs`).
- `img(path_or_bytes)`, `svg().path(..)` (needs an `AssetSource`, see `animation.rs`).
- Overlays: `anchored()` + `deferred()` (`popover.rs`). Custom drawing: `canvas()` (`painting.rs`).

## Files

- Open dialog: `cx.prompt_for_paths(PathPromptOptions { files, directories,
  multiple, prompt })` returns a oneshot receiver; await it in `cx.spawn`.
- Drop from Finder: `.on_drop(cx.listener(|this, paths: &ExternalPaths, w, cx| ..))`
  and `.drag_over::<ExternalPaths>(|s, _, _, _| s.bg(..))` for the hover style.
  Platform events are `FileDropEvent::{Entered, Pending, Submit, Exited}`
  (`zed/crates/gpui/src/interactive.rs:758`). In-app drag: `on_drag` + `on_drop::<T>`
  (`examples/drag_drop.rs`).

## Async work

- `cx.spawn(async move |this, cx| { .. this.update(cx, |v, cx| ..) })` on the UI
  thread; heavy work in `cx.background_spawn(async move { .. })` and await it.
  Call prybar-service from the background task, never from `render`.

## Tests

`zed/crates/gpui/examples/testing.rs` is the tutorial; API in
`zed/crates/gpui/src/app/test_context.rs`.

- `#[gpui::test] fn t(cx: &mut TestAppContext)`; async tests allowed.
- Window: `cx.update(|cx| cx.open_window(..))`, then
  `VisualTestContext::from_window(window.into(), cx)`, `window.root(&mut cx)`.
- Input: `simulate_click(point, Modifiers::none())`, `simulate_mouse_move`,
  `simulate_keystrokes("cmd-1")`, `simulate_input("text")`, `simulate_event(FileDropEvent::..)`.
- Finding elements: `.debug_selector(|| "name".into())` on the element, then
  `cx.debug_bounds("name")` — only in debug builds (returns `None` in release).
- `cx.run_until_parked()` runs pending tasks. Real I/O in a test needs
  `cx.executor().allow_parking()`.
- Our sample: `crates/prybar-gui/src/hello/tests.rs`.

## Pitfalls found

- Missing Metal Toolchain breaks the build with "cannot execute tool 'metal'";
  fixed by the `runtime_shaders` feature.
- `debug_bounds` needs `debug_selector`, not just `.id`.
- `screencapture -l <id>` from a background process fails without the Screen
  Recording permission for the terminal; the window still opens.
- GPUI is pre-1.0: bumping the pinned rev can break APIs. Bump deliberately.
