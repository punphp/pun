//! What the rules that add a parameter type share: where php-parser's printer writes a new `Param::$type`, Rector's
//! `StaticTypeMapper::mapPHPStanTypeToPhpParserNode` for a parameter (`TypeKind::PARAM`) on Mago's types, and the
//! array types Rector's `NodeTypeResolver::getType` gives after its `NodeTypeCorrector`.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Expression, FunctionLikeParameter, Node};
use pun_problem::{Edit, Span};

use super::declare::{atomic_declaration, union_order};
use super::facts::Facts;
use crate::rules::class_method::type_of;

/// `PhpVersionFeature::NULL_FALSE_TRUE_STANDALONE_TYPE`.
const NULL_FALSE_TRUE_STANDALONE_TYPE: u32 = 80200;
/// `PhpVersionFeature::UNION_TYPES`.
const UNION_TYPES: u32 = 80000;
/// `PhpVersionFeature::NULLABLE_TYPE`.
const NULLABLE_TYPE: u32 = 70100;
/// `PhpVersionFeature::DNF_TYPES`.
const DNF_TYPES: u32 = 80200;

/// Where a new type goes: before the `&`, the `...` or the variable (after attributes and modifiers).
pub(crate) fn hint_at(param: &FunctionLikeParameter<'_>) -> usize {
    param
        .ampersand
        .or(param.ellipsis)
        .map_or(param.variable.span.start.offset, |s| s.start.offset) as usize
}

/// The edit that gives `param` the type `declaration`.
pub(crate) fn add_hint(param: &FunctionLikeParameter<'_>, declaration: &str) -> Edit {
    let at = hint_at(param);
    Edit::new(Span::new(at, at), format!("{declaration} "))
}


/// Whether PHPStan's type is a `MixedType`: Mago's union holds `mixed` (a union with `mixed` is `mixed` in PHPStan).
pub(crate) fn is_mixed(t: &TUnion) -> bool {
    t.types.is_empty() || t.types.iter().any(|a| matches!(a, TAtomic::Mixed(_)))
}

/// `mapPHPStanTypeToPhpParserNode($type, TypeKind::PARAM)` for the union `t`: one type maps alone, more as a union
/// (`TypeKind::UNION` for each member, repeats dropped, `null` with one other written `?T`).
pub(crate) fn param_declaration(
    t: &TUnion,
    codebase: &CodebaseMetadata,
    php_version: u32,
) -> Option<String> {
    param_declaration_in(t, codebase, php_version, &[])
}

/// [`param_declaration`] with the classes of a union in the order `order` names them (lowercase short names, see
/// [`doc_order`]): PHPStan keeps a docblock union as written, where Mago sorts it.
pub(crate) fn param_declaration_in(
    t: &TUnion,
    codebase: &CodebaseMetadata,
    php_version: u32,
    order: &[String],
) -> Option<String> {
    if has_array_intersection(t) {
        return None;
    }
    let expanded = expand(&t.types)?;
    declaration_of(normalize_bool(union_order(&expanded)), codebase, php_version, order)
}

/// [`param_declaration`] of the members of a union in the order given (PHPStan's `UnionType` as Rector builds it,
/// not sorted).
pub(crate) fn param_declaration_unsorted(
    types: &[TAtomic],
    codebase: &CodebaseMetadata,
    php_version: u32,
) -> Option<String> {
    let expanded = expand(types)?;
    declaration_of(normalize_bool(expanded.iter().collect()), codebase, php_version, &[])
}

fn declaration_of(
    types: Vec<&TAtomic>,
    codebase: &CodebaseMetadata,
    php_version: u32,
    order: &[String],
) -> Option<String> {
    if let [one] = types.as_slice() {
        return single(one, codebase, php_version);
    }
    let mut names: Vec<String> = Vec::new();
    for t in &types {
        let name = match t {
            TAtomic::Null => "null".to_string(),
            TAtomic::Scalar(TScalar::Bool(b)) if b.is_false() => "false".to_string(),
            TAtomic::Scalar(TScalar::Bool(b)) if b.is_true() => {
                if php_version < NULL_FALSE_TRUE_STANDALONE_TYPE {
                    "bool".to_string()
                } else {
                    "true".to_string()
                }
            }
            TAtomic::Void | TAtomic::Never | TAtomic::Mixed(_) => return None,
            _ => member(t, codebase, php_version)?,
        };
        if name.contains('&') && php_version < DNF_TYPES {
            return None;
        }
        if !names.contains(&name) {
            names.push(name);
        }
    }
    order_classes(&mut names, order);
    match names.as_slice() {
        [] => None,
        [one] => Some(one.clone()),
        [a, b] if !a.contains('&') && !b.contains('&') && (a == "null" || b == "null") => {
            let other = if a == "null" { b } else { a };
            if php_version < NULLABLE_TYPE {
                return None;
            }
            if other == "false" && php_version < NULL_FALSE_TRUE_STANDALONE_TYPE {
                return Some("?bool".to_string());
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

/// Sort the class names of `names` among their own slots by where `order` names them; the rest stay put.
fn order_classes(names: &mut [String], order: &[String]) {
    let slots: Vec<usize> = (0..names.len()).filter(|&i| names[i].starts_with('\\')).collect();
    let mut classes: Vec<String> = slots.iter().map(|&i| names[i].clone()).collect();
    classes.sort_by_key(|n| {
        let short = n.rsplit('\\').next().unwrap_or(n).to_ascii_lowercase();
        order.iter().position(|o| *o == short).unwrap_or(order.len())
    });
    for (slot, class) in slots.into_iter().zip(classes) {
        names[slot] = class;
    }
}

/// The words of the docblock of the innermost function, method or closure of `ancestors`, lowercased, in the order
/// written: the order of the classes of a union PHPStan read from it.
pub(crate) fn doc_order(ctx: &crate::Context<'_>, ancestors: &[Node<'_, '_>]) -> Vec<String> {
    let Some(at) = ancestors
        .iter()
        .rposition(|a| matches!(a, Node::Method(_) | Node::Function(_)))
    else {
        return Vec::new();
    };
    let start = crate::syntax::node_span(&ancestors[at]).start;
    let Some(doc) = crate::rules::lint_b_doc::Doc::of(ctx, &ancestors[..at], start) else {
        return Vec::new();
    };
    doc.text_all()
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '\\'))
        .filter(|w| !w.is_empty())
        .map(|w| w.rsplit('\\').next().unwrap_or(w).to_ascii_lowercase())
        .collect()
}

/// The values' types of a constant array in the order PHPStan's `ConstantArrayType` keeps them, the order its keys
/// were set: the literal's own where `expr` is one; Mago's where its keys are integers. `None` for a constant array
/// with string keys that is not a literal, whose order Mago does not keep (it sorts the keys).
pub(crate) fn constant_values<'ast, 'arena>(
    facts: &Facts,
    ancestors: &[Node<'ast, 'arena>],
    expr: &'ast Expression<'arena>,
    values: &[TUnion],
) -> Option<Vec<TAtomic>> {
    use mago_syntax::cst::ArrayElement;
    let flat: Vec<TAtomic> = values.iter().flat_map(|v| v.types.iter().cloned()).collect();
    // a variable last assigned a literal with the same values
    let source = assigned(ancestors, expr);
    let expr = if std::ptr::eq(source, expr) {
        expr
    } else {
        match constant_values(facts, ancestors, source, values) {
            Some(types) if types.len() == flat.len()
                && types.iter().all(|t| flat.contains(t)) =>
            {
                return Some(types);
            }
            _ => expr,
        }
    };
    let elements: Option<Vec<&ArrayElement<'_>>> = match crate::syntax::unparenthesize(expr) {
        Expression::Array(a) => Some(a.elements.iter().collect()),
        Expression::LegacyArray(a) => Some(a.elements.iter().collect()),
        _ => None,
    };
    // the values of a literal in the order written; a spread (`...$a`) takes Mago's order
    let written: Option<Vec<&Expression<'_>>> = elements.and_then(|elements| {
        elements.iter().map(|e| match e { ArrayElement::KeyValue(e) => Some(e.value), ArrayElement::Value(e) => Some(e.value), _ => None }).collect()
    });
    if let Some(written) = written {
        let mut out = Vec::new();
        for value in written {
            out.extend(type_of(facts, value)?.types.iter().cloned());
        }
        return Some(out);
    }
    let string_keyed = type_of(facts, expr).is_some_and(|t| {
        t.types.iter().any(|a| {
            matches!(a, TAtomic::Array(TArray::Keyed(k))
                if k.known_items.iter().flat_map(|i| i.keys()).any(|key| !key.is_integer()))
        })
    });
    (!string_keyed || flat.iter().all(|t| *t == flat[0])).then_some(flat)
}

/// Whether an array's item type has an array that is not a constant one but a list or non-empty: PHPStan has
/// `ArrayType&AccessoryArrayListType` (or `&NonEmptyArrayType`) there, which Rector's `NodeTypeCorrector` removes from
/// the type of an expression but not from its items, and `IntersectionTypeMapper` maps an intersection that is not of
/// objects to nothing (checked with Rector 2.6.1).
fn has_array_intersection(item: &TUnion) -> bool {
    item.types
        .iter()
        .any(|t| matches!(t, TAtomic::Array(a) if !a.is_sealed() && (matches!(a, TArray::List(_)) || a.is_non_empty())))
}

/// The members of a union as PHPStan has them: a template type is its bound, `array-key` is `int|string` and
/// `scalar` is `int|float|string|bool`. `None` for a template bound that is `mixed`.
fn expand(types: &[TAtomic]) -> Option<Vec<TAtomic>> {
    let mut out = Vec::new();
    for t in types {
        match t {
            TAtomic::GenericParameter(p) => {
                if is_mixed(&p.constraint) {
                    return None;
                }
                out.extend(expand(&p.constraint.types)?);
            }
            TAtomic::Scalar(TScalar::ArrayKey) => {
                out.push(TAtomic::Scalar(TScalar::int()));
                out.push(TAtomic::Scalar(TScalar::string()));
            }
            TAtomic::Scalar(TScalar::Generic) => {
                out.push(TAtomic::Scalar(TScalar::int()));
                out.push(TAtomic::Scalar(TScalar::float()));
                out.push(TAtomic::Scalar(TScalar::string()));
                out.push(TAtomic::Scalar(TScalar::bool()));
            }
            _ => out.push(t.clone()),
        }
    }
    Some(out)
}

/// PHPStan's `TypeCombinator`: `true` with `false` is `bool`, and `bool` takes in either.
fn normalize_bool(types: Vec<&TAtomic>) -> Vec<&TAtomic> {
    let is_true = |t: &TAtomic| matches!(t, TAtomic::Scalar(TScalar::Bool(b)) if b.is_true());
    let is_false = |t: &TAtomic| matches!(t, TAtomic::Scalar(TScalar::Bool(b)) if b.is_false());
    let is_bool = |t: &TAtomic| matches!(t, TAtomic::Scalar(TScalar::Bool(b)) if b.is_general());
    let has_bool = types.iter().any(|t| is_bool(t))
        || (types.iter().any(|t| is_true(t)) && types.iter().any(|t| is_false(t)));
    if !has_bool {
        return types;
    }
    let mut out: Vec<&TAtomic> = Vec::new();
    let mut placed = false;
    for t in types {
        if is_true(t) || is_false(t) || is_bool(t) {
            if !placed {
                out.push(&BOOL);
                placed = true;
            }
            continue;
        }
        out.push(t);
    }
    out
}

static BOOL: TAtomic = TAtomic::Scalar(TScalar::bool());

/// One type that is not a union, as a parameter type.
fn single(t: &TAtomic, codebase: &CodebaseMetadata, php_version: u32) -> Option<String> {
    match t {
        TAtomic::Null | TAtomic::Void | TAtomic::Never | TAtomic::Mixed(_) => None,
        TAtomic::Scalar(TScalar::Bool(b)) if !b.is_general() => {
            Some(if php_version < NULL_FALSE_TRUE_STANDALONE_TYPE {
                "bool"
            } else if b.is_true() {
                "true"
            } else {
                "false"
            })
            .map(str::to_string)
        }
        _ => member(t, codebase, php_version),
    }
}

/// A type that is not `null` or a constant `bool`, as a declaration: `$this` is `self` outside a return type; a
/// `static` that Mago keeps from a static call is the class it names (PHPStan resolves it there).
fn member(t: &TAtomic, codebase: &CodebaseMetadata, php_version: u32) -> Option<String> {
    match t {
        TAtomic::Object(TObject::Named(named))
            if named.is_this && named.intersection_types.is_none() =>
        {
            Some("self".to_string())
        }
        // `TypeCombinator::intersect()` keeps of `A&B` the one that is a subtype of the other
        TAtomic::Object(TObject::Named(named)) if !named.is_this && !named.is_static && named.intersection_types.is_some() => {
            let plain = |n: &mago_codex::ttype::atomic::object::named::TNamedObject| {
                let mut n = n.clone();
                n.intersection_types = None;
                n
            };
            let mut all = vec![plain(named)];
            for other in named.intersection_types.iter().flat_map(|o| o.iter()) {
                let TAtomic::Object(TObject::Named(other)) = other else {
                    return atomic_declaration(t, codebase, php_version);
                };
                all.push(plain(other));
            }
            let narrower = |n: &&mago_codex::ttype::atomic::object::named::TNamedObject| {
                !all.iter().any(|m| {
                    !m.name.as_bytes().eq_ignore_ascii_case(n.name.as_bytes())
                        && codebase.is_instance_of(m.name.as_bytes(), n.name.as_bytes())
                })
            };
            let kept: Vec<_> = all.iter().filter(narrower).cloned().collect();
            let (first, rest) = kept.split_first()?;
            let mut first = first.clone();
            first.intersection_types = (!rest.is_empty())
                .then(|| Box::new(rest.iter().map(|r| TAtomic::Object(TObject::Named(r.clone()))).collect()));
            atomic_declaration(&TAtomic::Object(TObject::Named(first)), codebase, php_version)
        }
        TAtomic::Object(TObject::Named(named)) if named.is_static => {
            let mut plain = named.clone();
            plain.is_static = false;
            atomic_declaration(&TAtomic::Object(TObject::Named(plain)), codebase, php_version)
        }
        _ => atomic_declaration(t, codebase, php_version),
    }
}

/// A `static` or `$this` of the class the code is in as `$this` (Rector writes `self` for it); one of another class
/// (`Carbon::now()`, `$date->addMinutes(5)`, `$class::cases()`) is the class it names.
pub(crate) fn own_static(t: &TAtomic, class: Option<&str>) -> TAtomic {
    match t {
        TAtomic::Object(TObject::Named(named)) if named.is_static || named.is_this => {
            let mut this = named.clone();
            this.is_this = class.is_some_and(|c| c.as_bytes().eq_ignore_ascii_case(named.name.as_bytes()));
            this.is_static = false;
            TAtomic::Object(TObject::Named(this))
        }
        other => other.clone(),
    }
}

/// The array types Rector's `getType()` tells apart, after `NodeTypeCorrector::correctType` (a list is a plain
/// `ArrayType`, a non-empty array is `array<mixed, mixed>`).
pub(crate) enum ArrayShape {
    /// `ConstantArrayType`, with its values.
    Constant(Vec<TUnion>),
    /// `ArrayType`, with its item type; `None` for `mixed`.
    General(Option<TUnion>),
}

/// The array shape of PHPStan's `getType()` of `expr`. A non-empty array counts only where a docblock makes it one:
/// Mago also knows it of built-in calls (`explode()`, `Enum::cases()`) and literals, where PHPStan does not.
pub(crate) fn array_shape_of(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
) -> Option<ArrayShape> {
    // `array_slice($array, 0)` with a positive length or none keeps a non-empty array non-empty
    if let Some(sliced) = head_slice(expr)
        && matches!(array_shape_of(facts, ancestors, sliced), Some(ArrayShape::General(None)))
    {
        return Some(ArrayShape::General(None));
    }
    let span = crate::syntax::node_span(crate::syntax::unparenthesize(expr));
    // a value PHPStan does not type, or a literal array of one, is (an array of) `mixed`
    let literal = match crate::syntax::unparenthesize(expr) {
        Expression::Array(a) => a.elements.iter().collect(),
        Expression::LegacyArray(a) => a.elements.iter().collect(),
        _ => Vec::new(),
    };
    if facts.is_untyped(span)
        || literal.iter().filter_map(|e| e.get_value()).any(|v| facts.is_untyped(crate::syntax::node_span(v)))
    {
        return None;
    }
    let t = type_of(facts, expr)?;
    let native_non_empty = facts.native_type(span).is_some_and(|n| {
        n.types
            .iter()
            .any(|a| matches!(a, TAtomic::Array(a) if a.is_non_empty()))
    });
    if cased_explode(ancestors, expr) {
        return Some(ArrayShape::General(Some(TUnion::from_vec(vec![TAtomic::Scalar(TScalar::string())]))));
    }
    if non_empty_call(expr) {
        let item = if let Some(ArrayShape::General(Some(item))) = array_shape(t, false) { accessory_string(&item) } else { None };
        return Some(ArrayShape::General(item));
    }
    array_shape(t, !native_non_empty || emptiness_checked(ancestors, expr))
}

/// Whether `expr` is a variable that an `empty()` or `!` before it in its function tests, which narrows it in PHPStan
/// as in Mago (an early return on it leaves it non-empty).
fn emptiness_checked(ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> bool {
    use mago_syntax::cst::{UnaryPrefixOperator, Variable};
    let Expression::Variable(Variable::Direct(variable)) = crate::syntax::unparenthesize(expr) else { return false };
    let Some(scope) = ancestors.iter().rev().find(|a| {
        matches!(a, Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_))
    }) else {
        return false;
    };
    let at = crate::syntax::node_span(expr).start;
    let is_it = |e: &Expression<'_>| {
        matches!(crate::syntax::unparenthesize(e), Expression::Variable(Variable::Direct(v)) if v.name == variable.name)
    };
    let mut checked = false;
    traverse(*scope, &mut |node| {
        checked |= crate::syntax::node_span(&node).end <= at
            && match node {
                Node::EmptyConstruct(e) => is_it(e.value),
                Node::UnaryPrefix(u) => matches!(u.operator, UnaryPrefixOperator::Not(_)) && is_it(u.operand),
                _ => false,
            };
        Flow::Continue
    });
    checked
}

/// `AccessoryNonEmptyArrayTypeCorrector`: the item type of a non-empty array whose items are strings with an accessory
/// (`non-empty-string`, `lowercase-string`, ...: an `IntersectionType` in PHPStan) is `string`, of others `mixed`.
fn accessory_string(item: &TUnion) -> Option<TUnion> {
    use mago_codex::ttype::atomic::scalar::string::TStringCasing;
    let accessory = |a: &TAtomic| {
        matches!(a, TAtomic::Scalar(TScalar::String(s)) if s.literal.is_none()
            && (s.is_non_empty || s.is_truthy || s.is_numeric || s.casing != TStringCasing::Unspecified))
    };
    (!item.types.is_empty() && item.types.iter().all(accessory))
        .then(|| TUnion::from_vec(vec![TAtomic::Scalar(TScalar::string())]))
}

/// The members of the item type of the array `$this->name` reads, as written in the property's docblock (`@var
/// array<int, string|int|null>`), each a lowercase short name, with `array` for an array type and `string` for a
/// class string: PHPStan keeps a declared union in the order written, where Mago sorts it. `None` where the property
/// is not one array or a member is not a plain type.
fn declared_items(facts: &Facts, ancestors: &[Node<'_, '_>], expr: &Expression<'_>) -> Option<Vec<String>> {
    use mago_syntax::cst::{Access, ClassLikeMemberSelector, Variable};
    let Expression::Access(Access::Property(fetch)) = crate::syntax::unparenthesize(expr) else { return None };
    let (Expression::Variable(Variable::Direct(this)), ClassLikeMemberSelector::Identifier(name)) =
        (crate::syntax::unparenthesize(fetch.object), &fetch.property)
    else {
        return None;
    };
    if this.name != b"$this" {
        return None;
    }
    let class = super::class_scope(ancestors)?.name?;
    let property = facts.documented().get_declaring_property(class.as_bytes(), &[b"$", name.value].concat())?;
    let span = property.type_metadata.as_ref()?.span;
    let source = facts.source_of(span.file_id)?;
    let text = source.get(span.start.offset as usize..span.end.offset as usize)?.trim();
    let item = match text.strip_suffix("[]") {
        Some(item) => item.trim().trim_start_matches('(').trim_end_matches(')'),
        None => {
            let lists = ["array", "list", "non-empty-array", "non-empty-list"];
            let (_, rest) = text.split_once('<').filter(|(head, _)| lists.contains(&head.trim().to_ascii_lowercase().as_str()))?;
            top_level(rest.strip_suffix('>')?, ',').pop()?
        }
    };
    top_level(item, '|').into_iter().map(declared_member).collect()
}

/// `text` split at each `separator` outside brackets.
fn top_level(text: &str, separator: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let (mut depth, mut start) = (0i32, 0);
    for (i, c) in text.char_indices() {
        match c {
            '<' | '(' | '{' | '[' => depth += 1,
            '>' | ')' | '}' | ']' => depth -= 1,
            _ if c == separator && depth == 0 => {
                parts.push(text[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(text[start..].trim());
    parts
}

/// A member of a docblock union as [`declared_items`] names it.
fn declared_member(member: &str) -> Option<String> {
    let lower = member.to_ascii_lowercase();
    if lower.ends_with("[]") || ["array<", "list<", "non-empty-array<", "non-empty-list<"].iter().any(|p| lower.starts_with(p)) {
        return Some("array".to_string());
    }
    if member.contains(['<', '(', '{', '&', ' ', '\'', '"']) {
        return None;
    }
    Some(
        match lower.as_str() {
            "integer" => "int",
            "boolean" => "bool",
            "double" => "float",
            "class-string" | "non-empty-string" => "string",
            "array" | "list" => "array",
            other => other.rsplit('\\').next().unwrap_or(other),
        }
        .to_string(),
    )
}

/// [`param_declaration`] of `item`, the item type of the array `expr`, its members in the declared order where
/// [`declared_items`] names each of them.
pub(crate) fn item_declaration(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    expr: &Expression<'_>,
    item: &TUnion,
    php_version: u32,
) -> Option<String> {
    if has_array_intersection(item) {
        return None;
    }
    let declaration = param_declaration(item, &facts.codebase, php_version)?;
    let names: Vec<&str> = declaration.split('|').collect();
    let key = |n: &str| n.rsplit('\\').next().unwrap_or(n).to_ascii_lowercase();
    let Some(order) = declared_items(facts, ancestors, expr) else { return Some(declaration) };
    if names.len() < 2 {
        return Some(declaration);
    }
    let mut ordered: Vec<&str> = Vec::new();
    // `TypeCombinator::union` puts the arrays after the other types, and `null` last
    let rank = |o: &String| match o.as_str() {
        "array" => 1,
        "null" => 2,
        _ => 0,
    };
    let mut order = order;
    order.sort_by_key(rank);
    for o in &order {
        if let Some(n) = names.iter().find(|n| key(n) == *o)
            && !ordered.contains(n)
        {
            ordered.push(n);
        }
    }
    Some(if ordered.len() == names.len() { ordered.join("|") } else { declaration })
}

/// The array of `array_slice($array, 0)` or `array_slice($array, 0, $n)` with a positive literal `$n`.
fn head_slice<'a>(expr: &'a Expression<'a>) -> Option<&'a Expression<'a>> {
    use crate::rules::lint_b_helpers::value;
    use crate::rules::rector_code_quality::named;
    use mago_syntax::cst::{Call, Literal};
    let Expression::Call(Call::Function(call)) = crate::syntax::unparenthesize(expr) else { return None };
    let args: Vec<&Expression<'_>> = call.argument_list.arguments.iter().map(value).collect();
    let int = |i: usize| match args.get(i).map(|e| crate::syntax::unparenthesize(e)) {
        Some(Expression::Literal(Literal::Integer(i))) => i.value,
        _ => None,
    };
    (named(call, "array_slice") && int(1) == Some(0) && (args.len() == 2 || int(2).is_some_and(|n| n > 0)))
        .then(|| args[0])
}

/// The function, method or closure around the code, or the file for code outside any.
fn code_scope<'ast, 'arena>(ancestors: &[Node<'ast, 'arena>]) -> Option<Node<'ast, 'arena>> {
    ancestors
        .iter()
        .rev()
        .find(|a| matches!(a, Node::Function(_) | Node::Method(_) | Node::Closure(_) | Node::ArrowFunction(_)))
        .or(ancestors.first())
        .copied()
}

/// The value of `expr` where it is a variable: the right-hand side of its last `=` before `expr` in its function.
fn assigned<'ast, 'arena>(
    ancestors: &[Node<'ast, 'arena>],
    expr: &'ast Expression<'arena>,
) -> &'ast Expression<'arena> {
    use mago_syntax::cst::{AssignmentOperator, Variable};
    let Expression::Variable(Variable::Direct(variable)) = crate::syntax::unparenthesize(expr) else { return expr };
    let Some(scope) = code_scope(ancestors) else { return expr };
    let at = crate::syntax::node_span(expr).start;
    let mut value = expr;
    traverse(scope, &mut |node| {
        if let Node::Assignment(a) = node
            && matches!(a.operator, AssignmentOperator::Assign(_))
            && crate::syntax::node_span(&node).end <= at
            && matches!(crate::syntax::unparenthesize(a.lhs), Expression::Variable(Variable::Direct(v)) if v.name == variable.name)
        {
            value = a.rhs;
        }
        Flow::Continue
    });
    value
}

/// Whether `expr` is (a variable assigned) `explode()` without a limit of a lowercase or uppercase string
/// (`strtolower()` and the like, or a variable assigned one): PHPStan's `ExplodeFunctionDynamicReturnTypeExtension`
/// gives it `non-empty-list<lowercase-string>`, whose item `AccessoryNonEmptyArrayTypeCorrector` makes `string`.
/// Not where a `str_contains()`, `str_starts_with()` or `str_ends_with()` of the haystack proves the delimiter is
/// there: the extension then gives an unsealed constant array. PHPStan finds that call only by its fully qualified
/// name, so an unqualified one counts outside a namespace only (checked with Rector 2.6.1).
fn cased_explode<'ast, 'arena>(ancestors: &[Node<'ast, 'arena>], expr: &'ast Expression<'arena>) -> bool {
    use crate::rules::lint_b_helpers::value;
    use crate::rules::rector_code_quality::named;
    use mago_syntax::cst::{Call, FunctionCall, Identifier, Variable};
    fn call_of<'b>(e: &'b Expression<'b>) -> Option<&'b FunctionCall<'b>> {
        match crate::syntax::unparenthesize(e) {
            Expression::Call(Call::Function(call)) => Some(call),
            _ => None,
        }
    }
    let Some(call) = call_of(assigned(ancestors, expr)).filter(|c| named(c, "explode")) else { return false };
    let args: Vec<&Expression<'_>> = call.argument_list.arguments.iter().map(value).collect();
    let [_, haystack] = args.as_slice() else { return false };
    let cased = call_of(assigned(ancestors, haystack)).is_some_and(|c| {
        ["strtolower", "mb_strtolower", "strtoupper", "mb_strtoupper"].iter().any(|n| named(c, n))
    });
    if !cased {
        return false;
    }
    let Expression::Variable(Variable::Direct(variable)) = crate::syntax::unparenthesize(haystack) else { return true };
    let Some(scope) = code_scope(ancestors) else { return true };
    let global = !ancestors.iter().any(|a| matches!(a, Node::Namespace(_)));
    let mut proven = false;
    traverse(scope, &mut |node| {
        if let Node::FunctionCall(c) = node
            && ["str_contains", "str_starts_with", "str_ends_with"].iter().any(|n| named(c, n))
            && (global || matches!(c.function, Expression::Identifier(Identifier::FullyQualified(_))))
            && c.argument_list.arguments.iter().next().is_some_and(|a| {
                matches!(crate::syntax::unparenthesize(value(a)), Expression::Variable(Variable::Direct(v)) if v.name == variable.name)
            })
        {
            proven = true;
        }
        Flow::Continue
    });
    !proven
}

/// A call PHPStan's return type extensions make a non-empty list where Mago gives a list (checked with Rector 2.6.1):
/// `explode()` but with a negative limit, and `range()` but of literal integers with 50 items or fewer (a constant
/// array).
fn non_empty_call(expr: &Expression<'_>) -> bool {
    use crate::rules::lint_b_helpers::value;
    use crate::rules::rector_code_quality::named;
    use mago_syntax::cst::{Call, Literal, UnaryPrefixOperator};
    let Expression::Call(Call::Function(call)) = crate::syntax::unparenthesize(expr) else {
        return false;
    };
    let args: Vec<&Expression<'_>> = call.argument_list.arguments.iter().map(value).collect();
    let int = |e: &&Expression<'_>| match crate::syntax::unparenthesize(e) {
        Expression::Literal(Literal::Integer(i)) => i.value,
        _ => None,
    };
    if named(call, "explode") {
        return !args.get(2).is_some_and(|limit| {
            matches!(
                crate::syntax::unparenthesize(limit),
                Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Negation(_))
            )
        });
    }
    if named(call, "range") {
        let step = match args.get(2) {
            Some(step) => int(step),
            None => Some(1),
        };
        return match (args.first().and_then(int), args.get(1).and_then(int), step) {
            (Some(from), Some(to), Some(step)) if step > 0 => from.abs_diff(to) / step + 1 > 50,
            _ => true,
        };
    }
    false
}

/// The array shape of `t`; with `non_empty`, a non-empty array is `array<mixed, mixed>`. Several arrays are PHPStan's
/// `TypeCombinator::union` of them, as far as Pun follows it: with a general array among them, one `ArrayType` whose
/// item type is the union of theirs (Mago keeps them apart), non-empty only when each of them is; constant arrays only
/// stay a union.
fn array_shape(t: &TUnion, non_empty: bool) -> Option<ArrayShape> {
    let arrays: Vec<&TArray> = t
        .types
        .iter()
        .map(|a| match a {
            TAtomic::Array(array) => Some(array),
            _ => None,
        })
        .collect::<Option<_>>()?;
    if let [one] = arrays.as_slice() {
        return one_array_shape(one, non_empty);
    }
    if non_empty && arrays.iter().all(|a| matches!(one_array_shape(a, true), Some(ArrayShape::General(None)))) {
        return Some(ArrayShape::General(None));
    }
    let shapes: Vec<ArrayShape> = arrays.iter().map(|a| one_array_shape(a, false)).collect::<Option<_>>()?;
    if shapes.iter().all(|s| matches!(s, ArrayShape::Constant(_))) {
        return None;
    }
    let mut items: Vec<TAtomic> = Vec::new();
    for shape in &shapes {
        let Some(item) = item_type(shape) else { continue };
        for a in item.types.iter() {
            if !items.contains(a) {
                items.push(a.clone());
            }
        }
    }
    Some(ArrayShape::General(
        (!items.is_empty()).then(|| TUnion::from_vec(items)),
    ))
}

fn one_array_shape(array: &TArray, non_empty: bool) -> Option<ArrayShape> {
    Some(match array {
        TArray::List(list) => match &list.known_elements {
            Some(known) if list.element_type.is_never() => {
                ArrayShape::Constant(known.values().map(|(_, v)| v.clone()).collect())
            }
            Some(_) => return None,
            None if list.non_empty && non_empty => ArrayShape::General(accessory_string(&list.element_type)),
            None => ArrayShape::General(Some((*list.element_type).clone())),
        },
        TArray::Keyed(keyed) => match (&keyed.known_items, &keyed.parameters) {
            (known, None) => {
                ArrayShape::Constant(known.iter().flat_map(|k| k.values()).map(|(_, v)| v.clone()).collect())
            }
            (Some(_), Some(_)) => return None,
            (None, Some((_, value))) if keyed.non_empty && non_empty => ArrayShape::General(accessory_string(value)),
            (None, Some((_, value))) => ArrayShape::General(Some((**value).clone())),
        },
    })
}

/// `ConstantArrayType::getItemType()` or `ArrayType::getItemType()`: the union of a constant array's values, or the
/// item type; `None` for `mixed` or a constant array without values (`never`).
pub(crate) fn item_type(shape: &ArrayShape) -> Option<TUnion> {
    match shape {
        ArrayShape::Constant(values) => {
            let mut types: Vec<TAtomic> = Vec::new();
            for v in values {
                for t in v.types.iter() {
                    if !types.contains(t) {
                        types.push(t.clone());
                    }
                }
            }
            (!types.is_empty()).then(|| TUnion::from_vec(types))
        }
        ArrayShape::General(value) => value.clone(),
    }
}

/// `TypeCombinator::union()` of a constant array's values (`types`, in order): the types that are not constant scalars
/// or arrays keep their order, then come the arrays, then the constant scalars (`null` and literal values), by kind in the order the kinds come.
pub(crate) fn constant_values_union(types: &[TAtomic]) -> Vec<TAtomic> {
    let kind = |t: &TAtomic| match t {
        TAtomic::Null => Some(0),
        TAtomic::Scalar(s) if s.is_literal_value() => Some(match s {
            TScalar::Integer(_) => 1,
            TScalar::String(_) => 2,
            TScalar::Float(_) => 3,
            _ => 4,
        }),
        _ => None,
    };
    // the arrays come after the other types
    let mut out: Vec<TAtomic> = types.iter().filter(|t| kind(t).is_none() && !matches!(t, TAtomic::Array(_))).cloned().collect();
    out.extend(types.iter().filter(|t| matches!(t, TAtomic::Array(_))).cloned());
    let mut kinds: Vec<u8> = Vec::new();
    for k in types.iter().filter_map(kind) {
        if !kinds.contains(&k) {
            kinds.push(k);
        }
    }
    for k in kinds {
        out.extend(types.iter().filter(|t| kind(t) == Some(k)).cloned());
    }
    out
}

/// What [`traverse`] does after a node.
pub(crate) enum Flow {
    Continue,
    Stop,
}

/// `traverseNodesWithCallable()` over the children of `node`, outer nodes first, without entering nested functions,
/// closures, arrow functions and classes (an anonymous class's arguments are walked, as php-parser keeps them on the
/// `new`), until `f` stops it. Returns whether it stopped.
pub(crate) fn traverse<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    f: &mut dyn FnMut(Node<'ast, 'arena>) -> Flow,
) -> bool {
    let mut stopped = false;
    node.visit_children(|child| {
        if stopped {
            return;
        }
        match child {
            Node::Function(_)
            | Node::Closure(_)
            | Node::ArrowFunction(_)
            | Node::Method(_)
            | Node::Class(_)
            | Node::Interface(_)
            | Node::Trait(_)
            | Node::Enum(_) => {}
            Node::AnonymousClass(class) => {
                if let Some(args) = &class.argument_list {
                    stopped = traverse(Node::PartialArgumentList(args), f);
                }
            }
            _ => {
                stopped = matches!(f(child), Flow::Stop) || traverse(child, f);
            }
        }
    });
    stopped
}

/// PHPStan's `TypeCombinator::union()` of `args` (each a type's members, a union when it has more than one), for
/// general types as Rector's `TypeFactory` leaves them: one type that is not a union or an array stays; otherwise the
/// members keep their order but for merging, where an equal type or a supertype later on takes the place (members of
/// one argument are not compared), the arrays become one array after them, then the enum cases, then `null`.
pub(crate) fn phpstan_union(codebase: &CodebaseMetadata, args: Vec<Vec<TAtomic>>) -> Vec<TAtomic> {
    if let [one] = args.as_slice()
        && let [single] = one.as_slice()
        && !matches!(single, TAtomic::Array(_))
    {
        return one.clone();
    }
    let mut types: Vec<(usize, TAtomic)> = Vec::new();
    let mut array: Option<TAtomic> = None;
    let mut cases: Vec<TAtomic> = Vec::new();
    let mut null = false;
    for (group, arg) in args.into_iter().enumerate() {
        for atomic in arg {
            match atomic {
                TAtomic::Null => null = true,
                TAtomic::Array(_) => {
                    array.get_or_insert(atomic);
                }
                TAtomic::Object(TObject::Enum(ref e)) if e.case.is_some() => {
                    if !cases.contains(&atomic) {
                        cases.push(atomic);
                    }
                }
                other => types.push((group, other)),
            }
        }
    }
    let mut i = 0;
    while i < types.len() {
        let mut j = i + 1;
        let mut removed = false;
        while j < types.len() {
            if types[i].0 == types[j].0 {
                j += 1;
                continue;
            }
            if is_supertype(codebase, &types[j].1, &types[i].1) {
                types.remove(i);
                removed = true;
                break;
            }
            if is_supertype(codebase, &types[i].1, &types[j].1) {
                types.remove(j);
                continue;
            }
            j += 1;
        }
        if !removed {
            i += 1;
        }
    }
    let mut out: Vec<TAtomic> = types.into_iter().map(|(_, t)| t).collect();
    out.extend(array);
    out.extend(cases);
    if null {
        out.push(TAtomic::Null);
    }
    out
}

/// `$a->isSuperTypeOf($b)->yes()` for general types: equal ones, or a class and its subclass.
fn is_supertype(codebase: &CodebaseMetadata, a: &TAtomic, b: &TAtomic) -> bool {
    if a == b {
        return true;
    }
    let (TAtomic::Object(TObject::Named(a)), TAtomic::Object(TObject::Named(b))) = (a, b) else {
        return false;
    };
    a.name.as_bytes().eq_ignore_ascii_case(b.name.as_bytes())
        || codebase.get_class_like(b.name.as_bytes()).is_some_and(|c| {
            c.all_parent_classes
                .iter()
                .chain(c.all_parent_interfaces.iter())
                .any(|p| p.as_bytes().eq_ignore_ascii_case(a.name.as_bytes()))
        })
}
