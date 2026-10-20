//! Level 4's NullsafePropertyFetchRule and NullsafeMethodCallRule (`nullsafe.neverNull`): `?->` on a type that is
//! never null.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::Level;

use super::super::members::{self, Method};
use super::super::{Checker, declared};
use super::flow::unparen;

pub const IDENTIFIERS: &[&str] = &["nullsafe.neverNull"];

const TIP: &str = "Because the type is coming from a PHPDoc, you can turn off this check by setting treatPhpDocTypesAsCertain: false in your %configurationFile%.";

/// The holders the rules read.
pub(super) fn typed(e: &Expression, spans: &mut Vec<Span>) {
    match e {
        Expression::Access(Access::NullSafeProperty(p)) => spans.push(p.object.span()),
        Expression::Call(Call::NullSafeMethod(m)) => spans.push(m.object.span()),
        _ => {}
    }
}

/// `getScopeNativeType()` of the holder may be null: a method call's native return type (a method without one
/// returns `mixed`).
// ponytail: only a user method call's native type is known; elsewhere the native type counts as the PHPDoc one
fn native_may_be_null(c: &Checker, e: &Expression) -> bool {
    let (object, method) = match unparen(e) {
        Expression::Call(Call::Method(m)) => (m.object, &m.method),
        Expression::Call(Call::NullSafeMethod(m)) => (m.object, &m.method),
        _ => return false,
    };
    let ClassLikeMemberSelector::Identifier(name) = method else { return false };
    let class = match unparen(object) {
        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => c.class_scope(),
        object => c.type_of(object).and_then(|t| {
            match t.types.iter().filter(|t| !t.is_null()).collect::<Vec<_>>().as_slice() {
                [TAtomic::Object(TObject::Named(o))] => c.codebase.get_class_like(o.name.as_bytes()),
                _ => None,
            }
        }),
    };
    let Some(Method::User { f, .. }) = class.and_then(|k| members::method(c, k, name.value)) else { return false };
    f.return_type_declaration_metadata.as_ref().is_none_or(|t| !c.to_phpstan(&t.type_union).is_null().no())
}

fn check(c: &mut Checker, object: &Expression, at: Span, kind: &str) {
    if c.level < 4 {
        return;
    }
    let Some(t) = c.phpstan_type(object) else { return };
    if !t.is_null().no() {
        return;
    }
    let described = declared::with_classes(c, || t.describe(Level::TypeOnly));
    let tip = native_may_be_null(c, object).then_some(TIP);
    c.report(
        at,
        "nullsafe.neverNull",
        format!("Using nullsafe {kind} on non-nullable type {described}. Use -> instead."),
        tip,
    );
}

pub(in super::super) fn property(c: &mut Checker, p: &NullSafePropertyAccess) {
    if super::super::property_access::undefined_allowed(c, p.span()) {
        return;
    }
    check(c, p.object, p.property.span(), "property access");
}

pub(in super::super) fn method(c: &mut Checker, m: &NullSafeMethodCall) {
    check(c, m.object, m.method.span(), "method call");
}
