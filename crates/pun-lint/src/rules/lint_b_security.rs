//! Mago 1.57.0's linter helpers for sensitive names (rule/utils/security.rs), shared by lint-b's ports of
//! `no-insecure-comparison`, `no-literal-password` and `no-tainted-data`.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::{
    Access, Call, ClassLikeConstantSelector, ClassLikeMemberSelector, Expression, Literal,
    LiteralString, MatchArm, Variable,
};

/// `get_password`: the span of the part of `expr` whose name looks like a password, token, secret or API key.
pub(super) fn get_password(expr: &Expression<'_>) -> Option<Span> {
    match expr {
        Expression::Parenthesized(p) => get_password(p.expression),
        Expression::Literal(Literal::String(s)) if is_password_literal(s) => Some(s.span()),
        Expression::Assignment(a) => get_password(a.lhs).or_else(|| get_password(a.rhs)),
        Expression::ArrayAccess(a) => get_password(a.index),
        Expression::Variable(v) => get_password_from_variable(v),
        Expression::Call(Call::Method(c)) => get_password_from_selector(&c.method),
        Expression::Call(Call::StaticMethod(c)) => get_password_from_selector(&c.method),
        Expression::Access(Access::Property(a)) => get_password_from_selector(&a.property),
        Expression::Access(Access::NullSafeProperty(a)) => get_password_from_selector(&a.property),
        Expression::Access(Access::StaticProperty(a)) => get_password_from_variable(&a.property),
        Expression::Access(Access::ClassConstant(a)) => match &a.constant {
            ClassLikeConstantSelector::Identifier(i) if is_password(i.value) => Some(i.span()),
            ClassLikeConstantSelector::Expression(e) => get_password(e.expression),
            _ => None,
        },
        _ => None,
    }
}

fn get_password_from_selector(selector: &ClassLikeMemberSelector<'_>) -> Option<Span> {
    match selector {
        ClassLikeMemberSelector::Identifier(i) if is_password(i.value) => Some(i.span()),
        ClassLikeMemberSelector::Variable(v) => get_password_from_variable(v),
        ClassLikeMemberSelector::Expression(e) => get_password(e.expression),
        _ => None,
    }
}

fn get_password_from_variable(variable: &Variable<'_>) -> Option<Span> {
    match variable {
        Variable::Direct(v) => is_password(&v.name[1..]).then_some(v.span()),
        Variable::Indirect(v) => get_password(v.expression),
        Variable::Nested(v) => get_password_from_variable(v.variable),
    }
}

/// `is_password_literal`: the literal's text between its quotes is a sensitive name.
pub(super) fn is_password_literal(literal: &LiteralString<'_>) -> bool {
    is_password(&literal.raw[1..literal.raw.len() - 1])
}

/// `is_password`: a name ending (ignoring ASCII case, after a `$`, `get` and `_`) in `password`, `token`, `secret`,
/// `apikey` or `api_key`; a name starting with `--` is not one.
pub(super) fn is_password(mut name: &[u8]) -> bool {
    if name.starts_with(b"--") {
        return false;
    }
    if let Some(rest) = name.strip_prefix(b"$") {
        name = rest;
    }
    if let Some(rest) = name.strip_prefix(b"get") {
        name = rest.strip_prefix(b"_").unwrap_or(rest);
    }
    let lower = name.to_ascii_lowercase();
    [&b"password"[..], b"token", b"secret", b"apikey", b"api_key"]
        .iter()
        .any(|s| lower.ends_with(s))
}

/// `is_user_input`: the value comes straight from a request superglobal (`$_GET`, `$_POST`, `$_REQUEST`, `$_COOKIE`,
/// `$_FILES`, `$_SERVER`, `$_SESSION`), through parentheses, an assignment, a ternary, an array access, a `match` arm,
/// a concatenation or `??`.
pub(super) fn is_user_input(expression: &Expression<'_>) -> bool {
    match expression {
        Expression::Parenthesized(p) => is_user_input(p.expression),
        Expression::Assignment(a) => is_user_input(a.rhs),
        Expression::Conditional(c) => match c.then {
            Some(then) => is_user_input(then) || is_user_input(c.r#else),
            None => is_user_input(c.condition) || is_user_input(c.r#else),
        },
        Expression::ArrayAccess(a) => is_user_input(a.array),
        Expression::Match(m) => m.arms.iter().any(|arm| {
            is_user_input(match arm {
                MatchArm::Expression(e) => e.expression,
                MatchArm::Default(d) => d.expression,
            })
        }),
        Expression::Binary(b) if b.operator.is_concatenation() || b.operator.is_null_coalesce() => {
            is_user_input(b.lhs) || is_user_input(b.rhs)
        }
        Expression::Variable(v) => is_variable_user_input(v),
        _ => false,
    }
}

fn is_variable_user_input(variable: &Variable<'_>) -> bool {
    match variable {
        Variable::Direct(v) => [
            &b"$_GET"[..],
            b"$_POST",
            b"$_REQUEST",
            b"$_COOKIE",
            b"$_FILES",
            b"$_SERVER",
            b"$_SESSION",
        ]
        .iter()
        .any(|n| v.name.eq_ignore_ascii_case(n)),
        Variable::Indirect(v) => is_user_input(v.expression),
        Variable::Nested(v) => is_variable_user_input(v.variable),
    }
}
