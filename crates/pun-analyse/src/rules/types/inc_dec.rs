//! InvalidIncDecOperationRule (`*.expr`: `++`/`--` on what is not a variable, an offset or a property).

use mago_syntax::cst::*;

use super::super::Checker;

fn check(c: &mut Checker, operand: &Expression, kind: &str, operator: &str) {
    if matches!(
        super::flow::unparen(operand),
        Expression::Variable(_)
            | Expression::ArrayAccess(_)
            | Expression::ArrayAppend(_)
            | Expression::Access(Access::Property(_) | Access::StaticProperty(_))
    ) {
        return;
    }
    c.report(
        operand,
        &format!("{kind}.expr"),
        format!("Cannot use {operator} on a non-variable."),
        None,
    );
}

pub(in super::super) fn prefix(c: &mut Checker, u: &UnaryPrefix) {
    match u.operator {
        UnaryPrefixOperator::PreIncrement(_) => check(c, u.operand, "preInc", "++"),
        UnaryPrefixOperator::PreDecrement(_) => check(c, u.operand, "preDec", "--"),
        _ => {}
    }
}

pub(in super::super) fn postfix(c: &mut Checker, u: &UnaryPostfix) {
    match u.operator {
        UnaryPostfixOperator::PostIncrement(_) => check(c, u.operand, "postInc", "++"),
        UnaryPostfixOperator::PostDecrement(_) => check(c, u.operand, "postDec", "--"),
    }
}
