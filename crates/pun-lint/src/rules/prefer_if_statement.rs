//! `lint/complexity/prefer-if-statement`, ported from Rector 2.8.4's `InlineIfToExplicitIfRector`
//! (rules/CodeQuality/Rector/Expression).
//!
//! `is_null($id) && $id = 5;` becomes `if (is_null($id)) { $id = 5; }`, and `is_null($id) || $id = 5;` becomes
//! `if (!is_null($id)) { $id = 5; }`. Only a statement that is one `&&` or `||` whose left side is certainly a `bool`
//! changes. `defined('X') || define('X', 1);` stays, and so does a chain of calls of one method of `$this`.
//!
//! Rector drops a comment between the operands; Pun leaves a statement with such a comment alone, and one that ends
//! in `?>` instead of `;`.

use mago_syntax::cst::{
    BinaryOperator, Call, ClassLikeMemberSelector, Expression, ExpressionStatement, Node, NodeKind,
};
use pun_problem::{Edit, Safety};

use super::php_parser::line_indent;
use super::rector_code_quality::{call_to, inverse_node, text};
use super::rector_stmts::reindent;
use super::syntax_types::is_bool;
use crate::syntax::{is_this, node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferIfStatement;

impl Rule for PreferIfStatement {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-if-statement"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::ExpressionStatement])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        if let Node::ExpressionStatement(statement) = node {
            check(statement, ancestors, ctx, out);
        }
    }
}

fn check(
    s: &ExpressionStatement<'_>,
    ancestors: &[Node<'_, '_>],
    ctx: &Context<'_>,
    out: &mut Vec<Report>,
) {
    let whole = unparenthesize(s.expression);
    let Expression::Binary(binary) = whole else {
        return;
    };
    let and = match binary.operator {
        BinaryOperator::And(_) => true,
        BinaryOperator::Or(_) => false,
        _ => return,
    };
    let (left, right) = (unparenthesize(binary.lhs), unparenthesize(binary.rhs));
    let negated = match left {
        Expression::UnaryPrefix(not) if not.operator.is_not() => not.operand,
        other => other,
    };
    if call_to(negated, "defined").is_some() {
        return;
    }
    if is_same_local_method_call_chain(whole) {
        return;
    }
    if !is_bool(ancestors, left) {
        return;
    }
    // The code outside the two operands: parentheses, the operator and the `;`. A comment there has no place in
    // the `if`.
    let (span, l, r) = (node_span(s), node_span(left), node_span(right));
    let source = ctx.source;
    let outside = [
        &source[span.start..l.start],
        &source[l.end..r.start],
        &source[r.end..span.end],
    ];
    if outside.iter().any(|code| code.contains(['/', '#'])) || !outside[2].ends_with(';') {
        return;
    }
    let condition = if and {
        text(ctx, left).to_string()
    } else {
        inverse_node(ctx, left, None).0
    };
    let indent = line_indent(source, span.start);
    let body = reindent(source, r, indent, &format!("{indent}    "));
    out.push(Report::fixed(
        span,
        "Write the inline condition as an `if` statement.",
        Safety::Safe,
        vec![Edit::new(
            span,
            format!("if ({condition}) {{\n{indent}    {body};\n{indent}}}"),
        )],
    ));
}

/// `isSameLocalMethodCallChain`: every operand of the `&&` and `||` tree calls the same method of `$this`.
fn is_same_local_method_call_chain(expr: &Expression<'_>) -> bool {
    let mut leaves = Vec::new();
    collect_operands(expr, &mut leaves);
    let mut first: Option<&[u8]> = None;
    for leaf in leaves {
        let Expression::Call(Call::Method(call)) = leaf else {
            return false;
        };
        let ClassLikeMemberSelector::Identifier(name) = &call.method else {
            return false;
        };
        if !is_this(call.object) {
            return false;
        }
        match first {
            None => first = Some(name.value),
            Some(f) if f.eq_ignore_ascii_case(name.value) => {}
            Some(_) => return false,
        }
    }
    true
}

fn collect_operands<'a>(expr: &'a Expression<'a>, leaves: &mut Vec<&'a Expression<'a>>) {
    match unparenthesize(expr) {
        Expression::Binary(b)
            if matches!(b.operator, BinaryOperator::And(_) | BinaryOperator::Or(_)) =>
        {
            collect_operands(b.lhs, leaves);
            collect_operands(b.rhs, leaves);
        }
        leaf => leaves.push(leaf),
    }
}
