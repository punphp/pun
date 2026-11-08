//! `lint/complexity/prefer-in-array-over-foreach`, ported from Rector 2.8.4's `ForeachToInArrayRector`
//! (rules/CodeQuality/Rector/Foreach_).
//!
//! A `foreach` over an array whose only statement is `if ($item === <expr>) { return true; }` (or `==`, or
//! `false`), followed by `return false;` (the other boolean), becomes `return in_array(<expr>, $array, true);` (no
//! `true` for `==`, a `!` for a `false` found). The compared expression must not use the loop variable. In each
//! statement list Rector looks at the first `foreach` followed by a `return`, and stops there if it does not fit.
//!
//! Rector reads that the `foreach` runs over an array from PHPStan's native type; Pun from the syntax (an array
//! literal, a property or untouched parameter declared `array`, a local written only with array literals), and
//! skips the rest (lint.md Build 8). Rector also ignores an `else` or `elseif` of the `if`, which then goes; Pun
//! leaves such a loop alone.

use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::rector_batch5::{foreach_list, native_array, replace_run};
use super::rector_code_quality::{is_constant, text};
use super::rector_stmts::{HOLDERS, has_equal_variable, holder_list, if_view};
use crate::syntax::{node_span, same_code, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferInArrayOverForeach;

impl Rule for PreferInArrayOverForeach {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-in-array-over-foreach"
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
        let mut path = ancestors.to_vec();
        path.push(node);
        for key in 1..list.stmts.len() {
            let Statement::Return(r#return) = list.stmts[key] else {
                continue;
            };
            let Statement::Foreach(foreach) = list.stmts[key - 1] else {
                continue;
            };
            let Some(code) = refactor(ctx, &path, foreach, r#return) else {
                return;
            };
            let Some(edits) = replace_run(ctx, &list, key - 1, key, &code) else {
                return;
            };
            out.push(Report {
                span: node_span(foreach),
                message: "Use `in_array()` instead of this loop.".to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits,
                }),
            });
            return;
        }
    }
}

/// `refactor()`'s checks for a `foreach` and the `return` after it; the new `return` statement when they pass.
fn refactor(
    ctx: &Context<'_>,
    path: &[Node<'_, '_>],
    foreach: &Foreach<'_>,
    r#return: &Return<'_>,
) -> Option<String> {
    let source = ctx.source;
    // shouldSkipForeach
    let ForeachTarget::Value(target) = &foreach.target else {
        return None;
    };
    let body = foreach_list(source, foreach)?;
    if body.len() != 1 || !native_array(foreach.expression, path) {
        return None;
    }
    // A comment alone is php-parser's `Nop`, no `if`.
    let Some(Statement::If(r#if)) = body.stmts.first() else {
        return None;
    };
    let view = if_view(source, r#if);
    if view.else_.is_some() || !view.elseifs.is_empty() {
        return None;
    }
    // shouldSkipIf
    let Expression::Binary(condition) = unparenthesize(r#if.condition) else {
        return None;
    };
    let strict = match condition.operator {
        BinaryOperator::Identical(_) => true,
        BinaryOperator::Equal(_) => false,
        _ => return None,
    };
    // matchNodes: one side is the loop variable, the other the compared expression.
    let value = node_span(unparenthesize(target.value));
    let is_value = |e: &Expression<'_>| {
        matches!(unparenthesize(e), Expression::Variable(_))
            && same_code(source, node_span(unparenthesize(e)), value)
    };
    let compared = if is_value(condition.lhs) {
        condition.rhs
    } else if is_value(condition.rhs) {
        condition.lhs
    } else {
        return None;
    };
    if has_equal_variable(source, Node::Expression(compared), value) {
        return None;
    }
    // isIfBodyABoolReturnNode
    let Statement::Return(Return {
        value: Some(found), ..
    }) = view.stmts.stmts.first()?
    else {
        return None;
    };
    let bool_of = |e: &Expression<'_>| {
        if is_constant(e, "true") {
            Some(true)
        } else if is_constant(e, "false") {
            Some(false)
        } else {
            None
        }
    };
    let found = bool_of(found)?;
    let otherwise = bool_of(r#return.value?)?;
    // Cannot be "return true;" + "return true;".
    if found == otherwise {
        return None;
    }
    let strict = if strict { ", true" } else { "" };
    let call = format!(
        "in_array({}, {}{strict})",
        text(ctx, unparenthesize(compared)),
        text(ctx, unparenthesize(foreach.expression))
    );
    Some(if found {
        format!("return {call};")
    } else {
        format!("return !{call};")
    })
}
