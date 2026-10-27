//! `lint/phpunit/no-final-constructor-override`, ported from rector-phpunit's
//! `RemoveOverrideFinalConstructTestCaseRector` (rules/PHPUnit120/Rector/Class_).
//!
//! PHPUnit 12 made `TestCase::__construct()` final, so the first constructor of a test class is removed.

use mago_syntax::cst::{ClassLikeMember, Node, NodeKind};
use pun_problem::Safety;

use super::{class_body, is_in_test_class, remove_class_members, report};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoFinalConstructorOverride;

impl Rule for NoFinalConstructorOverride {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-final-constructor-override"
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
        let Some((at, method)) = members
            .iter()
            .enumerate()
            .find_map(|(at, member)| match member {
                ClassLikeMember::Method(m) if m.name.value.eq_ignore_ascii_case(b"__construct") => {
                    Some((at, m))
                }
                _ => None,
            })
        else {
            return;
        };
        let mut only = vec![false; members.len()];
        only[at] = true;
        out.push(report(
            node_span(&method.name),
            "Remove the constructor: `TestCase::__construct()` is final since PHPUnit 12.",
            Safety::Unsafe,
            remove_class_members(ctx.source, node, &only),
        ));
    }
}
