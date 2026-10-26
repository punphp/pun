//! `lint/phpunit/prefer-same-for-scalars`, ported from rector-phpunit's `AssertEqualsToSameRector`
//! (rules/CodeQuality/Rector/MethodCall).
//!
//! In a test class, `assertEquals()` becomes `assertSame()` (and `assertNotEquals()` `assertNotSame()`) when the
//! expected value is a scalar, an enum case, a class constant or an array of scalars, and the comparison cannot be
//! one `assertEquals()` makes loosely: the actual value's native type is not a union, `mixed`, `never`, a number of
//! the other kind, a string for a number, or a `Stringable` object for a string, and an array compares to an array
//! with the same keys in the same order.

use mago_syntax::cst::{
    Access, ArrayElement, CompositeString, Expression, Literal, Node, NodeKind, Statement,
};
use pun_problem::Safety;

use super::ptype::{Key, PType, native, native_without_null};
use super::{AnyCall, arg_value, is_in_test_class, renamed, report, rewrite_args};
use crate::rules::laravel::is_a;
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

const RENAME: &[(&str, &str)] = &[
    ("assertEquals", "assertSame"),
    ("assertNotEquals", "assertNotSame"),
];

pub struct PreferSameForScalars;

impl Rule for PreferSameForScalars {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-same-for-scalars"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall, NodeKind::StaticMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(call) = AnyCall::of(node) else {
            return;
        };
        if !is_in_test_class(ctx, ancestors, None)
            || !call.is_name(&["assertEquals", "assertNotEquals"])
        {
            return;
        }
        let args = call.args();
        if args.len() < 2 {
            return;
        }
        let (expected, actual) = (arg_value(args[0]), arg_value(args[1]));
        if !is_scalar_or_enum_value(ctx, expected) {
            return;
        }
        if skip_constant_array(ctx, ancestors, expected, actual)
            || skip_loose(ctx, expected, actual)
        {
            return;
        }
        let Some(name) = renamed(&call, RENAME) else {
            return;
        };
        out.push(report(
            call.span,
            &format!("Use `{name}()` to compare scalars strictly."),
            Safety::Unsafe,
            rewrite_args(
                ctx,
                &call,
                Some(name),
                &super::old_args(&call, 0).collect::<Vec<_>>(),
            ),
        ));
    }
}

/// `isScalarType`: `FloatType`, `IntegerType`, `StringType`, `ConstantArrayType` or a `BooleanType` that is not a
/// constant.
fn is_scalar_type(t: &PType) -> bool {
    matches!(
        t,
        PType::Float { .. }
            | PType::Integer { .. }
            | PType::String { .. }
            | PType::ConstantArray { .. }
            | PType::Boolean
    )
}

fn is_scalar_or_enum_value(ctx: &Context<'_>, expr: &Expression<'_>) -> bool {
    match unparenthesize(expr) {
        Expression::Access(Access::ClassConstant(_))
        | Expression::CompositeString(CompositeString::Interpolated(_)) => true,
        e => native(ctx, e).is_some_and(|t| is_scalar_type(&t)),
    }
}

/// `hasNonScalarType`: an item (at any depth) is not a scalar.
fn has_non_scalar_type(values: &[PType]) -> bool {
    values.iter().any(|v| match v {
        PType::ConstantArray { values, .. } => has_non_scalar_type(values),
        other => !is_scalar_type(other),
    })
}

fn skip_constant_array(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    expected: &Expression<'_>,
    actual: &Expression<'_>,
) -> bool {
    let Some(PType::ConstantArray { values, .. }) = native(ctx, expected) else {
        return false;
    };
    if has_non_scalar_type(&values) {
        return true;
    }
    let Some(PType::ConstantArray {
        keys: actual_keys, ..
    }) = native(ctx, actual)
    else {
        return true;
    };
    let expected_keys = written_keys(ancestors, expected).unwrap_or_else(|| {
        let Some(PType::ConstantArray { keys, .. }) = native(ctx, expected) else {
            unreachable!("checked above");
        };
        keys
    });
    let actual_keys = written_keys(ancestors, actual).unwrap_or(actual_keys);
    expected_keys != actual_keys
}

/// The keys of an array literal in the order written, for `expr` or for the variable `expr` names when the
/// statements before it in its function assign it one array literal. Mago keeps a constant array's keys sorted, where
/// PHPStan keeps them in order.
fn written_keys(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> Option<Vec<Key>> {
    let literal = match unparenthesize(expr) {
        e @ (Expression::Array(_) | Expression::LegacyArray(_)) => e,
        Expression::Variable(mago_syntax::cst::Variable::Direct(v)) => {
            assigned_array(ancestors, v.name, crate::syntax::node_span(expr).start)?
        }
        _ => return None,
    };
    let elements: Vec<&ArrayElement<'_>> = match literal {
        Expression::Array(a) => a.elements.iter().collect(),
        Expression::LegacyArray(a) => a.elements.iter().collect(),
        _ => return None,
    };
    let mut next = 0i64;
    let mut keys = Vec::new();
    for element in elements {
        match element {
            ArrayElement::KeyValue(kv) => match unparenthesize(kv.key) {
                Expression::Literal(Literal::Integer(i)) => {
                    let i = i64::try_from(i.value?).ok()?;
                    keys.push(Key::Int(i));
                    next = next.max(i + 1);
                }
                Expression::Literal(Literal::String(s)) => {
                    keys.push(Key::Str(String::from_utf8_lossy(s.value?).into_owned()));
                }
                _ => return None,
            },
            ArrayElement::Value(_) => {
                keys.push(Key::Int(next));
                next += 1;
            }
            _ => return None,
        }
    }
    Some(keys)
}

/// The array literal last assigned to `$name` by a statement of the enclosing function body before `before`.
fn assigned_array<'a>(
    ancestors: &[Node<'a, 'a>],
    name: &[u8],
    before: usize,
) -> Option<&'a Expression<'a>> {
    let body = ancestors.iter().rev().find_map(|a| match a {
        Node::Block(b) => Some(*b),
        _ => None,
    })?;
    let mut found = None;
    for statement in body.statements.iter() {
        if crate::syntax::node_span(statement).end > before {
            break;
        }
        if let Statement::Expression(e) = statement
            && let Expression::Assignment(assign) = e.expression
            && let Expression::Variable(mago_syntax::cst::Variable::Direct(v)) =
                unparenthesize(assign.lhs)
            && v.name == name
        {
            found = Some(assign.rhs);
        }
    }
    found.filter(|e| {
        matches!(
            unparenthesize(e),
            Expression::Array(_) | Expression::LegacyArray(_)
        )
    })
}

fn skip_loose(ctx: &Context<'_>, expected: &Expression<'_>, actual: &Expression<'_>) -> bool {
    let (Some(first), Some(second)) = (native(ctx, expected), native_without_null(ctx, actual))
    else {
        return true;
    };
    if matches!(second, PType::Union | PType::Mixed | PType::Never) {
        return true;
    }
    if first.is_integer() && (second.is_float() || second.is_string()) {
        return true;
    }
    if first.is_float() && (second.is_integer() || second.is_string()) {
        return true;
    }
    if first.is_string()
        && let PType::Object(class) = &second
        && is_a(ctx, class, &["Stringable"])
    {
        return true;
    }
    matches!(
        unparenthesize(expected),
        Expression::Literal(Literal::String(s)) if s.value.is_some_and(is_numeric)
    )
}

/// PHP's `is_numeric()` on a string: optional leading whitespace, a sign, digits with an optional fraction and
/// exponent, and optional trailing whitespace.
fn is_numeric(value: &[u8]) -> bool {
    let s = String::from_utf8_lossy(value);
    let t = s.trim_matches([' ', '\t', '\n', '\r', '\x0b', '\x0c']);
    let t = t.strip_prefix(['+', '-']).unwrap_or(t);
    let (mantissa, exponent) = match t.find(['e', 'E']) {
        Some(i) => (&t[..i], Some(&t[i + 1..])),
        None => (t, None),
    };
    let (int, frac) = match mantissa.split_once('.') {
        Some((a, b)) => (a, Some(b)),
        None => (mantissa, None),
    };
    let digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
    let mantissa_ok = digits(int)
        && frac.is_none_or(digits)
        && !(int.is_empty() && frac.is_none_or(str::is_empty));
    let exponent_ok = exponent.is_none_or(|e| {
        let e = e.strip_prefix(['+', '-']).unwrap_or(e);
        !e.is_empty() && digits(e)
    });
    mantissa_ok && exponent_ok
}
