//! How php-parser's format-preserving printer writes what the rules of this module change: a parameter whose `type` a
//! rule set (`Param` prints attributes, modifiers, the type, `&`, `...`, then the variable), and a kept expression put
//! in the place of another.

use mago_syntax::cst::{ArgumentList, Expression, FunctionLikeParameter, Node};
use pun_problem::{Edit, Span};

use super::parent;
use crate::Context;
use crate::rules::lint_b_helpers::dereferenceable;
use crate::rules::rector_code_quality::moved;
use crate::rules::rector_conditions::{indentation, reprint};
use crate::syntax::{node_span, unparenthesize};

/// The edit that gives `param` the type `hint`: it replaces the type written, or goes before the `&`, `...` or
/// variable.
pub(crate) fn set_param_type(param: &FunctionLikeParameter<'_>, hint: &str) -> Edit {
    match &param.hint {
        Some(old) => Edit::new(node_span(old), hint),
        None => {
            let at = (param.ampersand)
                .or(param.ellipsis)
                .map_or(node_span(&param.variable).start, |s| {
                    s.start.offset as usize
                });
            Edit::new(Span::new(at, at), format!("{hint} "))
        }
    }
}

/// The text of `span` in `source` with `edit` (inside it) applied, printed where a node starting at `at` stood:
/// php-parser keeps a node's tokens and moves its lines by the difference between the indentation of `at`'s line and
/// its own (`TokenStream::getTokenCode`).
pub(crate) fn moved_with_edit(source: &str, span: Span, edit: &Edit, at: usize) -> String {
    let tokens = crate::rules::php_parser::lex(source);
    let adjust = indentation(source, at) as isize - indentation(source, span.start) as isize;
    format!(
        "{}{}{}",
        reprint(
            source,
            &tokens,
            Span::new(span.start, edit.span.start),
            adjust
        ),
        edit.text,
        reprint(source, &tokens, Span::new(edit.span.end, span.end), adjust)
    )
}

/// The code for `value`, a kept expression, in the place of the node at `span`: as `rector_code_quality::moved`
/// writes it, and in parentheses when it is a `new` that is now dereferenced (`(new Foo())->bar()`), as php-parser's
/// `FIXUP_DEREF_LHS` and `FIXUP_STATIC_DEREF_LHS` write it for the PHP 7.4 its printer targets.
pub(crate) fn moved_value(
    ctx: &Context<'_>,
    span: Span,
    ancestors: &[Node<'_, '_>],
    value: &Expression<'_>,
) -> String {
    let code = moved(ctx, span, ancestors, value);
    let dereferenced = parent(span, ancestors).is_some_and(|(_, p)| {
        node_span(p).start == span.start
            && matches!(
                p,
                Node::ArrayAccess(_)
                    | Node::PropertyAccess(_)
                    | Node::NullSafePropertyAccess(_)
                    | Node::MethodCall(_)
                    | Node::NullSafeMethodCall(_)
                    | Node::StaticMethodCall(_)
                    | Node::StaticPropertyAccess(_)
                    | Node::ClassConstantAccess(_)
            )
    });
    if dereferenced && matches!(unparenthesize(value), Expression::Instantiation(_)) {
        format!("({code})")
    } else {
        code
    }
}

/// The edits that print a method call anew as php-parser does when its arguments all go (`pDereferenceLhs($var) .
/// '->' . $name . '()'`): `tail` (`->name()`) right after the receiver's own text, which drops what stood between
/// them, and the receiver in parentheses only when `pDereferenceLhs` needs them. The receiver's text is left alone,
/// so a fix inside it applies in the same round.
pub(crate) fn reprinted_call(
    source: &str,
    receiver: &Expression<'_>,
    call_end: usize,
    tail: &str,
) -> Vec<Edit> {
    let outer = node_span(receiver);
    let inner = node_span(unparenthesize(receiver));
    let needs = dereferenceable(source, receiver).starts_with('(');
    let (open, close) = if needs { ("(", ")") } else { ("", "") };
    let mut edits = Vec::new();
    if source[outer.start..inner.start] != *open {
        edits.push(Edit::new(Span::new(outer.start, inner.start), open));
    }
    edits.push(if source[inner.end..outer.end] == *close {
        Edit::new(Span::new(outer.end, call_end), tail)
    } else {
        Edit::new(Span::new(inner.end, call_end), format!("{close}{tail}"))
    });
    edits
}

/// The kept arguments of `list` as php-parser prints them in a new call node printed at `at`: each argument's tokens,
/// moved by the difference between the indentation of `at`'s line and that of the argument's own first line
/// (`p()` of an original node: `getTokenCode` with `indentLevel - getIndentationBefore`).
pub(crate) fn kept_arguments(source: &str, list: &ArgumentList<'_>, at: usize) -> Vec<String> {
    kept_nodes(source, list.arguments.iter().map(node_span), at)
}

/// As [`kept_arguments`], for the nodes at `spans`.
pub(crate) fn kept_nodes(
    source: &str,
    spans: impl Iterator<Item = Span>,
    at: usize,
) -> Vec<String> {
    let tokens = crate::rules::php_parser::lex(source);
    let level = indentation(source, at) as isize;
    spans
        .map(|span| {
            reprint(
                source,
                &tokens,
                span,
                level - indentation(source, span.start) as isize,
            )
        })
        .collect()
}
