//! `lint/types/add-return-type-from-new-array` and `lint/types/add-private-method-array-return-type`, ported from
//! Rector 2.8.4's `ReturnTypeFromStrictNewArrayRector` and `PrivateMethodReturnTypeFromStrictNewArrayRector`
//! (rules/TypeDeclaration/Rector/ClassMethod) with the `StrictReturnNewArrayResolver` they share
//! (rules/TypeDeclaration/NodeAnalyzer).
//!
//! A function or method without a return type that starts a variable as an array and returns it (or whose every
//! `return` returns an array) gets `array`, and, without a `@return` tag, `@return` the array's item type.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{AssignmentOperator, Expression, Node, NodeKind, Statement};
use pun_phpstan::types::{Accessory, Type, intersect, mixed};
use pun_problem::{Edit, Safety, Span};

use super::declare::expression_order;
use super::docblock::{FunctionLike, assignments, class_name, variable_name};
use super::facts::{Facts, facts};
use super::silent_void::Place;
use super::type_factory::{Created, infer_returns};
use super::{SCALAR_TYPES, class_scope, has_only_return_with_expr, returns_scoped, untyped};
use crate::rules::phpdoc_writer::type_mapper::is_mixed;
use crate::rules::phpdoc_writer::{DocTarget, change_return_type, phpstan_type};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddReturnTypeFromNewArray;

pub struct AddPrivateMethodArrayReturnType;

impl Rule for AddReturnTypeFromNewArray {
    fn id(&self) -> &'static str {
        "lint/types/add-return-type-from-new-array"
    }

    fn min_php_version(&self) -> u32 {
        SCALAR_TYPES
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Function, NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = FunctionLike::of(node) else {
            return;
        };
        // private methods are handled by PrivateMethodReturnTypeFromStrictNewArrayRector
        if f.is_private() || untyped(node, ancestors, ctx).is_none() {
            return;
        }
        out.extend(resolve(&f, ancestors, ctx));
    }
}

impl Rule for AddPrivateMethodArrayReturnType {
    fn id(&self) -> &'static str {
        "lint/types/add-private-method-array-return-type"
    }

    fn min_php_version(&self) -> u32 {
        SCALAR_TYPES
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
        let Some(f) = FunctionLike::of(node).filter(|f| f.method.is_some()) else {
            return;
        };
        if !f.is_private() || f.return_type.is_some() {
            return;
        }
        out.extend(resolve(&f, ancestors, ctx));
    }
}

/// `$type->isArray()->yes()` of Mago's type.
fn is_array(t: Option<&TUnion>) -> bool {
    t.is_some_and(|t| !t.types.is_empty() && t.types.iter().all(|a| matches!(a, TAtomic::Array(_))))
}

/// `StrictReturnNewArrayResolver::resolve()`.
fn resolve(
    f: &FunctionLike<'_, '_>,
    ancestors: &[Node<'_, '_>],
    ctx: &Context<'_>,
) -> Option<Report> {
    let body = f.body?;
    let facts = facts(ctx);
    let mut variables = match_array_assigned_variables(&facts, body.statements.iter());
    if variables.is_empty() {
        return None;
    }
    let returns = returns_scoped(body);
    let scope = class_scope(ancestors);
    let place = Place {
        facts: &facts,
        ancestors,
        scope: scope.as_ref(),
    };
    if !has_only_return_with_expr(&place, body, &returns) {
        return None;
    }
    // matchVariableNotOverriddenByNonArray
    for a in assignments(Node::Block(body)) {
        let Some(name) = variable_name(a.lhs) else {
            continue;
        };
        if !variables.contains(&name)
            || matches!(a.rhs, Expression::Array(_) | Expression::LegacyArray(_))
        {
            continue;
        }
        if !is_array(facts.native_type(node_span(a.rhs))) {
            variables.retain(|v| *v != name);
        }
    }
    if variables.is_empty() {
        return None;
    }
    let return_type = if returns.len() > 1 {
        // ReturnTypeInferer::inferFunctionLike(): an interface's method has no body
        let types = match infer_returns(&place, body, &returns) {
            Created::Single(t) => vec![t],
            Created::Union(types) => types,
            Created::Mixed => return None,
        };
        if !types.iter().all(|t| matches!(t, TAtomic::Array(_))) {
            return None;
        }
        let types: Vec<Type> = types
            .iter()
            .map(|a| array_atomic(a, &[]))
            .collect::<Option<_>>()?;
        match types.len() {
            1 => types.into_iter().next().expect("one type"),
            _ => Type::Union(types),
        }
    } else {
        let value = unparenthesize(returns[0].value?);
        let name = variable_name(value)?;
        if !variables.contains(&name) {
            return None;
        }
        let native = facts.native_type(node_span(value));
        if !is_array(native) {
            return None;
        }
        // the values appended to it, each ternary's members in PHPStan's order
        let mut order = Vec::new();
        for a in assignments(Node::Block(body)) {
            if let Expression::ArrayAppend(append) = a.lhs
                && variable_name(append.array) == Some(name)
            {
                order.extend(
                    expression_order(a.rhs, |e| facts.native_type(node_span(e)))
                        .into_iter()
                        .flatten(),
                );
            }
        }
        array_type(native?, &order)?
    };
    // processAddArrayReturnType()
    let at = node_span(f.parameters).end;
    let mut edits = Vec::new();
    if should_add_return_array_doc_type(&return_type) {
        let target = f.target(ctx);
        let mut info = target.info(ctx.source);
        // skip already filled type, on purpose
        let current = info.return_tag().map_or(mixed(), |tag| {
            DocTarget::doc_type(tag, ancestors, class_name(ancestors).as_deref())
        });
        let single_value = match &return_type {
            Type::ConstArray(a) => a.values.len() == 1,
            _ => true,
        };
        if is_mixed(&current) && single_value {
            let item = return_type.iterable_value_type();
            let mut narrow = if matches!(item, Type::Intersection(_)) {
                return_type.clone()
            } else {
                Type::array(mixed(), item)
            };
            if return_type.is_list().yes() {
                narrow = intersect(vec![narrow, Type::Accessory(Accessory::List)]);
            }
            if change_return_type(&mut info, &narrow) {
                edits.push(target.edit(ctx.source, &info.print()));
            }
        }
    }
    edits.push(Edit::new(Span::new(at, at), ": array".to_string()));
    Some(Report::fixed(
        f.name,
        "Add the `array` return type of the array the function builds and returns.",
        Safety::Unsafe,
        edits,
    ))
}

/// `matchArrayAssignedVariable()`: the variables the body's own statements assign a native array.
fn match_array_assigned_variables<'s, 'arena: 's>(
    facts: &Facts,
    statements: impl Iterator<Item = &'s Statement<'arena>>,
) -> Vec<&'arena str> {
    let mut variables = Vec::new();
    for statement in statements {
        let Statement::Expression(e) = statement else {
            continue;
        };
        let Expression::Assignment(a) = unparenthesize(e.expression) else {
            continue;
        };
        if !matches!(a.operator, AssignmentOperator::Assign(_)) {
            continue;
        }
        let Some(name) = variable_name(a.lhs) else {
            continue;
        };
        if is_array(facts.native_type(node_span(a.rhs))) {
            variables.push(name);
        }
    }
    variables
}

/// `shouldAddReturnArrayDocType()`: only plain list<> and generic array<> doc types are worth adding; array shapes
/// (a constant array with keys that is no list, anywhere in the type) produce noisy, fragile doc types.
fn should_add_return_array_doc_type(t: &Type) -> bool {
    fn visit(t: &Type, found: &std::cell::Cell<bool>) -> Type {
        if let Type::ConstArray(a) = t
            && !a.keys.is_empty()
            && !t.is_list().yes()
        {
            found.set(true);
        }
        t.traverse(&|child| visit(child, found))
    }
    let found = std::cell::Cell::new(false);
    visit(t, &found);
    !found.get()
}

/// PHPStan's type of a native array as the scope builds it: Mago's non-empty list a `list<T>`
/// (`AccessoryArrayListType`), a non-empty array with `NonEmptyArrayType`, its items alike; anything else as
/// [`phpstan_type`] maps it. A union's members come in the order of `order` (PHPStan's union of the values in the order
/// they are added), the others after them.
fn array_type(t: &TUnion, order: &[&TAtomic]) -> Option<Type> {
    let mut atomics: Vec<&TAtomic> = t.types.iter().collect();
    atomics.sort_by_key(|a| order.iter().position(|o| o == a).unwrap_or(order.len()));
    let mut types: Vec<Type> = atomics
        .into_iter()
        .map(|a| array_atomic(a, order))
        .collect::<Option<_>>()?;
    Some(match types.len() {
        1 => types.remove(0),
        _ => Type::Union(types),
    })
}

fn array_atomic(a: &TAtomic, order: &[&TAtomic]) -> Option<Type> {
    match a {
        // a non-empty one is what straight-line appends build (PHPStan's constant list); one that may be empty comes
        // from a loop, whose generalization PHPStan's scope keeps a plain `array<int<0, max>, T>`
        TAtomic::Array(TArray::List(l)) if l.known_elements.is_none() => {
            let array = Type::array(
                Type::int_range(Some(0), None),
                array_type(&l.element_type, order)?,
            );
            Some(match l.non_empty {
                true => Type::Intersection(vec![array, Type::Accessory(Accessory::List)]),
                false => array,
            })
        }
        TAtomic::Array(TArray::Keyed(k)) if k.known_items.is_none() && k.parameters.is_some() => {
            let (key, value) = k.parameters.as_ref()?;
            let array = Type::array(phpstan_type(key)?, array_type(value, order)?);
            Some(if k.non_empty {
                Type::Intersection(vec![array, Type::Accessory(Accessory::NonEmptyArray)])
            } else {
                array
            })
        }
        _ => phpstan_type(&TUnion::from_atomic(a.clone())),
    }
}
