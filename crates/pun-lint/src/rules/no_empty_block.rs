//! `lint/dead-code/no-empty-block`, ported from Rector 2.6.1's `RemoveDeadIfForeachForRector`
//! (rules/DeadCode/Rector/For_), with `ConditionInverter` and `BinaryOpManipulator::invertCondition`.
//!
//! In a statement list, an `if`, `for` or `foreach` whose body has no statement (php-parser's `stmts === []`: a
//! comment is a `Nop` statement) is removed, unless:
//!
//! - an `if` has `elseif` branches, or its condition calls something or assigns (`=`);
//! - a `for` has a variable in its header that the statements after it use;
//! - a `foreach` iterates over something that calls or assigns, or the statements after it use a variable of the
//!   iterated expression or of the value (Rector checks the value twice and never the key).
//!
//! An `if` with an empty body and an `else` becomes `if (<inverted condition>) <else body>`. Rector reprints the
//! whole `if` then, braces and all; its printer drops the parentheses of `!($a ? $b : $c)`, which changes what the
//! condition means, and Pun keeps them (lint.md: meaning over bytes).
//!
//! Mago 1.57.0's `no-empty-loop` (mapped "same") differs, checked on these fixtures: it looks at `while` and
//! `do`-`while` too but never at `if`, counts a body of only comments as empty, and deletes the loop even when its
//! header calls something or a later statement uses its variables (it fires on Rector's `skip_*` fixtures). This
//! rule follows Rector.

use mago_syntax::cst::{
    AssignmentOperator, Binary, BinaryOperator, Expression, ForBody, ForeachBody, ForeachTarget,
    IfBody, Node, NodeKind, Sequence, Statement, UnaryPrefixOperator,
};
use pun_problem::{Edit, Fix, Safety, Span};

use super::php_parser::{
    body_list, enclosing, keeps_parentheses, line_indent, moved_text, open_list, precedence,
    remove, tokens,
};
use super::used_variable::{try_of, used_in_next};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoEmptyBlock;

impl Rule for NoEmptyBlock {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-empty-block"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::If, NodeKind::For, NodeKind::Foreach])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(body) = body_of(node) else { return };
        // A cheap look before the file is read into tokens: no statement in the body, by Mago's count.
        let looks_empty = match body {
            Body::One(Statement::Block(b)) => b.statements.iter().all(dropped),
            Body::One(s) => dropped(s),
            Body::Open(s, ..) => s.iter().all(dropped),
        };
        if !looks_empty {
            return;
        }
        let source = ctx.source;
        let tokens = tokens(ctx);
        let list = match body {
            Body::One(s) => body_list(&tokens, s),
            Body::Open(s, from, to) => open_list(&tokens, s, from, to),
        };
        if !list.is_empty() {
            return;
        }
        // Rector works on the list that holds the statement.
        let (list, key) = enclosing(source, &tokens, ancestors);
        let try_ = try_of(ancestors);
        let fix = match node {
            Node::If(r#if) => {
                // With the `else`: its statements, and the text from the end of the `if` body to the end of the `else`.
                let (else_if, else_part) = match &r#if.body {
                    IfBody::Statement(b) => (
                        !b.else_if_clauses.is_empty(),
                        b.else_clause.as_ref().map(|e| {
                            let gone = Span::new(node_span(b.statement).end, node_span(e).end);
                            (body_list(&tokens, e.statement), gone)
                        }),
                    ),
                    IfBody::ColonDelimited(b) => (
                        !b.else_if_clauses.is_empty(),
                        b.else_clause.as_ref().map(|e| {
                            let endif = b.endif.span.start.offset as usize;
                            let gone = Span::new(node_span(&e.r#else).start, endif);
                            let list = open_list(
                                &tokens,
                                &e.statements,
                                e.colon.end.offset as usize,
                                endif,
                            );
                            (list, gone)
                        }),
                    ),
                };
                if else_if {
                    return;
                }
                match else_part {
                    // `$node->stmts = $node->else->stmts`: the same empty list, so php-parser's printer keeps the
                    // body as it is and writes only the condition again.
                    Some((else_list, gone)) if else_list.is_empty() => vec![
                        Edit::new(
                            node_span(r#if.condition),
                            inverted_condition(source, r#if.condition),
                        ),
                        Edit::new(gone, ""),
                    ],
                    Some((else_list, _)) => {
                        let span = node_span(r#if);
                        let indent = line_indent(source, span.start);
                        let inner = format!("{indent}    ");
                        let body = moved_text(source, &tokens, &else_list, &inner);
                        let condition = inverted_condition(source, r#if.condition);
                        vec![Edit::new(
                            span,
                            format!("if ({condition}) {{\n{inner}{body}\n{indent}}}"),
                        )]
                    }
                    None if has_side_effect(r#if.condition) => return,
                    None => remove(source, &tokens, &list, &[key]),
                }
            }
            Node::For(r#for) => {
                let header = r#for
                    .initializations
                    .iter()
                    .chain(r#for.conditions.iter())
                    .chain(r#for.increments.iter());
                for expr in header {
                    if variables(Node::Expression(expr))
                        .iter()
                        .any(|name| used_in_next(&list, key + 1, try_, name))
                    {
                        return;
                    }
                }
                remove(source, &tokens, &list, &[key])
            }
            _ => {
                let Node::Foreach(foreach) = node else { return };
                // the iterated expression itself may have side effects
                if has_side_effect(foreach.expression) {
                    return;
                }
                let value = match &foreach.target {
                    ForeachTarget::Value(t) => t.value,
                    ForeachTarget::KeyValue(t) => t.value,
                };
                for expr in [foreach.expression, value, value] {
                    if variables(Node::Expression(expr))
                        .iter()
                        .any(|name| used_in_next(&list, key + 1, try_, name))
                    {
                        return;
                    }
                }
                remove(source, &tokens, &list, &[key])
            }
        };
        out.push(Report {
            span: node_span(&node),
            message: "This statement has an empty body and does nothing; remove it.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: fix,
            }),
        });
    }
}

/// The statements php-parser does not keep.
fn dropped(s: &Statement<'_>) -> bool {
    matches!(
        s,
        Statement::Noop(_) | Statement::OpeningTag(_) | Statement::ClosingTag(_)
    )
}

/// A body: one statement (maybe a block), or the statements of an alternative syntax, from after its `:` to the
/// next branch or the keyword that ends it.
#[derive(Clone, Copy)]
enum Body<'ast, 'arena> {
    One(&'ast Statement<'arena>),
    Open(&'ast Sequence<'arena, Statement<'arena>>, usize, usize),
}

/// The body of an `if`, `for` or `foreach`, whose statement list is php-parser's `$stmt->stmts`.
fn body_of<'ast, 'arena>(node: Node<'ast, 'arena>) -> Option<Body<'ast, 'arena>> {
    let after = |s: mago_span::Span| s.end.offset as usize;
    let start = |s: mago_span::Span| s.start.offset as usize;
    Some(match node {
        Node::If(r#if) => match &r#if.body {
            IfBody::Statement(b) => Body::One(b.statement),
            IfBody::ColonDelimited(b) => {
                let to = b
                    .else_if_clauses
                    .first()
                    .map(|c| start(mago_span::HasSpan::span(c)))
                    .or_else(|| b.else_clause.as_ref().map(|c| start(c.r#else.span)))
                    .unwrap_or(start(b.endif.span));
                Body::Open(&b.statements, after(b.colon), to)
            }
        },
        Node::For(r#for) => match &r#for.body {
            ForBody::Statement(s) => Body::One(s),
            ForBody::ColonDelimited(b) => {
                Body::Open(&b.statements, after(b.colon), start(b.end_for.span))
            }
        },
        _ => {
            let Node::Foreach(f) = node else { return None };
            match &f.body {
                ForeachBody::Statement(s) => Body::One(s),
                ForeachBody::ColonDelimited(b) => {
                    Body::Open(&b.statements, after(b.colon), start(b.end_foreach.span))
                }
            }
        }
    })
}

/// `hasNodeSideEffect`: a call, a `new`, or an assignment with `=` anywhere in the expression.
fn has_side_effect(expr: &Expression<'_>) -> bool {
    fn walk(node: Node<'_, '_>) -> bool {
        match node {
            Node::Call(_)
            | Node::PartialApplication(_)
            | Node::Instantiation(_)
            | Node::AnonymousClass(_) => {
                return true;
            }
            // `$a = &$b` is php-parser's AssignRef, not an Assign.
            Node::Assignment(a)
                if matches!(a.operator, AssignmentOperator::Assign(_))
                    && !matches!(
                        a.rhs,
                        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Reference(_))
                    ) =>
            {
                return true;
            }
            _ => {}
        }
        let mut found = false;
        node.visit_children(|child| found = found || walk(child));
        found
    }
    walk(Node::Expression(expr))
}

/// The names (without `$`) of the variables in `node`, as `BetterNodeFinder::findInstanceOf(..., Variable::class)`
/// and `getName` give them: a variable variable has no name, which Rector turns into `''`.
fn variables(node: Node<'_, '_>) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    fn walk(node: Node<'_, '_>, parent: Option<Node<'_, '_>>, out: &mut Vec<Vec<u8>>) {
        match node {
            Node::DirectVariable(v) => {
                if !matches!(
                    parent,
                    Some(Node::PropertyAbstractItem(_) | Node::PropertyConcreteItem(_))
                ) {
                    out.push(v.name[1..].to_vec());
                }
                return;
            }
            Node::IndirectVariable(_) | Node::NestedVariable(_) => out.push(Vec::new()),
            _ => {}
        }
        node.visit_children(|child| walk(child, Some(node), out));
    }
    walk(node, None, &mut out);
    out
}

/// `ConditionInverter::createInvertedCondition`, printed.
pub(crate) fn inverted_condition(source: &str, condition: &Expression<'_>) -> String {
    let text = |e: &Expression| {
        let s = node_span(e);
        &source[s.start..s.end]
    };
    let condition = unparenthesize(condition);
    match condition {
        Expression::Binary(binary) => {
            if let Some(op) = inverted_operator(binary) {
                // A new node: php-parser prints its operands without parentheses unless precedence needs them.
                let operand = |e: &Expression| -> String {
                    let inner = unparenthesize(e);
                    let own = match inner {
                        Expression::Binary(b) => precedence(&b.operator),
                        _ if keeps_parentheses(inner, None) => 0,
                        _ => u8::MAX,
                    };
                    if own <= precedence(&binary.operator) {
                        format!("({})", text(inner))
                    } else {
                        text(inner).to_string()
                    }
                };
                return format!("{} {op} {}", operand(binary.lhs), operand(binary.rhs));
            }
            if matches!(binary.operator, BinaryOperator::Instanceof(_)) {
                return format!("!{}", text(condition));
            }
            format!("!({})", text(condition))
        }
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
            text(unparenthesize(u.operand)).to_string()
        }
        // `!$a = f()` is `!($a = f())` in PHP, and Rector prints it so.
        Expression::Assignment(_) => format!("!{}", text(condition)),
        _ if keeps_parentheses(condition, None) => format!("!({})", text(condition)),
        _ => format!("!{}", text(condition)),
    }
}

/// `BinaryOpManipulator::invertCondition` as `createInvertedCondition` uses it: the comparison's inverse, unless a
/// side is `||`. `||` itself inverts to `&&`, which `createInvertedCondition` then turns into `!(...)`.
fn inverted_operator(binary: &Binary<'_>) -> Option<&'static str> {
    let is_or = |e: &Expression| {
        matches!(
            unparenthesize(e),
            Expression::Binary(Binary {
                operator: BinaryOperator::Or(_),
                ..
            })
        )
    };
    if is_or(binary.lhs) || is_or(binary.rhs) {
        return None;
    }
    Some(match binary.operator {
        BinaryOperator::Identical(_) => "!==",
        BinaryOperator::NotIdentical(_) => "===",
        BinaryOperator::Equal(_) => "!=",
        BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => "==",
        BinaryOperator::GreaterThan(_) => "<=",
        BinaryOperator::LessThan(_) => ">=",
        BinaryOperator::GreaterThanOrEqual(_) => "<",
        BinaryOperator::LessThanOrEqual(_) => ">",
        _ => return None,
    })
}
