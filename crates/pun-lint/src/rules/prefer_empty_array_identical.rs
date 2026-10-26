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
use super::types::facts::facts;
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
        // `in_not`: an `empty()` inside a `!` that Rector looked at in the outer scope, written `!($v === [])`.
        let (span, empty, negated, in_not) = match node {
            Node::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
                let Expression::Construct(Construct::Empty(empty)) = unparenthesize(u.operand)
                else {
                    return;
                };
                if arrow_body_param(ancestors, node_span(u), empty.value) {
                    return;
                }
                (node_span(u), empty, true, false)
            }
            Node::EmptyConstruct(empty) => {
                // `!empty(...)` is the `!`'s.
                let negated = ancestors.iter().rposition(|n| {
                    !matches!(
                        n,
                        Node::Expression(_) | Node::Construct(_) | Node::Parenthesized(_)
                    )
                });
                let in_not = match negated.map(|i| (i, ancestors[i])) {
                    Some((i, Node::UnaryPrefix(u)))
                        if matches!(u.operator, UnaryPrefixOperator::Not(_)) =>
                    {
                        if !arrow_body_param(&ancestors[..i], node_span(u), empty.value) {
                            return;
                        }
                        true
                    }
                    _ => false,
                };
                if arrow_body_param(ancestors, node_span(empty), empty.value) {
                    return;
                }
                (node_span(empty), empty, false, in_not)
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
            && !native_array_variable(ctx, ancestors, value)
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
                edits: vec![Edit::new(
                    span,
                    match in_not {
                        true => format!("({text} {operator} [])"),
                        false => format!("{text} {operator} []"),
                    },
                )],
            }),
        });
    }
}

/// A variable whose native type is an array. `isAllowedVariable`'s `isNonTypedFromParam` passes exactly when the
/// native type is no implicit `mixed` and covers the PHPDoc type, which an array native type does; then `getType()`
/// is an array too. A variable that may be undefined fails its `hasVariableType()->yes()`, and Rector 2.6.1 leaves an
/// untyped parameter alone even where a check narrows it (`Pun/skip_is_array_guard`).
fn native_array_variable(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    value: &Expression<'_>,
) -> bool {
    let Expression::Variable(Variable::Direct(v)) = value else {
        return false;
    };
    let name = v.name;
    // The variable's scope (the file at the top level), its parameters and its statements.
    let scope = ancestors
        .iter()
        .rev()
        .find(|n| {
            matches!(
                n,
                Node::Function(_)
                    | Node::Method(_)
                    | Node::Closure(_)
                    | Node::ArrowFunction(_)
                    | Node::Program(_)
            )
        })
        .copied();
    let (params, statements) = match scope {
        Some(Node::Function(f)) => (Some(&f.parameter_list), Some(&f.body.statements)),
        Some(Node::Method(Method {
            parameter_list,
            body: MethodBody::Concrete(b),
            ..
        })) => (Some(parameter_list), Some(&b.statements)),
        Some(Node::Closure(c)) => (Some(&c.parameter_list), Some(&c.body.statements)),
        Some(Node::ArrowFunction(a)) => (Some(&a.parameter_list), None),
        _ => (None, None),
    };
    let untyped_param = params.is_some_and(|l| {
        (l.parameters.iter()).any(|p| p.variable.name == name && p.hint.is_none())
    });
    // An untyped parameter keeps its `mixed` native type until a write that always runs: Rector's scope on the right
    // of `&&` is not narrowed by the left, and Mago merges a write in one branch into an array.
    let mentions = |s: &&Statement<'_>| {
        super::side_effect::any_node(
            Node::Statement(s),
            &|m| matches!(m, Node::DirectVariable(d) if d.name == name),
        )
    };
    let reassigned = statements
        .and_then(|list| list.iter().find(mentions))
        .is_some_and(|s| {
            matches!(s, Statement::Expression(e)
                if matches!(unparenthesize(e.expression), Expression::Assignment(a)
                    if matches!(unparenthesize(a.lhs), Expression::Variable(Variable::Direct(l)) if l.name == name)))
        });
    let facts = facts(ctx);
    let is_array = |e: &Expression<'_>| {
        (facts.native_type(node_span(e))).is_some_and(|t| {
            t.is_array() && !t.possibly_undefined() && !t.possibly_undefined_from_try()
        })
    };
    // Mago's merged type is not always PHPStan's (`$x = $a ?: $untyped->m();` then an `array_filter($x)` in an `if`
    // is `mixed` in PHPStan): every write in the scope must be a plain `=` of a native array too.
    let names = |n: Node<'_, '_>| {
        super::side_effect::any_node(
            n,
            &|m| matches!(m, Node::DirectVariable(d) if d.name == name),
        )
    };
    let other_write = |n: Node<'_, '_>| match n {
        Node::Assignment(a) => match unparenthesize(a.lhs) {
            Expression::Variable(Variable::Direct(l)) => {
                l.name == name && !(a.operator.is_assign() && is_array(a.rhs))
            }
            // `[$a, $b] = ...`
            lhs @ (Expression::Array(_) | Expression::LegacyArray(_) | Expression::List(_)) => {
                names(Node::Expression(lhs))
            }
            _ => false,
        },
        // `&$v` and `use (&$v)`
        Node::UnaryPrefix(u) => {
            matches!(u.operator, UnaryPrefixOperator::Reference(_))
                && names(Node::Expression(u.operand))
        }
        Node::ClosureUseClauseVariable(c) => c.ampersand.is_some() && c.variable.name == name,
        Node::ForeachTarget(_) | Node::Global(_) | Node::Static(_) | Node::List(_) => names(n),
        _ => false,
    };
    (!untyped_param || reassigned)
        && is_array(value)
        && scope.is_some_and(|s| !super::side_effect::any_node(s, &other_write))
}

/// Whether the code at `span` is the whole body of an arrow function (`fn (array $v) => empty($v)`) and checks one of
/// its parameters: Rector gives that node the scope around the arrow function, where the parameter is undefined.
fn arrow_body_param(
    ancestors: &[Node<'_, '_>],
    span: pun_problem::Span,
    value: &Expression<'_>,
) -> bool {
    let Some(Node::ArrowFunction(arrow)) = ancestors.iter().rev().find(|n| {
        matches!(
            n,
            Node::ArrowFunction(_) | Node::Closure(_) | Node::Function(_) | Node::Method(_)
        )
    }) else {
        return false;
    };
    node_span(unparenthesize(arrow.expression)) == span
        && matches!(unparenthesize(value), Expression::Variable(Variable::Direct(v))
            if arrow.parameter_list.parameters.iter().any(|p| p.variable.name == v.name))
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
        Node::Method(Method {
            body: MethodBody::Concrete(b),
            ..
        }) => Some(b),
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
