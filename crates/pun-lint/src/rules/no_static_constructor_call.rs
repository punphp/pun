//! `lint/classes/no-static-constructor-call`, ported from phpstan-strict-rules 2.0.12's
//! `IllegalConstructorStaticCallRule` (identifier `constructor.call`).
//!
//! `Foo::__construct()` (the method name in any case) is reported unless it is in a method named exactly
//! `__construct` (a closure or arrow function inside counts as the method) of a class, and `Foo` (or `parent`) names a
//! class the class extends, directly or not. A trait is not analysed on its own upstream, so Pun leaves its methods
//! alone. Where Pun cannot follow the parents to the top (a class it does not know, such as one from vendor/, which
//! PHPStan reads), any class name but `self`, `static` and the class's own is allowed.

use mago_syntax::cst::{ClassLikeMemberSelector, Expression, Node, NodeKind};

use super::rector_batch5::resolve_class_name;
use super::types::class_scope;
use super::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoStaticConstructorCall;

impl Rule for NoStaticConstructorCall {
    fn id(&self) -> &'static str {
        "lint/classes/no-static-constructor-call"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::StaticMethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::StaticMethodCall(call) = node else {
            return;
        };
        let ClassLikeMemberSelector::Identifier(name) = &call.method else {
            return;
        };
        if !name.value.eq_ignore_ascii_case(b"__construct")
            || is_calling_parent_constructor(ctx, ancestors, call.class)
        {
            return;
        }
        out.push(Report {
            span: node_span(&node),
            message:
                "Static call to __construct() is only allowed on a parent class in the constructor."
                    .to_string(),
            help: None,
            fix: None,
        });
    }
}

/// `isCollectCallingConstructor`, and whether the call is in a trait, which Pun leaves alone.
fn is_calling_parent_constructor(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    class: &Expression<'_>,
) -> bool {
    // `$scope->getFunction()`: closures and arrow functions keep the function they are in.
    let function = ancestors.iter().rev().find_map(|a| match a {
        Node::Method(m) => Some(m.name.value),
        Node::Function(f) => Some(f.name.value),
        _ => None,
    });
    if function != Some(b"__construct") {
        return false;
    }
    let class_like = ancestors.iter().rev().find(|a| {
        matches!(
            a,
            Node::Class(_)
                | Node::AnonymousClass(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_)
        )
    });
    if matches!(class_like, Some(Node::Trait(_))) {
        return true;
    }
    let Some(scope) = class_scope(ancestors) else {
        return false;
    };
    let called = match class {
        Expression::Parent(_) => match &scope.parent_class {
            Some(parent) => parent.clone(),
            None => return false,
        },
        Expression::Identifier(id) => resolve_class_name(ancestors, id),
        _ => return false,
    };
    // `getParentClassesNames()`, as far as the codebase knows them.
    let codebase = &facts(ctx).codebase;
    let mut next = scope.parent_class;
    while let Some(parent) = next {
        if parent.eq_ignore_ascii_case(&called) {
            return true;
        }
        let Some(class) = codebase.get_class_like(parent.as_bytes()) else {
            // ponytail: an unknown class's own parents are unknown, so any other name may be one of them.
            return scope
                .name
                .is_none_or(|own| !own.eq_ignore_ascii_case(&called));
        };
        next = class
            .direct_parent_class
            .as_ref()
            .map(|p| String::from_utf8_lossy(p.as_ref()).into_owned());
    }
    false
}
