//! `lint/operators/prefer-boolean-and-or`, ported from Rector 2.6.1's `LogicalToBooleanRector`
//! (rules/CodeQuality/Rector/LogicalAnd).
//!
//! `and` and `or` become `&&` and `||`, in the whole chain of `and`/`or` operations the outermost one heads. The new
//! operators bind tighter than assignments, so an assignment operand gets parentheses (`$f = false or true` becomes
//! `($f = false) || true`). Rector reprints the chain the way php-parser's pretty printer does: every binary operation
//! in it on one line, an assignment, a compound assignment, a ternary, `print` or `yield` operand of `&&`/`||` in
//! parentheses where precedence needs them, the other operands as they were written but without their parentheses
//! and comments. Pun writes the same.
//!
//! Rector skips the chain when its left operand is an assignment whose native type is not a boolean (`$file =
//! fopen(...) or die()`). Pun reads that type from the syntax alone: the assigned value is `true`, `false`, a
//! comparison, a `!`, a boolean operation, `isset()`, `empty()` or a `(bool)` cast. Any other value (a call, a
//! variable) is skipped, also where PHPStan would know it is a boolean; that waits for analyse's types (lint.md
//! Build 8).

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_code_quality::{binary, is_constant, operand as kept_operand, prec, text, wrap};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferBooleanAndOr;

impl Rule for PreferBooleanAndOr {
    fn id(&self) -> &'static str {
        "lint/operators/prefer-boolean-and-or"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(b) = node else { return };
        if !is_logical(&b.operator) || !refactors(b) || in_refactored_chain(b, ancestors) {
            return;
        }
        let code = Printer { ctx }.infix(b, MAX, MAX);
        out.push(Report {
            span: node_span(b),
            message: "Use `&&` and `||` instead of `and` and `or`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![Edit::new(node_span(b), code)],
            }),
        });
    }
}

/// php-parser's `MAX_PRECEDENCE`.
const MAX: i32 = 1000;

fn is_logical(op: &BinaryOperator<'_>) -> bool {
    matches!(op, BinaryOperator::LowAnd(_) | BinaryOperator::LowOr(_))
}

/// `refactor()`'s check: a left operand that is an assignment must have a boolean native type.
fn refactors(b: &Binary<'_>) -> bool {
    match unparenthesize(b.lhs) {
        Expression::Assignment(a) if is_plain_assign(a) => native_bool(a.rhs),
        _ => true,
    }
}

/// Whether an `and`/`or` above `b`, through `and`/`or` operands only, is refactored: `refactorLogicalToBoolean` then
/// turns `b` with it (Rector visits the outer node first), and `b` is not refactored again.
fn in_refactored_chain(b: &Binary<'_>, ancestors: &[Node<'_, '_>]) -> bool {
    let mut child = node_span(b);
    for ancestor in ancestors.iter().rev() {
        match ancestor {
            Node::Expression(_) | Node::Parenthesized(_) => continue,
            Node::Binary(parent)
                if is_logical(&parent.operator)
                    && [parent.lhs, parent.rhs]
                        .into_iter()
                        .any(|e| node_span(unparenthesize(e)) == child) =>
            {
                if refactors(parent) {
                    return true;
                }
                child = node_span(*parent);
            }
            _ => return false,
        }
    }
    false
}

/// An `Assign` node of php-parser: `=`, not `=&` (`AssignRef`) nor a compound assignment (`AssignOp`).
fn is_plain_assign(a: &Assignment<'_>) -> bool {
    matches!(a.operator, AssignmentOperator::Assign(_))
        && !matches!(a.rhs, Expression::UnaryPrefix(u) if u.operator.is_reference())
}

/// Whether PHPStan's native type of `expr` is a boolean, decided from the syntax alone.
fn native_bool(expr: &Expression<'_>) -> bool {
    match unparenthesize(expr) {
        e if is_constant(e, "true") || is_constant(e, "false") => true,
        Expression::Binary(b) => !matches!(
            b.operator,
            BinaryOperator::Addition(_)
                | BinaryOperator::Subtraction(_)
                | BinaryOperator::Multiplication(_)
                | BinaryOperator::Division(_)
                | BinaryOperator::Modulo(_)
                | BinaryOperator::Exponentiation(_)
                | BinaryOperator::BitwiseAnd(_)
                | BinaryOperator::BitwiseOr(_)
                | BinaryOperator::BitwiseXor(_)
                | BinaryOperator::LeftShift(_)
                | BinaryOperator::RightShift(_)
                | BinaryOperator::NullCoalesce(_)
                | BinaryOperator::Spaceship(_)
                | BinaryOperator::StringConcat(_)
        ),
        Expression::UnaryPrefix(u) => matches!(
            u.operator,
            UnaryPrefixOperator::Not(_)
                | UnaryPrefixOperator::BoolCast(..)
                | UnaryPrefixOperator::BooleanCast(..)
        ),
        Expression::Construct(Construct::Isset(_) | Construct::Empty(_)) => true,
        Expression::Assignment(a) => is_plain_assign(a) && native_bool(a.rhs),
        _ => false,
    }
}

/// php-parser's text of a binary operator it prints anew; `and`/`or` are printed as `&&`/`||`.
fn operator(op: &BinaryOperator<'_>) -> &'static str {
    match op {
        BinaryOperator::Addition(_) => "+",
        BinaryOperator::Subtraction(_) => "-",
        BinaryOperator::Multiplication(_) => "*",
        BinaryOperator::Division(_) => "/",
        BinaryOperator::Modulo(_) => "%",
        BinaryOperator::Exponentiation(_) => "**",
        BinaryOperator::BitwiseAnd(_) => "&",
        BinaryOperator::BitwiseOr(_) => "|",
        BinaryOperator::BitwiseXor(_) => "^",
        BinaryOperator::LeftShift(_) => "<<",
        BinaryOperator::RightShift(_) => ">>",
        BinaryOperator::NullCoalesce(_) => "??",
        BinaryOperator::Equal(_) => "==",
        BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => "!=",
        BinaryOperator::Identical(_) => "===",
        BinaryOperator::NotIdentical(_) => "!==",
        BinaryOperator::LessThan(_) => "<",
        BinaryOperator::LessThanOrEqual(_) => "<=",
        BinaryOperator::GreaterThan(_) => ">",
        BinaryOperator::GreaterThanOrEqual(_) => ">=",
        BinaryOperator::Spaceship(_) => "<=>",
        BinaryOperator::StringConcat(_) => ".",
        BinaryOperator::Instanceof(_) => "instanceof",
        BinaryOperator::And(_) | BinaryOperator::LowAnd(_) => "&&",
        BinaryOperator::Or(_) | BinaryOperator::LowOr(_) => "||",
        BinaryOperator::LowXor(_) => "xor",
    }
}

/// The `precedenceMap` entry of a binary operator as it is printed (`and`/`or` as `&&`/`||`).
fn limits(op: &BinaryOperator<'_>) -> (i32, i32, i32) {
    match op {
        BinaryOperator::LowAnd(_) => (120, 121, 120),
        BinaryOperator::LowOr(_) => (130, 131, 130),
        _ => binary(op),
    }
}

fn is_boolean(op: &BinaryOperator<'_>) -> bool {
    matches!(
        op,
        BinaryOperator::And(_)
            | BinaryOperator::Or(_)
            | BinaryOperator::LowAnd(_)
            | BinaryOperator::LowOr(_)
    )
}

/// The text of an assignment operator.
fn assignment_operator(a: &Assignment<'_>) -> &'static str {
    match a.operator {
        AssignmentOperator::Assign(_) if is_plain_assign(a) => "=",
        AssignmentOperator::Assign(_) => "=&",
        AssignmentOperator::Addition(_) => "+=",
        AssignmentOperator::Subtraction(_) => "-=",
        AssignmentOperator::Multiplication(_) => "*=",
        AssignmentOperator::Division(_) => "/=",
        AssignmentOperator::Modulo(_) => "%=",
        AssignmentOperator::Exponentiation(_) => "**=",
        AssignmentOperator::Concat(_) => ".=",
        AssignmentOperator::BitwiseAnd(_) => "&=",
        AssignmentOperator::BitwiseOr(_) => "|=",
        AssignmentOperator::BitwiseXor(_) => "^=",
        AssignmentOperator::LeftShift(_) => "<<=",
        AssignmentOperator::RightShift(_) => ">>=",
        AssignmentOperator::Coalesce(_) => "??=",
    }
}

struct Printer<'c, 's> {
    ctx: &'c Context<'s>,
}

impl Printer<'_, '_> {
    /// `pInfixOp` for a binary operation printed anew: the new `&&`/`||`, and every binary operation under one
    /// (`BetterStandardPrinter::wrapBinaryOpWithBrackets` drops their original formatting).
    fn infix(&self, b: &Binary<'_>, precedence: i32, lhs_precedence: i32) -> String {
        let (op, l, r) = limits(&b.operator);
        let (open, close, lhs_precedence) = if op >= precedence {
            ("(", ")", MAX)
        } else {
            ("", "", lhs_precedence)
        };
        let boolean = is_boolean(&b.operator);
        format!(
            "{open}{} {} {}{close}",
            self.operand(b.lhs, l, l, boolean),
            operator(&b.operator),
            self.operand(b.rhs, r, lhs_precedence, boolean),
        )
    }

    /// An operand of a binary operation printed anew (`p($node, $precedence, $lhsPrecedence)`). Binary operations
    /// and assignments are printed anew too (`reprintNestedAssign`); under `&&`/`||`, so are ternaries, `print` and
    /// `yield` (`reprintLowerPrecedenceOperand`). Anything else keeps its code, without its parentheses unless its new
    /// place needs them.
    fn operand(
        &self,
        expr: &Expression<'_>,
        precedence: i32,
        lhs_precedence: i32,
        boolean: bool,
    ) -> String {
        match unparenthesize(expr) {
            Expression::Binary(b) if !matches!(b.operator, BinaryOperator::Instanceof(_)) => {
                self.infix(b, precedence, lhs_precedence)
            }
            Expression::Assignment(a) => self.assignment(a, lhs_precedence),
            Expression::Conditional(c) if boolean => self.ternary(c),
            Expression::Construct(Construct::Print(p)) if boolean => {
                self.prefix("print ", 180, p.value, lhs_precedence)
            }
            Expression::Yield(Yield::From(y)) if boolean => {
                self.prefix("yield from ", 170, y.iterator, lhs_precedence)
            }
            // `yield <value>` gets `WRAPPED_IN_PARENTHESES`.
            Expression::Yield(
                y @ (Yield::Value(YieldValue { value: Some(_), .. }) | Yield::Pair(_)),
            ) if boolean => {
                format!("({})", text(self.ctx, y))
            }
            inner => self.kept(inner, precedence, lhs_precedence),
        }
    }

    /// A node php-parser prints with its original formatting. A prefix operator whose operand is an assignment
    /// Rector reprints (`!$y = 42`, `(int) $t = 42`) gets parentheses around it through `FIXUP_PREC_UNARY`.
    fn kept(&self, expr: &Expression<'_>, precedence: i32, lhs_precedence: i32) -> String {
        if let Some(code) = self.unary_assignment(expr) {
            return code;
        }
        // php-parser adds no parentheses here; Pun keeps those that the meaning needs.
        wrap(
            text(self.ctx, expr),
            prec(expr),
            precedence,
            Some(lhs_precedence),
        )
    }

    /// `!`, `~`, `-`, `+`, `@` or a cast over (such operators over) an assignment, the assignment in parentheses.
    fn unary_assignment(&self, expr: &Expression<'_>) -> Option<String> {
        let Expression::UnaryPrefix(u) = expr else {
            return None;
        };
        if matches!(
            u.operator,
            UnaryPrefixOperator::Reference(_)
                | UnaryPrefixOperator::PreIncrement(_)
                | UnaryPrefixOperator::PreDecrement(_)
        ) {
            return None;
        }
        let operand = unparenthesize(u.operand);
        let inner = match operand {
            Expression::Assignment(a) => format!("({})", self.assignment(a, MAX)),
            _ => self.unary_assignment(operand)?,
        };
        let before = &self.ctx.source[node_span(u).start..node_span(u.operand).start];
        Some(format!("{before}{inner}"))
    }

    /// `pAssignOp`, through `pPrefixOp`: in parentheses at or above the left limit, its operands as written.
    fn assignment(&self, a: &Assignment<'_>, lhs_precedence: i32) -> String {
        let rhs = match a.rhs {
            Expression::UnaryPrefix(u) if u.operator.is_reference() => u.operand,
            rhs => rhs,
        };
        let code = format!(
            "{} {} {}",
            text(self.ctx, unparenthesize(a.lhs)),
            assignment_operator(a),
            kept_operand(self.ctx, rhs, 160, None),
        );
        if 160 >= lhs_precedence {
            format!("({code})")
        } else {
            code
        }
    }

    /// `pExpr_Ternary`, through `pInfixOp`; its operands keep their code. Only an operand of `&&`/`||` gets here, which
    /// binds tighter than `?:`, so it is in parentheses.
    fn ternary(&self, c: &Conditional<'_>) -> String {
        let condition = kept_operand(self.ctx, c.condition, 150, Some(150));
        let otherwise = kept_operand(self.ctx, c.r#else, 150, None);
        let code = match c.then {
            Some(then) => format!(
                "{condition} ? {} : {otherwise}",
                text(self.ctx, unparenthesize(then))
            ),
            None => format!("{condition} ?: {otherwise}"),
        };
        format!("({code})")
    }

    /// `pPrefixOp`: in parentheses at or above the left limit.
    fn prefix(
        &self,
        op: &str,
        precedence: i32,
        operand: &Expression<'_>,
        lhs_precedence: i32,
    ) -> String {
        let code = format!("{op}{}", text(self.ctx, unparenthesize(operand)));
        if precedence >= lhs_precedence {
            format!("({code})")
        } else {
            code
        }
    }
}
