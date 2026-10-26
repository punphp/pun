//! Small report rules on calls and names, ported from Mago 1.57.0's linter (mago-linter src/rule/):
//!
//! - `lint/functions/require-assert-description` (`assert-description`, correctness/assert_description.rs): an
//!   `assert()` call without a second argument.
//! - `lint/style/no-ini-set` (`no-ini-set`, best_practices/no_ini_set.rs): a call to `ini_set` or `ini_alter`.
//! - `lint/style/no-sprintf-concat` (`no-sprintf-concat`, best_practices/no_sprintf_concat.rs): a `.` with a
//!   `sprintf()` call on either side, reported on the operator.
//! - `lint/style/no-ambiguous-function` (`ambiguous-function-call`, consistency/ambiguous_function_call.rs): an
//!   unqualified, unimported function call inside a named namespace, except to the names PHP never namespaces.
//! - `lint/style/no-ambiguous-constant` (`ambiguous-constant-access`, consistency/ambiguous_constant_access.rs): the
//!   same for a constant, except `true`, `false`, `null`, `STDIN`, `STDOUT` and `STDERR`.
//! - `lint/style/prefer-create-from-timestamp` (`prefer-datetimeimmutable-create-from-timestamp`,
//!   best_practices/prefer_timestamp_factory.rs; PHP 8.4 and later): `new DateTimeImmutable('@' . $x)`.
//!
//! A function name matches as Mago's `function_call_matches` does (lint_b_names.rs).

use mago_span::HasSpan;
use mago_syntax::cst::{Argument, BinaryOperator, Call, Expression, Literal, Node, NodeKind};

use super::lint_b_names::{function_call_matches, is_imported, resolve_class};
use super::mago_helpers::namespace;
use crate::syntax::{node_span, span};
use crate::{Context, Report, Rule};

fn report(at: pun_problem::Span, message: &str, help: String) -> Report {
    Report {
        span: at,
        message: message.into(),
        help: Some(help),
        fix: None,
    }
}

pub struct RequireAssertDescription;

impl Rule for RequireAssertDescription {
    fn id(&self) -> &'static str {
        "lint/functions/require-assert-description"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        if function_call_matches(ancestors, call, "assert")
            && call.argument_list.arguments.get(1).is_none()
        {
            out.push(report(
                node_span(call),
                "Missing description in assert function.",
                "Add a description to the assert function to clarify its purpose.".into(),
            ));
        }
    }
}

pub struct NoIniSet;

impl Rule for NoIniSet {
    fn id(&self) -> &'static str {
        "lint/style/no-ini-set"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        if function_call_matches(ancestors, call, "ini_set")
            || function_call_matches(ancestors, call, "ini_alter")
        {
            out.push(report(
                node_span(call),
                "ini_set should not be used.",
                "Use a framework specific configuration instead of `ini_set`.".into(),
            ));
        }
    }
}

pub struct NoSprintfConcat;

impl Rule for NoSprintfConcat {
    fn id(&self) -> &'static str {
        "lint/style/no-sprintf-concat"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(binary) = node else { return };
        if !matches!(binary.operator, BinaryOperator::StringConcat(_)) {
            return;
        }
        let is_sprintf = |e: &Expression<'_>| matches!(e, Expression::Call(Call::Function(call)) if function_call_matches(ancestors, call, "sprintf"));
        if is_sprintf(binary.lhs) || is_sprintf(binary.rhs) {
            out.push(report(
                span(binary.operator.span()),
                "String concatenation with `sprintf` can be simplified.",
                "Incorporate the concatenated content into the `sprintf` format argument.".into(),
            ));
        }
    }
}

pub struct NoAmbiguousFunction;

impl Rule for NoAmbiguousFunction {
    fn id(&self) -> &'static str {
        "lint/style/no-ambiguous-function"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        let Expression::Identifier(identifier) = call.function else {
            return;
        };
        if namespace(ancestors).is_empty()
            || identifier.is_qualified()
            || identifier.is_fully_qualified()
            || mago_names::is_never_namespaced_function(identifier.value())
            || is_imported(ancestors, identifier, false)
        {
            return;
        }
        let name = String::from_utf8_lossy(identifier.value());
        out.push(report(
            node_span(identifier),
            "Ambiguous function call detected.",
            format!(
                "Make the call explicit: for global functions, use `\\{name}(...)` or add a `use function {name};` statement."
            ),
        ));
    }
}

pub struct NoAmbiguousConstant;

impl Rule for NoAmbiguousConstant {
    fn id(&self) -> &'static str {
        "lint/style/no-ambiguous-constant"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ConstantAccess])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::ConstantAccess(access) = node else {
            return;
        };
        let identifier = &access.name;
        if namespace(ancestors).is_empty()
            || identifier.is_qualified()
            || identifier.is_fully_qualified()
            || is_imported(ancestors, identifier, true)
        {
            return;
        }
        let lower = identifier.value().to_ascii_lowercase();
        if matches!(
            lower.as_slice(),
            b"true" | b"false" | b"null" | b"stdin" | b"stdout" | b"stderr"
        ) {
            return;
        }
        let name = String::from_utf8_lossy(identifier.value());
        out.push(report(
            node_span(identifier),
            "Ambiguous constant access detected.",
            format!("Make the reference explicit: use `\\{name}` or add a `use const {name};` statement."),
        ));
    }
}

pub struct PreferCreateFromTimestamp;

impl Rule for PreferCreateFromTimestamp {
    fn id(&self) -> &'static str {
        "lint/style/prefer-create-from-timestamp"
    }

    fn min_php_version(&self) -> u32 {
        80400
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Instantiation])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Instantiation(new) = node else {
            return;
        };
        let Expression::Identifier(identifier) = new.class else {
            return;
        };
        if !resolve_class(ancestors, identifier).eq_ignore_ascii_case(b"datetimeimmutable") {
            return;
        }
        let Some(list) = &new.argument_list else {
            return;
        };
        let [Argument::Positional(argument)] = list.arguments.as_slice() else {
            return;
        };
        if argument.ellipsis.is_some() {
            return;
        }
        let Expression::Binary(binary) = argument.value else {
            return;
        };
        if !matches!(binary.operator, BinaryOperator::StringConcat(_)) {
            return;
        }
        let Expression::Literal(Literal::String(prefix)) = binary.lhs else {
            return;
        };
        if prefix.value != Some(b"@".as_slice()) {
            return;
        }
        out.push(report(
            node_span(new),
            "Use `DateTimeImmutable::createFromTimestamp()` instead of parsing a timestamp string.",
            "Use `DateTimeImmutable::createFromTimestamp($timestamp)` instead.".into(),
        ));
    }
}
