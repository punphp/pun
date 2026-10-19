//! `lint/style/prefer-php-version-id`, ported from Rector 2.6.1's `VersionCompareFuncCallToConstantRector`
//! (rules/CodingStyle/Rector/FuncCall).
//!
//! `version_compare(PHP_VERSION, '8.1', '>=')` becomes `PHP_VERSION_ID >= 80100`: `PHP_VERSION` turns into
//! `PHP_VERSION_ID`, a version string into its number (`PhpVersionFactory::createIntVersion`), and the operator into
//! the comparison (`==` into `===`, `!=` into `!==`). The other argument must be `PHP_VERSION` or a string, and the
//! operator a string `version_compare()` knows.

use mago_syntax::cst::{Argument, Expression, Literal, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::lint_b_helpers::value;
use super::rector_batch5::int_version;
use super::rector_code_quality::{Prec, args, in_parent, named};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferPhpVersionId;

impl Rule for PreferPhpVersionId {
    fn id(&self) -> &'static str {
        "lint/style/prefer-php-version-id"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(call) = node else {
            return;
        };
        let args = args(call);
        if !named(call, "version_compare")
            || args.len() != 3
            || args.iter().any(|a| matches!(a, Argument::Named(_)))
        {
            return;
        }
        let (left, right) = (value(args[0]), value(args[1]));
        if !is_php_version(left) && !is_php_version(right) {
            return;
        }
        let (Some(left), Some(right)) = (operand(left), operand(right)) else {
            return;
        };
        let Expression::Literal(Literal::String(op)) = value(args[2]) else {
            return;
        };
        // OPERATOR_TO_COMPARISON, with the precedence of each comparison.
        let (op, p) = match op.value.unwrap_or_default() {
            b"=" | b"==" | b"eq" => ("===", 80),
            b"!=" | b"<>" | b"ne" => ("!==", 80),
            b">" | b"gt" => (">", 70),
            b"<" | b"lt" => ("<", 70),
            b">=" | b"ge" => (">=", 70),
            b"<=" | b"le" => ("<=", 70),
            _ => return,
        };
        let span = node_span(call);
        let code = in_parent(
            span,
            ancestors,
            format!("{left} {op} {right}"),
            Prec::Infix(p, p, p),
        );
        out.push(Report {
            span,
            message: "Compare `PHP_VERSION_ID` instead of calling `version_compare()`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, code)],
            }),
        });
    }
}

/// `isPhpVersionConstant`: the constant `PHP_VERSION`, case and all.
fn is_php_version(expr: &Expression<'_>) -> bool {
    matches!(expr, Expression::ConstantAccess(c) if c.name.value().strip_prefix(b"\\").unwrap_or(c.name.value()) == b"PHP_VERSION")
}

/// `getNewNodeForArg`: `PHP_VERSION_ID`, or the number of a version string.
fn operand(expr: &Expression<'_>) -> Option<String> {
    if is_php_version(expr) {
        return Some("PHP_VERSION_ID".to_string());
    }
    let Expression::Literal(Literal::String(s)) = expr else {
        return None;
    };
    Some(int_version(&String::from_utf8_lossy(s.value?)).to_string())
}
