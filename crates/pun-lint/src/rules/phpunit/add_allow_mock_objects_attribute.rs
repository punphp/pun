//! `lint/phpunit/add-allow-mock-objects-attribute`, ported from rector-phpunit's
//! `AllowMockObjectsWhereParentClassRector` (rules/PHPUnit120/Rector/Class_).
//!
//! A test class that is a Symfony `TypeTestCase` gets `#[\PHPUnit\Framework\Attributes\AllowMockObjectsWithoutExpectations]`,
//! when that attribute class exists and the class has no such attribute yet.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::annotations::Annotated;
use super::{class_is, is_in_test_class};
use crate::rules::rector_batch5::resolve_class_name;
use crate::rules::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

/// `PHPUnitAttribute::ALLOW_MOCK_OBJECTS_WITHOUT_EXPECTATIONS`.
pub(super) const ALLOW_MOCK_OBJECTS: &str =
    "PHPUnit\\Framework\\Attributes\\AllowMockObjectsWithoutExpectations";
/// `PhpVersionFeature::ATTRIBUTES`.
pub(super) const ATTRIBUTES: u32 = 80000;

pub struct AddAllowMockObjectsAttribute;

impl Rule for AddAllowMockObjectsAttribute {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-allow-mock-objects-attribute"
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
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !should_add(ctx, &scope, node)
            || !class_is(
                ctx,
                &scope,
                &["Symfony\\Component\\Form\\Test\\TypeTestCase"],
            )
        {
            return;
        }
        add_attribute(ctx, ancestors, node, out);
    }
}

/// `shouldSkipClass()` negated: a test class, the attribute class exists, and the class has no such attribute.
pub(super) fn should_add(ctx: &Context<'_>, scope: &[Node<'_, '_>], node: Node<'_, '_>) -> bool {
    // the class's own attribute lists, as `has_attribute()` reads them
    let filled = node.children().into_iter().any(|child| {
        matches!(child, Node::AttributeList(list) if list.attributes.iter().any(|a| {
            resolve_class_name(scope, &a.name).eq_ignore_ascii_case(ALLOW_MOCK_OBJECTS)
        }))
    });
    is_in_test_class(ctx, scope, None)
        // attribute must exist for the rule to work
        && facts(ctx)
            .codebase
            .get_class_like(ALLOW_MOCK_OBJECTS.as_bytes())
            .is_some()
        // already filled
        && !filled
}

/// Report the class with the attribute added (`$node->attrGroups[] = ...`).
pub(super) fn add_attribute(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    node: Node<'_, '_>,
    out: &mut Vec<Report>,
) {
    let Some(edits) = Annotated::of(ctx.source, node, ancestors)
        .and_then(|a| a.edits(ctx.source, None, &[format!("#[\\{ALLOW_MOCK_OBJECTS}]")]))
    else {
        return;
    };
    out.push(Report::fixed(
        node_span(&node),
        "Allow mock objects without expectations in this test class.",
        Safety::Safe,
        edits,
    ));
}
