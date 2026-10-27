//! `lint/phpunit/prefer-mock-intersection-return`, ported from rector-phpunit's
//! `ChangeMockObjectReturnUnionToIntersectionRector` (rules/CodeQuality/Rector/ClassMethod).
//!
//! In a test class, a method whose first `@return` tag has a union type with `MockObject` or `Stub` (short, or fully
//! qualified with or without the leading `\`) among its names gets the intersection of the same types.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::Safety;

use super::annotations::Annotated;
use super::phpdoc_type::{TagType, typed_tag, with_tag_type};
use super::{is_in_test_class, report};
use crate::{Context, Report, Rule};

pub struct PreferMockIntersectionReturn;

impl Rule for PreferMockIntersectionReturn {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-mock-intersection-return"
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
        if !is_in_test_class(ctx, ancestors, Some(method)) {
            return;
        }
        let Some(annotated) = Annotated::of(ctx.source, node, ancestors) else {
            return;
        };
        let Some(doc) = annotated.doc.as_ref() else {
            return;
        };
        let Ok(Some(tag)) = typed_tag(doc, "@return", None) else {
            return;
        };
        let TagType::Union(members) = &tag.ty else {
            return;
        };
        // must contain a MockObject or Stub member to be a mock union
        let is_mock = |m: &&str| {
            [
                "PHPUnit\\Framework\\MockObject\\MockObject",
                "MockObject",
                "PHPUnit\\Framework\\MockObject\\Stub",
                "Stub",
            ]
            .contains(&m.trim_start_matches('\\'))
        };
        if !members.iter().any(is_mock) {
            return;
        }
        let new_doc = with_tag_type(Some(doc), "@return", None, &members.join("&")).unwrap();
        let Some(edits) = annotated.edits(ctx.source, Some(&new_doc), &[]) else {
            return;
        };
        out.push(report(
            doc.span,
            "Return the mock as an intersection type, not a union.",
            Safety::Safe,
            edits,
        ));
    }
}
