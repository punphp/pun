//! `lint/pest/no-negated-boolean-matcher`, ported from pest-plugin-rector's `ToBeTrueNotFalseRector`:
//! `expect($bool)->not->toBeFalse()` becomes `expect($bool)->toBeTrue()`, and `->not->toBeTrue()` becomes
//! `->toBeFalse()`, when the checked value is always a bool.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{
    Is, Trinary, identifier, is_name, matcher_subject, property_fetch, rename, report, type_is,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoNegatedBooleanMatcher;

impl Rule for NoNegatedBooleanMatcher {
    fn id(&self) -> &'static str {
        "lint/pest/no-negated-boolean-matcher"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else {
            return;
        };
        let Some(not) = property_fetch(call.object).filter(|p| is_name(&p.property, "not")) else {
            return;
        };
        let opposite = match identifier(&call.method) {
            Some(b"toBeFalse") => "toBeTrue",
            Some(b"toBeTrue") => "toBeFalse",
            _ => return,
        };
        let Some(subject) = matcher_subject(call) else {
            return;
        };
        if type_is(ctx, subject, Is::Boolean) != Trinary::Yes {
            return;
        }
        out.push(report(
            node_span(call),
            format!("Use `{opposite}()` instead of a negated boolean matcher."),
            Safety::Safe,
            vec![
                // `$methodCall->var = $notProperty->var`.
                Edit::new(
                    Span::new(
                        node_span(unparenthesize(not.object)).end,
                        node_span(unparenthesize(call.object)).end,
                    ),
                    "",
                ),
                rename(call, opposite),
            ],
        ));
    }
}
