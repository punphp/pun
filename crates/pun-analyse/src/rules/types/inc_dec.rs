//! InvalidIncDecOperationRule (`*.expr`: `++`/`--` on what is not a variable, an offset or a property; `*.type`: on a
//! type that is not a number, a (numeric) string, null, bool or an object with operator overloading, as of the PHP
//! version).

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::{Accessory, Level, Type, intersect, is_super_type_of};

use super::super::{Checker, declared};

pub const IDENTIFIERS: &[&str] = &["preInc.type", "postInc.type", "preDec.type", "postDec.type"];

/// The operand whose type the rule reads.
pub(super) fn typed(e: &Expression, spans: &mut Vec<Span>) {
    match e {
        Expression::UnaryPrefix(u) if u.operator.is_increment_or_decrement() => spans.push(u.operand.span()),
        Expression::UnaryPostfix(u) => spans.push(u.operand.span()),
        _ => {}
    }
}

/// `$var` is not a type `++`/`--` takes.
fn types(c: &mut Checker, operand: &Expression, kind: &str, operator: &str) {
    let this = matches!(super::flow::unparen(operand), Expression::Variable(Variable::Direct(v)) if v.name == b"$this");
    // (`checkThisOnly` at levels 0 and 1)
    if c.level < 2 && !this {
        return;
    }
    let dec = operator == "--";
    let version = c.php_version;
    let deprecated_string = if dec { version >= 80300 } else { version >= 80500 };
    let string = if deprecated_string {
        Type::Intersection(vec![Type::String, Type::Accessory(Accessory::NumericString)])
    } else {
        Type::String
    };
    let object = |name: &str| Type::Object(name.into(), vec![]);
    let mut allowed = vec![Type::Float, Type::Int, string, object("SimpleXMLElement"), object("GMP")];
    let deprecated_null = version >= 80300 && dec;
    if !deprecated_null {
        allowed.push(Type::Null);
    }
    let deprecated_bool = version >= 80300;
    if !deprecated_bool {
        allowed.push(Type::Bool);
    }
    if version >= 80400 {
        allowed.push(object("BcMath\\Number"));
    }
    let allowed = Type::Union(allowed);
    let found = declared::with_classes(c, || {
        let ok = |t: &Type| is_super_type_of(&allowed, t).yes();
        c.find_type_to_check(operand, &ok).ty.filter(|t| !ok(t))
    });
    let Some(t) = found else { return };
    // a `mixed` here is a StrictMixedType, which is no string, null or bool
    // ponytail: convert.rs types every `mixed` implicit, so only the template of mixed reaches here today
    let strict = match &t {
        Type::Template(tt) if matches!(tt.bound, Type::Mixed { .. }) => Some(Some(tt.name.clone())),
        t => matches!(t, Type::Mixed { .. }).then_some(None),
    };
    let mut tips = Vec::new();
    let may = |is: fn(&Type) -> pun_phpstan::types::Tri| strict.is_none() && !is(&t).no();
    if deprecated_string && may(Type::is_string) {
        let f = if dec { "str_decrement" } else { "str_increment" };
        tips.push(format!(
            "Operator {operator} is deprecated for non-numeric-strings. Either narrow the type to numeric-string, or use {f}()."
        ));
    }
    if deprecated_null && may(Type::is_null) {
        tips.push(format!("Operator {operator} is deprecated for null."));
    }
    if deprecated_bool && may(Type::is_boolean) {
        let bool = declared::with_classes(c, || intersect(vec![t.clone(), Type::Bool]).describe(Level::Value));
        tips.push(format!("Operator {operator} is deprecated for {bool}."));
    }
    let tip = match tips.as_slice() {
        [] => None,
        [one] => Some(one.clone()),
        many => Some(many.iter().map(|t| format!("• {t}")).collect::<Vec<_>>().join("\n")),
    };
    let described = match strict {
        Some(Some(name)) => format!("{name} of mixed"),
        _ => declared::with_classes(c, || t.describe(Level::Value)),
    };
    c.report(operand, &format!("{kind}.type"), format!("Cannot use {operator} on {described}."), tip.as_deref());
}

fn check(c: &mut Checker, operand: &Expression, kind: &str, operator: &str) {
    if matches!(
        super::flow::unparen(operand),
        Expression::Variable(_)
            | Expression::ArrayAccess(_)
            | Expression::ArrayAppend(_)
            | Expression::Access(Access::Property(_) | Access::StaticProperty(_))
    ) {
        types(c, operand, kind, operator);
        return;
    }
    c.report(operand, &format!("{kind}.expr"), format!("Cannot use {operator} on a non-variable."), None);
}

pub(in super::super) fn prefix(c: &mut Checker, u: &UnaryPrefix) {
    match u.operator {
        UnaryPrefixOperator::PreIncrement(_) => check(c, u.operand, "preInc", "++"),
        UnaryPrefixOperator::PreDecrement(_) => check(c, u.operand, "preDec", "--"),
        _ => {}
    }
}

pub(in super::super) fn postfix(c: &mut Checker, u: &UnaryPostfix) {
    match u.operator {
        UnaryPostfixOperator::PostIncrement(_) => check(c, u.operand, "postInc", "++"),
        UnaryPostfixOperator::PostDecrement(_) => check(c, u.operand, "postDec", "--"),
    }
}
