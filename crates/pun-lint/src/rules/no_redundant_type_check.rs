//! `lint/dead-code/no-redundant-type-check`, ported from Rector 2.8.4's `RemoveRedundantTypeCheckRector`
//! (rules/DeadCode/Rector/BinaryOp).
//!
//! `$value === null || !is_string($value)` on a `?string` is `$value === null`: without `null` the value is always a
//! string. `$items && is_array($items)` on an array is `$items`. The check must be `is_string`, `is_int`, `is_float`,
//! `is_bool`, `is_array`, `is_object` or an alias, written in lower case, with the variable as its only argument, and
//! the variable's native type must make it always true. A variable `isNonTypedFromParam` doubts stays.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{
    BinaryOperator, Call, Expression, FunctionCall, Identifier, Node, NodeKind, Variable,
};
use pun_problem::{Edit, Safety};

use super::no_useless_cast::{names_variable, within};
use super::rector_code_quality::{args, equal, is_constant, text};
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoRedundantTypeCheck;

impl Rule for NoRedundantTypeCheck {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-redundant-type-check"
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
        let Node::Binary(binary) = node else { return };
        let (left, right) = (unparenthesize(binary.lhs), unparenthesize(binary.rhs));
        let keep = match binary.operator {
            BinaryOperator::Or(_) => refactor_boolean_or(ctx, ancestors, left, right),
            BinaryOperator::And(_) => refactor_boolean_and(ctx, ancestors, left, right),
            _ => false,
        };
        if !keep {
            return;
        }
        let span = node_span(binary);
        out.push(Report::fixed(
            span,
            "This type check is always true for the variable's type; remove it.",
            Safety::Unsafe,
            vec![Edit::new(span, text(ctx, left).to_string())],
        ));
    }
}

/// Handles "null === $value || ! is_string($value)", where is_string() can never fail once null is excluded
fn refactor_boolean_or(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    left: &Expression<'_>,
    right: &Expression<'_>,
) -> bool {
    let Expression::Binary(identical) = left else {
        return false;
    };
    if !matches!(identical.operator, BinaryOperator::Identical(_)) {
        return false;
    }
    // `matchNullComparedExpr`
    let compared = if is_constant(identical.lhs, "null") {
        identical.rhs
    } else if is_constant(identical.rhs, "null") {
        identical.lhs
    } else {
        return false;
    };
    let compared = unparenthesize(compared);
    if !matches!(compared, Expression::Variable(Variable::Direct(_))) {
        return false;
    }
    let Expression::UnaryPrefix(not) = right else {
        return false;
    };
    if !not.operator.is_not() {
        return false;
    }
    let Some((name, _)) = match_type_check_func_call(ctx, not.operand, compared) else {
        return false;
    };
    let facts = facts(ctx);
    let Some(native) = native_type(&facts, ancestors, compared) else {
        return false;
    };
    let (nulls, rest): (Vec<&TAtomic>, Vec<&TAtomic>) = native
        .iter()
        .partition(|atomic| matches!(atomic, TAtomic::Null));
    !nulls.is_empty() && is_always_matching_type(name, &rest)
}

/// Handles "$items && is_array($items)", where is_array() can never fail on an array type
fn refactor_boolean_and(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    left: &Expression<'_>,
    right: &Expression<'_>,
) -> bool {
    if !matches!(left, Expression::Variable(Variable::Direct(_))) {
        return false;
    }
    let Some((name, argument)) = match_type_check_func_call(ctx, right, left) else {
        return false;
    };
    let facts = facts(ctx);
    if native_type(&facts, ancestors, left).is_none() {
        return false;
    }
    // the type is already narrowed by the truthy check on the left
    facts
        .native_type(node_span(argument))
        .is_some_and(|checked| {
            is_always_matching_type(name, &checked.types.iter().collect::<Vec<_>>())
        })
}

/// The native type of `variable`, unless `isNonTypedFromParam` doubts it (the docblock type can be wider than the
/// real value).
fn native_type<'f>(
    facts: &'f Facts,
    ancestors: &[Node<'_, '_>],
    variable: &Expression<'_>,
) -> Option<&'f [TAtomic]> {
    let span = node_span(variable);
    let native = facts.native_type(span)?;
    // `hasVariableType()` must be a yes; a union native type must equal the type, any other must contain it
    let doubted = native.possibly_undefined()
        || facts.full_type(span).is_some_and(|full| {
            if native.types.len() > 1 {
                full.types.len() != native.types.len()
                    || !full.types.iter().all(|a| native.types.contains(a))
            } else {
                !within(full, native)
            }
        })
        || names_variable(ancestors, span.start);
    (!doubted).then_some(native.types.as_ref())
}

/// `matchTypeCheckFuncCall`: the name of the function in "is_<type>($expectedExpr)", a call with a single argument,
/// and that argument.
fn match_type_check_func_call<'a>(
    ctx: &Context<'_>,
    expr: &'a Expression<'a>,
    expected: &Expression<'_>,
) -> Option<(&'a [u8], &'a Expression<'a>)> {
    let Expression::Call(Call::Function(call)) = unparenthesize(expr) else {
        return None;
    };
    let arguments = args(call);
    let [argument] = arguments.as_slice() else {
        return None;
    };
    if !equal(ctx, argument.value(), expected) {
        return None;
    }
    Some((function_name(call)?, unparenthesize(argument.value())))
}

/// `getName()` of a call: the name as written, without a leading backslash.
fn function_name<'a>(call: &'a FunctionCall<'a>) -> Option<&'a [u8]> {
    match call.function {
        Expression::Identifier(Identifier::Local(id)) => Some(id.value),
        Expression::Identifier(Identifier::FullyQualified(id)) => Some(&id.value[1..]),
        _ => None,
    }
}

/// `isAlwaysMatchingType`: PHPStan's `isString()`, `isInteger()` and so on say yes for every part of the type.
fn is_always_matching_type(name: &[u8], types: &[&TAtomic]) -> bool {
    let matches: fn(&TAtomic) -> bool = match name {
        b"is_string" => |a| {
            matches!(
                a,
                TAtomic::Scalar(TScalar::String(_) | TScalar::ClassLikeString(_))
            )
        },
        b"is_int" | b"is_integer" | b"is_long" => {
            |a| matches!(a, TAtomic::Scalar(TScalar::Integer(_)))
        }
        b"is_float" | b"is_double" => |a| matches!(a, TAtomic::Scalar(TScalar::Float(_))),
        b"is_bool" => |a| matches!(a, TAtomic::Scalar(TScalar::Bool(_))),
        b"is_array" => |a| matches!(a, TAtomic::Array(_)),
        b"is_object" => |a| matches!(a, TAtomic::Object(_)),
        _ => return false,
    };
    !types.is_empty() && types.iter().all(|a| matches(a))
}
