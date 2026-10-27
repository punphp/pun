//! `lint/phpunit/require-exact-test-argument-type`, ported from rector-phpunit's `ScalarArgumentToExpectedParamTypeRector`
//! (rules/CodeQuality/Rector/MethodCall), with `MethodParametersAndReturnTypesResolver`.
//!
//! In a test class or a Behat `Context`, a method call, static call or `new` whose method has native parameter types
//! gets each literal argument cast to its parameter's type (positional, or the parameter of a named argument's name):
//! an integer or a float for a `string` (or `?string`) parameter becomes the string PHP casts it to, and a string for
//! an `int` parameter the integer PHP casts it to.

use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_syntax::cst::{ArgumentList, Expression, Literal, Node, NodeKind, Variable};
use pun_problem::{Edit, Safety};

use super::prefer_test_with_attribute::php_g;
use super::{AnyCall, class_is, class_parents, is_in_test_class, own_class, report, resolve};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `BehatClassName::CONTEXT`.
const CONTEXT: &str = "Behat\\Behat\\Context\\Context";

pub struct RequireExactTestArgumentType;

impl Rule for RequireExactTestArgumentType {
    fn id(&self) -> &'static str {
        "lint/phpunit/require-exact-test-argument-type"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::MethodCall,
            NodeKind::StaticMethodCall,
            NodeKind::Instantiation,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let args: &ArgumentList<'_> = match node {
            Node::Instantiation(new) => match &new.argument_list {
                Some(list) => list,
                None => return,
            },
            _ => AnyCall::of(node).unwrap().args,
        };
        // shouldSkipCall
        if !is_in_test_or_context(ctx, ancestors)
            || !args.arguments.iter().any(|a| {
                matches!(
                    a.value(),
                    Expression::Literal(
                        Literal::Integer(_) | Literal::String(_) | Literal::Float(_)
                    )
                )
            })
        {
            return;
        }
        let Some(method) = called_method(ctx, ancestors, node) else {
            return;
        };
        let mut edits = Vec::new();
        for (key, arg) in args.arguments.iter().enumerate() {
            let value = arg.value();
            let Expression::Literal(literal) = value else {
                continue;
            };
            let mut parameter = method.parameters.get(key);
            if let mago_syntax::cst::Argument::Named(named) = arg
                && let Some(p) = method
                    .parameters
                    .iter()
                    .find(|p| p.name.0.as_bytes()[1..] == *named.name.value)
            {
                parameter = Some(p);
            }
            let Some(native) = parameter.and_then(|p| p.type_declaration_metadata.as_ref()) else {
                continue;
            };
            // remove null
            let kept: Vec<&TAtomic> = native
                .type_union
                .types
                .iter()
                .filter(|a| !matches!(a, TAtomic::Null))
                .collect();
            let [only] = kept.as_slice() else {
                continue;
            };
            let replacement = match (only, literal) {
                (TAtomic::Scalar(TScalar::String(_)), Literal::Integer(i)) => {
                    i.value.map(|v| format!("'{v}'"))
                }
                (TAtomic::Scalar(TScalar::String(_)), Literal::Float(f)) => {
                    Some(format!("'{}'", php_g(*f.value, 14)))
                }
                (TAtomic::Scalar(TScalar::Integer(_)), Literal::String(s)) => {
                    // `pScalar_Int()`: `PHP_INT_MIN` has no literal
                    s.value.map(|v| match php_int_cast(v) {
                        i64::MIN => "(-9223372036854775807-1)".to_string(),
                        i => i.to_string(),
                    })
                }
                _ => None,
            };
            if let Some(replacement) = replacement {
                edits.push(Edit::new(node_span(value), replacement));
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(report(
            node_span(&node),
            "Pass literal arguments of the type the parameter declares.",
            Safety::Safe,
            edits,
        ));
    }
}

/// `isInTestClass()`: the call is in a Behat `Context` or a test class.
fn is_in_test_or_context(ctx: &Context<'_>, ancestors: &[Node<'_, '_>]) -> bool {
    let class_like = ancestors.iter().rposition(|a| {
        matches!(
            a,
            Node::Class(_)
                | Node::AnonymousClass(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_)
        )
    });
    class_like.is_some_and(|at| class_is(ctx, &ancestors[..=at], &[CONTEXT]))
        || is_in_test_class(ctx, ancestors, None)
}

/// The method `resolveCallParameterTypes()` reads: a `new`'s constructor, or the method of a call on an object of a
/// single class (`$this` and `self` and `static` the current one).
fn called_method(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    node: Node<'_, '_>,
) -> Option<FunctionLikeMetadata> {
    let facts = facts(ctx);
    let (class, method): (String, &[u8]) = match node {
        Node::Instantiation(new) => {
            let Expression::Identifier(id) = new.class else {
                return None;
            };
            (resolve(ancestors, id), b"__construct")
        }
        // a method or a static method call
        _ => {
            let call = AnyCall::of(node).unwrap();
            let name = call.name()?;
            let class = match unparenthesize(call.receiver) {
                Expression::Variable(Variable::Direct(v))
                    if !call.is_static && v.name == b"$this" =>
                {
                    own_class(ancestors)?
                }
                Expression::Self_(_) | Expression::Static(_) if call.is_static => {
                    own_class(ancestors)?
                }
                Expression::Parent(_) if call.is_static => {
                    class_parents(ctx, &own_class(ancestors)?)
                        .into_iter()
                        .next()?
                }
                Expression::Identifier(id) if call.is_static => resolve(ancestors, id),
                object => object_type_class(ctx, object)?,
            };
            (class, name)
        }
    };
    facts.codebase.get_class_like(class.as_bytes())?;
    facts
        .codebase
        .get_declaring_method(class.as_bytes(), method)
        .cloned()
}

/// The class of an `ObjectType`: a single named object, not an intersection.
fn object_type_class(ctx: &Context<'_>, expr: &Expression<'_>) -> Option<String> {
    let facts = facts(ctx);
    match facts.full_type(node_span(expr))?.types.as_ref() {
        [TAtomic::Object(TObject::Named(n))] if n.intersection_types.is_none() => {
            Some(n.name.to_string())
        }
        _ => None,
    }
}

/// PHP's `(int)` of a string: its leading numeric part (white space, a sign, digits, a fraction, an exponent), an
/// integer that does not fit or a number with a fraction or an exponent cast from the float, capped; `0` without one.
fn php_int_cast(s: &[u8]) -> i64 {
    let mut i = 0;
    while matches!(
        s.get(i),
        Some(b' ' | b'\t' | b'\n' | b'\r' | b'\x0B' | b'\x0C')
    ) {
        i += 1;
    }
    let start = i;
    if matches!(s.get(i), Some(b'+' | b'-')) {
        i += 1;
    }
    let digits_start = i;
    while s.get(i).is_some_and(u8::is_ascii_digit) {
        i += 1;
    }
    let mut integer = true;
    let int_digits = i - digits_start;
    let mut frac_digits = 0;
    if s.get(i) == Some(&b'.') {
        let mut j = i + 1;
        while s.get(j).is_some_and(u8::is_ascii_digit) {
            j += 1;
        }
        frac_digits = j - i - 1;
        if int_digits + frac_digits > 0 {
            integer = false;
            i = j;
        }
    }
    if int_digits + frac_digits == 0 {
        return 0;
    }
    if matches!(s.get(i), Some(b'e' | b'E')) {
        let mut j = i + 1;
        if matches!(s.get(j), Some(b'+' | b'-')) {
            j += 1;
        }
        if s.get(j).is_some_and(u8::is_ascii_digit) {
            while s.get(j).is_some_and(u8::is_ascii_digit) {
                j += 1;
            }
            integer = false;
            i = j;
        }
    }
    let text = std::str::from_utf8(&s[start..i]).unwrap_or("0");
    if integer && let Ok(value) = text.parse::<i64>() {
        return value;
    }
    let value: f64 = text.parse().unwrap_or(0.0);
    if !value.is_finite() {
        return 0;
    }
    if value >= 9.223_372_036_854_776e18 {
        i64::MAX
    } else if value < -9.223_372_036_854_776e18 {
        i64::MIN
    } else {
        value as i64
    }
}
