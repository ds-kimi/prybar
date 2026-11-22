use super::expr_ops::step;
use prybar_mdl::ir::FlexOp;
use prybar_mdl::ModelIr;

/// Precedence of an expression on the stack: sums, products, unary
/// minus, then atoms and calls, which never need parentheses.
pub type Term = (String, u8);
pub const SUM: u8 = 1;
pub const PRODUCT: u8 = 2;
pub const ATOM: u8 = 4;

/// Postfix flex rule back to the infix text `$model` flex rules use.
/// Malformed rules (stack underflow) degrade to `0` instead of failing.
pub fn infix(ir: &ModelIr, ops: &[FlexOp]) -> String {
    let mut stack: Vec<Term> = Vec::new();
    for op in ops {
        step(ir, &mut stack, *op);
    }
    stack.pop().map_or_else(|| "0".to_owned(), |t| t.0)
}

pub fn pop(stack: &mut Vec<Term>) -> Term {
    stack.pop().unwrap_or_else(|| ("0".to_owned(), ATOM))
}

/// Right operands also wrap at equal precedence since `-` and `/` are
/// not associative.
pub fn binary(stack: &mut Vec<Term>, sym: &str, prec: u8) {
    let r = pop(stack);
    let l = pop(stack);
    let l = if l.1 < prec {
        format!("({})", l.0)
    } else {
        l.0
    };
    let r = if r.1 <= prec {
        format!("({})", r.0)
    } else {
        r.0
    };
    stack.push((format!("{l} {sym} {r}"), prec));
}
