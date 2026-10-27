//! PHPStan's `Type::describe(VerbosityLevel::typeOnly())` for the types Mago's analyser gives, as the report rules
//! ported from phpstan-strict-rules print them in their messages (`Variable method call on %s.`).
//!
//! A union is described as PHPStan's `UnionType` describes itself at that level: each member generalized (a literal
//! `'a'` is `string`, `1` is `int`, `true` is `bool`), the same descriptions merged, sorted as
//! `UnionTypeHelper::sortTypes` sorts them (`null` last, the rest by `strcasecmp` of their descriptions), and joined
//! with `|`; anything with `mixed` in it is `mixed`. An object is its class name, with its type arguments in `<>` when
//! it has them, a class the codebase does not know too; `$this` is `$this(Class)` and `static` is `static(Class)`.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::TType;
use mago_codex::ttype::union::TUnion;

/// One member of a union, generalized; an intersection (`A&B`) is its members described and sorted as
/// `IntersectionType::describe` sorts them (`UnionTypeHelper::sortTypes`).
fn atomic(t: &TAtomic, alone: bool) -> String {
    let Some(others) = t.get_intersection_types().filter(|o| !o.is_empty()) else {
        return single(t, alone);
    };
    let mut parts = vec![single(t, true)];
    parts.extend(others.iter().map(|o| single(o, true)));
    parts.sort_by_key(|p| p.to_ascii_lowercase());
    parts.join("&")
}

fn single(t: &TAtomic, alone: bool) -> String {
    match t {
        TAtomic::Null => "null".into(),
        TAtomic::Never => "never".into(),
        TAtomic::Void => "void".into(),
        TAtomic::Scalar(s) => match s {
            // A lone literal boolean is `true` or `false` (`ConstantBooleanType`); in a union it is generalized.
            TScalar::Bool(b) if alone && b.is_true() => "true".into(),
            TScalar::Bool(b) if alone && b.is_false() => "false".into(),
            TScalar::Bool(_) => "bool".into(),
            TScalar::Integer(_) => "int".into(),
            TScalar::Float(_) => "float".into(),
            TScalar::String(_) | TScalar::ClassLikeString(_) => "string".into(),
            TScalar::ArrayKey => "int|string".into(),
            TScalar::Numeric => "float|int|numeric-string".into(),
            TScalar::Generic => "bool|float|int|string".into(),
        },
        TAtomic::Array(_) => "array".into(),
        TAtomic::Iterable(_) => "iterable".into(),
        TAtomic::Resource(_) => "resource".into(),
        TAtomic::Callable(c) => match c.is_closure() {
            true => "Closure".into(),
            false => "callable".into(),
        },
        TAtomic::Object(TObject::Named(n)) => {
            let mut name = n.name.to_string();
            if let Some(params) = &n.type_parameters {
                let params: Vec<String> = params.iter().map(describe).collect();
                name = format!("{name}<{}>", params.join(", "));
            }
            if n.is_this {
                format!("$this({name})")
            } else if n.is_static {
                format!("static({name})")
            } else {
                name
            }
        }
        TAtomic::Object(TObject::Enum(e)) => e.name.to_string(),
        TAtomic::Object(_) => "object".into(),
        // A class the codebase does not know stays a reference; PHPStan's `ObjectType` describes it by its name.
        TAtomic::Reference(TReference::Symbol { name, .. }) => name.to_string(),
        _ => "mixed".into(),
    }
}

/// `$type->describe(VerbosityLevel::typeOnly())` for `t`.
pub(crate) fn describe(t: &TUnion) -> String {
    let alone = t.types.len() == 1;
    let mut parts: Vec<String> = Vec::new();
    for t in t.types.iter() {
        for part in atomic(t, alone).split('|') {
            if !parts.iter().any(|p| p == part) {
                parts.push(part.to_string());
            }
        }
    }
    if parts.iter().any(|p| p == "mixed") {
        return "mixed".into();
    }
    if parts.iter().any(|p| p == "bool") {
        parts.retain(|p| p != "true" && p != "false");
    }
    parts.sort_by(|a, b| {
        (a == "null")
            .cmp(&(b == "null"))
            .then_with(|| a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()))
            .then_with(|| a.cmp(b))
    });
    parts.join("|")
}
