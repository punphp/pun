//! `lint/complexity/prefer-coalesce-over-foreach`, ported from Rector 2.8.4's `SimplifyForeachToCoalescingRector`
//! (rules/CodeQuality/Rector/Foreach_).
//!
//! A `foreach ($array as $key => $value)` over an array whose only statement is `if ($key === $x)` with one
//! statement, `return $value;` or `$target = $value;`, becomes `return $array[$x] ?? <next return value>;` (taking the
//! place of the `return` after it, which must follow) or `$target = $array[$x] ?? $target;`. A loop that returns or
//! assigns something other than its value stops the rule for the whole statement list.
//!
//! Rector reads that the loop runs over an array from PHPStan's native type; Pun from the syntax (an array literal, a
//! property or untouched parameter declared `array`, a local written only with array literals), and skips the rest
//! (lint.md Build 8).

use mago_syntax::cst::*;
use pun_problem::{Fix, Safety};

use super::rector_batch5::{foreach_list, merge, native_array, replace_run};
use super::rector_code_quality::{equal, operand, text};
use super::rector_stmts::{HOLDERS, holder_list, if_view};
use crate::syntax::unparenthesize;
use crate::{Context, Report, Rule};

pub struct PreferCoalesceOverForeach;

/// The statement inside the loop's `if`.
enum Inner<'a> {
    Return(&'a Expression<'a>),
    Assign(&'a Assignment<'a>),
}

impl Rule for PreferCoalesceOverForeach {
    fn id(&self) -> &'static str {
        "lint/complexity/prefer-coalesce-over-foreach"
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
        let mut edits = Vec::new();
        let mut skip_next = false;
        for key in 0..list.stmts.len() {
            if std::mem::take(&mut skip_next) {
                continue;
            }
            let Statement::Foreach(foreach) = list.stmts[key] else {
                continue;
            };
            let ForeachTarget::KeyValue(target) = &foreach.target else {
                continue;
            };
            let Some((condition, inner)) = match_inner(ctx, &path, foreach) else {
                continue;
            };
            // The other side of `$key === ...`.
            let checked = if equal(ctx, condition.lhs, target.key) {
                condition.rhs
            } else if equal(ctx, condition.rhs, target.key) {
                condition.lhs
            } else {
                return;
            };
            let fetch = format!(
                "{}[{}]",
                text(ctx, unparenthesize(foreach.expression)),
                text(ctx, unparenthesize(checked))
            );
            let replaced = match inner {
                Inner::Assign(assign) => {
                    if !equal(ctx, target.value, assign.rhs) {
                        return;
                    }
                    let var = text(ctx, unparenthesize(assign.lhs));
                    replace_run(ctx, &list, key, key, &format!("{var} = {fetch} ?? {var};"))
                }
                Inner::Return(value) => {
                    if !equal(ctx, target.value, value) {
                        return;
                    }
                    let Some(Statement::Return(next)) = list.stmts.get(key + 1) else {
                        continue;
                    };
                    let default = match next.value {
                        Some(v) => operand(ctx, v, 141, None),
                        None => operand(ctx, checked, 141, None),
                    };
                    skip_next = true;
                    replace_run(
                        ctx,
                        &list,
                        key,
                        key + 1,
                        &format!("return {fetch} ?? {default};"),
                    )
                }
            };
            let Some(replaced) = replaced else { return };
            edits.extend(replaced);
        }
        let edits = merge(edits);
        let Some(first) = edits.first() else { return };
        out.push(Report {
            span: first.span,
            message: "Read the array by key with `??` instead of this loop.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// `matchForeachReturnOrAssign`: one `if ($a === $b)` with one statement and no `else`, over a native array, holding
/// a `return` or an assignment (not to an array element).
fn match_inner<'a>(
    ctx: &Context<'_>,
    path: &[Node<'_, '_>],
    foreach: &'a Foreach<'a>,
) -> Option<(&'a Binary<'a>, Inner<'a>)> {
    let body = foreach_list(ctx.source, foreach)?;
    if body.len() != 1 {
        return None;
    }
    // A comment alone is php-parser's `Nop`, no `if`.
    let Some(Statement::If(r#if)) = body.stmts.first() else {
        return None;
    };
    let Expression::Binary(condition) = unparenthesize(r#if.condition) else {
        return None;
    };
    if !matches!(condition.operator, BinaryOperator::Identical(_)) {
        return None;
    }
    let view = if_view(ctx.source, r#if);
    if view.stmts.len() != 1 || view.else_.is_some() || !view.elseifs.is_empty() {
        return None;
    }
    if !native_array(foreach.expression, path) {
        return None;
    }
    let inner = match view.stmts.stmts.first()? {
        Statement::Return(Return {
            value: Some(value), ..
        }) => Inner::Return(value),
        Statement::Expression(e) => {
            let Expression::Assignment(assign) = unparenthesize(e.expression) else {
                return None;
            };
            if !matches!(assign.operator, AssignmentOperator::Assign(_))
                || matches!(assign.rhs, Expression::UnaryPrefix(u) if u.operator.is_reference())
                || matches!(
                    unparenthesize(assign.lhs),
                    Expression::ArrayAccess(_) | Expression::ArrayAppend(_)
                )
            {
                return None;
            }
            Inner::Assign(assign)
        }
        _ => return None,
    };
    Some((condition, inner))
}
