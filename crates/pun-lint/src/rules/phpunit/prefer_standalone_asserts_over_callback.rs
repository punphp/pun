//! `lint/phpunit/prefer-standalone-asserts-over-callback`, ported from rector-phpunit's
//! `WithCallbackIdenticalToStandaloneAssertsRector` (rules/CodeQuality/Rector/MethodCall), with
//! `FromBinaryAndAssertExpressionsFactory` and `ClosureUsesResolver::resolveFromArrowFunction`.
//!
//! In a test class, a `->with(..., $any->callback(fn), ...)` argument whose closure or arrow function returns a
//! comparison (`===`, `==`, `instanceof`, `isset($a[k])`, `array_key_exists()`, or a `&&` chain of those and method
//! calls) gets a `void` closure that asserts each part (`assertSame`, `assertEquals`, `assertCount`,
//! `assertInstanceOf`, `assertArrayHasKey`, `assertTrue`) and ends with `return;`. A static closure calls
//! `self::`; others call `$this->`.
//!
//! Printing follows php-parser. A closure that keeps statements besides its `return`s keeps its text: the `return`s go
//! (with the comments before them), the assertions follow the last kept statement at the first statement's
//! indentation, and the return type becomes `void`. A closure of `return`s only, and an arrow function (which becomes a
//! closure that `use`s its outer variables, never static), is printed anew: parameters as `pMaybeMultiline` writes
//! them, the body one level below the line the function starts on. Each moved expression keeps its text, reindented.

use mago_syntax::cst::{
    ArrowFunction, Binary, BinaryOperator, Call, Closure, Construct, Expression, Node, NodeKind,
    Statement,
};
use pun_problem::{Edit, Safety, Span};

use super::{AnyCall, arg_value, is_in_test_class, maybe_multiline, report, resolve};
use crate::rules::rector_stmts::{List, block_list, line_indent, reindent};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferStandaloneAssertsOverCallback;

impl Rule for PreferStandaloneAssertsOverCallback {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-standalone-asserts-over-callback"
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
        let call = AnyCall::of(node).unwrap();
        if !call.is_name(&["with"]) || !is_in_test_class(ctx, ancestors, None) {
            return;
        }
        let mut scope = ancestors.to_vec();
        scope.push(node);
        let mut edits = Vec::new();
        for arg in call.args() {
            let Some(function) = match_callback_function_like(arg_value(arg)) else {
                continue;
            };
            let Some(joined_exprs) = resolve_inner_return_joined_exprs(&function) else {
                continue;
            };
            let Some(asserts) = create(&scope, &joined_exprs) else {
                continue;
            };
            let edit = match function {
                FunctionLike::Closure(closure) => closure_edit(ctx.source, closure, &asserts),
                FunctionLike::Arrow(arrow) => arrow_edit(ctx.source, arrow, &asserts),
            };
            edits.push(edit);
        }
        if edits.is_empty() {
            return;
        }
        out.push(report(
            call.name_span(),
            "Assert each condition in the callback instead of returning their combination.",
            Safety::Safe,
            edits,
        ));
    }
}

#[derive(Clone, Copy)]
enum FunctionLike<'a> {
    Closure(&'a Closure<'a>),
    Arrow(&'a ArrowFunction<'a>),
}

/// An assertion to write: the method and its arguments, each an original expression or a new name.
struct Assert<'a> {
    method: &'static str,
    args: Vec<AssertArg<'a>>,
}

enum AssertArg<'a> {
    Expr(&'a Expression<'a>),
    New(String),
}

/// `matchCallbackArgAndFunctionLike`: `$any->callback(<closure or arrow function>, ...)`.
fn match_callback_function_like<'a>(value: &'a Expression<'a>) -> Option<FunctionLike<'a>> {
    let Expression::Call(Call::Method(call)) = unparenthesize(value) else {
        return None;
    };
    let call = AnyCall::of(Node::MethodCall(call))?;
    if !call.is_name(&["callback"]) {
        return None;
    }
    match unparenthesize(arg_value(call.args().into_iter().next()?)) {
        Expression::Closure(closure) => Some(FunctionLike::Closure(closure)),
        Expression::ArrowFunction(arrow) => Some(FunctionLike::Arrow(arrow)),
        _ => None,
    }
}

/// `matchInnerSoleExpr`: a closure's first top-level `return`'s value, or an arrow function's expression.
fn match_inner_sole_expr<'a>(function: &FunctionLike<'a>) -> Option<&'a Expression<'a>> {
    match function {
        FunctionLike::Closure(closure) => closure.body.statements.iter().find_map(|s| match s {
            Statement::Return(r) => Some(r.value),
            _ => None,
        })?,
        FunctionLike::Arrow(arrow) => Some(arrow.expression),
    }
}

/// `resolveInnerReturnJoinedExprs`; `None` for upstream's `[]`.
fn resolve_inner_return_joined_exprs<'a>(
    function: &FunctionLike<'a>,
) -> Option<Vec<&'a Expression<'a>>> {
    let inner = unparenthesize(match_inner_sole_expr(function)?);
    if let Expression::Binary(binary) = inner
        && matches!(binary.operator, BinaryOperator::And(_))
    {
        return extract_joined_exprs(binary);
    }
    let accepted = match inner {
        Expression::Binary(b) => matches!(
            b.operator,
            BinaryOperator::Identical(_) | BinaryOperator::Equal(_) | BinaryOperator::Instanceof(_)
        ),
        Expression::Construct(Construct::Isset(_)) => true,
        _ => function_call_named(inner, "array_key_exists").is_some(),
    };
    accepted.then(|| vec![inner])
}

/// `extractJoinedExprs`: the operands of a chain of `&&`, right to left; `None` when a right operand is `||`.
fn extract_joined_exprs<'a>(binary: &'a Binary<'a>) -> Option<Vec<&'a Expression<'a>>> {
    let mut joined_exprs = Vec::new();
    let mut current = binary;
    loop {
        let right = unparenthesize(current.rhs);
        // is binary op, but not "&&"
        if matches!(right, Expression::Binary(b) if matches!(b.operator, BinaryOperator::Or(_))) {
            return None;
        }
        joined_exprs.push(right);
        let left = unparenthesize(current.lhs);
        match left {
            Expression::Binary(b) if matches!(b.operator, BinaryOperator::And(_)) => current = b,
            _ => {
                joined_exprs.push(left);
                return Some(joined_exprs);
            }
        }
    }
}

/// A function call named `name` (`isName`: ASCII case-insensitive, a leading `\` aside), with its arguments' values.
fn function_call_named<'a>(
    expr: &'a Expression<'a>,
    name: &str,
) -> Option<Vec<&'a Expression<'a>>> {
    let Expression::Call(Call::Function(call)) = unparenthesize(expr) else {
        return None;
    };
    let Expression::Identifier(id) = call.function else {
        return None;
    };
    let written = id.value();
    let written = written.strip_prefix(b"\\").unwrap_or(written);
    written
        .eq_ignore_ascii_case(name.as_bytes())
        .then(|| call.argument_list.arguments.iter().map(arg_value).collect())
}

/// `FromBinaryAndAssertExpressionsFactory::create`, without the call: the assertions in the order of the operands.
fn create<'a>(ancestors: &[Node<'_, '_>], exprs: &[&'a Expression<'a>]) -> Option<Vec<Assert<'a>>> {
    let mut assert_method_calls = Vec::new();
    let assert = |method, args| Assert { method, args };
    for &expr in exprs {
        // implicit bool compare
        if let Expression::Call(Call::Method(_)) = expr {
            assert_method_calls.push(assert("assertTrue", vec![AssertArg::Expr(expr)]));
            continue;
        }
        if let Some(args) = function_call_named(expr, "array_key_exists") {
            let (dim, variable) = (*args.first()?, *args.get(1)?);
            assert_method_calls.push(assert(
                "assertArrayHasKey",
                vec![AssertArg::Expr(dim), AssertArg::Expr(variable)],
            ));
            continue;
        }
        if let Expression::Construct(Construct::Isset(isset)) = expr {
            for value in isset.values.iter() {
                let Expression::ArrayAccess(access) = unparenthesize(value) else {
                    // not supported yet
                    return None;
                };
                assert_method_calls.push(assert(
                    "assertArrayHasKey",
                    vec![AssertArg::Expr(access.index), AssertArg::Expr(access.array)],
                ));
            }
            continue;
        }
        let Expression::Binary(binary) = expr else {
            // not supported expr
            return None;
        };
        match binary.operator {
            BinaryOperator::Instanceof(_) => {
                let class = match unparenthesize(binary.rhs) {
                    Expression::Identifier(id) => {
                        AssertArg::New(format!("\\{}::class", resolve(ancestors, id)))
                    }
                    Expression::Self_(k) | Expression::Static(k) | Expression::Parent(k) => {
                        AssertArg::New(format!("{}::class", String::from_utf8_lossy(k.value)))
                    }
                    other => AssertArg::Expr(other),
                };
                assert_method_calls.push(assert(
                    "assertInstanceOf",
                    vec![class, AssertArg::Expr(binary.lhs)],
                ));
            }
            BinaryOperator::Identical(_) | BinaryOperator::Equal(_) => {
                let (left, right) = (unparenthesize(binary.lhs), unparenthesize(binary.rhs));
                if let Some(args) = function_call_named(left, "count") {
                    // create assertCount() for an integer (php-parser's `Int_`: not one above `PHP_INT_MAX`), else
                    // no change
                    if !matches!(
                        right,
                        Expression::Literal(mago_syntax::cst::Literal::Integer(i))
                            if i.value.is_some_and(|v| v <= i64::MAX as u64)
                    ) {
                        return None;
                    }
                    assert_method_calls.push(assert(
                        "assertCount",
                        vec![AssertArg::Expr(right), AssertArg::Expr(*args.first()?)],
                    ));
                    continue;
                }
                let method = if matches!(binary.operator, BinaryOperator::Identical(_)) {
                    "assertSame"
                } else {
                    "assertEquals"
                };
                assert_method_calls.push(assert(
                    method,
                    vec![AssertArg::Expr(right), AssertArg::Expr(left)],
                ));
            }
            // not supported expr
            _ => return None,
        }
    }
    // to keep order from binary
    assert_method_calls.reverse();
    Some(assert_method_calls)
}

/// The assertion statements and the final `return;`, each on a line of its own at `indent` (none for the first).
fn statements(source: &str, asserts: &[Assert<'_>], is_static: bool, indent: &str) -> Vec<String> {
    let receiver = if is_static { "self::" } else { "$this->" };
    let mut lines: Vec<String> = asserts
        .iter()
        .map(|assert| {
            let args: Vec<String> = assert
                .args
                .iter()
                .map(|arg| match arg {
                    AssertArg::Expr(expr) => {
                        let span = node_span(*expr);
                        reindent(source, span, line_indent(source, span.start), indent)
                    }
                    AssertArg::New(text) => text.clone(),
                })
                .collect();
            format!("{receiver}{}({});", assert.method, args.join(", "))
        })
        .collect();
    lines.push("return;".to_string());
    lines
}

/// The new body `{ ... }` of a function printed anew at `indent`.
fn body_anew(lines: &[String], indent: &str) -> String {
    let inner = format!("{indent}    ");
    format!(
        "{{\n{inner}{}\n{indent}}}",
        lines.join(&format!("\n{inner}"))
    )
}

fn params(
    source: &str,
    list: &mago_syntax::cst::FunctionLikeParameterList<'_>,
    indent: &str,
) -> String {
    let spans: Vec<Span> = list.parameters.iter().map(node_span).collect();
    maybe_multiline(
        source,
        list.left_parenthesis.end.offset as usize,
        &spans,
        indent,
    )
}

/// The edit of a closure: kept and changed when it has statements besides its `return`s, else printed anew.
fn closure_edit(source: &str, closure: &Closure<'_>, asserts: &[Assert<'_>]) -> Edit {
    let span = node_span(closure);
    let is_static = closure.r#static.is_some();
    let list = block_list(source, &closure.body);
    let kept: Vec<usize> = (0..list.stmts.len())
        .filter(|&i| !matches!(list.stmts[i], Statement::Return(_)))
        .collect();
    if kept.is_empty() && !list.nop {
        let indent = line_indent(source, span.start);
        let mut text = String::new();
        if is_static {
            text.push_str("static ");
        }
        text.push_str("function ");
        if closure.ampersand.is_some() {
            text.push('&');
        }
        text.push('(');
        text.push_str(&params(source, &closure.parameter_list, indent));
        text.push(')');
        if let Some(clause) = &closure.use_clause {
            let uses: Vec<String> = clause
                .variables
                .iter()
                .map(|v| {
                    format!(
                        "{}{}",
                        if v.ampersand.is_some() { "&" } else { "" },
                        String::from_utf8_lossy(v.variable.name)
                    )
                })
                .collect();
            text.push_str(&format!(" use ({})", uses.join(", ")));
        }
        text.push_str(": void ");
        text.push_str(&body_anew(
            &statements(source, asserts, is_static, &format!("{indent}    ")),
            indent,
        ));
        return Edit::new(span, text);
    }
    kept_closure_edit(source, closure, &list, &kept, asserts)
}

/// php-parser's format-preserving print of a closure whose statements lose their `return`s and gain the assertions:
/// the head with a `void` return type, each kept statement with what stood before it, the assertions after the last
/// kept one at its indentation, then what followed the last statement.
fn kept_closure_edit(
    source: &str,
    closure: &Closure<'_>,
    list: &List<'_>,
    kept: &[usize],
    asserts: &[Assert<'_>],
) -> Edit {
    let span = node_span(closure);
    let open = list.inner.start;
    let mut text = match &closure.return_type_hint {
        Some(hint) => {
            let hint = node_span(&hint.hint);
            format!(
                "{}void{}",
                &source[span.start..hint.start],
                &source[hint.end..open]
            )
        }
        None => {
            let after = match &closure.use_clause {
                Some(clause) => clause.right_parenthesis.end.offset as usize,
                None => closure.parameter_list.right_parenthesis.end.offset as usize,
            };
            format!(
                "{}: void{}",
                &source[span.start..after],
                &source[after..open]
            )
        }
    };
    let mut indent = "";
    for &i in kept {
        let stmt = node_span(list.stmts[i]);
        text.push_str(&source[list.before(source, i)..stmt.end]);
        indent = line_indent(source, stmt.start);
    }
    let mut tail_from = list.stmts.last().map_or(open, |s| node_span(*s).end);
    if let Some(nop) = list.nop_span(source) {
        text.push_str(&source[tail_from..nop.end]);
        tail_from = nop.end;
        if kept.is_empty() {
            indent = line_indent(source, nop.start);
        }
    }
    for line in statements(source, asserts, closure.r#static.is_some(), indent) {
        text.push('\n');
        text.push_str(indent);
        text.push_str(&line);
    }
    text.push_str(&source[tail_from..span.end]);
    Edit::new(span, text)
}

/// An arrow function becomes a closure, printed anew.
fn arrow_edit(source: &str, arrow: &ArrowFunction<'_>, asserts: &[Assert<'_>]) -> Edit {
    let span = node_span(arrow);
    let indent = line_indent(source, span.start);
    let mut text = String::from("function (");
    text.push_str(&params(source, &arrow.parameter_list, indent));
    text.push(')');
    let uses = resolve_from_arrow_function(arrow);
    if !uses.is_empty() {
        let uses: Vec<String> = uses.iter().map(|u| format!("${u}")).collect();
        text.push_str(&format!(" use ({})", uses.join(", ")));
    }
    text.push_str(": void ");
    text.push_str(&body_anew(
        &statements(source, asserts, false, &format!("{indent}    ")),
        indent,
    ));
    Edit::new(span, text)
}

/// `ClosureUsesResolver::resolveFromArrowFunction`: the names of the variables of the arrow function's expression
/// (`findInstancesOfScoped`: not in a closure, a function or a class), its parameters' (ASCII case-insensitively) and
/// `this` aside, each once, in the order found.
fn resolve_from_arrow_function(arrow: &ArrowFunction<'_>) -> Vec<String> {
    let param_names: Vec<&[u8]> = arrow
        .parameter_list
        .parameters
        .iter()
        .map(|p| &p.variable.name[1..])
        .collect();
    let mut names: Vec<String> = Vec::new();
    let mut stack = vec![Node::Expression(arrow.expression)];
    while let Some(node) = stack.pop() {
        match node {
            Node::Class(_) | Node::AnonymousClass(_) | Node::Closure(_) | Node::Function(_) => {
                if let Node::AnonymousClass(class) = node
                    && let Some(list) = &class.argument_list
                {
                    stack.push(Node::PartialArgumentList(list));
                }
                continue;
            }
            Node::DirectVariable(v) => {
                let name = &v.name[1..];
                if !param_names.iter().any(|p| p.eq_ignore_ascii_case(name)) {
                    let name = String::from_utf8_lossy(name).into_owned();
                    if name != "this" && !names.contains(&name) {
                        names.push(name);
                    }
                }
            }
            _ => {}
        }
        let mut children = Vec::new();
        node.visit_children(|child| children.push(child));
        stack.extend(children.into_iter().rev());
    }
    names
}
