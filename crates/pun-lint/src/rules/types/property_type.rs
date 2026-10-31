//! What Rector 2.6.1's property type rules share (rules/TypeDeclaration/Rector/Property and Class_): PHPStan's types as
//! these rules combine them (`TypeFactory`, `TypeCombinator::union`, `TypeHasher`, `TypeComparator`), read from Mago's
//! (see [`super::facts`]); `StaticTypeMapper` for a property (`TypeKind::PROPERTY`); `PropertyTypeChangeGuard` and
//! `PropertyTypeOverrideGuard`; `AssignToPropertyTypeInferer`; and the `@var` edits (`PhpDocTypeChanger::changeVarType`,
//! `VarTagRemover::removeVarTagIfUseless`).
//!
//! The union rules are a local port of `TypeFactory::createMixedPassedOrUnionType` and `uniquateTypes`, as far as
//! these rules need them. pun-ty-return ports the same for the return rules; the two merge once both land.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::float::TFloat;
use mago_codex::ttype::atomic::scalar::int::TInteger;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    Access, AnonymousClass, Argument, Assignment, Call, Class, ClassLikeMember,
    ClassLikeMemberSelector, Expression, Hint, Literal, Method, Modifier, Node, PlainProperty,
    Property, PropertyItem, Sequence, UnaryPrefixOperator, Variable,
};
use pun_problem::{Edit, Span};

use super::facts::Facts;
use super::names::resolve_class_name;
use crate::rules::lint_b_doc::Doc;
use crate::rules::phpunit::annotations::Annotated;
use crate::rules::phpunit::phpdoc_type::{TagType, TypedTag, typed_tag, with_tag_type};
use crate::syntax::{node_span, unparenthesize};

/// `PhpVersionFeature::NULLABLE_TYPE`.
const NULLABLE_TYPE: u32 = 70100;
/// `PhpVersionFeature::OBJECT_TYPE`.
const OBJECT_TYPE: u32 = 70200;
/// `PhpVersionFeature::UNION_TYPES`.
const UNION_TYPES: u32 = 80000;
/// `PhpVersionFeature::INTERSECTION_TYPES`.
const INTERSECTION_TYPES: u32 = 80100;
/// `PhpVersionFeature::NULL_FALSE_TRUE_STANDALONE_TYPE` and `DNF_TYPES`.
const PHP_82: u32 = 80200;
/// `PhpVersionFeature::TYPED_PROPERTIES`.
pub(crate) const TYPED_PROPERTIES: u32 = 70400;

/// A PHPStan type, as far as the property rules tell types apart.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Ty {
    Mixed,
    Null,
    /// `bool`, or the constant `true` or `false`.
    Bool(Option<bool>),
    Int(Scalar<i64>),
    Float(Scalar<String>),
    Str(Scalar<Vec<u8>>),
    /// A `class-string`.
    ClassString,
    /// `array<key, value>`.
    Array(Box<Ty>, Box<Ty>),
    /// An array whose items are known (`ConstantArrayType`): each key and value.
    ConstArray(Vec<(Ty, Ty)>),
    /// An object of a class or enum, by its name as Mago spells it.
    Object(String),
    /// `static` or `$this` in a class.
    Static(String),
    /// `parent` in a declaration (`ParentStaticType`).
    Parent(String),
    /// `object`, or an object of an anonymous class.
    ObjectAny,
    /// `new class extends A implements B {}` as `getNativeType` reads it (`ObjectWithoutClassTypeWithParentTypes`): the
    /// classes and interfaces it names, in order.
    AnonymousWithParents(Vec<String>),
    Closure,
    Callable,
    Iterable,
    Intersection(Vec<String>),
    Union(Vec<Ty>),
    /// Anything else, by Mago's id.
    Other(String),
}

/// A scalar type: general, one constant value, or a refinement (`int<0, max>`, `non-empty-string`).
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Scalar<T> {
    General,
    Literal(T),
    Refined,
}

impl Ty {
    /// The members of a union, or the type alone.
    pub(crate) fn members(&self) -> Vec<&Ty> {
        match self {
            Ty::Union(types) => types.iter().collect(),
            other => vec![other],
        }
    }

    /// Whether this is a `UnionType`.
    pub(crate) fn is_union(&self) -> bool {
        matches!(self, Ty::Union(_))
    }

    fn is_array(&self) -> bool {
        self.members()
            .iter()
            .all(|t| matches!(t, Ty::Array(..) | Ty::ConstArray(_)))
    }

    /// Whether the type is a constant value (`isConstantValue()`).
    fn is_constant(&self) -> bool {
        matches!(
            self,
            Ty::Null
                | Ty::Bool(Some(_))
                | Ty::Int(Scalar::Literal(_))
                | Ty::Float(Scalar::Literal(_))
                | Ty::Str(Scalar::Literal(_))
                | Ty::ConstArray(_)
        )
    }

    /// `removeValueFromConstantType`.
    fn without_value(&self) -> Ty {
        match self {
            Ty::Int(Scalar::Literal(_)) => Ty::Int(Scalar::General),
            Ty::Float(Scalar::Literal(_)) => Ty::Float(Scalar::General),
            Ty::Str(Scalar::Literal(_)) => Ty::Str(Scalar::General),
            Ty::Bool(Some(_)) => Ty::Bool(None),
            other => other.clone(),
        }
    }

    /// `describe(VerbosityLevel::precise())`, as far as two types it tells apart differ.
    fn describe(&self) -> String {
        match self {
            Ty::Mixed => "mixed".into(),
            Ty::Null => "null".into(),
            Ty::Bool(None) => "bool".into(),
            Ty::Bool(Some(b)) => b.to_string(),
            Ty::Int(Scalar::General) => "int".into(),
            Ty::Int(Scalar::Literal(v)) => v.to_string(),
            Ty::Int(Scalar::Refined) => "int<refined>".into(),
            Ty::Float(Scalar::General) => "float".into(),
            Ty::Float(Scalar::Literal(v)) => v.clone(),
            Ty::Float(Scalar::Refined) => "float<refined>".into(),
            Ty::Str(Scalar::General) => "string".into(),
            Ty::Str(Scalar::Literal(v)) => format!("'{}'", String::from_utf8_lossy(v)),
            Ty::Str(Scalar::Refined) => "non-empty-string".into(),
            Ty::ClassString => "class-string".into(),
            Ty::Array(k, v) => format!("array<{}, {}>", k.describe(), v.describe()),
            Ty::ConstArray(items) => format!(
                "array{{{}}}",
                items
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k.describe(), v.describe()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Ty::Object(name) | Ty::Static(name) => name.clone(),
            Ty::Parent(name) => format!("static({name})"),
            Ty::ObjectAny | Ty::AnonymousWithParents(_) => "object".into(),
            Ty::Closure => "Closure".into(),
            Ty::Callable => "callable".into(),
            Ty::Iterable => "iterable".into(),
            Ty::Intersection(names) => names.join("&"),
            Ty::Union(types) => types.iter().map(Ty::describe).collect::<Vec<_>>().join("|"),
            Ty::Other(id) => id.clone(),
        }
    }

    /// `TypeHasher::createTypeHash`.
    fn hash(&self) -> String {
        match self {
            Ty::Mixed => "mixed".into(),
            Ty::Array(k, v) => format!("{}{}{}[]", v.hash(), k.hash(), v.describe()),
            Ty::Object(name) | Ty::Static(name) | Ty::Parent(name) => name.to_ascii_lowercase(),
            Ty::Closure => "closure".into(),
            Ty::Null => "NullType".into(),
            Ty::Bool(Some(_)) => "ConstantBooleanType".into(),
            Ty::Int(Scalar::Literal(_)) => "ConstantIntegerType".into(),
            Ty::Float(Scalar::Literal(_)) => "ConstantFloatType".into(),
            Ty::Str(Scalar::Literal(_)) => "ConstantStringType".into(),
            Ty::ConstArray(_) => "ConstantArrayType".into(),
            Ty::Union(types) => {
                let mut hashes: Vec<String> = types.iter().map(Ty::describe).collect();
                hashes.sort();
                hashes.join("|")
            }
            other => other.describe(),
        }
    }
}

/// PHPStan's type from Mago's.
pub(crate) fn ty(t: &TUnion) -> Ty {
    let mut members: Vec<Ty> = Vec::new();
    for atomic in t.types.iter() {
        let member = atomic_ty(atomic);
        if !members.contains(&member) {
            members.push(member);
        }
    }
    // `true` and `false` are `bool`
    if members.contains(&Ty::Bool(Some(true))) && members.contains(&Ty::Bool(Some(false))) {
        let at = members
            .iter()
            .position(|m| matches!(m, Ty::Bool(Some(_))))
            .unwrap_or(0);
        members.retain(|m| !matches!(m, Ty::Bool(Some(_))));
        members.insert(at.min(members.len()), Ty::Bool(None));
    }
    if members.contains(&Ty::Mixed) {
        return Ty::Mixed;
    }
    match members.len() {
        0 => Ty::Other("never".into()),
        1 => members.remove(0),
        _ => Ty::Union(members),
    }
}

fn atomic_ty(atomic: &TAtomic) -> Ty {
    match atomic {
        TAtomic::Null => Ty::Null,
        TAtomic::Mixed(_) => Ty::Mixed,
        TAtomic::Scalar(TScalar::Bool(b)) => Ty::Bool(b.value),
        TAtomic::Scalar(TScalar::Integer(i)) => Ty::Int(match i {
            TInteger::Literal(v) => Scalar::Literal(*v),
            TInteger::Unspecified => Scalar::General,
            _ => Scalar::Refined,
        }),
        TAtomic::Scalar(TScalar::Float(f)) => Ty::Float(match f {
            TFloat::Literal(v) => Scalar::Literal(v.to_string()),
            TFloat::Float => Scalar::General,
            TFloat::UnspecifiedLiteral => Scalar::Refined,
        }),
        TAtomic::Scalar(TScalar::String(s)) => Ty::Str(match s.get_known_literal_value() {
            Some(v) => Scalar::Literal(v.to_vec()),
            None if s.literal.is_none() && !s.is_non_empty && !s.is_truthy && !s.is_numeric => {
                Scalar::General
            }
            None => Scalar::Refined,
        }),
        TAtomic::Scalar(TScalar::ClassLikeString(_)) => Ty::ClassString,
        TAtomic::Array(TArray::List(list)) => match &list.known_elements {
            Some(known) if list.element_type.is_never() => Ty::ConstArray(
                known
                    .iter()
                    .map(|(i, (_, v))| (Ty::Int(Scalar::Literal(*i as i64)), ty(v)))
                    .collect(),
            ),
            _ if list.element_type.is_never() => Ty::ConstArray(Vec::new()),
            _ => Ty::Array(
                Box::new(Ty::Int(Scalar::Refined)),
                Box::new(ty(&list.element_type)),
            ),
        },
        TAtomic::Array(TArray::Keyed(keyed)) => match (&keyed.known_items, &keyed.parameters) {
            (known, None) => Ty::ConstArray(
                known
                    .iter()
                    .flat_map(|m| m.iter())
                    .map(|(k, (_, v))| {
                        let key = match k {
                            ArrayKey::Integer(i) => Ty::Int(Scalar::Literal(*i)),
                            ArrayKey::String(s) => Ty::Str(Scalar::Literal(s.as_bytes().to_vec())),
                            _ => Ty::Other("key".into()),
                        };
                        (key, ty(v))
                    })
                    .collect(),
            ),
            (_, Some((k, v))) => {
                let key = if k
                    .types
                    .iter()
                    .all(|a| matches!(a, TAtomic::Scalar(TScalar::ArrayKey)))
                {
                    Ty::Mixed
                } else {
                    ty(k)
                };
                Ty::Array(Box::new(key), Box::new(ty(v)))
            }
        },
        TAtomic::Object(TObject::Named(named)) => {
            let name = named.name.to_string();
            if name.starts_with("{anonymous-class:") {
                Ty::ObjectAny
            } else if let Some(others) = &named.intersection_types {
                let mut names = vec![name];
                for other in others.iter() {
                    match other {
                        TAtomic::Object(TObject::Named(o)) => names.push(o.name.to_string()),
                        _ => return Ty::Other(atomic.get_id().to_string()),
                    }
                }
                Ty::Intersection(names)
            } else if named.is_this || named.is_static {
                Ty::Static(name)
            } else if name.eq_ignore_ascii_case("closure") {
                Ty::Closure
            } else {
                Ty::Object(name)
            }
        }
        TAtomic::Object(TObject::Enum(e)) => Ty::Object(e.name.to_string()),
        TAtomic::Object(TObject::Any) => Ty::ObjectAny,
        TAtomic::Callable(c) if c.is_closure() => Ty::Closure,
        TAtomic::Callable(_) => Ty::Callable,
        TAtomic::Iterable(_) => Ty::Iterable,
        other => Ty::Other(other.get_id().to_string()),
    }
}

/// The union of `types` as `TypeCombinator::union` leaves it, as far as these rules tell: repeats dropped, a constant
/// dropped when its general type is there, `true` with `false` made `bool`, `mixed` absorbing the rest.
pub(crate) fn union(types: Vec<Ty>) -> Ty {
    let mut flat: Vec<Ty> = Vec::new();
    for t in types {
        for m in t.members() {
            if !flat.contains(m) {
                flat.push(m.clone());
            }
        }
    }
    if flat.contains(&Ty::Mixed) {
        return Ty::Mixed;
    }
    if flat.contains(&Ty::Bool(Some(true))) && flat.contains(&Ty::Bool(Some(false))) {
        let at = flat
            .iter()
            .position(|m| matches!(m, Ty::Bool(Some(_))))
            .unwrap_or(0);
        flat.retain(|m| !matches!(m, Ty::Bool(Some(_))));
        flat.insert(at.min(flat.len()), Ty::Bool(None));
    }
    let generals: Vec<Ty> = flat.iter().filter(|t| !t.is_constant()).cloned().collect();
    flat.retain(|t| {
        !t.is_constant() || matches!(t, Ty::Null) || !generals.contains(&t.without_value())
    });
    flat.dedup();
    match flat.len() {
        0 => Ty::Other("never".into()),
        1 => flat.remove(0),
        _ => Ty::Union(flat),
    }
}

/// `TypeFactory::createMixedPassedOrUnionType`.
pub(crate) fn mixed_passed_or_union(types: Vec<Ty>, keep_constant: bool) -> Ty {
    let mut unwrapped = Vec::new();
    for t in &types {
        for m in t.members() {
            match m {
                Ty::ConstArray(items) => unwrapped.extend(unwrap_const_array(items)),
                other => unwrapped.push(other.clone()),
            }
        }
    }
    let mut unique = uniquate(unwrapped, keep_constant);
    match unique.len() {
        0 => Ty::Mixed,
        1 => unique.remove(0),
        _ if unique.contains(&Ty::Mixed) => Ty::Mixed,
        _ => Ty::Union(unique),
    }
}

/// `unwrapConstantArrayTypes`: one `array<key, value>` per item of the flattened key and value types.
fn unwrap_const_array(items: &[(Ty, Ty)]) -> Vec<Ty> {
    let mut keys: Vec<Ty> = Vec::new();
    let mut values: Vec<Ty> = Vec::new();
    for (k, v) in items {
        for m in k.members() {
            if !keys.contains(m) {
                keys.push(m.clone());
            }
        }
        for m in v.members() {
            if !values.contains(m) {
                values.push(m.clone());
            }
        }
    }
    if values.is_empty() {
        values.push(Ty::Other("never".into()));
    }
    values
        .into_iter()
        .enumerate()
        .flat_map(|(i, value)| {
            let key = keys.get(i).cloned().unwrap_or(Ty::Mixed);
            match &value {
                Ty::ConstArray(inner) => unwrap_const_array(inner)
                    .into_iter()
                    .map(|a| Ty::Array(Box::new(key.clone()), Box::new(a)))
                    .collect(),
                _ => vec![Ty::Array(Box::new(key), Box::new(value))],
            }
        })
        .collect()
}

/// `TypeFactory::uniquateTypes`.
fn uniquate(types: Vec<Ty>, keep_constant: bool) -> Vec<Ty> {
    let (mut has_true, mut has_false) = (false, false);
    let mut unique: Vec<(String, Ty)> = Vec::new();
    let mut constant_hashes: Vec<(String, String)> = Vec::new();
    let total = types.len();
    for t in types {
        // normalizeObjectType
        let t = match t {
            Ty::AnonymousWithParents(parents) if total > 1 => Ty::Object(parents[0].clone()),
            other => other,
        };
        // normalizeBooleanType
        if t == Ty::Bool(Some(true)) {
            has_true = true;
        }
        if t == Ty::Bool(Some(false)) {
            has_false = true;
        }
        let t = if has_true && has_false && matches!(t, Ty::Bool(Some(_))) {
            Ty::Bool(None)
        } else {
            t
        };
        let removed = t.without_value();
        let (t, hash) = if keep_constant && t != removed {
            let hash = t.hash();
            constant_hashes.push((hash.clone(), removed.hash()));
            (t, hash)
        } else {
            let hash = removed.hash();
            (removed, hash)
        };
        match unique.iter_mut().find(|(h, _)| *h == hash) {
            Some(slot) => slot.1 = t,
            None => unique.push((hash, t)),
        }
    }
    for (constant, general) in constant_hashes {
        if unique.iter().any(|(h, _)| *h == general) {
            unique.retain(|(h, _)| *h != constant);
        }
    }
    unique.into_iter().map(|(_, t)| t).collect()
}

/// `TypeComparator::areTypesEqual`, as far as these rules need it: the same hash, or both all strings, all integers,
/// all floats or all booleans of the same kind.
pub(crate) fn are_equal(a: &Ty, b: &Ty) -> bool {
    if a.hash() == b.hash() {
        return true;
    }
    let all = |t: &Ty, f: fn(&Ty) -> bool| t.members().iter().all(|m| f(m));
    let general = |t: &Ty| !t.is_union() && !t.is_constant();
    let string = |t: &Ty| matches!(t, Ty::Str(_));
    let int = |t: &Ty| matches!(t, Ty::Int(_));
    let float = |t: &Ty| matches!(t, Ty::Float(_));
    let boolean = |t: &Ty| matches!(t, Ty::Bool(_));
    (all(a, string) && all(b, string) && general(a) == general(b) && a.is_union() == b.is_union())
        || (all(a, int) && all(b, int) && general(a) == general(b) && a.is_union() == b.is_union())
        || (all(a, float) && all(b, float))
        || (all(a, boolean) && all(b, boolean))
}

/// The type of the declaration Rector writes for `t` (`mapPhpParserNodePHPStanType` of it): constants as their general
/// type, arrays as `array`.
pub(crate) fn declared_ty(t: &Ty) -> Ty {
    let one = |m: &Ty| match m {
        Ty::Array(..) | Ty::ConstArray(_) => Ty::Array(Box::new(Ty::Mixed), Box::new(Ty::Mixed)),
        Ty::Int(_) => Ty::Int(Scalar::General),
        Ty::Float(_) => Ty::Float(Scalar::General),
        Ty::Str(_) | Ty::ClassString => Ty::Str(Scalar::General),
        Ty::Bool(_) => Ty::Bool(None),
        other => other.clone(),
    };
    match t {
        Ty::Union(types) => mixed_passed_or_union(types.iter().map(one).collect(), true),
        other => one(other),
    }
}

/// The type of a type declaration as written (`StaticTypeMapper::mapPhpParserNodePHPStanType`): names resolved, a
/// union or `?T` kept in the written order (`TypeFactory` with constants kept).
pub(crate) fn hint_ty(
    hint: &Hint<'_>,
    ancestors: &[Node<'_, '_>],
    scope: &ClassView<'_, '_>,
) -> Ty {
    match hint {
        Hint::Identifier(id) => {
            let name = resolve_class_name(ancestors, id);
            if name.eq_ignore_ascii_case("closure") {
                Ty::Closure
            } else {
                Ty::Object(name)
            }
        }
        Hint::Parenthesized(p) => hint_ty(p.hint, ancestors, scope),
        Hint::Nullable(n) => {
            mixed_passed_or_union(vec![hint_ty(n.hint, ancestors, scope), Ty::Null], false)
        }
        Hint::Union(_) => {
            let mut members = Vec::new();
            union_members(hint, &mut members);
            mixed_passed_or_union(
                members
                    .into_iter()
                    .map(|m| hint_ty(m, ancestors, scope))
                    .collect(),
                true,
            )
        }
        Hint::Intersection(_) => {
            let mut members = Vec::new();
            intersection_members(hint, &mut members);
            let names: Vec<String> = members
                .into_iter()
                .filter_map(|m| match m {
                    Hint::Identifier(id) => Some(resolve_class_name(ancestors, id)),
                    _ => None,
                })
                .collect();
            Ty::Intersection(names)
        }
        Hint::Null(_) => Ty::Null,
        Hint::True(_) => Ty::Bool(Some(true)),
        Hint::False(_) => Ty::Bool(Some(false)),
        Hint::Array(_) => Ty::Array(Box::new(Ty::Mixed), Box::new(Ty::Mixed)),
        Hint::Callable(_) => Ty::Callable,
        Hint::Static(_) | Hint::Self_(_) => Ty::Static(scope.name.clone()),
        Hint::Parent(_) => match &scope.parent {
            Some(parent) => Ty::Parent(parent.clone()),
            None => Ty::Mixed,
        },
        Hint::Void(_) | Hint::Never(_) => Ty::Other("void".into()),
        Hint::Float(_) => Ty::Float(Scalar::General),
        Hint::Bool(_) => Ty::Bool(None),
        Hint::Integer(_) => Ty::Int(Scalar::General),
        Hint::String(_) => Ty::Str(Scalar::General),
        Hint::Object(_) => Ty::ObjectAny,
        Hint::Mixed(_) => Ty::Mixed,
        Hint::Iterable(_) => Ty::Iterable,
    }
}

fn union_members<'a>(hint: &'a Hint<'a>, out: &mut Vec<&'a Hint<'a>>) {
    match hint {
        Hint::Union(u) => {
            union_members(u.left, out);
            union_members(u.right, out);
        }
        other => out.push(other),
    }
}

fn intersection_members<'a>(hint: &'a Hint<'a>, out: &mut Vec<&'a Hint<'a>>) {
    match hint {
        Hint::Intersection(i) => {
            intersection_members(i.left, out);
            intersection_members(i.right, out);
        }
        Hint::Parenthesized(p) => intersection_members(p.hint, out),
        other => out.push(other),
    }
}

/// A class's name as declared, fully qualified.
fn class_text(codebase: &CodebaseMetadata, name: &str) -> String {
    match codebase.get_class_like(name.as_bytes()) {
        Some(class) => format!("\\{}", class.original_name),
        None => format!("\\{}", name.trim_start_matches('\\')),
    }
}

/// `StaticTypeMapper::mapPHPStanTypeToPhpParserNode($type, TypeKind::PROPERTY)`: the declaration Rector writes for a
/// property of type `t` at `php_version`, or `None` where it writes none.
pub(crate) fn property_declaration(
    t: &Ty,
    codebase: &CodebaseMetadata,
    php_version: u32,
) -> Option<String> {
    match t {
        Ty::Union(types) => union_declaration(types, codebase, php_version),
        Ty::Bool(_) => Some("bool".into()),
        Ty::Null | Ty::Callable | Ty::Mixed | Ty::Other(_) => None,
        Ty::Static(_) => Some("self".into()),
        other => member_declaration(other, codebase, php_version),
    }
}

/// One member of a union, mapped with `TypeKind::UNION`, or a type that maps the same with any kind.
fn member_declaration(t: &Ty, codebase: &CodebaseMetadata, php_version: u32) -> Option<String> {
    Some(match t {
        Ty::Int(_) => "int".into(),
        Ty::Float(_) => "float".into(),
        Ty::Str(_) | Ty::ClassString => "string".into(),
        Ty::Array(..) | Ty::ConstArray(_) => "array".into(),
        Ty::Iterable => "iterable".into(),
        Ty::Closure => "\\Closure".into(),
        Ty::Static(_) => "self".into(),
        Ty::Parent(_) => "parent".into(),
        // ObjectWithoutClassTypeMapper
        Ty::AnonymousWithParents(parents) if parents.len() == 1 => {
            class_text(codebase, &parents[0])
        }
        Ty::ObjectAny | Ty::AnonymousWithParents(_) if php_version >= OBJECT_TYPE => {
            "object".into()
        }
        Ty::Object(name) => class_text(codebase, name),
        Ty::Intersection(names) if php_version >= INTERSECTION_TYPES => names
            .iter()
            .map(|n| class_text(codebase, n))
            .collect::<Vec<_>>()
            .join("&"),
        _ => return None,
    })
}

/// `UnionTypeMapper::mapToPhpParserNode` for a property.
fn union_declaration(
    types: &[Ty],
    codebase: &CodebaseMetadata,
    php_version: u32,
) -> Option<String> {
    let mut names: Vec<String> = Vec::new();
    for t in types {
        let name = match t {
            Ty::Null if php_version >= NULLABLE_TYPE => "null".into(),
            Ty::Bool(Some(false)) => "false".into(),
            Ty::Bool(Some(true)) if php_version >= PHP_82 => "true".into(),
            Ty::Bool(_) => "bool".into(),
            // `PropertyAnalyzer::isForbiddenType`
            Ty::Callable => return None,
            other => member_declaration(other, codebase, php_version)?,
        };
        if !names.contains(&name) {
            names.push(name);
        }
    }
    match names.as_slice() {
        [] => None,
        [one] => Some(one.clone()),
        [a, b] if a == "null" || b == "null" => {
            let other = if a == "null" { b } else { a };
            if other.contains('&') {
                return (php_version >= PHP_82).then(|| format!("({other})|null"));
            }
            if other == "false" && php_version < PHP_82 {
                return Some("?bool".into());
            }
            Some(format!("?{other}"))
        }
        many => {
            if php_version < UNION_TYPES {
                return None;
            }
            let object = many.iter().any(|n| n == "object");
            let class = many
                .iter()
                .any(|n| n.starts_with('\\') || matches!(n.as_str(), "self" | "parent"));
            if object && class {
                return None;
            }
            if many.iter().any(|n| n.contains('&')) && php_version < PHP_82 {
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

/// The class a property rule looks at: a class or an anonymous class (php-parser's `Class_`).
pub(crate) struct ClassView<'ast, 'arena> {
    pub node: Node<'ast, 'arena>,
    pub members: &'ast Sequence<'arena, ClassLikeMember<'arena>>,
    /// The class's ancestors and the class itself.
    pub path: Vec<Node<'ast, 'arena>>,
    /// The name the codebase knows the class by.
    pub name: String,
    pub parent: Option<String>,
}

impl<'ast, 'arena> ClassView<'ast, 'arena> {
    pub(crate) fn of(
        facts: &Facts,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
    ) -> Option<Self> {
        let mut path = ancestors.to_vec();
        path.push(node);
        let (members, name, extends) = match node {
            Node::Class(Class {
                members,
                name,
                extends,
                ..
            }) => (
                members,
                match super::namespace_of(ancestors) {
                    Some(ns) => format!("{ns}\\{}", String::from_utf8_lossy(name.value)),
                    None => String::from_utf8_lossy(name.value).into_owned(),
                },
                extends.as_ref(),
            ),
            Node::AnonymousClass(
                c @ AnonymousClass {
                    members, extends, ..
                },
            ) => (
                members,
                facts.anonymous_class_name(mago_span::HasSpan::span(c)),
                extends.as_ref(),
            ),
            _ => return None,
        };
        let parent = extends
            .and_then(|e| e.types.first())
            .map(|id| resolve_class_name(ancestors, id));
        Some(ClassView {
            node,
            members,
            path,
            name,
            parent,
        })
    }

    /// `Class_::getMethod()`: the method of that name, case-insensitively.
    pub(crate) fn method(&self, name: &[u8]) -> Option<&'ast Method<'arena>> {
        self.members.iter().find_map(|m| match m {
            ClassLikeMember::Method(method) if method.name.value.eq_ignore_ascii_case(name) => {
                Some(method)
            }
            _ => None,
        })
    }

    /// The class's untyped plain properties with one item (the others no rule types).
    pub(crate) fn properties(&self) -> Vec<Prop<'ast, 'arena>> {
        self.plain_properties().map(|(_, prop)| prop).collect()
    }

    /// The same selection as [`Self::properties`], retaining the plain declaration for its attributes.
    pub(crate) fn plain_properties(
        &self,
    ) -> impl Iterator<Item = (&'ast PlainProperty<'arena>, Prop<'ast, 'arena>)> {
        self.members.iter().filter_map(|m| match m {
            ClassLikeMember::Property(p @ Property::Plain(plain)) if plain.hint.is_none() => {
                let items: Vec<&PropertyItem<'_>> = plain.items.iter().collect();
                let [item] = items[..] else {
                    return Some((
                        plain,
                        Prop {
                            property: p,
                            item: None,
                        },
                    ));
                };
                Some((
                    plain,
                    Prop {
                        property: p,
                        item: Some(item),
                    },
                ))
            }
            _ => None,
        })
    }
}

/// An untyped property, and its item when it has exactly one.
pub(crate) struct Prop<'ast, 'arena> {
    pub property: &'ast Property<'arena>,
    pub item: Option<&'ast PropertyItem<'arena>>,
}

impl<'ast, 'arena> Prop<'ast, 'arena> {
    /// The property's name, without `$`.
    pub(crate) fn name(&self) -> &'arena [u8] {
        let item = self.item.expect("a single item");
        &item.variable().name[1..]
    }

    pub(crate) fn modifiers(&self) -> &'ast Sequence<'arena, Modifier<'arena>> {
        match self.property {
            Property::Plain(p) => &p.modifiers,
            Property::Hooked(p) => &p.modifiers,
        }
    }

    pub(crate) fn is_private(&self) -> bool {
        self.modifiers()
            .iter()
            .any(|m| matches!(m, Modifier::Private(_)))
    }

    pub(crate) fn is_protected(&self) -> bool {
        self.modifiers()
            .iter()
            .any(|m| matches!(m, Modifier::Protected(_)))
    }

    pub(crate) fn is_public(&self) -> bool {
        !self.is_private() && !self.is_protected()
    }

    pub(crate) fn default(&self) -> Option<&'arena Expression<'arena>> {
        match self.item? {
            PropertyItem::Concrete(c) => Some(c.value),
            PropertyItem::Abstract(_) => None,
        }
    }
}

/// `PropertyTypeChangeGuard::isLegal` (with `MakePropertyTypedGuard`'s check that the property is untyped): one item,
/// no trait the class uses declares or writes the property, and the property's own type is not forbidden. Its
/// callers select private properties before asking with `inline_public = false`; the protected-property fallback
/// of the upstream shared guard is therefore never needed by these rules.
pub(crate) fn is_legal(
    facts: &Facts,
    view: &ClassView<'_, '_>,
    prop: &Prop<'_, '_>,
    inline_public: bool,
) -> bool {
    if prop.item.is_none() {
        return false;
    }
    let name = prop.name();
    if has_trait_with_same_property_or_written(facts, view, name)
        || has_forbidden_type(facts, view, name)
    {
        return false;
    }
    inline_public || prop.is_private()
}

/// `PropertyTypeOverrideGuard::isLegal`'s parent check: the nearest parent class that has the property decides; it
/// must declare a type there.
pub(crate) fn parent_allows(facts: &Facts, view: &ClassView<'_, '_>, name: &[u8]) -> bool {
    for parent in parent_classes(facts, view) {
        if let Some(p) = facts
            .codebase
            .get_declaring_property(parent.as_bytes(), &dollar(name))
        {
            return p.type_declaration_metadata.is_some();
        }
    }
    true
}

fn dollar(name: &[u8]) -> Vec<u8> {
    [b"$".as_slice(), name].concat()
}

/// The parent classes, nearest first.
fn parent_classes(facts: &Facts, view: &ClassView<'_, '_>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut next = view.parent.clone();
    while let Some(name) = next.take() {
        if out.iter().any(|o| o.eq_ignore_ascii_case(&name)) {
            break;
        }
        next = facts
            .codebase
            .get_class_like(name.as_bytes())
            .and_then(|c| c.direct_parent_class)
            .map(|p| p.to_string());
        out.push(name);
    }
    out
}

/// `PropertyManipulator::hasTraitWithSamePropertyOrWritten`: a trait the class uses has the property, or writes it.
fn has_trait_with_same_property_or_written(
    facts: &Facts,
    view: &ClassView<'_, '_>,
    name: &[u8],
) -> bool {
    let traits: Vec<String> = view
        .members
        .iter()
        .filter_map(|m| match m {
            ClassLikeMember::TraitUse(u) => Some(u),
            _ => None,
        })
        .flat_map(|u| {
            u.trait_names
                .iter()
                .map(|id| resolve_class_name(&view.path, id))
        })
        .collect();
    traits.iter().any(|t| {
        if facts
            .codebase
            .get_declaring_property(t.as_bytes(), &dollar(name))
            .is_some()
        {
            return true;
        }
        let Some(class) = facts.codebase.get_class_like(t.as_bytes()) else {
            return false;
        };
        let Some(source) = facts.source_of(class.span.file_id) else {
            return false;
        };
        let (start, end) = (
            class.span.start.offset as usize,
            class.span.end.offset as usize,
        );
        source
            .get(start..end)
            .is_some_and(|text| trait_writes(text, name))
    })
}

/// `PropertyFetchAnalyzer::containsWrittenPropertyFetchName` on a trait's source text: a `$this->name`, `self::$name`
/// or `static::$name` that is assigned, assigned by reference, changed by an operator, unset, incremented or
/// decremented, or under an array offset.
fn trait_writes(trait_text: &str, name: &[u8]) -> bool {
    let code = format!("<?php {trait_text}");
    let arena = mago_allocator::LocalArena::new();
    let file = mago_database::file::File::ephemeral(
        std::borrow::Cow::Borrowed(b"trait.php"),
        std::borrow::Cow::Owned(code.into_bytes()),
    );
    let program = mago_syntax::parser::parse_file(&arena, &file);
    let mut found = false;
    let mut stack = vec![Node::Program(program)];
    while let Some(node) = stack.pop() {
        if found {
            break;
        }
        let written: Vec<&Expression<'_>> = match node {
            Node::Assignment(a) => {
                let mut out = vec![a.lhs];
                if let Expression::UnaryPrefix(u) = a.rhs
                    && matches!(u.operator, UnaryPrefixOperator::Reference(_))
                {
                    out.push(u.operand);
                }
                out
            }
            Node::Unset(u) => u.values.iter().copied().collect(),
            Node::UnaryPrefix(u)
                if matches!(
                    u.operator,
                    UnaryPrefixOperator::PreIncrement(_) | UnaryPrefixOperator::PreDecrement(_)
                ) =>
            {
                vec![u.operand]
            }
            Node::UnaryPostfix(u) => vec![u.operand],
            Node::ArrayAccess(a) => vec![a.array],
            Node::ArrayAppend(a) => vec![a.array],
            _ => Vec::new(),
        };
        if written.iter().any(|e| is_trait_local_fetch(e, name)) {
            found = true;
        }
        node.visit_children(|child| stack.push(child));
    }
    found
}

/// `$this->name`, `self::$name` or `static::$name`.
fn is_trait_local_fetch(expr: &Expression<'_>, name: &[u8]) -> bool {
    match unparenthesize(expr) {
        Expression::Access(Access::Property(p)) => {
            matches!(unparenthesize(p.object), Expression::Variable(Variable::Direct(v)) if v.name == b"$this")
                && matches!(&p.property, ClassLikeMemberSelector::Identifier(id) if id.value == name)
        }
        Expression::Access(Access::StaticProperty(p)) => {
            matches!(p.class, Expression::Self_(_) | Expression::Static(_))
                && matches!(&p.property, Variable::Direct(v) if &v.name[1..] == name)
        }
        _ => false,
    }
}

/// The type PHPStan reads for the property, docblock included (`NodeTypeResolver::getType($property)`): the `@var`
/// type, or `mixed` without one.
pub(crate) fn doc_type(facts: &Facts, view: &ClassView<'_, '_>, name: &[u8]) -> Option<TUnion> {
    let p = facts
        .documented()
        .get_property(view.name.as_bytes(), &dollar(name))?;
    Some(p.type_metadata.as_ref()?.type_union.clone())
}

/// `PropertyAnalyzer::hasForbiddenType`: the property's type is `null`, a class that does not exist
/// (`NonExistingObjectType`), or `callable` or has a `callable` member.
fn has_forbidden_type(facts: &Facts, view: &ClassView<'_, '_>, name: &[u8]) -> bool {
    let Some(t) = doc_type(facts, view, name) else {
        return false;
    };
    if t.types.is_empty() || t.is_mixed() {
        return false;
    }
    if t.types.iter().all(|a| matches!(a, TAtomic::Null)) {
        return true;
    }
    if let [TAtomic::Object(TObject::Named(n))] = t.types.as_ref()
        && n.intersection_types.is_none()
        && !n.is_this
        && !n.is_static
        && facts.codebase.get_class_like(n.name.as_bytes()).is_none()
    {
        return true;
    }
    // an unknown class alone (`NonExistingObjectType`), not in an intersection
    if let [TAtomic::Reference(r)] = t.types.as_ref()
        && !matches!(
            r,
            TReference::Symbol {
                intersection_types: Some(_),
                ..
            }
        )
    {
        return true;
    }
    t.types
        .iter()
        .any(|a| matches!(a, TAtomic::Callable(c) if !c.is_closure()))
}

/// `PropertyFetchAnalyzer::isLocalPropertyFetchName`: `$this->name`, `self::$name`, `static::$name` or the class's
/// own `X::$name`, or a property fetch on an object PHPStan types as the class.
pub(crate) fn is_local_fetch(
    facts: &Facts,
    view: &ClassView<'_, '_>,
    expr: &Expression<'_>,
    name: &[u8],
) -> Option<bool> {
    match unparenthesize(expr) {
        Expression::Access(Access::Property(p)) => {
            let ClassLikeMemberSelector::Identifier(id) = &p.property else {
                return Some(false);
            };
            Some(id.value == name && local_object(facts, view, p.object))
        }
        Expression::Access(Access::StaticProperty(p)) => {
            let Variable::Direct(v) = &p.property else {
                return Some(false);
            };
            let local = match p.class {
                Expression::Self_(_) | Expression::Static(_) => true,
                Expression::Identifier(id) => {
                    resolve_class_name(&view.path, id).eq_ignore_ascii_case(&view.name)
                }
                _ => false,
            };
            Some(local && &v.name[1..] == name)
        }
        _ => Some(false),
    }
}

/// Whether a property fetch's object is the class's own: `$this`, or a value of the class's type.
fn local_object(facts: &Facts, view: &ClassView<'_, '_>, object: &Expression<'_>) -> bool {
    let object = unparenthesize(object);
    if matches!(object, Expression::Variable(Variable::Direct(v)) if v.name == b"$this") {
        return true;
    }
    facts
        .native_type(node_span(object))
        .is_some_and(|t| match t.types.as_ref() {
            [TAtomic::Object(TObject::Named(n))] => {
                !n.is_this
                    && n.intersection_types.is_none()
                    && n.name.as_bytes().eq_ignore_ascii_case(view.name.as_bytes())
            }
            _ => false,
        })
}

/// Whether `expr` writes a dynamic property of the class (`$this->$name = ...`).
fn is_dynamic_local_fetch(facts: &Facts, view: &ClassView<'_, '_>, expr: &Expression<'_>) -> bool {
    match unparenthesize(expr) {
        Expression::Access(Access::Property(p)) => {
            !matches!(p.property, ClassLikeMemberSelector::Identifier(_))
                && local_object(facts, view, p.object)
        }
        Expression::Access(Access::StaticProperty(p)) => {
            !matches!(p.property, Variable::Direct(_))
                && matches!(p.class, Expression::Self_(_) | Expression::Static(_))
        }
        _ => false,
    }
}

/// `PropertyAssignMatcher::matchPropertyAssignExpr`: the value of a plain assignment to the property, or to an offset
/// of it.
pub(crate) fn match_property_assign<'a>(
    facts: &Facts,
    view: &ClassView<'_, '_>,
    a: &'a Assignment<'a>,
    name: &[u8],
) -> Option<(&'a Expression<'a>, bool)> {
    if !a.operator.is_assign() || is_by_ref(a) {
        return None;
    }
    if is_local_fetch(facts, view, a.lhs, name) == Some(true) {
        return Some((a.rhs, false));
    }
    let var = match unparenthesize(a.lhs) {
        Expression::ArrayAccess(x) => x.array,
        Expression::ArrayAppend(x) => x.array,
        _ => return None,
    };
    (is_local_fetch(facts, view, var, name) == Some(true)).then_some((a.rhs, true))
}

/// `$a = &$b`, which php-parser reads as an `AssignRef`.
pub(crate) fn is_by_ref(a: &Assignment<'_>) -> bool {
    matches!(a.rhs, Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Reference(_)))
}

/// Every node under the class's members, in source order (`SimpleCallableNodeTraverser` over `$classLike->stmts`).
pub(crate) fn class_nodes<'ast, 'arena>(view: &ClassView<'ast, 'arena>) -> Vec<Node<'ast, 'arena>> {
    let mut out = Vec::new();
    for member in view.members.iter() {
        let mut stack = vec![Node::ClassLikeMember(member)];
        while let Some(node) = stack.pop() {
            out.push(node);
            let mut children = Vec::new();
            node.visit_children(|c| children.push(c));
            stack.extend(children.into_iter().rev());
        }
    }
    out
}

/// What [`assigned_type`] found.
pub(crate) enum Assigned {
    /// `null`: no type (a dynamic property write, no assignment and no default, or a default the assignments do not
    /// allow).
    None,
    Type(Ty),
}

/// `AssignToPropertyTypeInferer::inferPropertyInClassLike`.
pub(crate) fn assigned_type(
    facts: &Facts,
    view: &ClassView<'_, '_>,
    prop: &Prop<'_, '_>,
    constructor_assigned: bool,
) -> Assigned {
    let name = prop.name();
    let nodes = class_nodes(view);
    // hasAssignDynamicPropertyValue
    if nodes.iter().any(|n| matches!(n, Node::Assignment(a) if a.operator.is_assign() && !is_by_ref(a) && is_dynamic_local_fetch(facts, view, a.lhs))) {
        return Assigned::None;
    }
    let mut types: Vec<Ty> = Vec::new();
    for node in &nodes {
        let Node::Assignment(a) = node else { continue };
        let Some((value, dim)) = match_property_assign(facts, view, a, name) else {
            continue;
        };
        if is_non_typed_from_param(facts, value) {
            types = vec![Ty::Mixed];
            break;
        }
        let t = assigned_native_ty(facts, view, value);
        types.push(if dim {
            Ty::Array(Box::new(Ty::Mixed), Box::new(t))
        } else {
            t
        });
    }
    let has_default = prop.default().is_some();
    // shouldAddNullType (NullTypeAssignDetector never asks for null)
    if !types.is_empty() && !constructor_assigned && !has_default {
        types.push(Ty::Null);
    }
    // resolveTypeWithVerifyDefaultValue
    if types.is_empty() {
        return match prop.default() {
            Some(default) => Assigned::Type(full_ty(facts, default)),
            None => Assigned::None,
        };
    }
    let inferred = mixed_passed_or_union(types, false);
    if let Some(default) = prop.default()
        && !is_null_literal(default)
        && is_disjoint(&inferred, &native_ty(facts, default))
    {
        return Assigned::None;
    }
    Assigned::Type(inferred)
}

/// `$value` is the literal `null`.
pub(crate) fn is_null_literal(expr: &Expression<'_>) -> bool {
    matches!(unparenthesize(expr), Expression::Literal(Literal::Null(_)))
}

/// `ExprAnalyzer::isNonTypedFromParam`: a variable whose native type PHPStan does not know, or knows less precisely
/// than its documented type.
fn is_non_typed_from_param(facts: &Facts, expr: &Expression<'_>) -> bool {
    let Expression::Variable(v) = unparenthesize(expr) else {
        return false;
    };
    if !matches!(v, Variable::Direct(_)) {
        return true;
    }
    let span = node_span(unparenthesize(expr));
    let Some(native) = facts.native_type(span) else {
        return true;
    };
    if native.types.is_empty() || native.is_mixed() {
        return true;
    }
    let full = facts.full_type(span);
    // a template `of object` is a `TemplateObjectWithoutClassType`, an `ObjectWithoutClassType` too
    let object_without_class = |a: &TAtomic| match a {
        TAtomic::Object(TObject::Any) => true,
        TAtomic::GenericParameter(p) => {
            p.intersection_types.is_none()
                && matches!(p.constraint.types.as_ref(), [TAtomic::Object(TObject::Any)])
        }
        _ => false,
    };
    if matches!(native.types.as_ref(), [TAtomic::Object(TObject::Any)])
        && full.is_some_and(|f| !matches!(f.types.as_ref(), [a] if object_without_class(a)))
    {
        return true;
    }
    let native_ty = ty(native);
    if native_ty.is_union() && full.is_some_and(|f| ty(f).hash() != native_ty.hash()) {
        return true;
    }
    false
}

/// `NodeTypeResolver::getNativeType` of a value the class assigns: [`native_ty`], but a direct `new class` that names
/// classes or interfaces is [`Ty::AnonymousWithParents`].
pub(crate) fn assigned_native_ty(
    facts: &Facts,
    view: &ClassView<'_, '_>,
    expr: &Expression<'_>,
) -> Ty {
    if let Expression::AnonymousClass(class) = unparenthesize(expr) {
        let parents = super::add_return_type_from_new::anonymous_parents(&view.path, class);
        if !parents.is_empty() {
            return Ty::AnonymousWithParents(parents);
        }
    }
    parse_url_component(facts, expr)
        .or_else(|| method_exists_narrowed(facts, view, expr))
        .unwrap_or_else(|| native_ty(facts, expr))
}

/// PHPStan's `MethodExistsTypeSpecifyingExtension` for a variable `method_exists()` narrowed, which Mago types as a
/// bare `has-method<...>` whatever it was before: the declared type intersected with `class-string|(object&hasMethod)`.
/// For a parameter of the enclosing method that its body never assigns, no type gives `class-string|object` and
/// `object` gives `object`; for anything else Pun does not know the type before, and it is `mixed`.
fn method_exists_narrowed(
    facts: &Facts,
    view: &ClassView<'_, '_>,
    expr: &Expression<'_>,
) -> Option<Ty> {
    let expr = unparenthesize(expr);
    let native = facts.native_type(node_span(expr))?;
    if !matches!(
        native.types.as_ref(),
        [TAtomic::Object(TObject::HasMethod(_))]
    ) {
        return None;
    }
    let Expression::Variable(Variable::Direct(var)) = expr else {
        return Some(Ty::Mixed);
    };
    let at = node_span(expr);
    let method = view.members.iter().find_map(|m| match m {
        ClassLikeMember::Method(m)
            if node_span(m).start <= at.start && at.end <= node_span(m).end =>
        {
            Some(m)
        }
        _ => None,
    });
    let Some(param) = method.and_then(|m| {
        m.parameter_list
            .parameters
            .iter()
            .find(|p| p.variable.name == var.name)
    }) else {
        return Some(Ty::Mixed);
    };
    let assigned = |method: &Method<'_>| {
        let mut found = false;
        let mut stack = vec![Node::Method(method)];
        while let Some(node) = stack.pop() {
            if let Node::Assignment(a) = node
                && matches!(unparenthesize(a.lhs), Expression::Variable(Variable::Direct(v)) if v.name == var.name)
            {
                found = true;
            }
            node.visit_children(|c| stack.push(c));
        }
        found
    };
    if method.is_some_and(assigned) {
        return Some(Ty::Mixed);
    }
    Some(match &param.hint {
        None => Ty::Union(vec![Ty::ObjectAny, Ty::ClassString]),
        Some(Hint::Object(_)) => Ty::ObjectAny,
        Some(_) => Ty::Mixed,
    })
}

/// PHPStan's `ParseUrlFunctionDynamicReturnTypeExtension` for `parse_url($url, $component)` with a constant
/// component and a URL that is no constant string, whose members Mago orders otherwise: `string|false|null` for a
/// part, `int<0, 65535>|false|null` for the port, `false` for any other component.
fn parse_url_component(facts: &Facts, expr: &Expression<'_>) -> Option<Ty> {
    let Expression::Call(Call::Function(call)) = unparenthesize(expr) else {
        return None;
    };
    let Expression::Identifier(name) = call.function else {
        return None;
    };
    if !name
        .value()
        .strip_prefix(b"\\")
        .unwrap_or(name.value())
        .eq_ignore_ascii_case(b"parse_url")
    {
        return None;
    }
    let args: Vec<&Expression<'_>> = (call.argument_list.arguments.iter())
        .map(|a| match a {
            Argument::Positional(p) if p.ellipsis.is_none() => Some(p.value),
            _ => None,
        })
        .collect::<Option<_>>()?;
    let [url, component] = args[..] else {
        return None;
    };
    let url = facts.native_type(node_span(unparenthesize(url)))?;
    if url.types.iter().any(|a| matches!(a, TAtomic::Scalar(TScalar::String(s)) if s.get_known_literal_value().is_some())) {
        return None;
    }
    let component = facts.native_type(node_span(unparenthesize(component)))?;
    let [TAtomic::Scalar(TScalar::Integer(TInteger::Literal(component)))] =
        component.types.as_ref()
    else {
        return None;
    };
    let part = match component {
        // PHP_URL_SCHEME, PHP_URL_HOST, PHP_URL_USER, PHP_URL_PASS, PHP_URL_PATH, PHP_URL_QUERY, PHP_URL_FRAGMENT
        0 | 1 | 3..=7 => Ty::Str(Scalar::General),
        // PHP_URL_PORT
        2 => Ty::Int(Scalar::Refined),
        -1 => return None,
        _ => return Some(Ty::Bool(Some(false))),
    };
    Some(Ty::Union(vec![part, Ty::Bool(Some(false)), Ty::Null]))
}

/// `getNativeType` of an expression; `mixed` where the analyser did not reach it.
pub(crate) fn native_ty(facts: &Facts, expr: &Expression<'_>) -> Ty {
    facts
        .native_type(node_span(unparenthesize(expr)))
        .filter(|t| !t.types.is_empty())
        .map_or(Ty::Mixed, ty)
}

/// `getType` of an expression; `mixed` where the analyser did not reach it.
pub(crate) fn full_ty(facts: &Facts, expr: &Expression<'_>) -> Ty {
    let span = node_span(unparenthesize(expr));
    facts
        .full_type(span)
        .or_else(|| facts.native_type(span))
        .filter(|t| !t.types.is_empty())
        .map_or(Ty::Mixed, ty)
}

/// `$inferred->isSuperTypeOf($default)->no()`: no value of `default` is one of `inferred`.
fn is_disjoint(inferred: &Ty, default: &Ty) -> bool {
    if matches!(inferred, Ty::Mixed) || matches!(default, Ty::Mixed) {
        return false;
    }
    !default
        .members()
        .iter()
        .any(|d| inferred.members().iter().any(|i| overlaps(i, d)))
}

/// Whether some value is of both types. `is_disjoint` has ruled out `mixed`, and the inferred members have already
/// had their scalar constants generalized by `mixed_passed_or_union(types, false)`.
fn overlaps(a: &Ty, b: &Ty) -> bool {
    match (a, b) {
        (Ty::Null, Ty::Null) => true,
        (Ty::Bool(x), Ty::Bool(y)) => x.is_none() || y.is_none() || x == y,
        (Ty::Int(_), Ty::Int(_)) => true,
        (Ty::Float(_), Ty::Float(_)) => true,
        (Ty::Str(_) | Ty::ClassString, Ty::Str(_) | Ty::ClassString) => true,
        (
            Ty::Array(..) | Ty::ConstArray(_) | Ty::Iterable,
            Ty::Array(..) | Ty::ConstArray(_) | Ty::Iterable,
        ) => true,
        (Ty::Callable, _) | (_, Ty::Callable) => true,
        (
            Ty::Object(_)
            | Ty::Static(_)
            | Ty::Parent(_)
            | Ty::ObjectAny
            | Ty::AnonymousWithParents(_)
            | Ty::Closure
            | Ty::Intersection(_)
            | Ty::Iterable,
            Ty::Object(_)
            | Ty::Static(_)
            | Ty::Parent(_)
            | Ty::ObjectAny
            | Ty::AnonymousWithParents(_)
            | Ty::Closure
            | Ty::Intersection(_)
            | Ty::Iterable,
        ) => true,
        (Ty::Other(_), _) | (_, Ty::Other(_)) => true,
        _ => false,
    }
}

/// `$default->isSuperTypeOf($type)->yes()`: every value of `t` is one of `default`.
pub(crate) fn contains_all(default: &Ty, t: &Ty) -> bool {
    t.members()
        .iter()
        .all(|m| default.members().iter().any(|d| contains(d, m)))
}

fn contains(outer: &Ty, inner: &Ty) -> bool {
    match (outer, inner) {
        (Ty::Mixed, _) => true,
        (Ty::Null, Ty::Null) => true,
        (Ty::Bool(None), Ty::Bool(_)) => true,
        (Ty::Bool(Some(x)), Ty::Bool(Some(y))) => x == y,
        (Ty::Int(Scalar::General), Ty::Int(_)) => true,
        (Ty::Int(Scalar::Literal(x)), Ty::Int(Scalar::Literal(y))) => x == y,
        (Ty::Float(Scalar::General), Ty::Float(_)) => true,
        (Ty::Float(Scalar::Literal(x)), Ty::Float(Scalar::Literal(y))) => x == y,
        (Ty::Str(Scalar::General), Ty::Str(_) | Ty::ClassString) => true,
        (Ty::Str(Scalar::Literal(x)), Ty::Str(Scalar::Literal(y))) => x == y,
        (Ty::Array(..), Ty::Array(..) | Ty::ConstArray(_)) => true,
        (Ty::Iterable, Ty::Array(..) | Ty::ConstArray(_) | Ty::Iterable) => true,
        (
            Ty::ObjectAny,
            Ty::Object(_)
            | Ty::Static(_)
            | Ty::Parent(_)
            | Ty::ObjectAny
            | Ty::AnonymousWithParents(_)
            | Ty::Closure
            | Ty::Intersection(_),
        ) => true,
        (Ty::Object(o), Ty::Object(i) | Ty::Static(i) | Ty::Parent(i)) => o.eq_ignore_ascii_case(i),
        (Ty::Closure, Ty::Closure) => true,
        (a, b) => a == b,
    }
}

/// The type of a property's default value as `PropertyTypeDefaultValueAnalyzer` reads it
/// (`mapPhpParserNodePHPStanType`): a plain string is `string`, any other value its type.
pub(crate) fn default_ty(facts: &Facts, expr: &Expression<'_>) -> Ty {
    match unparenthesize(expr) {
        Expression::Literal(Literal::String(_)) => Ty::Str(Scalar::General),
        other => full_ty(facts, other),
    }
}

/// `PropertyTypeDefaultValueAnalyzer::doesConflictWithDefaultValue`.
pub(crate) fn conflicts_with_default(default: &Ty, t: &Ty) -> bool {
    if default.is_array() && t.is_array() {
        return false;
    }
    !contains_all(default, t)
}

/// The class `Doctrine\Common\Collections\Collection` or one that implements it (`isInstanceOfCollectionType`).
pub(crate) fn is_doctrine_collection(facts: &Facts, t: &Ty) -> bool {
    const COLLECTION: &str = "Doctrine\\Common\\Collections\\Collection";
    let Ty::Object(name) = t else { return false };
    if name.eq_ignore_ascii_case(COLLECTION) {
        return true;
    }
    facts
        .codebase
        .get_class_like(name.as_bytes())
        .is_some_and(|c| {
            c.all_parent_interfaces
                .iter()
                .chain(c.all_parent_classes.iter())
                .any(|p| p.as_bytes().eq_ignore_ascii_case(COLLECTION.as_bytes()))
        })
}

/// The `@var` of a property and what Rector reads of it.
pub(crate) struct VarTag<'s> {
    pub annotated: Annotated<'s>,
    /// The `@var` tag; `Err` for one Pun does not read (a generic or shape with a space or `,` inside, a literal),
    /// which says more than a native type and is kept.
    pub tag: Result<Option<TypedTag<'s>>, ()>,
}

impl<'s> VarTag<'s> {
    /// The property's annotation context, even without a doc comment. An unreadable `@var` stays `Err` in `tag`.
    pub(crate) fn of(source: &'s str, property: &Property<'_>, ancestors: &[Node<'_, '_>]) -> Self {
        let annotated = Annotated::property(source, property, ancestors);
        let tag = match &annotated.doc {
            Some(doc) => typed_tag(doc, "@var", None),
            None => Ok(None),
        };
        VarTag { annotated, tag }
    }

    fn doc(&self) -> Option<&Doc<'s>> {
        self.annotated.doc.as_ref()
    }

    /// The edits that remove the `@var` tag, as `PhpDocInfo::removeByType()` and the printer leave the doc.
    pub(crate) fn remove(&self, source: &str) -> Option<Vec<Edit>> {
        let doc = self.doc()?;
        let tag = self.tag.as_ref().ok()?.as_ref()?;
        let removed: Vec<bool> = (0..doc.len()).map(|i| i == tag.index).collect();
        let printed = doc.print(&removed);
        self.edits(source, &printed)
    }

    /// The edits that give the property the doc `new_doc` (`""` removes it).
    fn edits(&self, source: &str, new_doc: &str) -> Option<Vec<Edit>> {
        (self.annotated.edits(source, Some(new_doc), &[])).or_else(|| {
            self.annotated
                .edit_with_comments(source, new_doc)
                .map(|e| vec![e])
        })
    }

    /// The edits that set the `@var` type to `ty` (`PhpDocTypeChanger::changeVarType`).
    pub(crate) fn set(&self, source: &str, ty: &str) -> Option<Vec<Edit>> {
        let new_doc = with_tag_type(self.doc(), "@var", ty)?;
        self.edits(source, &new_doc)
    }
}

/// `DeadVarTagValueNodeAnalyzer::isDead` and `PhpDocTypeChanger::isAllowed` as `VarTagRemover::removeVarTagIfUseless`
/// asks them: whether the `@var` says no more than the native type `native`.
pub(crate) fn is_var_tag_useless(tag: &TypedTag<'_>, doc: Option<&TUnion>, native: &Ty) -> bool {
    if !tag.description.is_empty() {
        return false;
    }
    let members: Vec<&str> = match &tag.ty {
        TagType::Name(n) | TagType::Other(n) => vec![n],
        TagType::Union(m) | TagType::Intersection(m) => m.clone(),
    };
    // `GenericTypeNode`, `T[]`, callable signatures, shapes and `class-string` are kept (`isAllowed`), and a whole
    // generic type is never dead
    if members.iter().any(|m| {
        m.contains(['<', '[', '{', '('])
            || m.eq_ignore_ascii_case("class-string")
            || m.contains("::")
    }) {
        return false;
    }
    let Some(doc) = doc else {
        return false;
    };
    // TemplateTypeRemovalGuard
    if doc
        .types
        .iter()
        .any(|a| matches!(a, TAtomic::GenericParameter(_)))
    {
        return false;
    }
    is_var_tag_useless_as(tag, &ty(doc), native)
}

/// [`is_var_tag_useless`] for a tag whose doc type is `doc_ty`, once the description, the kept forms and template
/// types have been ruled out.
pub(crate) fn is_var_tag_useless_as(tag: &TypedTag<'_>, doc_ty: &Ty, native: &Ty) -> bool {
    let doc_ty = doc_ty.clone();
    if matches!(tag.ty, TagType::Intersection(_)) {
        return false;
    }
    if native.is_union() && !doc_ty.is_union() {
        return true;
    }
    if let (Ty::Object(n), Ty::Object(d)) = (native, &doc_ty) {
        if n.eq_ignore_ascii_case(d) {
            return true;
        }
    }
    if php_parser_and_doc_types_equal(native, &doc_ty) {
        return true;
    }
    doc_ty.is_union() && {
        let without_null = union(
            doc_ty
                .members()
                .into_iter()
                .filter(|m| **m != Ty::Null)
                .cloned()
                .collect(),
        );
        are_equal(&without_null, native)
    }
}

/// `TypeComparator::arePhpParserAndPhpStanPhpDocTypesEqual`.
fn php_parser_and_doc_types_equal(native: &Ty, doc: &Ty) -> bool {
    let equal = are_equal(native, doc);
    if !equal && contains_all(native, doc) && !matches!(native, Ty::Mixed) {
        return false;
    }
    let normalize = |t: &Ty| match t {
        Ty::Bool(Some(_)) => Ty::Bool(None),
        other => other.clone(),
    };
    let (native, doc) = (normalize(native), normalize(doc));
    let different_scalars = are_different_scalars(&native, &doc);
    if !different_scalars && !are_equal(&native, &doc) {
        return false;
    }
    !matches!(doc, Ty::Static(_) | Ty::Parent(_))
}

/// `ScalarTypeComparator::areDifferentScalarTypes`.
fn are_different_scalars(a: &Ty, b: &Ty) -> bool {
    let scalar = |t: &Ty| {
        t.members().iter().all(|m| {
            matches!(
                m,
                Ty::Int(_) | Ty::Float(_) | Ty::Str(_) | Ty::Bool(_) | Ty::ClassString
            )
        })
    };
    let kind = |t: &Ty| match t {
        Ty::Int(_) => 0,
        Ty::Float(_) => 1,
        Ty::Str(_) | Ty::ClassString => 2,
        Ty::Union(_) => 4,
        _ => 3,
    };
    let all = |t: &Ty, f: fn(&Ty) -> bool| t.members().iter().all(|m| f(m));
    let string = |t: &Ty| matches!(t, Ty::Str(_) | Ty::ClassString);
    let integer = |t: &Ty| matches!(t, Ty::Int(_));
    scalar(a)
        && scalar(b)
        && !(all(a, string) && all(b, string))
        && !(all(a, integer) && all(b, integer))
        && kind(a) != kind(b)
}

/// `StaticTypeMapper::mapPHPStanTypeToPHPStanPhpDocTypeNode` for the types Pun prints: scalars, `array`, classes,
/// `object`, `iterable`, and unions of them. `None` for any other.
pub(crate) fn doc_text(t: &Ty, codebase: &CodebaseMetadata) -> Option<String> {
    Some(match t {
        Ty::Union(types) => {
            let mut parts: Vec<String> = Vec::new();
            for m in types {
                let p = doc_text(m, codebase)?;
                if !parts.contains(&p) {
                    parts.push(p);
                }
            }
            parts.join("|")
        }
        Ty::Null => "null".into(),
        Ty::Bool(None) => "bool".into(),
        Ty::Bool(Some(b)) => b.to_string(),
        // IntegerTypeMapper prints `int` even for a constant
        Ty::Int(_) => "int".into(),
        // ConstantFloatType::toPhpDocNode, `castFloatToString` (precision -1, `.0` added); PHP spells values outside
        // this range with an exponent, which Pun does not print
        Ty::Float(Scalar::Literal(v)) => {
            let f: f64 = v.parse().ok()?;
            if f != 0.0 && !(1e-4..1e15).contains(&f.abs()) {
                return None;
            }
            let s = f.to_string();
            if s.contains('.') { s } else { format!("{s}.0") }
        }
        Ty::Float(_) => "float".into(),
        // ConstantStringType::toPhpDocNode: single-quoted, `'` and `\` escaped; a multi-line value prints as its
        // generalized accessory types, which Pun does not print
        Ty::Str(Scalar::Literal(v)) => {
            let s = std::str::from_utf8(v).ok().filter(|s| !s.contains('\n'))?;
            format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
        }
        Ty::Str(_) => "string".into(),
        Ty::Object(name) => class_text(codebase, name),
        Ty::Parent(_) => "static".into(),
        Ty::Closure => "\\Closure".into(),
        Ty::ObjectAny => "object".into(),
        Ty::Iterable => "iterable".into(),
        _ => return None,
    })
}

/// The span of a property's variable, where its type goes (`$name`).
pub(crate) fn variable_span(prop: &Prop<'_, '_>) -> Span {
    node_span(prop.item.expect("a single item").variable())
}

/// The span from the end of a property's variable to the end of its default value (` = value`).
pub(crate) fn default_span(prop: &Prop<'_, '_>) -> Option<Span> {
    let default = prop.default()?;
    Some(Span::new(variable_span(prop).end, node_span(default).end))
}
