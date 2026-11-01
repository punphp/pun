//! `lint/pest/prefer-not-modifier`, ported from pest-plugin-rector's `SimplifyExpectNotRector`:
//! `expect(!$x)->toBeTrue()` becomes `expect($x)->toBeFalse()` (and the other way) when `$x` is always a bool, and
//! `expect(!$x)->toBeEmpty()` becomes `expect($x)->not->toBeEmpty()` (or loses its `->not`).

use mago_syntax::cst::{Expression, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{
    Is, Trinary, expect_call, first_value, identifier, is_name, property_fetch, rename,
    set_first_argument, type_is,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferNotModifier;

impl Rule for PreferNotModifier {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-not-modifier"
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
        let Some(expect) = expect_call(call) else {
            return;
        };
        let Some(Expression::UnaryPrefix(not)) =
            first_value(&expect.argument_list).map(unparenthesize)
        else {
            return;
        };
        if !not.operator.is_not() {
            return;
        }
        // `isCalledDirectlyOnExpect`.
        let mut var = unparenthesize(call.object);
        let modifier = property_fetch(var).filter(|p| is_name(&p.property, "not"));
        if let Some(modifier) = modifier {
            var = unparenthesize(modifier.object);
        }
        if node_span(var) != node_span(expect) {
            return;
        }
        let mut edits = set_first_argument(ctx, &expect.argument_list, not.operand);
        let toggled = match identifier(&call.method) {
            Some(b"toBeTrue" | b"toBeFalse") => {
                if type_is(ctx, not.operand, Is::Boolean) != Trinary::Yes {
                    return;
                }
                let flipped = if identifier(&call.method) == Some(b"toBeTrue") {
                    "toBeFalse"
                } else {
                    "toBeTrue"
                };
                rename(call, flipped)
            }
            // `toggleNotModifier`.
            Some(b"toBeEmpty") => match modifier {
                Some(modifier) => Edit::new(
                    Span::new(
                        node_span(unparenthesize(modifier.object)).end,
                        node_span(unparenthesize(call.object)).end,
                    ),
                    "",
                ),
                None => {
                    let end = node_span(unparenthesize(call.object)).end;
                    Edit::new(Span::new(end, end), "->not")
                }
            },
            _ => return,
        };
        edits.push(toggled);
        out.push(Report::fixed(
            node_span(call),
            "Negate the expectation instead of its value.",
            Safety::Safe,
            edits,
        ));
    }
}
