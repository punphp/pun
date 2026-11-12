//! `lint/types/prefer-explicit-null-instanceof`, ported from Rector 2.8.4's `ExplicitNullableObjectInstanceRector`
//! (rules/CodeQuality/Rector/If_).
//!
//! A value that is an object of one class or `null` is checked with `instanceof`: `if ($item)` becomes
//! `if ($item instanceof Item)`, and `!$item`, `$item === null` and `empty($item)` become `!$item instanceof Item`.
//! The places are the condition of an `if`, an `elseif`, a ternary, a `while` or a `do`, the two sides of `&&` and
//! `||`, a comparison with `null` and `empty()`. Rector writes the class fully qualified.
//!
//! Rector drops a comment inside the code it replaces (`! /* why */ $item`); Pun leaves such a check alone.
//! Mago types a `?\Closure` as a closure, which is no `ObjectType` for PHPStan, and narrows the right side of
//! `$item instanceof Item && $item`; Pun leaves both alone where Rector rewrites them.
//! Rector prints a ternary value without parentheses (`$flag ? $a : $b instanceof Item`), which checks one branch
//! only; Pun writes `($flag ? $a : $b) instanceof Item`, and leaves a ternary Rector types itself alone.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{BinaryOperator, Construct, Expression, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::facts::facts;
use super::prefer_instanceof::needs_parentheses;
use super::rector_type::{RectorType, get_type};
use crate::rules::rector_code_quality::{is_constant, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferExplicitNullInstanceof;

impl Rule for PreferExplicitNullInstanceof {
    fn id(&self) -> &'static str {
        "lint/types/prefer-explicit-null-instanceof"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::If,
            NodeKind::IfStatementBodyElseIfClause,
            NodeKind::IfColonDelimitedBodyElseIfClause,
            NodeKind::Conditional,
            NodeKind::Binary,
            NodeKind::EmptyConstruct,
            NodeKind::While,
            NodeKind::DoWhile,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let facts = facts(ctx);
        let native = |expr: &Expression<'_>| {
            let t = facts.native_type(node_span(expr))?;
            nullable_object(t)
        };
        // each change: the code to replace, the value to check, its class, and whether the check is negated
        let mut changes: Vec<(Span, &Expression<'_>, String, bool)> = Vec::new();
        match node {
            // `refactorExplicitBool`
            Node::If(_)
            | Node::IfStatementBodyElseIfClause(_)
            | Node::IfColonDelimitedBodyElseIfClause(_)
            | Node::Conditional(_) => {
                let condition = match node {
                    Node::If(i) => i.condition,
                    Node::IfStatementBodyElseIfClause(c) => c.condition,
                    Node::IfColonDelimitedBodyElseIfClause(c) => c.condition,
                    // skip short ternary
                    Node::Conditional(t) if t.then.is_some() => t.condition,
                    _ => return,
                };
                let condition = unparenthesize(condition);
                let (value, negated) = match condition {
                    Expression::UnaryPrefix(not) if not.operator.is_not() => {
                        (unparenthesize(not.operand), true)
                    }
                    other => (other, false),
                };
                // cheap bail - these shapes can never be a nullable object
                if bails(value) {
                    return;
                }
                // a native type that is `mixed` or a `bool` stays; the class is the one of the type with its docblock
                if native(value).is_none() {
                    return;
                }
                let RectorType::Scope(Some(t)) = get_type(&facts, value) else {
                    return;
                };
                if let Some(class) = nullable_object(t) {
                    changes.push((node_span(condition), value, class, negated));
                }
            }
            Node::Binary(binary) => match binary.operator {
                // `refactorNullCompare`
                BinaryOperator::Identical(_) | BinaryOperator::NotIdentical(_) => {
                    let value = if is_constant(binary.lhs, "null") {
                        binary.rhs
                    } else if is_constant(binary.rhs, "null") {
                        binary.lhs
                    } else {
                        return;
                    };
                    let value = unparenthesize(value);
                    if let Some(class) = native(value) {
                        let negated = matches!(binary.operator, BinaryOperator::Identical(_));
                        changes.push((node_span(binary), value, class, negated));
                    }
                }
                // `refactorBinaryOp`
                BinaryOperator::And(_) | BinaryOperator::Or(_) => {
                    let sides = [unparenthesize(binary.lhs), unparenthesize(binary.rhs)];
                    if sides.iter().any(|s| matches!(s, Expression::Assignment(_))) {
                        return;
                    }
                    for side in sides {
                        // `processNegationBooleanOr`: the value under a `!`, which stays
                        let value = match side {
                            Expression::UnaryPrefix(not)
                                if not.operator.is_not()
                                    && matches!(binary.operator, BinaryOperator::Or(_)) =>
                            {
                                unparenthesize(not.operand)
                            }
                            other => other,
                        };
                        // `isNeverNullableObject`
                        let never = matches!(value, Expression::UnaryPrefix(_)) || bails(value);
                        if never {
                            continue;
                        }
                        if let Some(class) = native(value) {
                            changes.push((node_span(value), value, class, false));
                        }
                    }
                }
                _ => {}
            },
            // `refactorEmpty`
            Node::EmptyConstruct(empty) => {
                let value = unparenthesize(empty.value);
                if matches!(value, Expression::ArrayAccess(_)) {
                    return;
                }
                let Some(class) = native(value) else { return };
                // `!empty($value)` is the value's own check
                let mut span = node_span(empty);
                let mut negated = true;
                for parent in ancestors.iter().rev() {
                    match parent {
                        Node::Parenthesized(_) | Node::Expression(_) | Node::Construct(_) => {}
                        Node::UnaryPrefix(not) if not.operator.is_not() => {
                            span = node_span(*not);
                            negated = false;
                            break;
                        }
                        _ => break,
                    }
                }
                changes.push((span, value, class, negated));
            }
            // `refactorLoop`
            _ => {
                let condition = if let Node::While(w) = node {
                    w.condition
                } else {
                    let Node::DoWhile(d) = node else { return };
                    d.condition
                };
                let value = unparenthesize(condition);
                // an assignment stays, and a comparison with `null` is the comparison's own change
                if matches!(value, Expression::Assignment(_) | Expression::Binary(_)) {
                    return;
                }
                if let Some(class) = native(value) {
                    changes.push((node_span(value), value, class, false));
                }
            }
        }
        let source = ctx.source;
        // a comment in the replaced code but outside the value has no place in the new code
        changes.retain(|(span, value, ..)| {
            let inner = node_span(*value);
            ![
                &source[span.start..inner.start],
                &source[inner.end..span.end],
            ]
            .iter()
            .any(|code| code.contains(['/', '#']))
        });
        let Some(first) = changes.first() else { return };
        let span = first.0;
        let edits = changes
            .iter()
            .map(|(span, value, class, negated)| {
                let value = if needs_parentheses(value) {
                    format!("({})", text(ctx, value))
                } else {
                    text(ctx, value).to_string()
                };
                let not = if *negated { "!" } else { "" };
                Edit::new(*span, format!("{not}{value} instanceof \\{class}"))
            })
            .collect();
        out.push(Report::fixed(
            span,
            "Check a nullable object with `instanceof`.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// The shapes that can never be a nullable object: a binary operation, `empty()` and `isset()`.
fn bails(value: &Expression<'_>) -> bool {
    matches!(
        value,
        Expression::Binary(_) | Expression::Construct(Construct::Empty(_) | Construct::Isset(_))
    )
}

/// The class of `t` when it is `null` or an object of one class (PHPStan's `ObjectType` once `null` is removed).
fn nullable_object(t: &TUnion) -> Option<String> {
    let object = match t.types.as_ref() {
        [TAtomic::Null, object] | [object, TAtomic::Null] => object,
        _ => return None,
    };
    let name = match object {
        TAtomic::Object(TObject::Named(named))
            if named.intersection_types.is_none() && !named.is_this && !named.is_static =>
        {
            named.name
        }
        TAtomic::Object(TObject::Enum(e)) => e.name,
        _ => return None,
    };
    Some(name.to_string())
}
