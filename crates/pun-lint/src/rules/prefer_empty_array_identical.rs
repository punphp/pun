//! `lint/complexity/prefer-empty-array-identical`, ported from Rector 2.6.1's `SimplifyEmptyCheckOnEmptyArrayRector`
//! (rules/CodeQuality/Rector/Empty_).
//!
//! `empty($array)` is `$array === []` and `!empty($array)` is `$array !== []` when `$array` is an array: an `array`
//! parameter, a local only assigned array literals, or an own property declared `array` (`syntax_types::is_array`).

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::rector_batch5::called_function_name;
use super::rector_stmts::{if_view, is_exit_or_throw};
use super::syntax_types::{in_trait, is_array};
use crate::syntax::{node_span, same_expression, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferEmptyArrayIdentical;

impl Rule for PreferEmptyArrayIdentical {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-empty-array-identical"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::EmptyConstruct, NodeKind::UnaryPrefix])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let (span, empty, negated) = match node {
            Node::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
                let Expression::Construct(Construct::Empty(empty)) = unparenthesize(u.operand)
                else {
                    return;
                };
                (node_span(u), empty, true)
            }
            Node::EmptyConstruct(empty) => {
                // `!empty(...)` is the `!`'s.
                let negated = ancestors.iter().rev().find(|n| {
                    !matches!(
                        n,
                        Node::Expression(_) | Node::Construct(_) | Node::Parenthesized(_)
                    )
                });
                if matches!(negated, Some(Node::UnaryPrefix(u)) if matches!(u.operator, UnaryPrefixOperator::Not(_)))
                {
                    return;
                }
                (node_span(empty), empty, false)
            }
            _ => return,
        };
        let value = unparenthesize(empty.value);
        if matches!(value, Expression::Variable(Variable::Direct(v)) if is_superglobal(v.name)) {
            return;
        }
        let source = ctx.source;
        if !is_array(source, ancestors, value)
            && !narrowed(source, ancestors, node_span(empty), value)
        {
            return;
        }
        let s = node_span(value);
        let text = &source[s.start..s.end];
        let operator = if negated { "!==" } else { "===" };
        out.push(Report {
            span,
            message: "Compare the array with `[]` instead of using `empty()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, format!("{text} {operator} []"))],
            }),
        });
    }
}

/// `ReservedKeywordAnalyzer::isNativeVariable`.
fn is_superglobal(name: &[u8]) -> bool {
    matches!(
        name,
        b"$GLOBALS"
            | b"$_SERVER"
            | b"$_GET"
            | b"$_POST"
            | b"$_FILES"
            | b"$_COOKIE"
            | b"$_SESSION"
            | b"$_REQUEST"
            | b"$_ENV"
    )
}

/// Whether an earlier statement of the function is `if (!is_array($v)) { ...; return; }`, after which PHPStan narrows
/// the native type to `array`. Rector 2.6.1 run on its own leaves `is_array($v) && empty($v)` alone (its
/// `replace_is_array_check` fixture says otherwise), so that guard does not count.
fn narrowed(
    source: &str,
    ancestors: &[Node<'_, '_>],
    at: pun_problem::Span,
    value: &Expression<'_>,
) -> bool {
    let checks = |e: &Expression<'_>| match unparenthesize(e) {
        Expression::Call(Call::Function(call)) => {
            called_function_name(ancestors, call.function)
                .is_some_and(|n| n.eq_ignore_ascii_case("is_array"))
                && matches!(call.argument_list.arguments.first(),
                    Some(Argument::Positional(a)) if a.ellipsis.is_none() && same_expression(source, a.value, value))
        }
        _ => false,
    };
    if in_trait(ancestors) {
        return false;
    }
    let body = ancestors.iter().rev().find_map(|n| match n {
        Node::Function(f) => Some(&f.body),
        Node::Method(m) => match &m.body {
            MethodBody::Concrete(b) => Some(b),
            _ => None,
        },
        Node::Closure(c) => Some(&c.body),
        _ => None,
    });
    let Some(body) = body else {
        return false;
    };
    for statement in body.statements.iter() {
        if node_span(statement).end > at.start {
            break;
        }
        let Statement::If(r#if) = statement else {
            continue;
        };
        let view = if_view(source, r#if);
        let negated_check = matches!(unparenthesize(view.condition),
            Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) && checks(u.operand));
        let exits = match view.stmts.last() {
            Some(Statement::Return(_)) => true,
            Some(Statement::Expression(e)) => is_exit_or_throw(e.expression),
            _ => false,
        };
        if negated_check && view.elseifs.is_empty() && view.else_.is_none() && exits {
            return true;
        }
    }
    false
}
