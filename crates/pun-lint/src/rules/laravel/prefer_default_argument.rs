//! `lint/laravel/prefer-default-argument`, ported from rector-laravel 2.7.3's `ApplyDefaultInsteadOfNullCoalesceRector`
//! (src/Rector/Coalesce) with its default list (`defaultLaravelMethods`): `config()`, `env()`, `data_get()` and
//! `Illuminate\Support\Env::get()`.
//!
//! `config('a') ?? 'b'` becomes `config('a', 'b')` when the call has no default argument yet, and `config('a') ?? null`
//! becomes `config('a')`. A `throw` on the right is left alone. As upstream, `config`, `env` and `data_get` match a
//! method or static call of that name too.

use mago_syntax::cst::{ArgumentList, BinaryOperator, Call, Expression, Node, NodeKind};
use pun_problem::{Edit, Safety, Span};

use super::{class_name, is_a, is_name, member_name};
use crate::rules::lint_b_helpers::{Constant, append_argument, constant};
use crate::rules::rector_code_quality::{named, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferDefaultArgument;

/// `(method name, the class it must be called on, the argument count that means no default yet)`.
const METHODS: [(&str, Option<&str>, usize); 4] = [
    ("config", None, 1),
    ("env", None, 1),
    ("data_get", None, 2),
    ("get", Some("Illuminate\\Support\\Env"), 1),
];

impl Rule for PreferDefaultArgument {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-default-argument"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Binary])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Binary(coalesce) = node else {
            return;
        };
        if !matches!(coalesce.operator, BinaryOperator::NullCoalesce(_)) {
            return;
        }
        if matches!(unparenthesize(coalesce.rhs), Expression::Throw(_)) {
            return;
        }
        // The call's name, its class (static calls) and its arguments.
        let (name, class, list): (&[u8], Option<String>, &ArgumentList<'_>) =
            match unparenthesize(coalesce.lhs) {
                Expression::Call(Call::Function(f)) => {
                    let name = METHODS
                        .iter()
                        .find(|(m, ..)| named(f, m))
                        .map_or(&b""[..], |(m, ..)| m.as_bytes());
                    (name, None, &f.argument_list)
                }
                Expression::Call(Call::Method(m)) => {
                    let Some(name) = member_name(&m.method) else {
                        return;
                    };
                    (name, None, &m.argument_list)
                }
                Expression::Call(Call::StaticMethod(s)) => {
                    let Some(name) = member_name(&s.method) else {
                        return;
                    };
                    (name, class_name(s.class, ancestors), &s.argument_list)
                }
                _ => return,
            };
        let found = METHODS.iter().find(|(method, on, _)| {
            is_name(name, method)
                && match on {
                    Some(on) => class.as_deref().is_some_and(|c| is_a(ctx, c, &[on])),
                    None => true,
                }
        });
        let Some(&(method, _, position)) = found else {
            return;
        };
        if list.arguments.len() != position {
            return;
        }
        let call = node_span(unparenthesize(coalesce.lhs));
        let whole = node_span(coalesce);
        // Parentheses around the call go with the `??`.
        let mut edits = vec![Edit::new(Span::new(call.end, whole.end), "")];
        if whole.start < call.start {
            edits.push(Edit::new(Span::new(whole.start, call.start), ""));
        }
        if constant(ancestors, coalesce.rhs, node_span(coalesce).start) != Some(Constant::Null) {
            edits.push(append_argument(ctx.source, list, text(ctx, coalesce.rhs)));
        }
        out.push(Report::fixed(
            node_span(coalesce),
            format!("Pass the default to `{method}()` instead of using `??`."),
            Safety::Unsafe,
            edits,
        ));
    }
}
