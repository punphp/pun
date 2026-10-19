//! `lint/dead-code/no-dead-php-version-check`, ported from Rector 2.6.1's `RemovePhpVersionIdCheckRector`
//! (rules/DeadCode/Rector/ConstFetch).
//!
//! An `if` whose condition compares `PHP_VERSION_ID` with an integer by `<`, `>` or `>=` that the project's minimum
//! PHP version decides: the `if` goes when its statements never run, and is replaced by them when they always do.
//! As Rector, `PHP_VERSION_ID > 80000` counts as true from PHP 8.0 on.
//!
//! Rector removes the `if` with its `else` and `elseif` branches when the statements never run, which drops the
//! branch that does; Pun leaves such an `if` alone. Where the statements always run, the other branches are dead
//! and go, as in Rector.

use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::rector_batch5::replace_with;
use super::rector_stmts::if_view;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoDeadPhpVersionCheck;

impl Rule for NoDeadPhpVersionCheck {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-dead-php-version-check"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Statement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Statement(s @ Statement::If(r#if)) = node else {
            return;
        };
        let Some(runs) = decided(r#if.condition, i64::from(ctx.php_version)) else {
            return;
        };
        let source = ctx.source;
        let view = if_view(source, r#if);
        if !runs && (view.else_.is_some() || !view.elseifs.is_empty()) {
            return;
        }
        let version = i64::from(ctx.php_version);
        let edits = replace_with(source, s, ancestors, runs.then_some(&view.stmts), |other| {
            let Statement::If(i) = other else {
                return false;
            };
            let view = if_view(source, i);
            decided(i.condition, version)
                .is_some_and(|runs| runs || (view.else_.is_none() && view.elseifs.is_empty()))
        });
        out.push(Report {
            span: node_span(s),
            message: "This `PHP_VERSION_ID` check is decided by the project's minimum PHP version."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `refactor()` with `refactorConstFetch` and its helpers: `Some(true)` when the statements are returned,
/// `Some(false)` for `REMOVE_NODE`.
fn decided(cond: &Expression<'_>, version: i64) -> Option<bool> {
    let Expression::Binary(b) = unparenthesize(cond) else {
        return None;
    };
    let (left, right) = (unparenthesize(b.lhs), unparenthesize(b.rhs));
    let on_left = is_php_version_id(left);
    if !on_left && !is_php_version_id(right) {
        return None;
    }
    let value = integer(if on_left { right } else { left })?;
    match (&b.operator, on_left) {
        // `PHP_VERSION_ID < 80000`, `80000 >= PHP_VERSION_ID`, `80000 > PHP_VERSION_ID`: never from that version on.
        (BinaryOperator::LessThan(_), true)
        | (BinaryOperator::GreaterThanOrEqual(_), false)
        | (BinaryOperator::GreaterThan(_), false) => (version >= value).then_some(false),
        // `80000 < PHP_VERSION_ID`, `PHP_VERSION_ID >= 80000`, `PHP_VERSION_ID > 80000`: always.
        (BinaryOperator::LessThan(_), false)
        | (BinaryOperator::GreaterThanOrEqual(_), true)
        | (BinaryOperator::GreaterThan(_), true) => (version >= value).then_some(true),
        _ => None,
    }
}

/// `$this->isName($constFetch->name, 'PHP_VERSION_ID')`.
fn is_php_version_id(expr: &Expression<'_>) -> bool {
    let Expression::ConstantAccess(c) = expr else {
        return false;
    };
    let name = c.name.value();
    name.strip_prefix(b"\\")
        .unwrap_or(name)
        .eq_ignore_ascii_case(b"PHP_VERSION_ID")
}

/// php-parser's `Int_` value.
fn integer(expr: &Expression<'_>) -> Option<i64> {
    match expr {
        Expression::Literal(Literal::Integer(i)) => i64::try_from(i.value?).ok(),
        _ => None,
    }
}
