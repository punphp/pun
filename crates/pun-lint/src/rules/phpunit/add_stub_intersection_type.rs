//! `lint/phpunit/add-stub-intersection-type`, ported from rector-phpunit's `AddStubIntersectionVarToStubPropertyRector`
//! (rules/CodeQuality/Rector/Class_), with `MockObjectPropertyDetector`.
//!
//! In a test class, a property typed `PHPUnit\Framework\MockObject\Stub` gets the `@var` type
//! `\PHPUnit\Framework\MockObject\Stub&\X` when `setUp()` assigns it `createStub(X::class)`, or else
//! `\PHPUnit\Framework\MockObject\Stub&Y` when its first `@var` tag has a single name `Y` other than `Stub` and
//! `MockObject`; not when that tag has an intersection type already. Rector runs it only for PHPUnit 11 and later; Pun
//! leaves that to the rule's group.

use mago_syntax::cst::{ClassLikeMember, Node, NodeKind};
use pun_problem::Safety;

use super::add_mock_intersection_type::{created_properties, is_typed, mocked_class, set_var_type};
use super::phpdoc_type::{TagType, short_name};
use super::{class_body, is_in_test_class};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

/// `PHPUnitClassName::STUB`.
const STUB: &str = "PHPUnit\\Framework\\MockObject\\Stub";

pub struct AddStubIntersectionType;

impl Rule for AddStubIntersectionType {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-stub-intersection-type"
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
        let calls = created_properties(ctx, members.iter(), "createStub").unwrap_or_default();
        let mut edits = Vec::new();
        let mut first = None;
        for member in members {
            let ClassLikeMember::Property(property) = member else {
                continue;
            };
            // only properties typed as a native Stub
            if !is_typed(&scope, property, STUB) {
                continue;
            }
            let name = &property.variables()[0].name[1..];
            // 1. prefer the stubbed class from a setUp() createStub() call
            let stubbed = calls
                .iter()
                .find(|(n, _)| *n == name)
                .and_then(|(_, call)| mocked_class(&scope, call));
            let resolved = set_var_type(ctx, &scope, property, |tag| match (stubbed, tag) {
                (Some(class), _) => Some(format!("\\{STUB}&\\{class}")),
                // 2. fall back to a bare single-class @var docblock
                (None, Some(TagType::Name(name)))
                    if !["Stub", "MockObject"].contains(&short_name(name)) =>
                {
                    Some(format!("\\{STUB}&{name}"))
                }
                _ => None,
            });
            if let Some(e) = resolved {
                first.get_or_insert(node_span(member));
                edits.extend(e);
            }
        }
        let Some(first) = first else {
            return;
        };
        out.push(Report::fixed(
            first,
            "Add the stubbed class to the stub property's @var as an intersection.",
            Safety::Safe,
            edits,
        ));
    }
}
