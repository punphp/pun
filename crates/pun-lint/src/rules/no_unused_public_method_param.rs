//! `lint/dead-code/no-unused-public-method-param`, ported from Rector 2.6.1's `RemoveUnusedPublicMethodParameterRector`
//! (rules/DeadCode/Rector/ClassMethod), with `ClassMethodParamRemover` and `ComplexNodeRemover::
//! processRemoveParamWithKeys`.
//!
//! In a final class that extends and implements nothing, a public method's unused parameters go when they are the
//! last ones (a used parameter after an unused one keeps them all), with their `@param` tags. Callers are left
//! alone: PHP ignores the extra arguments. Constructors and other magic methods are skipped, and so is `__invoke` of
//! a Symfony `#[AsMessageHandler]`.

use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::class_method::is_magic;
use super::rector_batch5::{has_attribute, member_from, param_used, remove_params};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoUnusedPublicMethodParam;

impl Rule for NoUnusedPublicMethodParam {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-unused-public-method-param"
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
        // It may have a child, or override a parent method whose signature it must follow.
        if !class
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Final(_)))
            || class.extends.is_some()
            || class.implements.is_some()
        {
            return;
        }
        let source = ctx.source;
        for (index, member) in class.members.iter().enumerate() {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };
            if skip(ancestors, class, method) {
                continue;
            }
            let params: Vec<&FunctionLikeParameter> =
                method.parameter_list.parameters.iter().collect();
            let removed: Vec<bool> = params.iter().map(|p| !param_used(method, p)).collect();
            // processRemoveParamWithKeys: only when no kept parameter follows an unused one.
            let Some(first) = removed.iter().position(|r| *r) else {
                continue;
            };
            if !removed[first..].iter().all(|r| *r) {
                continue;
            }
            let from = member_from(&class.members, class.left_brace, index);
            out.push(Report {
                span: node_span(method),
                message: "Remove the unused parameters of this public method.".to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: remove_params(source, method, from, &removed),
                }),
            });
        }
    }
}

/// `shouldSkipClassMethod`.
fn skip(ancestors: &[Node<'_, '_>], class: &Class<'_>, method: &Method<'_>) -> bool {
    let public = !method
        .modifiers
        .iter()
        .any(|m| matches!(m, Modifier::Private(_) | Modifier::Protected(_)));
    if !public || method.parameter_list.parameters.is_empty() {
        return true;
    }
    let name = String::from_utf8_lossy(method.name.value).to_ascii_lowercase();
    if name == "__invoke" {
        let mut path = ancestors.to_vec();
        path.push(Node::Class(class));
        return has_attribute(
            &path,
            &class.attribute_lists,
            "Symfony\\Component\\Messenger\\Attribute\\AsMessageHandler",
        );
    }
    is_magic(method.name.value)
}
