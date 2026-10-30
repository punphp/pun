//! `lint/php-8.5/prefer-ord-first-byte`, ported from Rector 2.6.1's `OrdSingleByteRector` (rules/Php85/Rector/FuncCall).
//!
//! PHP 8.5 deprecates passing `ord()` more than one byte. A string argument becomes `$str[0]`; an int literal of
//! several digits becomes its first digit. Rector asks PHPStan for the argument's native type and value; Pun knows
//! them where the syntax does (see `lint_b_helpers::native` and `constant`).

use mago_syntax::cst::{Expression, Literal, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety};

use super::lint_b_helpers::{Constant, constant, dereferenceable, native, value};
use super::rector_code_quality::{args, named};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct PreferOrdFirstByte;

impl Rule for PreferOrdFirstByte {
    fn id(&self) -> &'static str {
        "lint/php-8.5/prefer-ord-first-byte"
    }

    fn min_php_version(&self) -> u32 {
        // PhpVersionFeature::DEPRECATE_ORD_WITH_MULTIBYTE_STRING
        80500
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::FunctionCall(c) = node else { return };
        let call = c;
        if !named(call, "ord") {
            return;
        }
        let Some(arg) = args(call).first().map(|a| value(a)) else {
            return;
        };
        let at = node_span(call).start;
        // php-parser has no node for parentheses.
        let expr = crate::syntax::unparenthesize(arg);
        let int_literal = match expr {
            Expression::Literal(Literal::String(s)) if s.value.is_some_and(|v| v.len() == 1) => {
                return;
            }
            Expression::Literal(Literal::Integer(i)) => {
                let value = i.value.unwrap();
                if value.to_string().len() == 1 {
                    return;
                }
                Some(value)
            }
            _ => None,
        };
        if native(ancestors, expr, at).is_none() {
            return;
        }
        let new = match int_literal {
            // refactorInt: the first digit.
            Some(v) => v.to_string()[..1].to_string(),
            None => {
                // refactorStringType
                if matches!(expr, Expression::ArrayAccess(_))
                    || matches!(constant(ancestors, expr, at), Some(Constant::Int(_)))
                {
                    return;
                }
                format!("{}[0]", dereferenceable(ctx.source, expr))
            }
        };
        out.push(Report {
            span: node_span(arg),
            message: "Pass `ord()` a single byte.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                // php-parser's span of the argument's value leaves out its parentheses, which stay.
                edits: vec![Edit::new(node_span(expr), new)],
            }),
        });
    }
}
