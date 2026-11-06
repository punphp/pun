//! `lint/pest/prefer-to-throw`, ported from pest-plugin-rector's `UseToThrowRector`: in a test's closure, a
//! `try { action(); } catch (E $e) { expect($e->getMessage())->toBe('m'); }` becomes
//! `expect(fn() => action())->toThrow(E::class, 'm');`.

use mago_syntax::cst::{
    ArrayElement, Assignment, AssignmentOperator, Block, Expression, ForeachTarget, Hint, Node,
    NodeKind, Statement, Try, UnaryPrefixOperator, Variable,
};
use pun_problem::{Edit, Safety, Span};

use super::{
    Item, comments_start, expect_argument, function_name, indent_at, is_expect_chain, is_name,
    kept_text, method_call, old_statements, p_stmts, statement_expression,
};
use crate::rules::php_parser::{block_list, tokens};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferToThrow;

const TEST_FUNCTIONS: [&[u8]; 7] = [
    b"test",
    b"it",
    b"describe",
    b"beforeEach",
    b"afterEach",
    b"beforeAll",
    b"afterAll",
];

/// `Scope::SUPERGLOBAL_VARIABLES`: no closure can `use` them.
const SUPERGLOBALS: [&[u8]; 9] = [
    b"$GLOBALS",
    b"$_SERVER",
    b"$_GET",
    b"$_POST",
    b"$_FILES",
    b"$_COOKIE",
    b"$_SESSION",
    b"$_REQUEST",
    b"$_ENV",
];

impl Rule for PreferToThrow {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-to-throw"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        if !function_name(call)
            .is_some_and(|n| TEST_FUNCTIONS.iter().any(|f| f.eq_ignore_ascii_case(n)))
        {
            return;
        }
        let mut edits = Vec::new();
        for argument in call.argument_list.arguments.iter() {
            if let Expression::Closure(closure) = unparenthesize(argument.value()) {
                find_try(ctx, Node::Block(&closure.body), &mut edits);
            }
        }
        if edits.is_empty() {
            return;
        }
        out.push(Report::fixed(
            node_span(call),
            "Use `expect(...)->toThrow()` instead of a `try`/`catch`.",
            Safety::Safe,
            edits,
        ));
    }
}

/// `transformTestClosure`'s traversal: every `try` outside nested closures, converted where it can be; a converted
/// one is not entered.
fn find_try(ctx: &Context<'_>, node: Node<'_, '_>, edits: &mut Vec<Edit>) {
    match node {
        Node::Closure(_) => return,
        Node::Try(t) => {
            if let Some(edit) = convert(ctx, t) {
                edits.push(edit);
                return;
            }
        }
        _ => {}
    }
    for child in node.children() {
        find_try(ctx, child, edits);
    }
}

/// `convertTryCatch`, and the statement that replaces the `try` (its comments go).
fn convert(ctx: &Context<'_>, t: &Try<'_>) -> Option<Edit> {
    let ([catch], None) = (t.catch_clauses.as_slice(), &t.finally_clause) else {
        return None;
    };
    if matches!(catch.hint, Hint::Union(_)) {
        return None;
    }
    let tokens = tokens(ctx);
    let body = block_list(&tokens, &t.block);
    if body.is_empty() {
        return None;
    }
    let message = message_assertion(ctx, &catch.block, catch.variable.as_ref().map(|v| v.name));
    if !block_list(&tokens, &catch.block).is_empty() && message.is_none() {
        return None;
    }
    let span = node_span(t);
    let indent = indent_at(ctx, span.start);
    let callable = match (body.stmts.as_slice(), body.nop) {
        ([statement], false) if statement_expression(statement).is_some() => {
            let expr = unparenthesize(statement_expression(statement)?);
            format!("fn() => {}", kept_text(ctx, node_span(expr), indent))
        }
        _ => {
            let uses = use_variables(&body.stmts);
            let uses = match uses.is_empty() {
                true => String::new(),
                false => format!(" use ({})", uses.join(", ")),
            };
            let olds = old_statements(ctx, &body);
            let items: Vec<Item<'_>> = (0..olds.len()).map(Item::Old).collect();
            format!(
                "function (){uses} {{{}\n{}}}",
                p_stmts(ctx, &olds, &items, indent + 4),
                " ".repeat(indent)
            )
        }
    };
    let mut args = format!("{}::class", kept_text(ctx, node_span(&catch.hint), indent));
    if let Some(m) = message {
        args.push_str(", ");
        args.push_str(&kept_text(ctx, node_span(unparenthesize(m)), indent));
    }
    Some(Edit::new(
        Span::new(comments_start(ctx, span.start), span.end),
        format!("expect({callable})->toThrow({args});"),
    ))
}

/// `extractMessageAssertion`: the value in a catch body that is only `expect($e->getMessage())->toBe(value);`.
fn message_assertion<'a>(
    ctx: &Context<'_>,
    block: &'a Block<'a>,
    var: Option<&[u8]>,
) -> Option<&'a Expression<'a>> {
    let tokens = tokens(ctx);
    let list = block_list(&tokens, block);
    let ([statement], false) = (list.stmts.as_slice(), list.nop) else {
        return None;
    };
    let call = method_call(statement_expression(statement)?)?;
    if !is_expect_chain(call) || !is_name(&call.method, "toBe") {
        return None;
    }
    let [argument] = call.argument_list.arguments.as_slice() else {
        return None;
    };
    let subject = method_call(expect_argument(call)?)?;
    if !is_name(&subject.method, "getMessage") {
        return None;
    }
    let Expression::Variable(Variable::Direct(object)) = unparenthesize(subject.object) else {
        return None;
    };
    (Some(object.name) == var).then(|| argument.value())
}

/// `AnonymousFunctionFactory::createUseVariablesFromParams` with no parameters: the variables of `stmts` in order,
/// but `$this`, superglobals and those assigned (by `=`, a destructuring, a `foreach` or as a parameter) before or
/// where they first show up.
fn use_variables(stmts: &[&Statement<'_>]) -> Vec<String> {
    let mut assigned_spans = Vec::new();
    let mut variables = Vec::new();
    for statement in stmts {
        collect(
            Node::Statement(statement),
            &mut assigned_spans,
            &mut variables,
        );
    }
    let mut assigned: Vec<&[u8]> = Vec::new();
    let mut uses: Vec<&[u8]> = Vec::new();
    for (name, span) in variables {
        if name.eq_ignore_ascii_case(b"$this") || SUPERGLOBALS.contains(&name) {
            continue;
        }
        if assigned_spans.contains(&span) {
            assigned.push(name);
        }
        if !assigned.contains(&name) && !uses.contains(&name) {
            uses.push(name);
        }
    }
    uses.iter()
        .map(|n| String::from_utf8_lossy(n).into_owned())
        .collect()
}

/// The direct variables under `node` in php-parser's order, and the spans of those Rector marks as assigned.
fn collect<'a>(
    node: Node<'_, 'a>,
    assigned: &mut Vec<Span>,
    variables: &mut Vec<(&'a [u8], Span)>,
) {
    let mut mark = |expr: &Expression<'_>| {
        let expr = match expr {
            Expression::UnaryPrefix(u)
                if matches!(u.operator, UnaryPrefixOperator::Reference(_)) =>
            {
                u.operand
            }
            e => e,
        };
        if let Expression::Variable(Variable::Direct(v)) = expr {
            assigned.push(node_span(v));
        }
    };
    match node {
        Node::DirectVariable(v) => variables.push((v.name, node_span(v))),
        Node::Assignment(Assignment {
            lhs,
            operator: AssignmentOperator::Assign(_),
            ..
        }) => {
            mark(lhs);
            let elements = match lhs {
                Expression::Array(a) => Some(a.elements.as_slice()),
                Expression::List(l) => Some(l.elements.as_slice()),
                _ => None,
            };
            for element in elements.unwrap_or_default() {
                match element {
                    ArrayElement::KeyValue(e) => mark(e.value),
                    ArrayElement::Value(e) => mark(e.value),
                    _ => {}
                }
            }
        }
        Node::Foreach(f) => match &f.target {
            ForeachTarget::Value(t) => mark(t.value),
            ForeachTarget::KeyValue(t) => {
                mark(t.key);
                mark(t.value);
            }
        },
        Node::FunctionLikeParameter(p) => assigned.push(node_span(&p.variable)),
        _ => {}
    }
    for child in node.children() {
        collect(child, assigned, variables);
    }
}
