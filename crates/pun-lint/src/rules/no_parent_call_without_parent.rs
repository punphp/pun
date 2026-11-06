//! `lint/dead-code/no-parent-call-without-parent`, ported from Rector 2.6.1's `RemoveParentCallWithoutParentRector`
//! (rules/DeadCode/Rector/StaticCall) and `ClassMethodManipulator::hasParentMethodOrInterfaceMethod`.
//!
//! In a named class's method, a statement `parent::name(...);` goes, and `$x = parent::name(...);` assigns `null`,
//! when the class extends nothing, or no class it extends has the method (or `__call`) and no interface it
//! implements declares it. A class whose parents Pun cannot follow to the top (one it does not know, such as one
//! from vendor/) is left alone, as Rector leaves a class whose parent or a further ancestor does not load. Pun's
//! codebase has no doc comments, so a parent's `@method` (without a `__call` to back it) does not keep a call.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety};

use super::class_method::qualified;
use super::rector_batch5::resolve_class_name;
use super::rector_stmts::{block_list, remove};
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoParentCallWithoutParent;

impl Rule for NoParentCallWithoutParent {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-parent-call-without-parent"
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
        let facts = facts(ctx);
        let codebase = &facts.codebase;
        let parent = class
            .extends
            .as_ref()
            .and_then(|e| e.types.first())
            .map(|id| resolve_class_name(ancestors, id));
        // `shouldSkipClass` and `hasUnresolvableAncestor`: every class above is known.
        let mut next = parent.clone();
        while let Some(name) = next {
            let Some(known) = codebase.get_class_like(name.as_bytes()) else {
                return;
            };
            next = known
                .direct_parent_class
                .as_ref()
                .map(|p| String::from_utf8_lossy(p.as_ref()).into_owned());
        }
        let own = codebase.get_class_like(qualified(ancestors, class.name.value).as_bytes());
        let has_method = |method: &[u8]| {
            let has = |class: &[u8], method: &[u8]| {
                codebase.get_class_like(class).is_some_and(|c| {
                    c.declaring_method_ids
                        .keys()
                        .any(|m| m.as_bytes().eq_ignore_ascii_case(method))
                })
            };
            parent.is_some()
                && own.is_some_and(|own| {
                    own.all_parent_classes
                        .iter()
                        .any(|p| has(p.as_bytes(), method) || has(p.as_bytes(), b"__call"))
                        || own
                            .all_parent_interfaces
                            .iter()
                            .any(|i| has(i.as_bytes(), method))
                })
        };
        for member in class.members.iter() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            let MethodBody::Concrete(body) = &method.body else {
                continue;
            };
            let list = block_list(ctx.source, body);
            for (index, statement) in list.stmts.iter().enumerate() {
                let Statement::Expression(e) = statement else {
                    continue;
                };
                let (call, edits) = match unparenthesize(e.expression) {
                    Expression::Call(Call::StaticMethod(call)) => {
                        (call, remove(ctx, &list, &[index]))
                    }
                    Expression::Assignment(a)
                        if matches!(a.operator, AssignmentOperator::Assign(_)) =>
                    {
                        let Expression::Call(Call::StaticMethod(call)) = unparenthesize(a.rhs)
                        else {
                            continue;
                        };
                        (call, vec![Edit::new(node_span(call), "null")])
                    }
                    _ => continue,
                };
                let (Expression::Parent(_), ClassLikeMemberSelector::Identifier(name)) =
                    (call.class, &call.method)
                else {
                    continue;
                };
                if has_method(name.value) {
                    continue;
                }
                out.push(Report {
                    span: node_span(call),
                    message: format!(
                        "Remove the call to `parent::{}()`, which no parent class has.",
                        String::from_utf8_lossy(name.value)
                    ),
                    help: None,
                    fix: Some(Fix {
                        safety: Safety::Safe,
                        edits,
                    }),
                });
            }
        }
    }
}
