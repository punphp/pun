//! The matching and safety checks shared by rector-laravel's AbortIf, ReportIf and ThrowIf rectors.

use mago_syntax::cst::{Access, BinaryOperator, Expression, Node, Variable};

use super::{argument_texts, member_name, replace};
use crate::rules::php_parser::{line_indent, tokens};
use crate::rules::rector_code_quality::{call_to, operand, text};
use crate::rules::rector_conditions::{comments_between, print_comments};
use crate::rules::rector_stmts::{expression, if_view};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report};

pub(super) fn visit(node: Node<'_, '_>, ctx: &Context<'_>, helper: &str, out: &mut Vec<Report>) {
    let Node::If(if_) = node else { return };
    let view = if_view(ctx.source, if_);
    if !view.elseifs.is_empty() || view.else_.is_some() || view.stmts.len() != 1 {
        return;
    }
    let statement = view.stmts.stmts[0];
    let Some(expr) = expression(statement) else {
        return;
    };
    let args = if helper == "throw" {
        let Expression::Throw(throw) = expr else {
            return;
        };
        if unsafe_throw(Node::Expression(throw.exception), view.condition) {
            return;
        }
        vec![text(ctx, throw.exception)]
    } else {
        let Some(call) = call_to(expr, helper) else {
            return;
        };
        let assigned = Node::Expression(view.condition).filter_map(|n| match n {
            Node::Assignment(a) if a.operator.is_assign() && !a.rhs.is_reference() => {
                Some(variable_name(a.lhs))
            }
            _ => None,
        });
        let variables = Node::Expression(expr).filter_map(|n| match n {
            Node::Variable(v) => Some(direct_name(v)),
            _ => None,
        });
        if variables.iter().any(|name| assigned.contains(name)) {
            return;
        }
        argument_texts(ctx, &call.argument_list)
    };
    let (condition, suffix) = match unparenthesize(view.condition) {
        Expression::UnaryPrefix(u) if u.operator.is_not() => (u.operand, "unless"),
        other => (other, "if"),
    };
    // An assignment or an `and`/`or` condition becomes an argument, not a top-level expression.
    let mut args_text = vec![operand(ctx, condition, 200, None)];
    args_text.extend(args.into_iter().map(str::to_string));
    let span = node_span(if_);
    let tokens = tokens(ctx.source);
    let comments = comments_between(&tokens, view.stmts.inner.start, node_span(statement).start);
    let nl = format!("\n{}", line_indent(ctx.source, span.start));
    let comments = print_comments(ctx.source, &comments, &nl);
    let code = format!("{helper}_{suffix}({});", args_text.join(", "));
    let code = if comments.is_empty() {
        code
    } else {
        format!("{comments}{nl}{code}")
    };
    out.push(replace(
        span,
        format!("Use `{helper}_{suffix}()` for this condition."),
        code,
    ));
}

fn direct_name<'a>(v: &Variable<'a>) -> Option<&'a [u8]> {
    match v {
        Variable::Direct(v) => Some(&v.name[1..]),
        _ => None,
    }
}

fn variable_name<'a>(expr: &'a Expression<'a>) -> Option<&'a [u8]> {
    match unparenthesize(expr) {
        Expression::Variable(v) => direct_name(v),
        Expression::Access(Access::Property(p)) => member_name(&p.property),
        Expression::Access(Access::StaticProperty(p)) => direct_name(&p.property),
        _ => None,
    }
}

fn first_short_circuit<'a>(node: Node<'a, 'a>) -> Option<&'a Expression<'a>> {
    if let Node::Binary(b) = node
        && matches!(b.operator, BinaryOperator::And(_) | BinaryOperator::Or(_))
    {
        return Some(b.rhs);
    }
    let mut found = None;
    node.visit_children(|child| {
        if found.is_none() {
            found = first_short_circuit(child);
        }
    });
    found
}

fn unsafe_throw(node: Node<'_, '_>, condition: &Expression<'_>) -> bool {
    if matches!(
        node,
        Node::MethodCall(_)
            | Node::StaticMethodCall(_)
            | Node::FunctionCall(_)
            | Node::ArrayAccess(_)
            | Node::PropertyAccess(_)
            | Node::StaticPropertyAccess(_)
    ) {
        return true;
    }
    if let Node::Variable(v) = node
        && let Some(right) = first_short_circuit(Node::Expression(condition))
        && Node::Expression(right)
            .filter_map(|n| match n {
                Node::Assignment(a)
                    if a.operator.is_assign()
                        && !a.rhs.is_reference()
                        && matches!(unparenthesize(a.lhs), Expression::Variable(_)) =>
                {
                    Some(variable_name(a.lhs))
                }
                _ => None,
            })
            .contains(&direct_name(v))
    {
        return true;
    }
    let mut unsafe_ = false;
    node.visit_children(|child| unsafe_ = unsafe_ || unsafe_throw(child, condition));
    unsafe_
}
