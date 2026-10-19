//! `lint/dead-code/no-future-php-version-check`, ported from Rector 2.6.1's `UnwrapFutureCompatibleIfPhpVersionRector`
//! (rules/DeadCode/Rector/If_), with `ConditionResolver` and `ConditionEvaluator` (rules/DeadCode).
//!
//! An `if` without `elseif` whose condition is a `version_compare()` that the project's minimum PHP version decides
//! is replaced by the branch that runs: the `if` statements when it holds, else the `else` statements, or nothing.
//! `PHP_VERSION` and `PHP_VERSION_ID` stand for that minimum version. A `version_compare(..., '<')` against a version
//! above the minimum is left alone, since newer PHP versions decide it the other way.
//!
//! Rector reads a version or an expected value that is not a literal from its type; Pun reads literals only and
//! leaves the `if` alone otherwise (lint.md Build 8).

use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::rector_batch5::{Value, float_string, int_version, replace_with, value};
use super::rector_code_quality::call_to;
use super::rector_stmts::if_view;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoFuturePhpVersionCheck;

impl Rule for NoFuturePhpVersionCheck {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-future-php-version-check"
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
        let source = ctx.source;
        let view = if_view(source, r#if);
        if !view.elseifs.is_empty() {
            return;
        }
        let version = i64::from(ctx.php_version);
        let Some(holds) = evaluate(r#if.condition, version) else {
            return;
        };
        let stmts = if holds {
            Some(&view.stmts)
        } else {
            view.else_.as_ref().map(|(list, _)| list)
        };
        let edits = replace_with(
            source,
            s,
            ancestors,
            stmts,
            |other| matches!(other, Statement::If(i) if if_view(source, i).elseifs.is_empty() && evaluate(i.condition, version).is_some()),
        );
        out.push(Report {
            span: node_span(s),
            message: "This PHP version check is decided by the project's minimum PHP version."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// The result of `version_compare()`: a boolean with a compare sign, else -1, 0 or 1.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Compared {
    Bool(bool),
    Int(i64),
}

/// `ConditionResolver::resolveFromExpr` and `ConditionEvaluator::evaluate`, then `refactor()`'s reading of the
/// result: `Some(true)` when the `if` statements run, `Some(false)` when they do not.
fn evaluate(cond: &Expression<'_>, version: i64) -> Option<bool> {
    let cond = unparenthesize(cond);
    let result = if let Some(call) = call_to(cond, "version_compare") {
        compare(call, version)?
    } else {
        let Expression::Binary(b) = cond else {
            return None;
        };
        let (call, other) = match (
            call_to(b.lhs, "version_compare"),
            call_to(b.rhs, "version_compare"),
        ) {
            (Some(call), _) => (call, b.rhs),
            (None, Some(call)) => (call, b.lhs),
            _ => return None,
        };
        let negated = match b.operator {
            BinaryOperator::Identical(_) | BinaryOperator::Equal(_) => false,
            BinaryOperator::NotIdentical(_)
            | BinaryOperator::NotEqual(_)
            | BinaryOperator::AngledNotEqual(_) => true,
            _ => return None,
        };
        let result = compare(call, version)?;
        // `isEvaluatedAsTrue`: the expected value compared with `===` (`!==`), whatever the operator.
        let expected = value(other)?;
        let same = match (expected, result) {
            (Value::Bool(e), Compared::Bool(r)) => e == r,
            (Value::Int(e), Compared::Int(r)) => e == r,
            // `ValueResolver` reads a constant as its name, a string.
            _ => false,
        };
        Compared::Bool(same != negated)
    };
    Some(match result {
        Compared::Bool(b) => b,
        Compared::Int(i) => i > 0,
    })
}

/// `evaluateVersionCompareCondition` for a `version_compare()` call.
fn compare(call: &FunctionCall<'_>, version: i64) -> Option<Compared> {
    let args: Vec<&Expression> = call
        .argument_list
        .arguments
        .iter()
        .map(|a| match a {
            Argument::Positional(a) => a.value,
            Argument::Named(a) => a.value,
        })
        .collect();
    let first = argument_version(args.first()?, version)?;
    let second = argument_version(args.get(1)?, version)?;
    let Some(sign) = args.get(2) else {
        return Some(Compared::Int(
            (first > second) as i64 - (first < second) as i64,
        ));
    };
    let Some(Value::Str(sign)) = value(sign) else {
        return None;
    };
    if sign == "<" && version < second {
        return None;
    }
    Some(Compared::Bool(match sign.as_str() {
        "<" | "lt" => first < second,
        "<=" | "le" => first <= second,
        ">" | "gt" => first > second,
        ">=" | "ge" => first >= second,
        "==" | "eq" => first == second,
        "!=" | "<>" | "ne" => first != second,
        // version_compare() throws on any other sign.
        _ => return None,
    }))
}

/// `resolveArgumentValue`: `PHP_VERSION` and `PHP_VERSION_ID` are the project's version, a string or float the
/// version it writes, an integer itself.
fn argument_version(expr: &Expression<'_>, version: i64) -> Option<i64> {
    match value(expr)? {
        Value::Constant(name) | Value::Str(name)
            if name == "PHP_VERSION" || name == "PHP_VERSION_ID" =>
        {
            Some(version)
        }
        Value::Str(s) => Some(int_version(&s)),
        Value::Float(f) => Some(int_version(&float_string(f)?)),
        Value::Int(i) => Some(i),
        _ => None,
    }
}
