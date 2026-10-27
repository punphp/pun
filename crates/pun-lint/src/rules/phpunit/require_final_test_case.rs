//! `lint/phpunit/require-final-test-case`, ported from Rector 2.6.1's `FinalizeTestCaseClassRector`
//! (rules/Privatization/Rector/Class_).
//!
//! A class that is neither abstract nor final, whose name ends in `Test` (not `TestCase`) and that is a
//! `PHPUnit\Framework\TestCase`, becomes final.

use mago_syntax::cst::{Modifier, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{TEST_CASE, class_is, report};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct RequireFinalTestCase;

impl Rule for RequireFinalTestCase {
    fn id(&self) -> &'static str {
        "lint/phpunit/require-final-test-case"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Class(class) = node else { return };
        // skip obvious cases
        if class
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Abstract(_) | Modifier::Final(_)))
        {
            return;
        }
        let name = class.name.value;
        // skip classes without "Test" suffix, as those are meant to be extended
        if name.ends_with(b"TestCase") || !name.ends_with(b"Test") {
            return;
        }
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !class_is(ctx, &scope, &[TEST_CASE]) {
            return;
        }
        // `Modifiers::toString()` writes `final` before `readonly`.
        let at = class
            .modifiers
            .first()
            .map_or(class.class.span.start.offset as usize, |m| {
                node_span(m).start
            });
        out.push(report(
            node_span(&class.name),
            "Make the test case final.",
            Safety::Safe,
            vec![Edit::new(Span::new(at, at), "final ")],
        ));
    }
}
