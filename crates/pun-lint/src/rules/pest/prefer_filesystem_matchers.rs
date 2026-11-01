//! `lint/pest/prefer-filesystem-matchers`, ported from pest-plugin-rector's `SimplifyFilesystemMatchersRector`:
//! `expect(is_file($path) && is_readable($path))->toBeTrue()` becomes `expect($path)->toBeReadableFile()` (and the
//! same for directories and `is_writable()`), and `->toBeFile()->toBeReadable()` becomes `->toBeReadableFile()`.

use mago_syntax::cst::{BinaryOperator, Expression, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{
    func_call, function_name, identifier, is_expect_chain, is_name, matcher_subject, method_call,
    rename, subject_holder,
};
use crate::syntax::same_expression;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferFilesystemMatchers;

impl Rule for PreferFilesystemMatchers {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-filesystem-matchers"
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
        if !is_expect_chain(call) {
            return;
        }
        let message = "Use one filesystem matcher instead of two checks.".to_string();
        // `refactorBooleanAnd`.
        if is_name(&call.method, "toBeTrue")
            && let Some(holder) = subject_holder(call)
            && let Some(Expression::Binary(and)) = matcher_subject(call).map(unparenthesize)
            && matches!(and.operator, BinaryOperator::And(_))
            && let (Some(left), Some(right)) = (func_call(and.lhs), func_call(and.rhs))
            && let (Some(left_name), Some(right_name)) = (function_name(left), function_name(right))
            && let ([left_argument], [right_argument]) = (
                left.argument_list.arguments.as_slice(),
                right.argument_list.arguments.as_slice(),
            )
        {
            let value = left_argument.value();
            if !same_expression(ctx.source, value, right_argument.value()) {
                return;
            }
            let Some(matcher) = function_matcher(left_name, right_name)
                .or_else(|| function_matcher(right_name, left_name))
            else {
                return;
            };
            let mut edits = holder.set_arguments(ctx, value);
            let end = node_span(call).end;
            if call.argument_list.arguments.is_empty() {
                // `$node->var = $expectCall`: what came between the holder and the matcher goes.
                let object = node_span(unparenthesize(call.object)).end;
                if object > holder.span.end {
                    edits.push(Edit::new(Span::new(holder.span.end, object), ""));
                }
                edits.push(rename(call, matcher));
            } else {
                edits.push(Edit::new(
                    Span::new(holder.span.end, end),
                    format!("->{matcher}()"),
                ));
            }
            out.push(Report::fixed(
                node_span(call),
                message,
                Safety::Unsafe,
                edits,
            ));
            return;
        }
        // `refactorChainedMatchers`.
        let outer = match identifier(&call.method) {
            Some(o @ (b"toBeReadable" | b"toBeWritable")) => o,
            _ => return,
        };
        let Some(inner) = method_call(call.object) else {
            return;
        };
        let Some(matcher) = identifier(&inner.method).and_then(|i| chain_matcher(i, outer)) else {
            return;
        };
        if !call.argument_list.arguments.is_empty() || !inner.argument_list.arguments.is_empty() {
            return;
        }
        out.push(Report::fixed(
            node_span(call),
            message,
            Safety::Unsafe,
            vec![
                rename(inner, matcher),
                Edit::new(Span::new(node_span(inner).end, node_span(call).end), ""),
            ],
        ));
    }
}

/// `COMBINED_FUNCTION_MATCHERS[first][second]`.
fn function_matcher(first: &[u8], second: &[u8]) -> Option<&'static str> {
    match (first, second) {
        (b"is_dir", b"is_readable") => Some("toBeReadableDirectory"),
        (b"is_dir", b"is_writable" | b"is_writeable") => Some("toBeWritableDirectory"),
        (b"is_file", b"is_readable") => Some("toBeReadableFile"),
        (b"is_file", b"is_writable" | b"is_writeable") => Some("toBeWritableFile"),
        _ => None,
    }
}

/// `COMBINED_CHAIN_MATCHERS[inner][outer]`.
fn chain_matcher(inner: &[u8], outer: &[u8]) -> Option<&'static str> {
    match (inner, outer) {
        (b"toBeDirectory", b"toBeReadable") => Some("toBeReadableDirectory"),
        (b"toBeDirectory", b"toBeWritable") => Some("toBeWritableDirectory"),
        (b"toBeFile", b"toBeReadable") => Some("toBeReadableFile"),
        (b"toBeFile", b"toBeWritable") => Some("toBeWritableFile"),
        _ => None,
    }
}
