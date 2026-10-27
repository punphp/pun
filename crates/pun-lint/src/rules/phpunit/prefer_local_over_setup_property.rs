//! `lint/phpunit/prefer-local-over-setup-property`, ported from rector-phpunit's
//! `NarrowUnusedSetUpDefinedPropertyRector` (rules/CodeQuality/Rector/Class_).
//!
//! In a test class with a `setUp()` method, a property declared alone, private (or in a final class), not static and
//! without a default, that no other method fetches as `$this->name`, becomes a local variable of `setUp()`: the
//! property goes and each `$this->name` in `setUp()` becomes `$name`. Not when a closure in `setUp()` fetches it, or
//! an arrow function does before an assignment to it has ended, or a used trait declares or fetches it.
//!
//! Rector reads the traits from their files; Pun reads the traits of the same file, and keeps every property of a
//! class that uses a trait from another file (tests/rector/NarrowUnusedSetUpDefinedPropertyRector/Pun).

use mago_syntax::cst::{
    Access, ClassLikeMember, ClassLikeMemberSelector, Expression, Method, Modifier, Node, NodeKind,
    Property, Variable,
};
use pun_problem::{Edit, Safety};

use super::no_unused_mock_property::traits_in_file;
use super::{class_body, is_in_test_class, remove_class_members, report};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferLocalOverSetupProperty;

impl Rule for PreferLocalOverSetupProperty {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-local-over-setup-property"
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
        let Some((members, _, _)) = class_body(node) else {
            return;
        };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        let methods: Vec<&Method<'_>> = members
            .iter()
            .filter_map(|m| match m {
                ClassLikeMember::Method(m) => Some(m),
                _ => None,
            })
            .collect();
        let Some(set_up) = methods
            .iter()
            .copied()
            .find(|m| m.name.value.eq_ignore_ascii_case(b"setUp"))
        else {
            return;
        };
        let is_final = matches!(node, Node::Class(c) if c.modifiers.iter().any(|m| matches!(m, Modifier::Final(_))));
        let traits = traits_in_file(ctx, &scope, members.iter());
        let mut removed = vec![false; members.len()];
        let mut edits = Vec::new();
        for (at, member) in members.iter().enumerate() {
            let ClassLikeMember::Property(property) = member else {
                continue;
            };
            let (modifiers, items) = match property {
                Property::Plain(p) => (&p.modifiers, p.items.iter().collect::<Vec<_>>()),
                Property::Hooked(p) => (&p.modifiers, vec![&p.item]),
            };
            if items.len() != 1 {
                continue;
            }
            let item = items[0];
            let name = &item.variable().name[1..];
            // shouldSkipProperty: possibly used by child, for caching or re-use, or by a trait
            if !is_final && !modifiers.iter().any(|m| matches!(m, Modifier::Private(_))) {
                continue;
            }
            if modifiers.iter().any(|m| matches!(m, Modifier::Static(_)))
                || matches!(item, mago_syntax::cst::PropertyItem::Concrete(_))
            {
                continue;
            }
            let Some(traits) = &traits else {
                continue;
            };
            if traits.iter().any(|t| trait_uses(*t, name)) {
                continue;
            }
            // isPropertyUsedOutsideSetUpClassMethod
            if methods
                .iter()
                .any(|m| !std::ptr::eq(*m, set_up) && !fetches(Node::Method(m), name).is_empty())
            {
                continue;
            }
            if is_used_in_unsafe_nested_function(set_up, name) {
                continue;
            }
            removed[at] = true;
            // change property to variable in setUp() method
            let variable = format!("${}", String::from_utf8_lossy(name));
            for span in fetches(Node::Method(set_up), name) {
                edits.push(Edit::new(span, variable.clone()));
            }
        }
        let Some(first) = removed.iter().position(|r| *r) else {
            return;
        };
        edits.extend(remove_class_members(ctx.source, node, &removed));
        out.push(report(
            node_span(members.iter().nth(first).unwrap()),
            "Use a local variable in setUp() for a property no test reads.",
            Safety::Safe,
            edits,
        ));
    }
}

/// Whether `expr` is `$this`.
fn is_this(expr: &Expression<'_>) -> bool {
    matches!(unparenthesize(expr), Expression::Variable(Variable::Direct(v)) if v.name.eq_ignore_ascii_case(b"$this"))
}

/// The spans of the `$this->name` fetches under `node`, however deep (an inner one is listed too).
fn fetches(node: Node<'_, '_>, name: &[u8]) -> Vec<pun_problem::Span> {
    let mut found = Vec::new();
    let mut stack = vec![node];
    while let Some(node) = stack.pop() {
        if let Node::PropertyAccess(a) = node
            && is_this(a.object)
            && matches!(&a.property, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name))
        {
            found.push(node_span(&node));
        }
        node.visit_children(|child| stack.push(child));
    }
    found
}

/// `isPropertyUsedInUnsafeNestedFunction()`.
fn is_used_in_unsafe_nested_function(set_up: &Method<'_>, name: &[u8]) -> bool {
    let assigned_ends: Vec<usize> = {
        let mut ends = Vec::new();
        let mut stack = vec![Node::Method(set_up)];
        while let Some(node) = stack.pop() {
            if let Node::Assignment(assign) = node
                && matches!(
                    assign.operator,
                    mago_syntax::cst::AssignmentOperator::Assign(_)
                )
                && let Expression::Access(Access::Property(fetch)) = assign.lhs
                && is_this(fetch.object)
                && matches!(&fetch.property, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name))
            {
                ends.push(node_span(&node).end);
            }
            node.visit_children(|child| stack.push(child));
        }
        ends
    };
    let mut stack = vec![Node::Method(set_up)];
    while let Some(node) = stack.pop() {
        match node {
            Node::Closure(_) if !fetches(node, name).is_empty() => return true,
            Node::ArrowFunction(_)
                if !fetches(node, name).is_empty()
                    && !assigned_ends
                        .iter()
                        .any(|end| *end <= node_span(&node).start) =>
            {
                return true;
            }
            _ => {}
        }
        node.visit_children(|child| stack.push(child));
    }
    false
}

/// `PropertyFetchAnalyzer::containsLocalPropertyFetchName()` on a trait: it declares the property, or fetches it on
/// `$this` (`->`; PHPStan has no type for `$this?->` there) or on `self` or `static` (`::$`).
fn trait_uses(declared: Node<'_, '_>, name: &[u8]) -> bool {
    let mut stack = vec![declared];
    while let Some(node) = stack.pop() {
        let found = match node {
            Node::Property(p) => p.variables().iter().any(|v| v.name[1..] == *name),
            Node::PropertyAccess(a) => {
                is_this(a.object)
                    && matches!(&a.property, ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name))
            }
            Node::StaticPropertyAccess(a) => {
                matches!(
                    unparenthesize(a.class),
                    Expression::Self_(_) | Expression::Static(_)
                ) && matches!(&a.property, Variable::Direct(v) if v.name[1..].eq_ignore_ascii_case(name))
            }
            _ => false,
        };
        if found {
            return true;
        }
        node.visit_children(|child| stack.push(child));
    }
    false
}
