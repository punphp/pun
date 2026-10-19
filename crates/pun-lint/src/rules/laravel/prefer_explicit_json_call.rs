//! `lint/laravel/prefer-explicit-json-call`, ported from rector-laravel 2.5.0's `JsonCallToExplicitJsonCallRector`
//! (src/Rector/MethodCall).
//!
//! On an object using `Illuminate\Foundation\Testing\Concerns\MakesHttpRequests`, `->json('post', $uri, ...)` becomes
//! `->postJson($uri, ...)` (likewise `get`, `put`, `patch`, `delete` and `options`). A `GET` with data moves the data
//! into its `route()` helper: `->json('GET', route('home'), $data)` becomes `->getJson(route('home', $data))`; a `GET`
//! with headers or options, or with a `route()` call of several arguments, is left alone.

use mago_syntax::cst::{Argument, Call, Expression, Literal, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::{is_object, member_name};
use crate::rules::lint_b_helpers::{remove_argument, value};
use crate::rules::rector_code_quality::{named, text};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const MAKES_HTTP_REQUESTS: &str = "Illuminate\\Foundation\\Testing\\Concerns\\MakesHttpRequests";

pub struct PreferExplicitJsonCall;

impl Rule for PreferExplicitJsonCall {
    fn id(&self) -> &'static str {
        "lint/laravel/prefer-explicit-json-call"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else {
            return;
        };
        if member_name(&call.method) != Some(b"json") {
            return;
        }
        if !is_object(ctx, ancestors, call.object, MAKES_HTTP_REQUESTS) {
            return;
        }
        let args = call.argument_list.arguments.as_slice();
        if args.len() < 2 {
            return;
        }
        let Expression::Literal(Literal::String(verb)) = value(&args[0]) else {
            return;
        };
        let verb = String::from_utf8_lossy(verb.value.unwrap_or_default()).to_ascii_lowercase();
        if !["get", "post", "put", "patch", "delete", "options"].contains(&verb.as_str()) {
            return;
        }
        let name = node_span(&call.method);
        let mut edits = vec![Edit::new(name, format!("{verb}Json"))];
        if verb == "get" && args.len() > 2 {
            // `refactorGetMethodCall`: exactly three arguments, a `route()` of one argument, data unnamed or `data:`.
            let Expression::Call(Call::Function(route)) = unparenthesize(value(&args[1])) else {
                return;
            };
            if args.len() != 3 || !named(route, "route") || route.argument_list.arguments.len() != 1
            {
                return;
            }
            if let Argument::Named(n) = &args[2]
                && n.name.value != b"data"
            {
                return;
            }
            let list = &call.argument_list;
            edits.push(Edit::new(
                pun_problem::Span::new(
                    list.left_parenthesis.end.offset as usize,
                    list.right_parenthesis.start.offset as usize,
                ),
                format!(
                    "route({}, {})",
                    text(ctx, &route.argument_list.arguments.as_slice()[0]),
                    text(ctx, value(&args[2]))
                ),
            ));
        } else {
            edits.push(remove_argument(&call.argument_list, 0));
        }
        out.push(Report {
            span: node_span(call),
            message: format!("Use `{verb}Json()` instead of `json('{verb}', ...)`."),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits,
            }),
        });
    }
}
