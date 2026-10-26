//! `lint/phpunit/no-empty-test`, ported from rector-phpunit's `RemoveEmptyTestMethodRector`
//! (rules/CodeQuality/Rector/ClassMethod).
//!
//! A method of a test class whose name starts with `test` and whose body holds no statement (php-parser keeps a
//! comment in the body as a `Nop` statement) is removed.

use mago_syntax::cst::{ClassLikeMember, MethodBody, Node, NodeKind};
use pun_problem::Safety;

use super::{CLASS_LIKES, class_body, is_in_test_class, remove_class_members, report};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoEmptyTest;

impl Rule for NoEmptyTest {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-empty-test"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(CLASS_LIKES)
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
        let removed: Vec<bool> = members
            .iter()
            .map(|member| {
                let ClassLikeMember::Method(method) = member else {
                    return false;
                };
                let MethodBody::Concrete(block) = &method.body else {
                    return false;
                };
                let inner = &ctx.source
                    [block.left_brace.end.offset as usize..block.right_brace.start.offset as usize];
                method.name.value.starts_with(b"test")
                    && block.statements.is_empty()
                    && inner.trim().is_empty()
                    && is_in_test_class(ctx, &scope, Some(method))
            })
            .collect();
        for (at, member) in members.iter().enumerate() {
            if let (true, ClassLikeMember::Method(method)) = (removed[at], member) {
                let mut only = vec![false; removed.len()];
                only[at] = true;
                out.push(report(
                    node_span(&method.name),
                    "Remove the empty test method.",
                    Safety::Safe,
                    remove_class_members(ctx.source, node, &only),
                ));
            }
        }
    }
}
