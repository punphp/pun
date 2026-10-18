//! EnumSanityRule and EnumCaseOutsideEnumRule. A case's value is the constant Mago's codex infers for it, with class
//! constant and `X::Case->value` references followed through the codebase.

use std::collections::HashMap;

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::derived::TDerived;
use mago_codex::ttype::atomic::reference::{TReference, TReferenceMemberSelector};
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
use mago_codex::ttype::atomic::scalar::int::TInteger;
use mago_codex::ttype::atomic::scalar::string::{TString, TStringLiteral};
use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_phpstan::types::{Level, Type, is_super_type_of};

use super::Checker;

/// PHP's magic methods (php-parser's `ClassMethod::$magicNames`).
const MAGIC: &[&str] = &[
    "__construct",
    "__destruct",
    "__call",
    "__callstatic",
    "__get",
    "__set",
    "__isset",
    "__unset",
    "__sleep",
    "__wakeup",
    "__tostring",
    "__set_state",
    "__clone",
    "__invoke",
    "__debuginfo",
    "__serialize",
    "__unserialize",
];

/// EnumSanityRule.
pub fn sanity<'arena>(c: &mut Checker<'_, 'arena>, e: &Enum<'arena>) {
    let display = c
        .decls
        .last()
        .and_then(|d| d.display.clone())
        .expect("an enum's Decl");
    // php-parser lowercases a built-in type's name (`Int` is `int`)
    let scalar = e.backing_type_hint.as_ref().map(|b| {
        let written = c.written(b.hint.span());
        let written = match b.hint {
            Hint::Identifier(_) => written,
            _ => written.to_ascii_lowercase(),
        };
        (written, b.hint.span())
    });
    for m in e.members.iter() {
        let ClassLikeMember::Method(m) = m else {
            continue;
        };
        let name = String::from_utf8_lossy(m.name.value).into_owned();
        let lower = name.to_ascii_lowercase();
        if MAGIC.contains(&lower.as_str()) {
            if lower == "__construct" {
                c.report_non_ignorable(
                    m,
                    "enum.constructor",
                    format!("Enum {display} contains constructor."),
                    None,
                );
            } else if lower == "__destruct" {
                c.report_non_ignorable(
                    m,
                    "enum.destructor",
                    format!("Enum {display} contains destructor."),
                    None,
                );
            } else if !["__call", "__callstatic", "__invoke"].contains(&lower.as_str()) {
                c.report_non_ignorable(
                    m,
                    "enum.magicMethod",
                    format!("Enum {display} contains magic method {name}()."),
                    None,
                );
            }
        }
        if lower == "cases" || (scalar.is_some() && (lower == "from" || lower == "tryfrom")) {
            c.report_non_ignorable(
                m,
                "enum.methodRedeclaration",
                format!("Enum {display} cannot redeclare native method {name}()."),
                None,
            );
        }
    }
    if let Some((written, span)) = &scalar
        && written != "int"
        && written != "string"
    {
        c.report_non_ignorable(
            *span,
            "enum.backingType",
            format!("Backed enum {display} can have only \"int\" or \"string\" type."),
            None,
        );
    }
    let enum_name = c.resolved_span(e.name.span);
    if c.codebase
        .get_class_like(enum_name.as_bytes())
        .is_some_and(super::hierarchy::implements_serializable)
    {
        c.report_non_ignorable(
            e,
            "enum.serializable",
            format!("Enum {display} cannot implement the Serializable interface."),
            None,
        );
    }
    // value => case names, in PHP's array key order
    let mut values: Vec<(String, Vec<String>)> = Vec::new();
    let mut at: HashMap<String, usize> = HashMap::new();
    for m in e.members.iter() {
        let ClassLikeMember::EnumCase(case) = m else {
            continue;
        };
        let name = String::from_utf8_lossy(case.item.name().value).into_owned();
        let has_value = matches!(case.item, EnumCaseItem::Backed(_));
        let Some((written, _)) = &scalar else {
            if has_value && let Some(t) = case_value(c, e, &enum_name, &name, 0) {
                c.report_non_ignorable(
                    case,
                    "enum.caseWithValue",
                    format!(
                        "Enum {display} is not backed, but case {name} has value {}.",
                        t.describe(Level::Value)
                    ),
                    None,
                );
            }
            continue;
        };
        if !has_value {
            c.report_non_ignorable(
                case,
                "enum.missingCase",
                format!(
                    "Enum case {display}::{name} does not have a value but the enum is backed with the \"{written}\" type."
                ),
                None,
            );
            continue;
        }
        let Some(t) = case_value(c, e, &enum_name, &name, 0) else {
            continue;
        };
        let scalar_type = if written.eq_ignore_ascii_case("int") {
            Type::Int
        } else {
            Type::String
        };
        // getConstantScalarValues(): the int or string the case has
        let key = match &t {
            Type::ConstInt(i) => Some(i.to_string()),
            Type::ConstString(s) => Some(s.clone()),
            _ => None,
        };
        if let Some(key) = key.filter(|_| is_super_type_of(&scalar_type, &t).yes()) {
            let i = *at.entry(key.clone()).or_insert_with(|| {
                values.push((key, Vec::new()));
                values.len() - 1
            });
            values[i].1.push(name);
            continue;
        }
        c.report_non_ignorable(
            case,
            "enum.caseType",
            format!(
                "Enum case {display}::{name} value {} does not match the \"{}\" type.",
                t.describe(Level::Value),
                scalar_type.describe(Level::TypeOnly)
            ),
            None,
        );
    }
    for (value, names) in values {
        if names.len() > 1 {
            c.report_non_ignorable(
                e,
                "enum.duplicateValue",
                format!(
                    "Enum {display} has duplicate value {value} for cases {}.",
                    names.join(", ")
                ),
                None,
            );
        }
    }
}

/// The constant value of a case of the enum `e`: the codex's, or for `self::X->value` (which the codex leaves
/// unknown) the value of case X.
fn case_value(c: &Checker, e: &Enum, enum_name: &str, case: &str, depth: usize) -> Option<Type> {
    (depth < 32).then_some(())?;
    let inferred = c
        .codebase
        .get_enum_case(enum_name.as_bytes(), case.as_bytes())
        .and_then(|m| constant(c, m.value_type.as_ref()?, false, 0));
    if inferred.is_some() {
        return inferred;
    }
    let value = e.members.iter().find_map(|m| match m {
        ClassLikeMember::EnumCase(EnumCase {
            item: EnumCaseItem::Backed(b),
            ..
        }) if b.name.value == case.as_bytes() => Some(b.value),
        _ => None,
    })?;
    let mut value = value;
    while let Expression::Parenthesized(p) = value {
        value = p.expression;
    }
    // the codex types a float literal as `float`
    if let Expression::Literal(Literal::Float(f)) = value {
        return Some(Type::ConstFloat(f.value.0));
    }
    if let Expression::Access(Access::Property(p)) = value
        && let ClassLikeMemberSelector::Identifier(property) = &p.property
        && property.value == b"value"
        && let Expression::Access(Access::ClassConstant(k)) = p.object
        && matches!(k.class, Expression::Self_(_) | Expression::Static(_))
        && let ClassLikeConstantSelector::Identifier(other) = &k.constant
    {
        let other = String::from_utf8_lossy(other.value);
        return case_value(c, e, enum_name, &other, depth + 1);
    }
    None
}

/// A codex constant's inferred type as PHPStan's constant type (see [`constant()`]).
pub(super) fn constant_value(c: &Checker, a: &TAtomic) -> Option<Type> {
    constant(c, a, false, 0)
}

/// A codex constant type as PHPStan's constant type, following references to class constants and, in `value-of`
/// (the codex's `X::Case->value`, always of one case), to an enum case's value.
fn constant(c: &Checker, a: &TAtomic, value_of: bool, depth: usize) -> Option<Type> {
    // a constant defined by itself is another rule's error
    (depth < 32).then_some(())?;
    match a {
        TAtomic::Scalar(TScalar::Integer(TInteger::Literal(i))) => Some(Type::ConstInt(*i)),
        TAtomic::Scalar(TScalar::String(TString {
            literal: Some(TStringLiteral::Value(v)),
            ..
        })) => Some(Type::ConstString(
            String::from_utf8_lossy(v.as_bytes()).into_owned(),
        )),
        TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { value })) => Some(
            Type::ConstString(String::from_utf8_lossy(value.as_bytes()).into_owned()),
        ),
        TAtomic::Scalar(TScalar::Bool(b)) => b.value.map(Type::ConstBool),
        TAtomic::Null => Some(Type::Null),
        TAtomic::Reference(TReference::Member {
            class_like_name,
            member_selector: TReferenceMemberSelector::Identifier(name),
        }) => {
            let (class, name) = (class_like_name.as_bytes(), name.as_bytes());
            let value = if value_of {
                c.codebase.get_enum_case(class, name)?.value_type.as_ref()
            } else {
                c.codebase
                    .get_class_constant(class, name)?
                    .inferred_type
                    .as_ref()
            };
            constant(c, value?, false, depth + 1)
        }
        TAtomic::Derived(TDerived::ValueOf(v)) => {
            constant(c, v.get_target_type().types.first()?, true, depth + 1)
        }
        _ => None,
    }
}

/// EnumCaseOutsideEnumRule.
pub fn case_outside<'arena>(c: &mut Checker<'_, 'arena>, case: &EnumCase<'arena>) {
    let in_enum = c.trait_context.is_none()
        && c.decls
            .last()
            .is_some_and(|d| d.kind == mago_codex::symbol::SymbolKind::Enum);
    if !in_enum {
        c.report_non_ignorable(
            case,
            "enum.caseOutsideOfEnum",
            "Enum case can only be used in enums.".to_string(),
            None,
        );
    }
}
