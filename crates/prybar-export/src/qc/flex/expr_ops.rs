use super::expr::{binary, pop, Term, ATOM, PRODUCT, SUM};
use super::lookup::{controller, desc};
use crate::qc::fmt::num;
use prybar_mdl::ir::FlexOp;
use prybar_mdl::ModelIr;

pub fn step(ir: &ModelIr, st: &mut Vec<Term>, op: FlexOp) {
    let ctl = |i: i32| controller(ir, i);
    match op {
        FlexOp::Const(v) => st.push((num(v), ATOM)),
        FlexOp::Fetch(i) | FlexOp::DmeLowerEyelid(i) | FlexOp::DmeUpperEyelid(i) => {
            st.push((ctl(i), ATOM))
        }
        FlexOp::FetchFlex(i) => st.push((format!("%{}", desc(ir, i)), ATOM)),
        FlexOp::Add => binary(st, "+", SUM),
        FlexOp::Sub => binary(st, "-", SUM),
        FlexOp::Mul => binary(st, "*", PRODUCT),
        FlexOp::Div => binary(st, "/", PRODUCT),
        FlexOp::Neg => {
            let t = pop(st);
            st.push((format!("-({})", t.0), ATOM));
        }
        FlexOp::Max | FlexOp::Min => {
            let (r, l) = (pop(st), pop(st));
            let f = if op == FlexOp::Max { "max" } else { "min" };
            st.push((format!("{f}({}, {})", l.0, r.0), ATOM));
        }
        FlexOp::TwoWay0(i) => st.push((format!("min(max(-{}, 0), 1)", ctl(i)), ATOM)),
        FlexOp::TwoWay1(i) => st.push((format!("min(max({}, 0), 1)", ctl(i)), ATOM)),
        FlexOp::NWay(i) => nway(ir, st, i),
        FlexOp::Combo(n) | FlexOp::Dominate(n) => combine(st, op, n),
        FlexOp::Unknown { .. } => st.push(("0".to_owned(), ATOM)),
        FlexOp::Exp | FlexOp::Open | FlexOp::Close | FlexOp::Comma => {}
    }
}

/// `STUDIO_NWAY`: a trapezoid over the value controller (rising x..y,
/// flat y..z, falling z..w) scaled by the multiplier controller.
fn nway(ir: &ModelIr, st: &mut Vec<Term>, multiplier: i32) {
    let value = pop(st).0;
    let value = value.parse::<i32>().map_or(value, |i| controller(ir, i));
    let [w, z, y, x] = [pop(st).0, pop(st).0, pop(st).0, pop(st).0];
    let v = &value;
    let rise = format!("min(max(({v} - {x}) / ({y} - {x}), 0), 1)");
    let fall = format!("min(max(({w} - {v}) / ({w} - {z}), 0), 1)");
    let m = controller(ir, multiplier);
    st.push((format!("min({rise}, {fall}) * {m}"), PRODUCT));
}

/// Combo multiplies the last `n` terms; dominate scales the term below
/// them by `1 - product`. `n` is clamped so hostile counts cannot spin.
fn combine(st: &mut Vec<Term>, op: FlexOp, n: i32) {
    let n = usize::try_from(n).unwrap_or(0).min(st.len());
    let terms: Vec<String> = st
        .drain(st.len() - n..)
        .map(|t| format!("({})", t.0))
        .collect();
    let product = if terms.is_empty() {
        "1".to_owned()
    } else {
        terms.join(" * ")
    };
    if matches!(op, FlexOp::Combo(_)) {
        return st.push((product, PRODUCT));
    }
    let base = pop(st).0;
    st.push((format!("({base}) * (1 - {product})"), PRODUCT));
}
