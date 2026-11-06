//! `lint/phpunit/no-unused-mock-property`, ported from rector-phpunit's `RemoveNeverUsedMockPropertyRector`
//! (rules/CodeQuality/Rector/Class_), with `MockObjectPropertyDetector::collectFromClassMethod` and
//! `PropertyFetchFinder::findLocalPropertyFetchesByName`.
//!
//! In a test class whose name does not end in `TestCase`, a property that `setUp()` assigns `createMock()` to, and
//! that every other fetch of only calls methods on, goes: the property, its assignment in `setUp()`, and each
//! statement of a method that is a call chain on it.
//!
//! Rector also keeps a property that a used trait fetches, which it reads from the trait's file. Pun reads the traits
//! of the same file; a trait from another file keeps every property (tests/rector/RemoveNeverUsedMockPropertyRector/Pun).

use mago_syntax::cst::{
    Access, Call, ClassLikeMember, ClassLikeMemberSelector, Expression, MethodBody, Node, NodeKind,
    Property, Statement, Variable,
};
use pun_problem::Safety;

use super::no_bare_mock_assign::scoped;
use super::{
    AnyCall, class_body, is_in_test_class, object_class, own_class, remove_class_members, resolve,
};
use crate::rules::rector_batch5::remove_statements;
use crate::rules::rector_stmts::{assign, block_list, expression};
use crate::rules::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoUnusedMockProperty;

impl Rule for NoUnusedMockProperty {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-unused-mock-property"
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
        let (members, _, _) = class_body(node).unwrap();
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        // skip abstract test case classes, as most likely extended and mock used in child classes
        let class = own_class(&scope);
        if class.as_deref().is_some_and(|c| c.ends_with("TestCase")) {
            return;
        }
        let methods: Vec<_> = members
            .iter()
            .filter_map(|m| match m {
                ClassLikeMember::Method(m) => Some(m),
                _ => None,
            })
            .collect();
        let Some(set_up) = methods
            .iter()
            .find(|m| m.name.value.eq_ignore_ascii_case(b"setUp"))
        else {
            return;
        };
        let MethodBody::Concrete(set_up_body) = &set_up.body else {
            return;
        };
        let set_up_list = block_list(ctx.source, set_up_body);
        let names = collect_from_class_method(&set_up_list.stmts);
        let method_roots: Vec<Node<'_, '_>> = methods.iter().map(|m| Node::Method(m)).collect();
        let to_remove: Vec<&[u8]> = names
            .into_iter()
            .filter(|name| {
                let all = local_property_fetches(ctx, &scope, members.iter(), name);
                let mut calls = 0;
                scoped(&method_roots, &mut |n| {
                    if let Node::MethodCall(call) = n
                        && property_fetch_name(call.object)
                            .is_some_and(|p| p.eq_ignore_ascii_case(name))
                    {
                        calls += 1;
                    }
                });
                // -1 for the assign in setUp() method
                all.saturating_sub(1) == calls
                    && all > 0
                    && !is_property_used_in_trait(ctx, &scope, members.iter(), name)
            })
            .collect();
        if to_remove.is_empty() {
            return;
        }
        // 1. remove property
        let removed_members: Vec<bool> = members
            .iter()
            .map(|m| {
                let variable = match m {
                    ClassLikeMember::Property(Property::Plain(p)) => {
                        p.items.first().map(|i| i.variable())
                    }
                    ClassLikeMember::Property(Property::Hooked(p)) => Some(p.item.variable()),
                    _ => None,
                };
                variable.is_some_and(|v| {
                    to_remove
                        .iter()
                        .any(|n| v.name[1..].eq_ignore_ascii_case(n))
                })
            })
            .collect();
        let mut edits = Vec::new();
        if removed_members.contains(&true) {
            edits.extend(remove_class_members(ctx.source, node, &removed_members));
        }
        for method in &methods {
            let MethodBody::Concrete(body) = &method.body else {
                continue;
            };
            let list = block_list(ctx.source, body);
            let mut removed = vec![false; list.stmts.len()];
            let is_set_up = std::ptr::eq(*method, *set_up);
            for name in &to_remove {
                // 2. remove assign from setUp()
                if is_set_up
                    && let Some(at) = list.stmts.iter().enumerate().position(|(at, s)| {
                        !removed[at]
                            && expression(s).and_then(assign).is_some_and(|a| {
                                property_fetch_name(a.lhs)
                                    .is_some_and(|p| p.eq_ignore_ascii_case(name))
                            })
                    })
                {
                    removed[at] = true;
                }
                // 3. remove expression method calls on this property
                for (at, statement) in list.stmts.iter().enumerate() {
                    if let Some(Expression::Call(Call::Method(call))) = expression(statement) {
                        let mut current = call;
                        while let Expression::Call(Call::Method(inner)) = current.object {
                            current = inner;
                        }
                        if property_fetch_name(current.object)
                            .is_some_and(|p| p.eq_ignore_ascii_case(name))
                        {
                            removed[at] = true;
                        }
                    }
                }
            }
            if removed.contains(&true) {
                edits.extend(remove_statements(ctx, &list, &removed));
            }
        }
        out.push(Report::fixed(
            node_span(&node),
            "Remove the mock property that only gets expectations and is never used.",
            Safety::Safe,
            edits,
        ));
    }
}

/// `MockObjectPropertyDetector::collectFromClassMethod`: the names of the properties `setUp()` assigns a
/// `createMock()` call to (on any object, by a method or a static call), in the order of their first assignment.
fn collect_from_class_method<'a>(stmts: &[&'a Statement<'a>]) -> Vec<&'a [u8]> {
    let mut names: Vec<&[u8]> = Vec::new();
    for statement in stmts {
        let Some(assign) = expression(statement).and_then(assign) else {
            continue;
        };
        let Some(name) = property_fetch_name(assign.lhs) else {
            continue;
        };
        let call = match unparenthesize(assign.rhs) {
            Expression::Call(Call::Method(c)) => AnyCall::of(Node::MethodCall(c)),
            Expression::Call(Call::StaticMethod(c)) => AnyCall::of(Node::StaticMethodCall(c)),
            _ => None,
        };
        if !call.is_some_and(|c| c.is_name(&["createMock"])) {
            continue;
        }
        if !names.iter().any(|n| *n == name) {
            names.push(name);
        }
    }
    names
}

/// The name of a `PropertyFetch` (`$any->name`, not `?->`) with an identifier name.
pub(super) fn property_fetch_name<'a>(expr: &'a Expression<'a>) -> Option<&'a [u8]> {
    match unparenthesize(expr) {
        Expression::Access(Access::Property(a)) => match &a.property {
            ClassLikeMemberSelector::Identifier(id) => Some(id.value),
            _ => None,
        },
        _ => None,
    }
}

/// The number of `findLocalPropertyFetchesByName($class, $name)`: in the hooked properties and the methods, every
/// property fetch (`->`, `?->` or `::$`) of `name` whose object is `$this`, or has the class's own type.
pub(super) fn local_property_fetches<'a>(
    ctx: &Context<'_>,
    scope: &[Node<'_, '_>],
    members: impl Iterator<Item = &'a ClassLikeMember<'a>>,
    name: &[u8],
) -> usize {
    let own = own_class(scope);
    let is_local = |object: &Expression<'_>| match unparenthesize(object) {
        Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => true,
        other => own.as_ref().is_some_and(|own| {
            object_class(ctx, other).is_some_and(|c| c.eq_ignore_ascii_case(own))
        }),
    };
    let mut count = 0;
    let mut stack: Vec<Node<'_, '_>> = members
        .filter_map(|m| match m {
            ClassLikeMember::Method(m) => Some(Node::Method(m)),
            ClassLikeMember::Property(p @ Property::Hooked(_)) => Some(Node::Property(p)),
            _ => None,
        })
        .collect();
    stack.reverse();
    while let Some(node) = stack.pop() {
        let found = match node {
            Node::PropertyAccess(a) => {
                matches!(&a.property, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name))
                    && is_local(a.object)
            }
            Node::NullSafePropertyAccess(a) => {
                matches!(&a.property, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name))
                    && is_local(a.object)
            }
            Node::StaticPropertyAccess(a) => {
                matches!(&a.property, Variable::Direct(v) if v.name[1..].eq_ignore_ascii_case(name))
                    && match unparenthesize(a.class) {
                        Expression::Self_(_) | Expression::Static(_) => true,
                        Expression::Identifier(id) => own
                            .as_ref()
                            .is_some_and(|own| resolve(scope, id).eq_ignore_ascii_case(own)),
                        other => is_local(other),
                    }
            }
            _ => false,
        };
        if found {
            count += 1;
        }
        let mut children = Vec::new();
        node.visit_children(|child| children.push(child));
        stack.extend(children.into_iter().rev());
    }
    count
}

/// `isPropertyUsedInTrait`: a trait the class uses fetches the property (`->name` on anything). A trait declared in
/// another file counts as fetching it, as Pun does not read it; a name that is no trait does not.
fn is_property_used_in_trait<'a>(
    ctx: &Context<'_>,
    scope: &[Node<'_, '_>],
    members: impl Iterator<Item = &'a ClassLikeMember<'a>>,
    name: &[u8],
) -> bool {
    let Some(traits) = traits_in_file(ctx, scope, members) else {
        return true;
    };
    traits.into_iter().any(|declared| {
        let mut stack = vec![declared];
        while let Some(node) = stack.pop() {
            if let Node::PropertyAccess(a) = node
                && matches!(&a.property, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name))
            {
                return true;
            }
            node.visit_children(|child| stack.push(child));
        }
        false
    })
}

/// The declarations of the traits the class uses (`use A, B;` among `members`, names that are no trait left out),
/// which must all be in this file: `None` when one is not, as Pun does not read other files.
pub(super) fn traits_in_file<'n, 'a>(
    ctx: &Context<'_>,
    scope: &[Node<'n, 'n>],
    members: impl Iterator<Item = &'a ClassLikeMember<'a>>,
) -> Option<Vec<Node<'n, 'n>>> {
    let facts = facts(ctx);
    let mut traits = Vec::new();
    for member in members {
        let ClassLikeMember::TraitUse(trait_use) = member else {
            continue;
        };
        for id in trait_use.trait_names.iter() {
            let trait_name = resolve(scope, id);
            let Some(metadata) = facts
                .codebase
                .get_class_like(trait_name.as_bytes())
                .filter(|c| c.kind.is_trait())
            else {
                continue;
            };
            facts.text(metadata.span)?;
            // the file's root, whose descendants are its traits
            let mut stack: Vec<Node<'n, 'n>> = scope.first().copied().into_iter().collect();
            let mut declared = None;
            while let Some(node) = stack.pop() {
                if matches!(node, Node::Trait(_))
                    && node_span(&node).start == metadata.span.start.offset as usize
                {
                    declared = Some(node);
                    break;
                }
                node.visit_children(|child| stack.push(child));
            }
            traits.push(declared?);
        }
    }
    Some(traits)
}
