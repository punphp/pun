//! Rector's `StaticTypeMapper::mapPHPStanTypeToPhpParserNode` for a return type (`TypeKind::RETURN`), on Mago's
//! types: the declaration Rector writes for a native type, or none when PHP cannot declare it at the target version.
//! Classes are written fully qualified, a union keeps its order with `null` last, and one type with `null` is
//! written `?T`.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_codex::ttype::atomic::scalar::TScalar;

/// `PhpVersionFeature::OBJECT_TYPE`.
const OBJECT_TYPE: u32 = 70200;
/// `PhpVersionFeature::NULLABLE_TYPE`.
const NULLABLE_TYPE: u32 = 70100;
/// `PhpVersionFeature::UNION_TYPES` and `STATIC_RETURN_TYPE`.
const PHP_80: u32 = 80000;
/// `PhpVersionFeature::NULL_FALSE_TRUE_STANDALONE_TYPE`.
const NULL_FALSE_TRUE_STANDALONE_TYPE: u32 = 80200;
/// `PhpVersionFeature::DNF_TYPES`.
const DNF_TYPES: u32 = 80200;
/// `PhpVersionFeature::INTERSECTION_TYPES`.
const INTERSECTION_TYPES: u32 = 80100;

/// The declaration of the union of `types` as a return type at `php_version`. `order` lists types in the order
/// PHPStan has them (a declared type's own order, which Mago does not keep), as [`order_key`]s; the others follow.
/// With `keep_constant` (`TypeFactory::createMixedPassedOrUnionTypeAndKeepConstant`), `true` and `false` stay
/// themselves, written as such where PHP allows (`BooleanTypeMapper`); without, they are `bool`.
pub(crate) fn return_declaration<'a>(
    types: impl IntoIterator<Item = &'a TAtomic>,
    codebase: &CodebaseMetadata,
    php_version: u32,
    order: &[String],
    keep_constant: bool,
) -> Option<String> {
    let mut names: Vec<String> = Vec::new();
    let mut nullable = false;
    for atomic in types {
        let name = match atomic {
            TAtomic::Null => {
                nullable = true;
                continue;
            }
            TAtomic::Scalar(TScalar::Bool(b)) if keep_constant && b.is_true() => "true".to_string(),
            TAtomic::Scalar(TScalar::Bool(b)) if keep_constant && b.is_false() => {
                "false".to_string()
            }
            _ => atomic_declaration(atomic, codebase, php_version)?,
        };
        if !names.contains(&name) {
            names.push(name);
        }
    }
    // normalizeBooleanType: `true` with `false` is `bool`
    if names.iter().any(|n| n == "true") && names.iter().any(|n| n == "false") {
        let at = names
            .iter()
            .position(|n| n == "true" || n == "false")
            .unwrap_or(0);
        names.insert(at, "bool".to_string());
    }
    let union = names.len() > 1 || nullable;
    for name in &mut names {
        // `UnionTypeMapper` maps each member as `TypeKind::UNION`, where `StaticTypeMapper` writes `self`
        if union && name == "static" {
            *name = "self".to_string();
        }
        let standalone = php_version >= NULL_FALSE_TRUE_STANDALONE_TYPE;
        if (name == "true" && !standalone) || (name == "false" && !standalone && !union) {
            *name = "bool".to_string();
        }
    }
    let mut seen: Vec<String> = Vec::new();
    names.retain(|n| {
        let keep = !seen.contains(n)
            && !((n == "true" || n == "false") && seen.iter().any(|s| s == "bool"));
        seen.push(n.clone());
        keep
    });
    if names.iter().any(|n| n == "bool") {
        names.retain(|n| n != "true" && n != "false");
    }
    names.sort_by_key(|n| {
        let key = order_key(n);
        // a `self` written for a declared `static` keeps its place
        order
            .iter()
            .position(|o| *o == key || (key == "self" && o == "static"))
            .unwrap_or(order.len())
    });
    // a DNF type (PHP 8.2) wraps each intersection of a union in parentheses
    if (names.len() > 1 || nullable) && names.iter().any(|n| n.contains('&')) {
        if php_version < DNF_TYPES {
            return None;
        }
        for name in &mut names {
            if name.contains('&') {
                *name = format!("({name})");
            }
        }
    }
    match (names.as_slice(), nullable) {
        ([], nullable) => {
            (nullable && php_version >= NULL_FALSE_TRUE_STANDALONE_TYPE).then(|| "null".to_string())
        }
        ([one], false) => Some(one.clone()),
        ([one], true) if one == "void" || one == "never" || one == "mixed" => None,
        ([one], true) if one.starts_with('(') => Some(format!("{one}|null")),
        ([one], true) => (php_version >= NULLABLE_TYPE).then(|| format!("?{one}")),
        (many, _) => {
            if php_version < PHP_80
                || many
                    .iter()
                    .any(|n| n == "void" || n == "never" || n == "mixed")
            {
                return None;
            }
            let mut union = many.join("|");
            if nullable {
                union.push_str("|null");
            }
            Some(union)
        }
    }
}

/// The order of the members of a declared type, from its text (`null|int|Foo` gives `int`, `foo`).
pub(crate) fn text_order(text: &str) -> Vec<String> {
    text.split('|')
        .map(|t| {
            order_key(
                t.trim()
                    .trim_start_matches('?')
                    .trim_matches(|c| c == '(' || c == ')'),
            )
        })
        .filter(|k| k != "null")
        .collect()
}

/// A type's name for ordering: its last name segment, in lowercase.
fn order_key(name: &str) -> String {
    name.trim()
        .rsplit('\\')
        .next()
        .unwrap_or(name)
        .to_ascii_lowercase()
}

/// A class's name as declared, fully qualified.
fn class(codebase: &CodebaseMetadata, name: &impl AsRef<[u8]>) -> String {
    let name = String::from_utf8_lossy(name.as_ref());
    match codebase.get_class_like(name.as_bytes()) {
        Some(class) => format!("\\{}", class.original_name),
        None => format!("\\{name}"),
    }
}

/// The declaration of one type that is not `null`.
fn atomic_declaration(
    atomic: &TAtomic,
    codebase: &CodebaseMetadata,
    php_version: u32,
) -> Option<String> {
    Some(match atomic {
        TAtomic::Scalar(TScalar::Integer(_)) => "int".to_string(),
        TAtomic::Scalar(TScalar::Float(_)) => "float".to_string(),
        TAtomic::Scalar(TScalar::String(_) | TScalar::ClassLikeString(_)) => "string".to_string(),
        TAtomic::Scalar(TScalar::Bool(_)) => "bool".to_string(),
        TAtomic::Array(_) => "array".to_string(),
        TAtomic::Iterable(_) => "iterable".to_string(),
        TAtomic::Void => "void".to_string(),
        TAtomic::Object(TObject::Any) if php_version >= OBJECT_TYPE => "object".to_string(),
        TAtomic::Object(TObject::Enum(e)) => class(codebase, &e.get_name()),
        TAtomic::Callable(c) if c.is_closure() => "\\Closure".to_string(),
        TAtomic::Callable(_) => "callable".to_string(),
        TAtomic::Object(TObject::Named(named)) => {
            let name = if named.is_this || named.is_static {
                // Rector's `StaticTypeMapper`: `self` where PHP has no `static` return type.
                if php_version < PHP_80 {
                    "self"
                } else {
                    "static"
                }
                .to_string()
            } else if named.name.as_bytes().starts_with(b"{anonymous-class:") {
                if php_version < OBJECT_TYPE {
                    return None;
                }
                return Some("object".to_string());
            } else {
                class(codebase, &named.name)
            };
            match &named.intersection_types {
                None => name,
                Some(_) if php_version < INTERSECTION_TYPES => return None,
                Some(others) => {
                    let mut all = vec![name];
                    for other in others.iter() {
                        all.push(match other {
                            TAtomic::Object(TObject::Named(o)) => class(codebase, &o.name),
                            // a class Mago does not know
                            TAtomic::Reference(TReference::Symbol { name, .. }) => {
                                class(codebase, name)
                            }
                            _ => return None,
                        });
                    }
                    all.join("&")
                }
            }
        }
        _ => return None,
    })
}

/// The members of an expression's type in the order PHPStan's `TypeCombinator::union` leaves them: other types
/// first (in Mago's order, which is all Pun knows of the order they came in), then integer ranges, arrays, enum cases,
/// and last the constant values, grouped by kind in the order each kind first appears.
pub(crate) fn union_order(types: &[TAtomic]) -> Vec<&TAtomic> {
    let constant_kind = |t: &TAtomic| -> Option<u8> {
        match t {
            TAtomic::Null => Some(0),
            TAtomic::Scalar(TScalar::Bool(b)) if !b.is_general() => Some(1),
            TAtomic::Scalar(TScalar::Integer(i)) if i.is_literal() => Some(2),
            TAtomic::Scalar(TScalar::Float(f)) if f.is_literal() => Some(3),
            TAtomic::Scalar(TScalar::String(s)) if s.is_known_literal() => Some(4),
            _ => None,
        }
    };
    let group = |t: &TAtomic| -> u8 {
        match t {
            _ if constant_kind(t).is_some() => 4,
            TAtomic::Scalar(TScalar::Integer(i)) if !i.is_unspecified() => 1,
            TAtomic::Array(_) => 2,
            TAtomic::Object(TObject::Enum(e)) if e.case.is_some() => 3,
            _ => 0,
        }
    };
    let mut kinds: Vec<u8> = Vec::new();
    for t in types {
        if let Some(k) = constant_kind(t)
            && !kinds.contains(&k)
        {
            kinds.push(k);
        }
    }
    let mut ordered: Vec<&TAtomic> = types.iter().collect();
    ordered.sort_by_key(|t| {
        let kind = constant_kind(t)
            .and_then(|k| kinds.iter().position(|x| *x == k))
            .unwrap_or(0);
        (group(t), kind)
    });
    ordered
}

/// `UnionTypeMapper::mapToPhpParserNode` for a return type: each member mapped in order (`null` and `false` allowed),
/// repeats dropped; one left is that type, `null` with one other is `?T` (PHP 7.1), more are a union (PHP 8.0). A
/// non-empty array or a list is an intersection in PHPStan, which `IntersectionTypeMapper` does not write.
pub(crate) fn union_declaration(
    types: &[&TAtomic],
    codebase: &CodebaseMetadata,
    php_version: u32,
) -> Option<String> {
    let mut names: Vec<String> = Vec::new();
    for t in types {
        let name = match t {
            TAtomic::Null if php_version >= NULLABLE_TYPE => "null".to_string(),
            TAtomic::Scalar(TScalar::Bool(b)) if b.is_false() => "false".to_string(),
            TAtomic::Scalar(TScalar::Bool(b))
                if b.is_true() && php_version >= NULL_FALSE_TRUE_STANDALONE_TYPE =>
            {
                "true".to_string()
            }
            TAtomic::Void | TAtomic::Mixed(_) | TAtomic::Null => return None,
            _ if super::type_factory::is_accessory_array(t) => return None,
            _ => atomic_declaration(t, codebase, php_version)?,
        };
        // `StaticTypeMapper` writes `self` for a member of a union (`TypeKind::UNION`)
        let name = if name == "static" {
            "self".to_string()
        } else {
            name
        };
        // `IntersectionTypeMapper`: an intersection inside a union needs DNF types
        if name.contains('&') && php_version < DNF_TYPES {
            return None;
        }
        if !names.contains(&name) {
            names.push(name);
        }
    }
    match names.as_slice() {
        [] => None,
        // the members wrote the same (`array` for two arrays): `matchPhpParserUnionType` gives that one
        [one] => Some(one.clone()),
        [a, b] if !a.contains('&') && !b.contains('&') && (a == "null" || b == "null") => {
            let other = if a == "null" { b } else { a };
            if other == "false" && php_version < NULL_FALSE_TRUE_STANDALONE_TYPE {
                return Some("?bool".to_string());
            }
            Some(format!("?{other}"))
        }
        many => {
            if php_version < PHP_80 {
                return None;
            }
            let object = many.iter().any(|n| n == "object");
            let class = many
                .iter()
                .any(|n| n.starts_with('\\') || n == "self" || n == "static");
            if object && class {
                return None;
            }
            Some(
                many.iter()
                    .map(|n| {
                        if n.contains('&') {
                            format!("({n})")
                        } else {
                            n.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("|"),
            )
        }
    }
}
