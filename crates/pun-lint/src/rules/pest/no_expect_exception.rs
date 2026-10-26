//! `lint/pest/no-expect-exception`, ported from pest-plugin-rector's `ConvertExpectExceptionToThrowRector`:
//! `$this->expectException(E::class);` (with `expectExceptionMessage()`, or `expectExceptionObject()`) and the
//! statement after it become `expect(fn() => action())->toThrow(E::class, 'message');`.

use mago_syntax::cst::{Expression, MethodCall, Node, NodeKind, Statement, Variable};
use pun_problem::{Safety, Span};

use super::{
    Item, STATEMENT_LISTS, argument_value, comment_spans, identifier, kept_text, method_call,
    report, rewrite_list, statement_expression, statement_list,
};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoExpectException;

const SUPPORTED_METHODS: [&[u8]; 3] = [
    b"expectException",
    b"expectExceptionMessage",
    b"expectExceptionObject",
];

const ALL_EXPECT_EXCEPTION_METHODS: [&[u8]; 5] = [
    b"expectException",
    b"expectExceptionMessage",
    b"expectExceptionObject",
    b"expectExceptionCode",
    b"expectExceptionMessageMatches",
];

/// What `collectConversion` finds: the exception, the message and the action, and how many statements they take.
struct Conversion<'a> {
    exception: &'a Expression<'a>,
    message: Option<&'a Expression<'a>>,
    action: &'a Expression<'a>,
    consumed: usize,
}

impl Rule for NoExpectException {
    fn id(&self) -> &'static str {
        "lint/pest/no-expect-exception"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(STATEMENT_LISTS)
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let list = statement_list(ctx, node);
        let stmts = &list.stmts;
        let mut items: Vec<Item<'_>> = Vec::new();
        let mut i = 0;
        let mut changed = false;
        while i < stmts.len() {
            let Some(c) = collect_conversion(stmts, i) else {
                items.push(Item::Old(i));
                i += 1;
                continue;
            };
            // copyComments: the comments of every statement the conversion takes.
            let comments: Vec<Span> = stmts[i..i + c.consumed]
                .iter()
                .flat_map(|s| comment_spans(ctx, node_span(*s).start))
                .collect();
            let (exception, message, action) = (c.exception, c.message, c.action);
            items.push(Item::New {
                print: Box::new(move |indent| {
                    let text =
                        |e: &Expression<'_>| kept_text(ctx, node_span(unparenthesize(e)), indent);
                    let mut args = text(exception);
                    if let Some(m) = message {
                        args.push_str(", ");
                        args.push_str(&text(m));
                    }
                    format!("expect(fn() => {})->toThrow({args});", text(action))
                }),
                comments,
            });
            i += c.consumed;
            changed = true;
        }
        // The rest of the list keeps the Nop of its trailing comments.
        if list.nop {
            items.push(Item::Old(stmts.len()));
        }
        if !changed {
            return;
        }
        let Some(edits) = rewrite_list(ctx, node, ancestors.last().copied(), &list, &items) else {
            return;
        };
        out.push(report(
            node_span(stmts[0]),
            "Use `expect(fn() => ...)->toThrow()` instead of `$this->expectException()`."
                .to_string(),
            Safety::Safe,
            edits,
        ));
    }
}

/// `collectConversion` from the statement at `start`.
fn collect_conversion<'a>(stmts: &[&'a Statement<'a>], start: usize) -> Option<Conversion<'a>> {
    let mut exception = None;
    let mut exception_method: Option<&[u8]> = None;
    let mut message = None;
    let mut consumed = 0;
    for stmt in &stmts[start..] {
        let Some(call) = statement_expression(stmt).and_then(method_call) else {
            break;
        };
        let Some(name) = expectation_method(call, &SUPPORTED_METHODS) else {
            break;
        };
        let [argument] = call.argument_list.arguments.as_slice() else {
            return None;
        };
        let value = argument_value(argument);
        if name == b"expectExceptionMessage" {
            if exception_method == Some(b"expectExceptionObject") || message.is_some() {
                return None;
            }
            message = Some(value);
            consumed += 1;
            continue;
        }
        if exception.is_some() || (name == b"expectExceptionObject" && message.is_some()) {
            return None;
        }
        exception = Some(value);
        exception_method = Some(name);
        consumed += 1;
    }
    let exception = exception?;
    let action = statement_expression(stmts.get(start + consumed)?)?;
    if method_call(action)
        .is_some_and(|a| expectation_method(a, &ALL_EXPECT_EXCEPTION_METHODS).is_some())
    {
        return None;
    }
    Some(Conversion {
        exception,
        message,
        action,
        consumed: consumed + 1,
    })
}

/// `getExpectationMethodName`: the name of a `$this->method()` call among `methods`.
fn expectation_method<'a>(call: &MethodCall<'a>, methods: &[&[u8]]) -> Option<&'a [u8]> {
    let Expression::Variable(Variable::Direct(var)) = unparenthesize(call.object) else {
        return None;
    };
    if !var.name.eq_ignore_ascii_case(b"$this") {
        return None;
    }
    let name = identifier(&call.method)?;
    methods.contains(&name).then_some(name)
}
