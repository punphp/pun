//! phpstan-strict-rules' operand rules: BooleanInBooleanAndRule, BooleanInBooleanOrRule and BooleanInBooleanNotRule
//! (`strictRules.booleansInConditions`, over BooleanRuleHelper) with the condition rules over the same helper
//! (BooleanInIf/ElseIf/TernaryOperatorRule, and BooleanInWhile/DoWhileConditionRule of
//! `strictRules.booleansInLoopConditions`), and the OperandsInArithmetic*Rule /
//! OperandInArithmetic*Rule of `strictRules.numericOperandsInArithmeticOperators` (over OperatorRuleHelper; unary
//! `-` and `+` only with bleeding edge).

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Accessory, Cast, Level, StrictMixed, Type, cast_is_error, intersect, is_super_type_of, union};

use super::{Checker, declared};

pub const IDENTIFIERS: &[&str] = &[
    "booleanAnd.leftNotBoolean",
    "booleanAnd.rightNotBoolean",
    "logicalAnd.leftNotBoolean",
    "logicalAnd.rightNotBoolean",
    "booleanOr.leftNotBoolean",
    "booleanOr.rightNotBoolean",
    "logicalOr.leftNotBoolean",
    "logicalOr.rightNotBoolean",
    "booleanNot.exprNotBoolean",
    "if.condNotBoolean",
    "elseif.condNotBoolean",
    "ternary.condNotBoolean",
    "while.condNotBoolean",
    "doWhile.condNotBoolean",
    "plus.leftNonNumeric",
    "plus.rightNonNumeric",
    "minus.leftNonNumeric",
    "minus.rightNonNumeric",
    "mul.leftNonNumeric",
    "mul.rightNonNumeric",
    "div.leftNonNumeric",
    "div.rightNonNumeric",
    "mod.leftNonNumeric",
    "mod.rightNonNumeric",
    "pow.leftNonNumeric",
    "pow.rightNonNumeric",
    "preInc.nonNumeric",
    "postInc.nonNumeric",
    "preDec.nonNumeric",
    "postDec.nonNumeric",
    "unaryMinus.nonNumeric",
    "unaryPlus.nonNumeric",
];

/// The expressions whose types the rules read: the operands of the operators the switches check.
pub(super) fn typed_expressions(strict: crate::StrictRules, program: &Program) -> Vec<Span> {
    let mut t = Typed {
        booleans: strict.booleans_in_conditions,
        loops: strict.booleans_in_loop_conditions,
        numbers: strict.numeric_operands_in_arithmetic_operators,
        spans: Vec::new(),
    };
    if t.booleans || t.loops || t.numbers {
        t.walk_program(program, &mut ());
    }
    t.spans
}

struct Typed {
    booleans: bool,
    loops: bool,
    numbers: bool,
    spans: Vec<Span>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Typed {
    fn walk_in_binary(&mut self, b: &'ast Binary<'arena>, _: &mut ()) {
        if (self.booleans && logical(&b.operator).is_some()) || (self.numbers && arithmetic(&b.operator).is_some()) {
            self.spans.extend([b.lhs.span(), b.rhs.span()]);
        }
    }
    fn walk_in_assignment(&mut self, a: &'ast Assignment<'arena>, _: &mut ()) {
        if self.numbers && assign_arithmetic(&a.operator).is_some() {
            self.spans.extend([a.lhs.span(), a.rhs.span()]);
        }
    }
    fn walk_in_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, _: &mut ()) {
        if (self.booleans && matches!(u.operator, UnaryPrefixOperator::Not(_))) || (self.numbers && prefix(&u.operator).is_some()) {
            self.spans.push(u.operand.span());
        }
    }
    fn walk_in_unary_postfix(&mut self, u: &'ast UnaryPostfix<'arena>, _: &mut ()) {
        if self.numbers {
            self.spans.push(u.operand.span());
        }
    }
    fn walk_in_if(&mut self, i: &'ast If<'arena>, _: &mut ()) {
        if self.booleans {
            self.spans.push(i.condition.span());
            self.spans.extend(else_if_conditions(i).map(|(_, e)| e.span()));
        }
    }
    fn walk_in_conditional(&mut self, t: &'ast Conditional<'arena>, _: &mut ()) {
        if self.booleans && t.then.is_some() {
            self.spans.push(t.condition.span());
        }
    }
    fn walk_in_while(&mut self, w: &'ast While<'arena>, _: &mut ()) {
        if self.loops {
            self.spans.push(w.condition.span());
        }
    }
    fn walk_in_do_while(&mut self, d: &'ast DoWhile<'arena>, _: &mut ()) {
        if self.loops {
            self.spans.push(d.condition.span());
        }
    }
}

/// The `elseif` clauses of an `if`, each with its condition (`else if` is an `if` of its own).
pub(super) fn else_if_conditions<'a, 'arena>(
    i: &'a If<'arena>,
) -> impl Iterator<Item = (Span, &'a Expression<'arena>)> {
    let (statement, colon) = match &i.body {
        IfBody::Statement(b) => (Some(b.else_if_clauses.iter().map(|e| (e.span(), e.condition))), None),
        IfBody::ColonDelimited(b) => (None, Some(b.else_if_clauses.iter().map(|e| (e.span(), e.condition)))),
    };
    statement.into_iter().flatten().chain(colon.into_iter().flatten())
}

/// The Boolean*ConditionRule of a statement or ternary whose condition is `cond`: `place` is its message's.
pub(super) fn condition(c: &mut Checker, at: Span, cond: &Expression, place: &str, identifier: &'static str) {
    let Some(t) = c.phpstan_type(cond) else {
        return;
    };
    if !passes_as_boolean(c, cond, &t) {
        let message = format!("Only booleans are allowed in {place}, {} given.", describe(c, &t));
        c.report(at, identifier, message, None);
    }
}

/// `&&`, `||`, `and`, `or`: the identifier prefix and `getOperatorSigil()`.
fn logical(op: &BinaryOperator) -> Option<(&'static str, &'static str)> {
    Some(match op {
        BinaryOperator::And(_) => ("booleanAnd", "&&"),
        BinaryOperator::LowAnd(_) => ("logicalAnd", "and"),
        BinaryOperator::Or(_) => ("booleanOr", "||"),
        BinaryOperator::LowOr(_) => ("logicalOr", "or"),
        _ => return None,
    })
}

fn arithmetic(op: &BinaryOperator) -> Option<(&'static str, &'static str)> {
    Some(match op {
        BinaryOperator::Addition(_) => ("plus", "+"),
        BinaryOperator::Subtraction(_) => ("minus", "-"),
        BinaryOperator::Multiplication(_) => ("mul", "*"),
        BinaryOperator::Division(_) => ("div", "/"),
        BinaryOperator::Modulo(_) => ("mod", "%"),
        BinaryOperator::Exponentiation(_) => ("pow", "**"),
        _ => return None,
    })
}

fn assign_arithmetic(op: &AssignmentOperator) -> Option<(&'static str, &'static str)> {
    Some(match op {
        AssignmentOperator::Addition(_) => ("plus", "+"),
        AssignmentOperator::Subtraction(_) => ("minus", "-"),
        AssignmentOperator::Multiplication(_) => ("mul", "*"),
        AssignmentOperator::Division(_) => ("div", "/"),
        AssignmentOperator::Modulo(_) => ("mod", "%"),
        AssignmentOperator::Exponentiation(_) => ("pow", "**"),
        _ => return None,
    })
}

#[derive(Clone, Copy, PartialEq)]
enum Unary {
    Increment,
    Decrement,
    Number,
}

/// The prefix operators the rules check: the identifier, the operation in messages and its helper.
fn prefix(op: &UnaryPrefixOperator) -> Option<(&'static str, &'static str, Unary)> {
    Some(match op {
        UnaryPrefixOperator::PreIncrement(_) => ("preInc.nonNumeric", "pre-increment", Unary::Increment),
        UnaryPrefixOperator::PreDecrement(_) => ("preDec.nonNumeric", "pre-decrement", Unary::Decrement),
        UnaryPrefixOperator::Negation(_) => ("unaryMinus.nonNumeric", "unary -", Unary::Number),
        UnaryPrefixOperator::Plus(_) => ("unaryPlus.nonNumeric", "unary +", Unary::Number),
        _ => return None,
    })
}

fn describe(c: &Checker, t: &Type) -> String {
    declared::with_classes(c, || t.describe(Level::TypeOnly))
}

/// `findTypeToCheck()`, which levels 0 and 1 (`checkThisOnly`) answer with an ErrorType but for `$this`.
fn type_to_check(c: &Checker, e: &Expression, criteria: &dyn Fn(&Type) -> bool) -> Option<Type> {
    let this = matches!(e, Expression::Variable(Variable::Direct(v)) if v.name == b"$this");
    if c.level < 2 && !this {
        return None;
    }
    c.find_type_to_check(e, criteria).ty
}

/// `$type instanceof MixedType` (a template bound by `mixed` is a TemplateMixedType, an explicit one).
fn as_mixed(t: &Type) -> Option<bool> {
    match t {
        Type::Mixed { explicit } => Some(*explicit),
        Type::Template(tt) if matches!(tt.bound, Type::Mixed { .. }) => Some(true),
        _ => None,
    }
}

/// BooleanRuleHelper::passesAsBoolean().
fn passes_as_boolean(c: &Checker, e: &Expression, t: &Type) -> bool {
    if let Some(explicit) = as_mixed(t) {
        return !explicit;
    }
    declared::with_classes(c, || {
        type_to_check(c, e, &|t| t.is_boolean().yes()).is_none_or(|found| found.is_boolean().yes())
    })
}

/// `$expr + 1` (or `++$expr`) of an object is not an ErrorType: GmpOperatorTypeSpecifyingExtension and
/// BcMathNumberOperatorTypeSpecifyingExtension.
fn object_operand(t: &Type) -> bool {
    let objects = Type::Union(vec![Type::Object("GMP".into(), vec![], false), Type::Object("BcMath\\Number".into(), vec![], false)]);
    is_super_type_of(&objects, t).yes()
}

/// OperatorRuleHelper::isSubtypeOfNumber().
fn is_subtype_of_number(c: &Checker, e: &Expression) -> bool {
    let accepted = union(vec![
        Type::Int,
        Type::Float,
        intersect(vec![Type::String, Type::Accessory(Accessory::NumericString)]),
    ]);
    let Some(t) = type_to_check(c, e, &|t| is_super_type_of(&accepted, t).yes()) else {
        return true;
    };
    let is = is_super_type_of(&accepted, &t);
    if matches!(t, Type::Benevolent(_)) { !is.no() } else { is.yes() }
}

/// OperatorRuleHelper::isValidForArithmeticOperation(), isValidForIncrement() and isValidForDecrement().
fn valid(c: &Checker, e: &Expression, t: &Type, op: Unary) -> bool {
    if as_mixed(t).is_some() {
        return true;
    }
    declared::with_classes(c, || {
        // (already reported by PHPStan core)
        if op == Unary::Number && cast_is_error(t, Cast::Number, StrictMixed::default()) {
            return true;
        }
        if op == Unary::Increment && t.is_string().yes() {
            return true;
        }
        is_subtype_of_number(c, e) || object_operand(t)
    })
}

/// `getArrays()` is not empty.
fn has_arrays(t: &Type) -> bool {
    match t {
        Type::Array(..) | Type::ConstArray(_) => true,
        Type::Union(ts) | Type::Benevolent(ts) => ts.iter().all(has_arrays),
        Type::Intersection(ts) => ts.iter().any(has_arrays),
        _ => false,
    }
}

fn operands(c: &mut Checker, at: Span, name: &str, sigil: &str, left: &Expression, right: &Expression, plus: bool) {
    let (Some(l), Some(r)) = (c.phpstan_type(left), c.phpstan_type(right)) else {
        return;
    };
    if plus && has_arrays(&l) && has_arrays(&r) {
        return;
    }
    for (side, e, t) in [("left", left, l), ("right", right, r)] {
        if !valid(c, e, &t, Unary::Number) {
            let message = format!("Only numeric types are allowed in {sigil}, {} given on the {side} side.", describe(c, &t));
            let identifier = format!("{name}.{side}NonNumeric");
            let identifier = IDENTIFIERS.iter().find(|i| **i == identifier).expect("an identifier per operator");
            c.report(at, identifier, message, None);
        }
    }
}

pub(super) fn binary(c: &mut Checker, b: &Binary) {
    if c.strict.numeric_operands_in_arithmetic_operators
        && let Some((name, sigil)) = arithmetic(&b.operator)
    {
        operands(c, b.span(), name, sigil, b.lhs, b.rhs, name == "plus");
    }
    let Some((name, sigil)) = logical(&b.operator).filter(|_| c.strict.booleans_in_conditions) else {
        return;
    };
    for (side, e) in [("left", b.lhs), ("right", b.rhs)] {
        let Some(t) = c.phpstan_type(e) else {
            continue;
        };
        if !passes_as_boolean(c, e, &t) {
            let message = format!("Only booleans are allowed in {sigil}, {} given on the {side} side.", describe(c, &t));
            let identifier = format!("{name}.{side}NotBoolean");
            let identifier = IDENTIFIERS.iter().find(|i| **i == identifier).expect("an identifier per operator");
            c.report(b, identifier, message, None);
        }
    }
}

pub(super) fn assignment(c: &mut Checker, a: &Assignment) {
    if c.strict.numeric_operands_in_arithmetic_operators
        && let Some((name, sigil)) = assign_arithmetic(&a.operator)
    {
        operands(c, a.span(), name, sigil, a.lhs, a.rhs, name == "plus");
    }
}

fn unary(c: &mut Checker, at: Span, operand: &Expression, identifier: &'static str, operation: &str, op: Unary) {
    let Some(t) = c.phpstan_type(operand) else {
        return;
    };
    if !valid(c, operand, &t, op) {
        let message = format!("Only numeric types are allowed in {operation}, {} given.", describe(c, &t));
        c.report(at, identifier, message, None);
    }
}

pub(super) fn unary_prefix(c: &mut Checker, u: &UnaryPrefix) {
    if let UnaryPrefixOperator::Not(_) = u.operator {
        if c.strict.booleans_in_conditions
            && let Some(t) = c.phpstan_type(u.operand)
            && !passes_as_boolean(c, u.operand, &t)
        {
            let message = format!("Only booleans are allowed in a negated boolean, {} given.", describe(c, &t));
            c.report(u, "booleanNot.exprNotBoolean", message, None);
        }
        return;
    }
    let Some((identifier, operation, op)) = prefix(&u.operator) else {
        return;
    };
    if c.strict.numeric_operands_in_arithmetic_operators && (op != Unary::Number || c.bleeding_edge) {
        unary(c, u.span(), u.operand, identifier, operation, op);
    }
}

pub(super) fn unary_postfix(c: &mut Checker, u: &UnaryPostfix) {
    if !c.strict.numeric_operands_in_arithmetic_operators {
        return;
    }
    let (identifier, operation, op) = match u.operator {
        UnaryPostfixOperator::PostIncrement(_) => ("postInc.nonNumeric", "post-increment", Unary::Increment),
        UnaryPostfixOperator::PostDecrement(_) => ("postDec.nonNumeric", "post-decrement", Unary::Decrement),
    };
    unary(c, u.span(), u.operand, identifier, operation, op);
}
