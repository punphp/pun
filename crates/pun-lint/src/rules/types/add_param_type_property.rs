//! `lint/types/add-param-type-property`, ported from Rector 2.6.1's `AddParamTypeFromPropertyTypeRector`
//! (rules/TypeDeclaration/Rector/ClassMethod) with `PropertyFetchAnalyzer::isVariableAssignToThisPropertyFetch` and
//! `ParamTypeAddGuard`: an untyped parameter of a method that the method assigns to properties of its own class
//! (`$this->name = $name`) gets the union of those properties' native types, unless the method overrides a parent's.
//!
//! Known difference on laravel-framework: Rector writes `array|null|bool|int|float|string $value = true` on
//! `Connection::recordsHaveBeenModified()`, which assigns `$value` to an untyped property inside
//! `if (! $this->recordsModified)`: PHPStan's type of the property there is its falsy values (`0|0.0|''|'0'|array{}|
//! false|null`), generalized, where Pun reads the property's missing native type and skips it.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    AssignmentOperator, ClassLikeMemberSelector, Expression, MethodBody, Modifier, Node, NodeKind,
    Variable,
};

use super::facts::{Facts, facts};
use super::has_parent_class_method;
use super::param::{add_hint, is_mixed, param_declaration, report};
use super::class_scope;
use crate::rules::class_method::qualified;
use crate::rules::add_param_type_sprintf::is_legal;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `PhpVersionFeature::TYPED_PROPERTIES`.
const TYPED_PROPERTIES: u32 = 70400;

pub struct AddParamTypeProperty;

impl Rule for AddParamTypeProperty {
    fn id(&self) -> &'static str {
        "lint/types/add-param-type-property"
    }

    fn min_php_version(&self) -> u32 {
        TYPED_PROPERTIES
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
        let Node::Method(method) = node else { return };
        let MethodBody::Concrete(body) = &method.body else { return };
        let facts = facts(ctx);
        let Some(class) = own_class(&facts, ancestors) else { return };
        let mut edits = Vec::new();
        for param in method.parameter_list.parameters.iter() {
            if param.hint.is_some() || param.ellipsis.is_some() {
                continue;
            }
            if !is_legal(Node::Block(body), param.variable.name) {
                continue;
            }
            let Some(types) = property_types(&facts, &class, Node::Block(body), param.variable.name)
            else {
                continue;
            };
            let Some(declaration) = declaration(&facts, &types, ctx.php_version) else {
                continue;
            };
            let overrides = !method
                .modifiers
                .iter()
                .any(|m| matches!(m, Modifier::Private(_)))
                && class_scope(ancestors)
                    .is_some_and(|scope| has_parent_class_method(&facts, &scope, method.name.value));
            if overrides {
                return;
            }
            edits.push(add_hint(param, &declaration));
        }
        if edits.is_empty() {
            return;
        }
        out.push(report(
            node_span(&node),
            "Add the type of the property a parameter is assigned to.",
            edits,
        ));
    }
}

/// The name the codebase knows the class of `ancestors` by.
fn own_class(facts: &Facts, ancestors: &[Node<'_, '_>]) -> Option<String> {
    let at = ancestors.iter().rposition(|a| {
        matches!(
            a,
            Node::Class(_) | Node::AnonymousClass(_) | Node::Trait(_) | Node::Enum(_)
        )
    })?;
    match ancestors[at] {
        Node::AnonymousClass(c) => Some(facts.anonymous_class_name(mago_span::HasSpan::span(c))),
        Node::Class(c) => Some(qualified(&ancestors[..at], c.name.value)),
        // a trait's or an enum's method: Rector adds no type (checked with Rector 2.6.1)
        _ => None,
    }
}

/// The native types (`None` for `mixed`) of the properties `$name` is assigned to in `body`, outside nested classes
/// and functions (closures are looked into); `None` when it is assigned to none.
fn property_types(
    facts: &Facts,
    class: &str,
    body: Node<'_, '_>,
    name: &[u8],
) -> Option<Vec<Option<TUnion>>> {
    let mut types = Vec::new();
    walk(body, &mut |node| {
        let Node::Assignment(assign) = node else {
            return;
        };
        if !matches!(assign.operator, AssignmentOperator::Assign(_))
            || !matches!(unparenthesize(assign.rhs), Expression::Variable(Variable::Direct(v)) if v.name == name)
        {
            return;
        }
        let Some(property) = local_property(assign.lhs) else {
            return;
        };
        let declared = facts
            .codebase
            .get_declaring_property(class.as_bytes(), &property)
            .and_then(|p| p.type_declaration_metadata.as_ref());
        types.push(declared.map(|t| t.type_union.clone()));
    });
    (!types.is_empty()).then_some(types)
}

/// The property name (`$name`, as the codebase keys it) of `$this->name`, `self::$name` or `static::$name`.
fn local_property(expr: &Expression<'_>) -> Option<Vec<u8>> {
    match unparenthesize(expr) {
        Expression::Access(mago_syntax::cst::Access::Property(fetch))
            if matches!(unparenthesize(fetch.object), Expression::Variable(Variable::Direct(v)) if v.name == b"$this") =>
        {
            match &fetch.property {
                ClassLikeMemberSelector::Identifier(id) => Some([b"$", id.value].concat()),
                _ => None,
            }
        }
        Expression::Access(mago_syntax::cst::Access::StaticProperty(fetch))
            if matches!(fetch.class, Expression::Self_(_) | Expression::Static(_)) =>
        {
            match &fetch.property {
                Variable::Direct(v) => Some(v.name.to_vec()),
                _ => None,
            }
        }
        _ => None,
    }
}

/// `createMixedPassedOrUnionType` of the property types, as a parameter type; none when one is `mixed`, untyped or
/// declared so (Rector writes no type for a `mixed` property, checked with Rector 2.6.1).
fn declaration(facts: &Facts, types: &[Option<TUnion>], php_version: u32) -> Option<String> {
    let mut atomics: Vec<TAtomic> = Vec::new();
    for t in types {
        let t = t.as_ref()?;
        for a in t.types.iter() {
            if !atomics.contains(a) {
                atomics.push(a.clone());
            }
        }
    }
    let union = TUnion::from_vec(atomics);
    if is_mixed(&union) {
        return None;
    }
    param_declaration(&union, &facts.codebase, php_version)
}

/// Call `f` on every node under `node`, not entering classes and named functions.
fn walk<'ast, 'arena>(node: Node<'ast, 'arena>, f: &mut dyn FnMut(Node<'ast, 'arena>)) {
    node.visit_children(|child| match child {
        Node::Class(_)
        | Node::AnonymousClass(_)
        | Node::Interface(_)
        | Node::Trait(_)
        | Node::Enum(_)
        | Node::Function(_) => {}
        _ => {
            f(child);
            walk(child, f);
        }
    });
}
