//! `lint/complexity/prefer-coalesce-chain`, ported from Rector 2.8.4's
//! `ConsecutiveNullCompareReturnsToNullCoalesceQueueRector` (rules/CodeQuality/Rector/If_), with
//! `IfManipulator::matchIfNotNullReturnValue`.
//!
//! Two or more `if ($x !== null) { return $x; }` statements of a list, followed only by `return null;`, by
//! `return <expr>;` or by `throw`, become one `return $a ?? $b ?? <expr>;` (a `throw` as a throw expression). Any
//! other statement after the first such `if` keeps the list as it is. Before PHP 8.0, which has no throw expression,
//! a `throw` keeps it too.
//!
//! Rector prints a kept operand as it was written, without the parentheses `??` may need around it (`$c ?: $d`
//! becomes `... ?? $c ?: $d`, which PHP reads differently); Pun adds them. Rector also matches an `if` with an
//! `else`, which then goes; Pun does not match such an `if`.

use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::rector_batch5::replace_run;
use super::rector_code_quality::{Prec, equal, is_constant, operand, wrap};
use super::rector_stmts::{HOLDERS, holder_list, if_view};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferCoalesceChain;

impl Rule for PreferCoalesceChain {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-coalesce-chain"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(HOLDERS)
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let source = ctx.source;
        let Some(list) = holder_list(source, node, ancestors) else {
            return;
        };
        let mut matched: Vec<(usize, &Expression)> = Vec::new();
        for (key, statement) in list.stmts.iter().enumerate() {
            let Statement::If(r#if) = statement else {
                continue;
            };
            let Some(compared) = not_null_return(ctx, r#if) else {
                continue;
            };
            if key + 1 >= list.stmts.len() {
                return;
            }
            matched.push((key, compared));
        }
        // At least two coalescing nodes are needed.
        if matched.len() < 2 || list.nop {
            return;
        }
        let first = matched[0].0;
        let mut append: Option<&Expression> = None;
        for key in first..list.stmts.len() {
            if matched.iter().any(|(k, _)| *k == key) {
                continue;
            }
            match list.stmts[key] {
                // A throw expression needs PHP 8.0.
                Statement::Expression(e)
                    if matches!(unparenthesize(e.expression), Expression::Throw(_))
                        && ctx.php_version >= 80000 =>
                {
                    append = Some(unparenthesize(e.expression));
                }
                Statement::Return(Return {
                    value: Some(value), ..
                }) => {
                    if !is_constant(value, "null") {
                        append = Some(value);
                    }
                }
                _ => return,
            }
        }
        // createCoalesceReturn: `(a ?? b) ?? c`, each operand in parentheses where `??` needs them.
        let coalesce = Prec::Infix(140, 140, 141);
        let mut code = format!(
            "{} ?? {}",
            operand(ctx, matched[0].1, 140, Some(140)),
            operand(ctx, matched[1].1, 141, None)
        );
        for next in matched[2..].iter().map(|(_, e)| *e).chain(append) {
            code = format!(
                "{} ?? {}",
                wrap(&code, coalesce, 140, Some(140)),
                operand(ctx, next, 141, None)
            );
        }
        let last = list.stmts.len() - 1;
        let Some(edits) = replace_run(ctx, &list, first, last, &format!("return {code};"))
        else {
            return;
        };
        out.push(Report {
            span: node_span(list.stmts[first]),
            message: "Return the first value that is not null with `??`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `IfManipulator::matchIfNotNullReturnValue`: `if ($x !== null) { return $x; }` (or `null !== $x`), without
/// `elseif` and `else`; the compared expression.
fn not_null_return<'a>(ctx: &Context<'_>, r#if: &'a If<'a>) -> Option<&'a Expression<'a>> {
    let view = if_view(ctx.source, r#if);
    if view.else_.is_some() || !view.elseifs.is_empty() || view.stmts.len() != 1 {
        return None;
    }
    let Statement::Return(Return {
        value: Some(returned),
        ..
    }) = view.stmts.stmts.first()?
    else {
        return None;
    };
    let Expression::Binary(b) = unparenthesize(r#if.condition) else {
        return None;
    };
    if !matches!(b.operator, BinaryOperator::NotIdentical(_)) {
        return None;
    }
    if equal(ctx, b.lhs, returned) && is_constant(b.rhs, "null") {
        return Some(b.lhs);
    }
    if equal(ctx, b.rhs, returned) && is_constant(b.lhs, "null") {
        return Some(b.rhs);
    }
    None
}
