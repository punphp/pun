//! `lint/types/add-param-type-dim-fetch-key`, ported from Rector 2.6.1's `AddParamFromDimFetchKeyUseRector`
//! (rules/TypeDeclaration/Rector/ClassMethod) with `ArrayDimFetchFinder::findByDimName` and `ParamTypeAddGuard`: an
//! untyped parameter of a class's method that the method uses as an array key (`$items[$key]`) gets the key type of
//! the array, when every such array is a general or constant array.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{Access, ArrayAccess, ClassLikeMemberSelector, ArrayElement, Expression, Literal, MethodBody, Modifier, Node, NodeKind, Variable};

use super::class_scope;
use super::facts::{Facts, facts};
use super::has_parent_class_method;
use super::param::{Flow, add_hint, param_declaration, report, traverse};
use crate::rules::add_param_type_sprintf::is_legal;
use crate::rules::class_method::type_of;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `PhpVersionFeature::UNION_TYPES`.
const UNION_TYPES: u32 = 80000;

pub struct AddParamTypeDimFetchKey;

impl Rule for AddParamTypeDimFetchKey {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-dim-fetch-key"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else {
            return;
        };
        // a method of a class (`Class_`, anonymous ones included)
        let in_class = ancestors.iter().rev().find(|a| {
            matches!(
                a,
                Node::Class(_)
                    | Node::AnonymousClass(_)
                    | Node::Interface(_)
                    | Node::Trait(_)
                    | Node::Enum(_)
            )
        });
        if !matches!(in_class, Some(Node::Class(_) | Node::AnonymousClass(_))) {
            return;
        }
        let MethodBody::Concrete(body) = &method.body else {
            return;
        };
        let facts = facts(ctx);
        if !method.modifiers.iter().any(|m| matches!(m, Modifier::Private(_)))
            && class_scope(ancestors)
                .is_some_and(|scope| has_parent_class_method(&facts, &scope, method.name.value))
        {
            return;
        }
        let body = Node::Block(body);
        let mut edits = Vec::new();
        for param in method.parameter_list.parameters.iter() {
            if param.hint.is_some() {
                continue;
            }
            let name = param.variable.name;
            let mut fetches: Vec<&ArrayAccess<'_>> = Vec::new();
            traverse(body, &mut |node| {
                if let Node::ArrayAccess(access) = node
                    && matches!(unparenthesize(access.index), Expression::Variable(Variable::Direct(v)) if v.name == name)
                {
                    fetches.push(access);
                }
                Flow::Continue
            });
            if fetches.is_empty() || !is_legal(body, name) {
                continue;
            }
            let class = class_scope(ancestors).and_then(|scope| scope.name);
            let Some((key, order)) = last_key_type(&facts, class.as_deref(), body, &fetches) else {
                continue;
            };
            // a constant array's `int` and `string` keys keep their order (PHPStan's union of them is not sorted)
            let declaration = match order {
                Some(names) if ctx.php_version >= UNION_TYPES => Some(names.join("|")),
                _ => param_declaration(&key, &facts.codebase, ctx.php_version),
            };
            if let Some(declaration) = declaration {
                edits.push(add_hint(param, &declaration));
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(report(
            node_span(&node),
            "Add the key type of the arrays a parameter indexes to it.",
            edits,
        ));
    }
}

/// The key type of the last fetch's array, when every fetched array is an `ArrayType` or a `ConstantArrayType` and
/// no key variable's type is a union; `None` for `mixed` (Mago's `array-key` of an `array` without a docblock). With
/// it, the order of a constant array's `int` and `string` keys when it has both.
fn last_key_type(
    facts: &Facts,
    class: Option<&str>,
    body: Node<'_, '_>,
    fetches: &[&ArrayAccess<'_>],
) -> Option<(TUnion, Option<Vec<&'static str>>)> {
    let mut key = None;
    for fetch in fetches {
        // the target of an assignment: Mago's type is the one after it, PHPStan's the property's declared one
        let t = if is_assigned(body, fetch) {
            class
                .and_then(|class| declared_property_type(facts, class, fetch.array))
                .or_else(|| type_of(facts, fetch.array))?
        } else {
            type_of(facts, fetch.array)?
        };
        let [TAtomic::Array(array)] = t.types.as_ref() else {
            return None;
        };
        // after `isset($items[...])`, PHPStan's type of `$items` is an intersection with `hasOffset`
        if after_isset(body, fetch) {
            return None;
        }
        let fetched = key_type(array)?;
        let order = match &fetched {
            Some(t) if t.types.len() > 1 => Some(literal_key_order(body, fetch)?),
            _ => None,
        };
        key = Some(fetched.map(|t| (t, order)));
        // the index is the parameter, `mixed` or its narrowed type
        if type_of(facts, fetch.index).is_some_and(|t| t.types.len() > 1) {
            return None;
        }
    }
    key.flatten()
}

/// Whether an `isset()` of an offset of the variable `fetch` reads ends before it.
fn after_isset(body: Node<'_, '_>, fetch: &ArrayAccess<'_>) -> bool {
    let Expression::Variable(Variable::Direct(variable)) = unparenthesize(fetch.array) else {
        return false;
    };
    let mut found = false;
    traverse(body, &mut |node| {
        if let Node::IssetConstruct(isset) = node
            && node_span(isset).end <= node_span(fetch).start
            && isset.values.iter().any(|v| {
                matches!(unparenthesize(v), Expression::ArrayAccess(a)
                    if matches!(unparenthesize(a.array), Expression::Variable(Variable::Direct(w)) if w.name == variable.name))
            })
        {
            found = true;
            return Flow::Stop;
        }
        Flow::Continue
    });
    found
}

/// Whether `fetch` is the target of an assignment (`$this->items[$key] = $value`).
fn is_assigned(body: Node<'_, '_>, fetch: &ArrayAccess<'_>) -> bool {
    let mut found = false;
    traverse(body, &mut |node| {
        if let Node::Assignment(a) = node
            && node_span(unparenthesize(a.lhs)) == node_span(fetch)
        {
            found = true;
            return Flow::Stop;
        }
        Flow::Continue
    });
    found
}

/// The declared type (its docblock's, else the native one) of `$this->name`: the type PHPStan gives the property as
/// the target of an assignment.
fn declared_property_type<'f>(facts: &'f Facts, class: &str, expr: &Expression<'_>) -> Option<&'f TUnion> {
    let Expression::Access(Access::Property(fetch)) = unparenthesize(expr) else {
        return None;
    };
    if !matches!(unparenthesize(fetch.object), Expression::Variable(Variable::Direct(v)) if v.name == b"$this") {
        return None;
    }
    let ClassLikeMemberSelector::Identifier(name) = &fetch.property else {
        return None;
    };
    let property = facts
        .documented()
        .get_declaring_property(class.as_bytes(), &[b"$", name.value].concat())?;
    property.type_metadata.as_ref().map(|t| &t.type_union)
}

/// `getKeyType()` of an `ArrayType` (`Some(None)` when it is `mixed`) or a `ConstantArrayType` (its keys' types in
/// order, ints and strings), a list's `int`; `None` for a non-empty general array (an `IntersectionType`) and other
/// arrays.
fn key_type(array: &TArray) -> Option<Option<TUnion>> {
    match array {
        TArray::Keyed(keyed) => match (&keyed.known_items, &keyed.parameters) {
            (None, Some(_)) if keyed.non_empty => None,
            (None, Some((key, _))) => Some(
                (!key.types.iter().any(|a| matches!(a, TAtomic::Mixed(_) | TAtomic::Scalar(TScalar::ArrayKey))))
                    .then(|| (**key).clone()),
            ),
            (Some(items), None) => {
                let mut kinds: Vec<TAtomic> = Vec::new();
                for key in items.keys() {
                    let kind = match key {
                        ArrayKey::Integer(_) => TAtomic::Scalar(TScalar::int()),
                        _ => TAtomic::Scalar(TScalar::string()),
                    };
                    if !kinds.contains(&kind) {
                        kinds.push(kind);
                    }
                }
                Some(Some(TUnion::from_vec(kinds)))
            }
            _ => None,
        },
        TArray::List(list) if list.non_empty && list.known_elements.is_none() => None,
        TArray::List(_) => Some(Some(TUnion::from_vec(vec![TAtomic::Scalar(TScalar::int())]))),
    }
}

/// The key types of a constant array with `int` and `string` keys, in the order PHPStan keeps them (its keys'
/// order): that of the literal keys of the array literal last assigned to the fetched variable before the fetch, then
/// the kind they leave out (Mago sorts the keys).
fn literal_key_order(body: Node<'_, '_>, fetch: &ArrayAccess<'_>) -> Option<Vec<&'static str>> {
    let Expression::Variable(Variable::Direct(variable)) = unparenthesize(fetch.array) else {
        return None;
    };
    let mut literal = None;
    traverse(body, &mut |node| {
        if let Node::Assignment(assign) = node
            && node_span(assign).end <= node_span(fetch).start
            && matches!(unparenthesize(assign.lhs), Expression::Variable(Variable::Direct(v)) if v.name == variable.name)
        {
            literal = match unparenthesize(assign.rhs) {
                Expression::Array(array) => Some(&array.elements),
                Expression::LegacyArray(array) => Some(&array.elements),
                _ => None,
            };
        }
        Flow::Continue
    });
    let mut ordered: Vec<&'static str> = Vec::new();
    for element in literal?.iter() {
        let kind = match element {
            ArrayElement::KeyValue(pair) => match unparenthesize(pair.key) {
                Expression::Literal(Literal::String(_)) => "string",
                Expression::Literal(Literal::Integer(_)) => "int",
                // a constant's value: the key kind the literal keys leave out comes last
                _ => continue,
            },
            _ => "int",
        };
        if !ordered.contains(&kind) {
            ordered.push(kind);
        }
    }
    for kind in ["int", "string"] {
        if !ordered.contains(&kind) {
            ordered.push(kind);
        }
    }
    Some(ordered)
}
