//! `lint/types/add-param-type-data-provider`, ported from Rector 2.6.1's `AddParamTypeBasedOnPHPUnitDataProviderRector`
//! (rules/TypeDeclaration/Rector/ClassMethod) with `ParameterTypeFromDataProviderResolver`: in a PHPUnit test class,
//! an untyped parameter of a public test method gets the type of the values its data providers give at its position,
//! read from the provider's first returned array literal or from its yielded arrays.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::r#enum::TEnum;
use mago_codex::ttype::atomic::object::named::TNamedObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    ArrayElement, ClassLikeMember, Expression, Method, MethodBody, Modifier, Node, NodeKind,
    UnaryPrefixOperator, Variable,
};

use super::add_param_type_array_dim_fetch::is_variable;
use super::class_scope;
use super::facts::{Facts, facts};
use super::names::resolve_written;
use super::never::is_test_class;
use super::param::{Flow, add_hint, param_declaration_unsorted, phpstan_union, report, traverse};
use crate::rules::class_method::{class_info, method_data_providers, type_of};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddParamTypeDataProvider;

impl Rule for AddParamTypeDataProvider {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-data-provider"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class, NodeKind::AnonymousClass])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let facts = facts(ctx);
        let mut scoped = ancestors.to_vec();
        scoped.push(node);
        if !class_scope(&scoped).is_some_and(|scope| is_test_class(&facts, &scope)) {
            return;
        }
        let Some(class) = class_info(node, ancestors, &facts) else { return };
        // A `@dataProvider` tag whose name an import makes a class (`use ...\Attributes\DataProvider;`) is a Doctrine
        // annotation to Rector's docblock parser, which `DataProviderNodes` does not read.
        let doc_tags = facts
            .codebase
            .get_class_like(resolve_written(ancestors, "dataProvider").as_bytes())
            .is_none();
        let mut edits = Vec::new();
        for (index, member) in class.members.iter().enumerate() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            if method
                .modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Private(_) | Modifier::Protected(_)))
                || method.parameter_list.parameters.is_empty()
            {
                continue;
            }
            let providers: Vec<&Method<'_>> = method_data_providers(ctx.source, ancestors, &class, index, doc_tags)
                .iter()
                .filter_map(|name| {
                    class.members.iter().find_map(|m| match m {
                        ClassLikeMember::Method(m) if m.name.value == name.as_slice() => Some(m),
                        _ => None,
                    })
                })
                .collect();
            if providers.is_empty() {
                continue;
            }
            for (position, param) in method.parameter_list.parameters.iter().enumerate() {
                if param.hint.is_some() || param.ellipsis.is_some() {
                    continue;
                }
                let Some(types) = providers
                    .iter()
                    .map(|provider| provider_type(&facts, provider, position))
                    .collect::<Option<Vec<_>>>()
                else {
                    continue;
                };
                let types = phpstan_union(&facts.codebase, types);
                if let Some(declaration) =
                    param_declaration_unsorted(&types, &facts.codebase, ctx.php_version)
                {
                    edits.push(add_hint(param, &declaration));
                }
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(report(
            node_span(&node),
            "Add the types of the values the data providers give to the test methods' parameters.",
            edits,
        ));
    }
}

/// `resolveParameterTypeFromDataProvider()` with `createMixedPassedOrUnionType()`: the types (literal values made
/// general) the provider gives at `position`, in the order met; `None` for `mixed`.
fn provider_type(facts: &Facts, provider: &Method<'_>, position: usize) -> Option<Vec<TAtomic>> {
    let MethodBody::Concrete(body) = &provider.body else { return None };
    let body = Node::Block(body);
    let mut first_return: Option<Option<&Expression<'_>>> = None;
    let mut yields: Vec<Option<&Expression<'_>>> = Vec::new();
    let mut yield_from = false;
    traverse(body, &mut |node| {
        match node {
            Node::Return(r) if first_return.is_none() => first_return = Some(r.value),
            Node::YieldValue(y) => yields.push(y.value),
            Node::YieldPair(y) => yields.push(Some(y.value)),
            Node::YieldFrom(_) => yield_from = true,
            _ => {}
        }
        Flow::Continue
    });
    let mut types: Vec<TUnion> = Vec::new();
    if let Some(returned) = first_return {
        // resolveReturnStaticArrayTypeByParameterPosition
        let elements = match returned.map(unparenthesize) {
            Some(Expression::Array(a)) => &a.elements,
            Some(Expression::LegacyArray(a)) => &a.elements,
            _ => return None,
        };
        for set in elements.iter() {
            let items = match set.get_value().map(unparenthesize) {
                Some(Expression::Array(a)) => &a.elements,
                Some(Expression::LegacyArray(a)) => &a.elements,
                _ => return None,
            };
            let Some(item) = items.iter().nth(position).and_then(ArrayElement::get_value) else { continue };
            // `new static()` is PHPStan's `StaticType`, which no parameter type declares
            if matches!(unparenthesize(item), Expression::Instantiation(i) if matches!(i.class, Expression::Static(_))) {
                return None;
            }
            // PHPStan's `toObject()` of an array is `stdClass` (Mago's cast gives an object shape)
            if let Expression::UnaryPrefix(cast) = unparenthesize(item)
                && matches!(cast.operator, UnaryPrefixOperator::ObjectCast(..))
                && matches!(unparenthesize(cast.operand), Expression::Array(_) | Expression::LegacyArray(_))
            {
                let class = facts.codebase.get_class_like(b"stdClass")?;
                types.push(TUnion::from_vec(vec![TAtomic::Object(TObject::Named(TNamedObject::new(class.name)))]));
                continue;
            }
            // Rector's `getType()` of an enum case read in the array literal is the enum's `ObjectType`, not the case
            // (a yielded array's values keep the case), so `TypeCombinator::union` leaves it in place
            let t = type_of(facts, item)?;
            types.push(TUnion::from_vec(t.types.iter().map(|a| match a {
                TAtomic::Object(TObject::Enum(e)) => TAtomic::Object(TObject::Enum(TEnum { case: None, ..e.clone() })),
                other => other.clone(),
            }).collect()));
        }
    } else {
        // "yield from" data sets are not resolved here
        if yield_from {
            return None;
        }
        // resolveYieldStaticArrayTypeByParameterPosition
        for value in yields {
            // PHPStan types the yielded array in the scope before it, where a variable one item assigns
            // (`[$c = collect(), $c]`) is not defined yet in a later one: that item is `mixed`
            if let Some(Expression::Array(array)) = value.map(unparenthesize)
                && reads_assigned_item(array.elements.iter().collect(), position)
            {
                return None;
            }
            let t = type_of(facts, value?)?;
            for atomic in t.types.iter() {
                let values = constant_array_values(atomic)?;
                if let Some(v) = values.get(position) {
                    types.push(v.clone());
                }
            }
        }
    }
    let mut out: Vec<TAtomic> = Vec::new();
    for t in &types {
        for atomic in t.types.iter() {
            if matches!(atomic, TAtomic::Mixed(_)) {
                return None;
            }
            push_unique(&mut out, general(atomic));
        }
    }
    (!out.is_empty()).then_some(out)
}

/// Whether the item at `position` is a variable that an earlier item assigns.
fn reads_assigned_item(elements: Vec<&ArrayElement<'_>>, position: usize) -> bool {
    let Some(ArrayElement::Value(item)) = elements.get(position) else {
        return false;
    };
    let Expression::Variable(Variable::Direct(read)) = unparenthesize(item.value) else {
        return false;
    };
    elements[..position].iter().any(|e| {
        matches!(e, ArrayElement::Value(v) if matches!(unparenthesize(v.value),
            Expression::Assignment(a) if is_variable(a.lhs, read.name)))
    })
}

/// `ConstantArrayType::getValueTypes()` of a constant array (an array shape), in order; `None` for other types.
fn constant_array_values(atomic: &TAtomic) -> Option<Vec<TUnion>> {
    match atomic {
        TAtomic::Array(TArray::List(l)) if l.element_type.is_never() => {
            Some(l.known_elements.as_ref().map_or_else(Vec::new, |k| k.values().map(|(_, v)| v.clone()).collect()))
        }
        TAtomic::Array(TArray::Keyed(k)) if k.parameters.is_none() => {
            Some(k.known_items.as_ref().map_or_else(Vec::new, |k| k.values().map(|(_, v)| v.clone()).collect()))
        }
        _ => None,
    }
}

/// `TypeFactory::uniquateTypes` without constants: a literal value is its general type (`true` and `false` are
/// `bool`).
fn general(atomic: &TAtomic) -> TAtomic {
    match atomic {
        TAtomic::Scalar(s) if s.is_int() => TAtomic::Scalar(TScalar::int()),
        TAtomic::Scalar(s) if s.is_float() => TAtomic::Scalar(TScalar::float()),
        TAtomic::Scalar(s) if s.is_any_string() => TAtomic::Scalar(TScalar::string()),
        TAtomic::Scalar(s) if s.is_bool() => TAtomic::Scalar(TScalar::bool()),
        other => other.clone(),
    }
}

/// Add `atomic` unless an equal type is in already: an object of the same class, or an array (`unwrapConstantArrayTypes`
/// makes every array an `array`).
fn push_unique(types: &mut Vec<TAtomic>, atomic: TAtomic) {
    let class = |a: &TAtomic| match a {
        TAtomic::Object(TObject::Named(n)) => Some(n.name.as_bytes().to_ascii_lowercase()),
        _ => None,
    };
    let seen = types.iter().any(|t| {
        t == &atomic || matches!(t, TAtomic::Array(_)) && matches!(atomic, TAtomic::Array(_))
            || class(t).is_some() && class(t) == class(&atomic)
    });
    if !seen {
        types.push(atomic);
    }
}
