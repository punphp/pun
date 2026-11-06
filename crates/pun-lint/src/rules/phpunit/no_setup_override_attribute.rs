//! `lint/phpunit/no-setup-override-attribute`, ported from rector-phpunit's `NoSetupWithParentCallOverrideRector`
//! (rules/CodeQuality/Rector/ClassMethod).
//!
//! A test class's `setUp()` or `tearDown()` that calls its parent's (`ParentCallDetector::hasParentCall`) loses its
//! `#[Override]` attributes: an attribute group left empty goes, and with no group left php-parser prints the method
//! anew.

use mago_syntax::cst::{Expression, MethodBody, Node, NodeKind, Statement};
use pun_problem::{Edit, Safety};

use super::{is_in_test_class, method_anew, resolve};
use crate::rules::rector_batch5::{has_attribute, list_removal, merge};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoSetupOverrideAttribute;

impl Rule for NoSetupOverrideAttribute {
    fn id(&self) -> &'static str {
        "lint/phpunit/no-setup-override-attribute"
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
        let name = method.name.value;
        if !(name.eq_ignore_ascii_case(b"setUp") || name.eq_ignore_ascii_case(b"tearDown")) {
            return;
        }
        let MethodBody::Concrete(block) = &method.body else {
            return;
        };
        if !is_in_test_class(ctx, ancestors, Some(method)) {
            return;
        }
        let has_parent_call = block.statements.iter().any(|statement| {
            let Statement::Expression(statement) = statement else {
                return false;
            };
            let Expression::Call(mago_syntax::cst::Call::StaticMethod(call)) = statement.expression
            else {
                return false;
            };
            matches!(call.class, Expression::Parent(_))
                && matches!(&call.method, mago_syntax::cst::ClassLikeMemberSelector::Identifier(id) if id.value.eq_ignore_ascii_case(name))
        });
        if !has_parent_call || !has_attribute(ancestors, &method.attribute_lists, "Override") {
            return;
        }
        let lists: Vec<_> = method.attribute_lists.iter().collect();
        let removed: Vec<Vec<bool>> = lists
            .iter()
            .map(|list| {
                list.attributes
                    .iter()
                    .map(|a| resolve(ancestors, &a.name).eq_ignore_ascii_case("Override"))
                    .collect()
            })
            .collect();
        let empty: Vec<bool> = removed.iter().map(|r| !r.contains(&false)).collect();
        let list_spans: Vec<_> = lists.iter().map(|l| node_span(*l)).collect();
        let edits = match list_removal(&list_spans, &empty) {
            Some(mut edits) => {
                for (at, list) in lists.iter().enumerate() {
                    if !empty[at] {
                        let items: Vec<_> = list.attributes.iter().map(node_span).collect();
                        edits.extend(list_removal(&items, &removed[at]).unwrap_or_default());
                    }
                }
                merge(edits)
            }
            None => vec![Edit::new(
                node_span(method),
                method_anew(ctx, method, block),
            )],
        };
        out.push(Report::fixed(
            node_span(&method.name),
            "Remove `#[Override]`: the method calls its parent's.",
            Safety::Safe,
            edits,
        ));
    }
}
