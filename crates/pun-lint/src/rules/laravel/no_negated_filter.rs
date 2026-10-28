//! `lint/laravel/no-negated-filter`, ported from rector-laravel 2.5.0's `AvoidNegatedCollectionFilterOrRejectRector`
//! (src/Rector/MethodCall).
//!
//! On an `Illuminate\Support\Enumerable`, `filter(fn (...) => !$x)` becomes `reject(fn (...): bool => (bool) $x)`, and
//! `reject` becomes `filter` the same way. The `!` goes; its operand is cast to `bool` unless its type is a boolean,
//! and the arrow function's return type becomes `bool`. Only a call with one argument, an arrow function, counts.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::object_type::is_object_type;
use super::{is_name, member_name};
use crate::rules::rector_code_quality::{operand, text};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoNegatedFilter;

impl Rule for NoNegatedFilter {
    fn id(&self) -> &'static str {
        "lint/laravel/no-negated-filter"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else { return };
        let Some(name) = member_name(&call.method) else {
            return;
        };
        let (from, to) = if is_name(name, "filter") {
            ("filter", "reject")
        } else if is_name(name, "reject") {
            ("reject", "filter")
        } else {
            return;
        };
        let [argument] = call.argument_list.arguments.as_slice() else {
            return;
        };
        let Expression::ArrowFunction(arrow) = unparenthesize(argument.value()) else {
            return;
        };
        let Expression::UnaryPrefix(not) = unparenthesize(arrow.expression) else {
            return;
        };
        if !not.operator.is_not() {
            return;
        }
        if !is_object_type(
            ctx,
            ancestors,
            call.object,
            "Illuminate\\Support\\Enumerable",
        ) {
            return;
        }
        let inner = unparenthesize(not.operand);
        let is_boolean = facts(ctx).full_type(node_span(inner)).is_some_and(|t| {
            !t.types.is_empty()
                && t.types
                    .iter()
                    .all(|a| matches!(a, TAtomic::Scalar(TScalar::Bool(_))))
        });
        let body = if is_boolean {
            text(ctx, inner).to_string()
        } else {
            format!("(bool) {}", operand(ctx, inner, 10, None))
        };
        let return_type = match &arrow.return_type_hint {
            Some(hint) => Edit::new(node_span(&hint.hint), "bool"),
            None => {
                let at = node_span(&arrow.parameter_list).end;
                Edit::new(Span::new(at, at), ": bool")
            }
        };
        out.push(Report {
            span: node_span(call),
            message: format!("Use `{to}()` instead of `{from}()` with a negated condition."),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits: vec![
                    Edit::new(node_span(&call.method), to),
                    return_type,
                    Edit::new(node_span(not), body),
                ],
            }),
        });
    }
}
