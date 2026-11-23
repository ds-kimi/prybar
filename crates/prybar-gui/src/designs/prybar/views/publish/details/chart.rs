//! Subscribers over time as bars drawn with plain elements: each bar
//! tells its day and values on hover, the axis gives the scale.

use gpui::{div, prelude::*, px, relative};
use prybar_workshop::item::Point;

use super::super::super::super::widgets::Tipped as _;
use super::when::{date, small};
use crate::theme::prybar::Tokens;

/// About two months of daily points fit the panel's width.
const SHOWN: usize = 60;
const HEIGHT: f32 = 72.;

pub fn view(series: &[Point], t: Tokens) -> impl IntoElement {
    let pts = &series[series.len().saturating_sub(SHOWN)..];
    let (Some(first), Some(last)) = (pts.first(), pts.last()) else {
        let none = "No history yet: prybar saves the numbers each time it lists your items.";
        return div().child(small(none.into(), t));
    };
    let top = pts
        .iter()
        .map(|p| p.subscriptions)
        .max()
        .map_or(1, |m| m.max(1));
    let plot = div()
        .flex_1()
        .h(px(HEIGHT))
        .flex()
        .gap(px(1.))
        .border_b_1()
        .border_color(t.border)
        .children(pts.iter().enumerate().map(|(i, p)| bar(i, p, top, t)));
    div()
        .debug_selector(|| "pb-chart".into())
        .flex()
        .flex_col()
        .gap_1()
        .child(small(format!("Subscribers since {}", date(first.day)), t))
        .child(div().flex().gap_1().child(axis(top, t)).child(plot))
        .child(ends(first.day, last.day, t))
}

/// The first and last day under the bars.
fn ends(first: u32, last: u32, t: Tokens) -> impl IntoElement {
    let row = div().flex().justify_between();
    row.child(small(date(first), t)).child(small(date(last), t))
}

/// One day; its tooltip gives the exact values.
fn bar(i: usize, p: &Point, top: u64, t: Tokens) -> impl IntoElement {
    let frac = p.subscriptions as f32 / top as f32;
    let tip = format!(
        "{}: {} subscribers, {} favorites",
        date(p.day),
        p.subscriptions,
        p.favorites
    );
    div()
        .id(("pb-chart-bar", i))
        .flex_1()
        .h_full()
        .flex()
        .items_end()
        .hover(move |s| s.bg(t.hover))
        .child(
            div()
                .w_full()
                .h(relative(frac.max(0.02)))
                .rounded_t_sm()
                .bg(t.rust),
        )
        .tip(tip, t)
}

/// The scale: the highest value at the top, zero at the bottom.
fn axis(top: u64, t: Tokens) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .justify_between()
        .h(px(HEIGHT))
        .child(small(top.to_string(), t))
        .child(small("0".into(), t))
}
