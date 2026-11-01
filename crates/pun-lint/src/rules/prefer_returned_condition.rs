//! `lint/control-flow/prefer-returned-condition`, ported from Rector 2.6.1's `SimplifyIfReturnBoolRector`
//! (rules/CodeQuality/Rector/If_).
//!
//! An `if` without `elseif` or `else` whose one statement is `return true;` or `return false;`, followed by a
//! `return` of the other boolean, becomes one `return` of the condition: `if (c) { return true; } return false;` is
//! `return c;`, and with the booleans swapped `return !c;` (`!==` for `===`, `!=` for `==`, the operand of a `!`).
//! After `return false;` for a `!==` or `!=` condition, the next `return` may be anything; it must be `true` for
//! the change. A condition with `!=` anywhere in its code (`!=`, `!==`, `<>`) is only taken after `return false;`
//! when the condition itself is one of those comparisons. Rector changes the first such pair of a statement list.
//!
//! `ExprBoolCaster::boolCastOrNullCompareIfNeeded` decides the returned value: a condition of the file whose type is
//! not `bool` (PHPStan's, docblocks read) gets `(bool)`, unless it is a binary operation or a `!`; one whose type
//! has `null` and is else always truthy (objects, non-falsy constants) becomes `... !== null`. The comments of the
//! `if`, of the `return` after it and of the `return` in it go before the new `return`.
//!
//! php-parser's printer decides the bytes: a condition of the file keeps its code; the comparison, `!` or `!== null`
//! Rector builds is printed anew, and with it every binary operation it holds (`BetterStandardPrinter`'s
//! `wrapBinaryOpWithBrackets`), with php-parser's spacing and only the parentheses precedence needs; the other
//! operands keep their code. Where an operand that keeps its code would need parentheses there, Rector's output
//! means something else, and Pun leaves the code alone. See [`super::stmt_splice`] for the statements.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::int::TInteger;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    BinaryOperator, Expression, Node, NodeKind, Return, Statement, UnaryPrefixOperator,
};
use mago_syntax::token::TokenKind;
use pun_problem::{Fix, Safety, Span};

use super::class_method::type_of;
use super::php_parser::{List, Token, enclosing, previous_token_end, tokens};
use super::rector_code_quality::{binary, is_constant, needs_parens, prec, text};
use super::rector_conditions::{comments_between, if_parts};
use super::stmt_splice::{Change, Code, NewStatement, run_edits};
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferReturnedCondition;

impl Rule for PreferReturnedCondition {
    fn id(&self) -> &'static str {
        "lint/control-flow/prefer-returned-condition"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::If])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::If(_) = node else { return };
        let source = ctx.source;
        let tokens = tokens(source);
        let (list, index) = enclosing(source, &tokens, ancestors);
        let facts = facts(ctx);
        // `refactor()` changes the first pair of the list it takes
        let Some((at, (code, inner))) = (1..list.stmts.len())
            .find_map(|k| new_return(ctx, &tokens, &facts, &list, k).map(|c| (k, c)))
        else {
            return;
        };
        if at - 1 != index {
            return;
        }
        let r#if = node_span(list.stmts[at - 1]);
        let r#return = node_span(list.stmts[at]);
        // `CommentsMerger::keepComments($newReturn, [$if, $return, $ifInnerNode])`
        let comments: Vec<Span> = [r#if, r#return, inner]
            .into_iter()
            .flat_map(|s| {
                comments_between(
                    &tokens,
                    previous_token_end(source, &tokens, s.start),
                    s.start,
                )
            })
            .collect();
        let edits = run_edits(source, &tokens, ancestors, &|chain| {
            let span = node_span(&chain[chain.len() - 1]);
            if span == r#if {
                Some(Change::Remove)
            } else if span == r#return {
                Some(Change::Replace(vec![NewStatement {
                    comments: comments.clone(),
                    code: Code::Text(format!("return {code};")),
                }]))
            } else {
                None
            }
        });
        let Some(edits) = edits else { return };
        out.push(Report {
            span: Span::new(r#if.start, r#return.end),
            message: "Return the condition instead of `true` and `false`.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// The returned value of the `return` that takes the place of the `if` at `k - 1` of `list` and the `return` at `k`,
/// when `refactor()` changes them, and the span of the `return` in the `if`.
fn new_return(
    ctx: &Context<'_>,
    tokens: &[Token],
    facts: &Facts,
    list: &List<'_, '_>,
    k: usize,
) -> Option<(String, Span)> {
    let (Statement::If(r#if), Statement::Return(next)) = (list.stmts[k - 1], list.stmts[k]) else {
        return None;
    };
    let parts = if_parts(tokens, r#if);
    // `shouldSkipIfAndReturn`
    if parts.has_elseifs || parts.has_else || parts.stmts.stmts.len() != 1 || parts.stmts.nop {
        return None;
    }
    let inner_statement = parts.stmts.stmts[0];
    let Statement::Return(Return {
        value: Some(inner), ..
    }) = inner_statement
    else {
        return None;
    };
    let inner_span = node_span(inner_statement);
    let next = next.value?;
    let inner_true = is_constant(inner, "true");
    if !inner_true && !is_constant(inner, "false") {
        return None;
    }
    let next_true = is_constant(next, "true");
    let next_bool = next_true || is_constant(next, "false");
    if next_bool && inner_true == next_true {
        return None;
    }
    let cond = unparenthesize(r#if.condition);
    let not_equal = |op: &BinaryOperator<'_>| {
        matches!(
            op,
            BinaryOperator::NotIdentical(_)
                | BinaryOperator::NotEqual(_)
                | BinaryOperator::AngledNotEqual(_)
        )
    };
    let is_not_comparison = comparison(cond).is_some_and(|(_, op, _)| not_equal(op));
    if inner_true || !prints_not_equal(ctx, tokens, cond) {
        if !next_bool {
            return None;
        }
    } else if !is_not_comparison {
        return None;
    }
    // `resolveReturn`
    let flipped = |op: &BinaryOperator<'_>| match op {
        BinaryOperator::Identical(_) => Some("!=="),
        BinaryOperator::NotIdentical(_) => Some("==="),
        BinaryOperator::Equal(_) => Some("!="),
        BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => Some("=="),
        _ => None,
    };
    let code = if inner_true {
        cast(ctx, facts, cond)?
    } else if let Some((left, op, right)) = comparison(cond)
        && let Some(flipped) = flipped(op)
    {
        // after `return false;`, a `!==` or `!=` needs `return true;` next
        if not_equal(op) && !next_true {
            return None;
        }
        fresh_infix(ctx, left, flipped, right, binary(op), MAX, MAX)?
    } else if let Expression::UnaryPrefix(u) = cond
        && matches!(u.operator, UnaryPrefixOperator::Not(_))
    {
        cast(ctx, facts, unparenthesize(u.operand))?
    } else {
        format!("!{}", fresh(ctx, cond, 30, MAX)?)
    };
    Some((code, inner_span))
}

/// A binary operation's operands and operator.
fn comparison<'a>(
    e: &'a Expression<'a>,
) -> Option<(
    &'a Expression<'a>,
    &'a BinaryOperator<'a>,
    &'a Expression<'a>,
)> {
    match e {
        Expression::Binary(b) => Some((b.lhs, &b.operator, b.rhs)),
        _ => None,
    }
}

/// php-parser's `MAX_PRECEDENCE`.
const MAX: i32 = 1000;

/// Whether `print($cond)` holds `!=`: an operator `!=`, `!==` or `<>` (printed `!=`), or a string with it.
fn prints_not_equal(ctx: &Context<'_>, tokens: &[Token], cond: &Expression<'_>) -> bool {
    let span = node_span(cond);
    tokens
        .iter()
        .filter(|t| t.start >= span.start && t.end <= span.end && !t.kind.is_comment())
        .any(|t| {
            t.kind == TokenKind::LessThanGreaterThan || ctx.source[t.start..t.end].contains("!=")
        })
}

/// `boolCastOrNullCompareIfNeeded` on an expression of the file, printed (`None` as in [`leaf`]). Where Mago's
/// facts give no type, PHPStan's is `mixed`.
fn cast(ctx: &Context<'_>, facts: &Facts, expr: &Expression<'_>) -> Option<String> {
    let not = matches!(expr, Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)));
    let binary_op = matches!(expr, Expression::Binary(b) if !b.operator.is_instanceof());
    // a `!` and the binary operations but `??` are never null
    let coalesce = matches!(expr, Expression::Binary(b) if b.operator.is_null_coalesce());
    if not || (binary_op && !coalesce) {
        return Some(text(ctx, expr).to_string());
    }
    let t = type_of(facts, expr);
    if let Some(t) = t
        && t.has_null()
    {
        let rest: Vec<&TAtomic> = t
            .types
            .iter()
            .filter(|a| !matches!(a, TAtomic::Null))
            .collect();
        if !rest.is_empty() && rest.iter().all(|a| always_truthy(a)) {
            return Some(format!("{} !== null", fresh(ctx, expr, 80, 80)?));
        }
    }
    if binary_op || t.is_some_and(TUnion::is_bool) {
        return Some(text(ctx, expr).to_string());
    }
    Some(format!("(bool) {}", leaf(ctx, expr, 10, MAX)?))
}

/// `StaticTypeAnalyzer::isAlwaysTruableType` on one type of a union without its `null`.
fn always_truthy(atomic: &TAtomic) -> bool {
    match atomic {
        TAtomic::Object(TObject::Named(n)) => n.intersection_types.is_none(),
        TAtomic::Object(TObject::Enum(_)) => true,
        TAtomic::Array(TArray::Keyed(k)) => k.parameters.is_none(),
        TAtomic::Array(TArray::List(l)) => l.known_elements.is_some() && l.element_type.is_never(),
        TAtomic::Scalar(s) => match s {
            TScalar::Bool(b) => b.value == Some(true),
            TScalar::Integer(TInteger::Literal(v)) => *v != 0,
            TScalar::Float(f) => f.get_literal_value().is_some_and(|v| v != 0.0),
            TScalar::String(_) => s.get_known_literal_string_value().is_some() && s.is_truthy(),
            _ => false,
        },
        _ => false,
    }
}

/// An expression printed by php-parser at a place of precedence `limit` (`lhs` as in `pPrefixOp`): a binary
/// operation anew, anything else with its code; `None` where that code needed parentheses php-parser does not add.
fn fresh(ctx: &Context<'_>, expr: &Expression<'_>, limit: i32, lhs: i32) -> Option<String> {
    let expr = unparenthesize(expr);
    match expr {
        Expression::Binary(b) => match operator(&b.operator) {
            Some(op) => fresh_infix(ctx, b.lhs, op, b.rhs, binary(&b.operator), limit, lhs),
            // `instanceof` is no binary operation for php-parser
            None => leaf(ctx, expr, limit, lhs),
        },
        _ => leaf(ctx, expr, limit, lhs),
    }
}

/// The code of an operand php-parser keeps, or `None` where it needs parentheses at its new place.
fn leaf(ctx: &Context<'_>, expr: &Expression<'_>, limit: i32, lhs: i32) -> Option<String> {
    let expr = unparenthesize(expr);
    (!needs_parens(prec(expr), limit, Some(lhs))).then(|| text(ctx, expr).to_string())
}

/// `pInfixOp` for a new binary operation of `left` and `right` at a place of precedence `limit`.
fn fresh_infix(
    ctx: &Context<'_>,
    left: &Expression<'_>,
    op: &str,
    right: &Expression<'_>,
    (p, l, r): (i32, i32, i32),
    limit: i32,
    lhs: i32,
) -> Option<String> {
    let wrap = p >= limit;
    let lhs = if wrap { MAX } else { lhs };
    let code = format!(
        "{} {op} {}",
        fresh(ctx, left, l, l)?,
        fresh(ctx, right, r, lhs)?
    );
    Some(if wrap { format!("({code})") } else { code })
}

/// The operator of a php-parser `BinaryOp` as it prints it.
fn operator(op: &BinaryOperator<'_>) -> Option<&'static str> {
    Some(match op {
        BinaryOperator::Addition(_) => "+",
        BinaryOperator::Subtraction(_) => "-",
        BinaryOperator::Multiplication(_) => "*",
        BinaryOperator::Division(_) => "/",
        BinaryOperator::Modulo(_) => "%",
        BinaryOperator::Exponentiation(_) => "**",
        BinaryOperator::StringConcat(_) => ".",
        BinaryOperator::BitwiseAnd(_) => "&",
        BinaryOperator::BitwiseOr(_) => "|",
        BinaryOperator::BitwiseXor(_) => "^",
        BinaryOperator::LeftShift(_) => "<<",
        BinaryOperator::RightShift(_) => ">>",
        BinaryOperator::And(_) => "&&",
        BinaryOperator::Or(_) => "||",
        BinaryOperator::LowAnd(_) => "and",
        BinaryOperator::LowOr(_) => "or",
        BinaryOperator::LowXor(_) => "xor",
        BinaryOperator::Equal(_) => "==",
        BinaryOperator::NotEqual(_) | BinaryOperator::AngledNotEqual(_) => "!=",
        BinaryOperator::Identical(_) => "===",
        BinaryOperator::NotIdentical(_) => "!==",
        BinaryOperator::LessThan(_) => "<",
        BinaryOperator::LessThanOrEqual(_) => "<=",
        BinaryOperator::GreaterThan(_) => ">",
        BinaryOperator::GreaterThanOrEqual(_) => ">=",
        BinaryOperator::Spaceship(_) => "<=>",
        BinaryOperator::NullCoalesce(_) => "??",
        _ => return None,
    })
}
