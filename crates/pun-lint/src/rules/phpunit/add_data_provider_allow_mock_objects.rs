//! `lint/phpunit/add-data-provider-allow-mock-objects`, ported from rector-phpunit's
//! `AllowMockObjectsForDataProviderRector` (rules/PHPUnit120/Rector/Class_), with
//! `MockObjectExprDetector::hasMethodCallWithoutExpects()`.
//!
//! A test class with a test method that has a `#[DataProvider]` attribute and calls `->method()` (outside closures)
//! on anything but a method call gets `#[\PHPUnit\Framework\Attributes\AllowMockObjectsWithoutExpectations]`, when
//! that attribute class exists and the class has no such attribute yet.

use mago_syntax::cst::{ClassLikeMember, Expression, MethodBody, Node, NodeKind};

use super::add_allow_mock_objects_attribute::{ATTRIBUTES, add_attribute, should_add};
use super::annotations::named_tags;
use super::no_bare_mock_assign::scoped;
use super::{AnyCall, class_body, is_public};
use crate::rules::lint_b_doc::Doc;
use crate::rules::rector_batch5::has_attribute;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct AddDataProviderAllowMockObjects;

impl Rule for AddDataProviderAllowMockObjects {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-data-provider-allow-mock-objects"
    }

    fn min_php_version(&self) -> u32 {
        ATTRIBUTES
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
        if !should_add(ctx, &scope, node) {
            return;
        }
        // has a test method, that contains a ->method() call without expects() and has a data provider attribute?
        let found = members.iter().any(|m| {
            let ClassLikeMember::Method(method) = m else {
                return false;
            };
            let is_test = is_public(method)
                && (method.name.value.starts_with(b"test")
                    || has_attribute(
                        &scope,
                        &method.attribute_lists,
                        "PHPUnit\\Framework\\Attributes\\Test",
                    )
                    || Doc::of(ctx.source, &scope, node_span(method).start).is_some_and(|doc| {
                        named_tags(ctx, &scope, &doc, "test").is_some_and(|t| !t.is_empty())
                    }));
            if !is_test
                || !has_attribute(
                    &scope,
                    &method.attribute_lists,
                    "PHPUnit\\Framework\\Attributes\\DataProvider",
                )
            {
                return false;
            }
            let MethodBody::Concrete(body) = &method.body else {
                return false;
            };
            let roots: Vec<Node<'_, '_>> = body.statements.iter().map(Node::Statement).collect();
            let mut without_expects = false;
            scoped(&roots, &mut |n| {
                if let Node::MethodCall(call) = n
                    && AnyCall::of(n).is_some_and(|c| c.is_name(&["method"]))
                    && !matches!(
                        unparenthesize(call.object),
                        Expression::Call(mago_syntax::cst::Call::Method(_))
                    )
                {
                    without_expects = true;
                }
            });
            without_expects
        });
        if found {
            add_attribute(ctx, ancestors, node, out);
        }
    }
}
