//! `lint/dead-code/no-redundant-foreach-guard`, ported from Rector 2.6.1's
//! `RemoveUnusedNonEmptyArrayBeforeForeachRector` (rules/DeadCode/Rector/If_), with
//! `UselessIfCondBeforeForeachDetector` and `CountManipulator`.
//!
//! A `foreach` over an empty array does nothing, so a check that the array is not empty around it is dead: an `if`
//! without `elseif` or `else` that holds only a `foreach` over `$items` makes way for the `foreach` when its condition
//! is `$items` (or a property fetch) whose native type is an array, `$items !== []` (or `!=`, either side),
//! `!empty($items)` with `$items` an array, `count($items) > 0` (`>= 1`, `0 <`, `1 <=`) with `$items` natively an
//! array, or `$items && count($items) > 0`. The `foreach`'s comments follow the `if`'s. A variable must be natively an
//! array and no superglobal, and an array item one whose key the native type of the array has.
//!
//! In a statement list other than an `if`'s own, an `if (empty($items)) { }` or `if (empty($items)) { return; }` right
//! before a `foreach` over `$items` that ends the list goes, when `$items` is natively an array.
//!
//! php-parser's printer decides the bytes: see [`super::stmt_splice`].

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::string::TStringLiteral;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{
    Access, ArrayAccess, AssignmentOperator, BinaryOperator, Construct, Expression, Literal, Node,
    NodeKind, Statement, UnaryPrefixOperator, Variable,
};
use pun_problem::{Fix, Safety};

use super::class_method::{type_of, walk};
use super::no_implode_ternary::is_empty_array;
use super::php_parser::{Token, enclosing, previous_token_end, remove, tokens};
use super::rector_code_quality::{args, call_to};
use super::rector_conditions::{comments_between, if_parts, is_stmts_aware};
use super::stmt_splice::{Change, Code, NewStatement, run_edits};
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, same_expression, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoRedundantForeachGuard;

/// `Scope::SUPERGLOBAL_VARIABLES`.
const SUPERGLOBALS: &[&[u8]] = &[
    b"$GLOBALS",
    b"$_SERVER",
    b"$_GET",
    b"$_POST",
    b"$_FILES",
    b"$_COOKIE",
    b"$_SESSION",
    b"$_REQUEST",
    b"$_ENV",
];

impl Rule for NoRedundantForeachGuard {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-redundant-foreach-guard"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::If, NodeKind::Foreach])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let source = ctx.source;
        let tokens = tokens(ctx);
        let facts = facts(ctx);
        let edits = match node {
            Node::Foreach(foreach) => {
                let n = ancestors.len();
                if !is_stmts_aware(ancestors[n - 2], &ancestors[..n - 2])
                    || in_if_body(&ancestors[..n - 1])
                {
                    return;
                }
                let (list, index) = enclosing(source, &tokens, ancestors);
                if index == 0 || index + 1 != list.stmts.len() || list.nop {
                    return;
                }
                let Statement::If(previous) = list.stmts[index - 1] else {
                    return;
                };
                if !is_empty_guard(source, &tokens, &facts, previous, foreach.expression) {
                    return;
                }
                Some(remove(source, &tokens, &list, &[index - 1]))
            }
            Node::If(_) => run_edits(source, &tokens, ancestors, &|chain| {
                guard(source, &tokens, &facts, chain)
            }),
            _ => return,
        };
        let Some(edits) = edits else { return };
        out.push(Report {
            span: node_span(&ancestors[ancestors.len() - 1]),
            message: "A `foreach` over an empty array does nothing; this check is not needed."
                .to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

/// Whether the statement list `ancestors` ends with (its holder) is an `if`'s own body, which Rector's
/// `refactorStmtsAware` does not see (an `If_` goes to `refactorIf`).
fn in_if_body(ancestors: &[Node<'_, '_>]) -> bool {
    ancestors
        .iter()
        .rev()
        .find(|a| !matches!(a, Node::Block(_) | Node::Statement(_)))
        .is_some_and(|a| matches!(a, Node::IfStatementBody(_) | Node::IfColonDelimitedBody(_)))
}

/// `refactorIf`: the `if` `chain` ends with replaced by its `foreach`, with the `if`'s comments before the
/// `foreach`'s.
fn guard(source: &str, tokens: &[Token], facts: &Facts, chain: &[Node<'_, '_>]) -> Option<Change> {
    let Node::Statement(Statement::If(r#if)) = chain[chain.len() - 1] else {
        return None;
    };
    let parts = if_parts(tokens, r#if);
    if parts.has_else || parts.has_elseifs || parts.stmts.nop {
        return None;
    }
    let [Statement::Foreach(foreach)] = parts.stmts.stmts[..] else {
        return None;
    };
    let (list, index) = enclosing(source, tokens, chain);
    if !is_useless(
        source,
        facts,
        &list.stmts[..index],
        r#if.condition,
        foreach.expression,
    ) {
        return None;
    }
    let start = node_span(r#if).start;
    let mut comments = comments_between(tokens, previous_token_end(source, tokens, start), start);
    let inner = node_span(parts.stmts.stmts[0]);
    comments.extend(comments_between(
        tokens,
        previous_token_end(source, tokens, inner.start),
        inner.start,
    ));
    Some(Change::Replace(vec![NewStatement {
        comments,
        code: Code::Moved(inner),
    }]))
}

/// `isUselessBeforeForeachCheck` on the condition and the `foreach`'s expression.
fn is_useless(
    source: &str,
    facts: &Facts,
    preceding: &[&Statement<'_>],
    cond: &Expression<'_>,
    items: &Expression<'_>,
) -> bool {
    let items = unparenthesize(items);
    let cond = unparenthesize(cond);
    // the `if`'s scope: the types of the same expression where the condition reads it, before it narrows
    let at_if = first_equal(source, cond, items).unwrap_or(items);
    if should_skip_items(source, facts, preceding, at_if) {
        return false;
    }
    match cond {
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::And(_)) => {
            matches!(unparenthesize(b.lhs), Expression::Variable(_))
                && same_expression(source, b.lhs, items)
                && is_counter_higher_than_one(source, facts, b.rhs, items)
        }
        Expression::Variable(_)
        | Expression::Access(Access::Property(_) | Access::StaticProperty(_))
            if same_expression(source, cond, items) =>
        {
            native(facts, cond).is_some_and(is_array)
        }
        Expression::Binary(b)
            if matches!(
                b.operator,
                BinaryOperator::NotIdentical(_)
                    | BinaryOperator::NotEqual(_)
                    | BinaryOperator::AngledNotEqual(_)
            ) =>
        {
            (is_empty_array(b.lhs) && same_expression(source, items, b.rhs))
                || (is_empty_array(b.rhs) && same_expression(source, items, b.lhs))
        }
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
            // isMatchingNotEmpty: `!empty($items)`, the type (docblocks read) an array
            let Expression::Construct(Construct::Empty(empty)) = unparenthesize(u.operand) else {
                return false;
            };
            same_expression(source, empty.value, items)
                && type_of(facts, empty.value).is_some_and(is_array)
        }
        _ => is_counter_higher_than_one(source, facts, cond, items),
    }
}

/// The first expression in `cond` (in php-parser's order) that is the same code as `items`.
fn first_equal<'a>(
    source: &str,
    cond: &'a Expression<'a>,
    items: &Expression<'_>,
) -> Option<&'a Expression<'a>> {
    let mut found = None;
    walk(Node::Expression(cond), &mut |node, _| {
        if found.is_some() {
            return false;
        }
        if let Node::Expression(e) = node
            && !matches!(e, Expression::Parenthesized(_))
            && same_expression(source, e, items)
        {
            found = Some(e);
            return false;
        }
        true
    });
    found
}

/// `shouldSkipForeachExpr`: an array item whose key the native type of the array may lack, a superglobal, or a
/// variable that is not natively an array.
fn should_skip_items(
    source: &str,
    facts: &Facts,
    preceding: &[&Statement<'_>],
    items: &Expression<'_>,
) -> bool {
    match items {
        Expression::ArrayAccess(access) => {
            !has_offset(facts, access.array, access.index)
                && !(native(facts, access.array).is_some_and(|t| t.is_mixed())
                    && type_of(facts, access.array).is_some_and(is_array)
                    && assigned_offset(source, facts, preceding, access))
        }
        Expression::Variable(Variable::Direct(v)) if SUPERGLOBALS.contains(&v.name) => true,
        Expression::Variable(_) => !native(facts, items).is_some_and(is_array),
        _ => false,
    }
}

/// PHPStan's native scope remembers the key an assignment writes into a value of no native type but a documented
/// array type (`mixed` with `hasOffsetValue()`), which Mago's does not: an earlier statement of the list assigns
/// `$array[$key]` for the same array and key.
fn assigned_offset(
    source: &str,
    facts: &Facts,
    preceding: &[&Statement<'_>],
    access: &ArrayAccess<'_>,
) -> bool {
    let key = native(facts, access.index).and_then(constant_key);
    preceding.iter().any(|s| {
        let Statement::Expression(e) = s else {
            return false;
        };
        let Expression::Assignment(a) = unparenthesize(e.expression) else {
            return false;
        };
        let Expression::ArrayAccess(target) = unparenthesize(a.lhs) else {
            return false;
        };
        matches!(a.operator, AssignmentOperator::Assign(_))
            && same_expression(source, target.array, access.array)
            && (same_expression(source, target.index, access.index)
                || (key.is_some() && native(facts, target.index).and_then(constant_key) == key))
    })
}

/// `hasOffsetValueType($dimType)->yes()` on the native types: the array is one shape whose key `index` is known and
/// not optional, the key's type a constant.
fn has_offset(facts: &Facts, array: &Expression<'_>, index: &Expression<'_>) -> bool {
    let Some(key) = native(facts, index).and_then(constant_key) else {
        return false;
    };
    let Some(array) = native(facts, array) else {
        return false;
    };
    match array.types.as_ref() {
        [TAtomic::Array(TArray::Keyed(k))] => k
            .known_items
            .as_ref()
            .and_then(|items| items.get(&key))
            .is_some_and(|(optional, _)| !optional),
        [TAtomic::Array(TArray::List(l))] => match key {
            ArrayKey::Integer(i) => usize::try_from(i).ok().is_some_and(|i| {
                l.known_elements
                    .as_ref()
                    .and_then(|e| e.get(&i))
                    .is_some_and(|(optional, _)| !optional)
            }),
            _ => false,
        },
        _ => false,
    }
}

/// The array key a constant type stands for.
fn constant_key(t: &TUnion) -> Option<ArrayKey> {
    match t.types.as_ref() {
        [TAtomic::Scalar(TScalar::Integer(i))] => i.get_literal_value().map(ArrayKey::Integer),
        [TAtomic::Scalar(TScalar::String(s))] => match &s.literal {
            Some(TStringLiteral::Value(value)) => Some(ArrayKey::String(*value)),
            _ => None,
        },
        _ => None,
    }
}

/// `isCounterHigherThanOne`: `count($items) > 0`, `count($items) >= 1`, `0 < count($items)` or `1 <= count($items)`,
/// with `$items` natively an array.
fn is_counter_higher_than_one(
    source: &str,
    facts: &Facts,
    cond: &Expression<'_>,
    items: &Expression<'_>,
) -> bool {
    let Expression::Binary(b) = unparenthesize(cond) else {
        return false;
    };
    let (count, number, value) = match b.operator {
        BinaryOperator::GreaterThan(_) => (b.lhs, b.rhs, 0),
        BinaryOperator::GreaterThanOrEqual(_) => (b.lhs, b.rhs, 1),
        BinaryOperator::LessThan(_) => (b.rhs, b.lhs, 0),
        BinaryOperator::LessThanOrEqual(_) => (b.rhs, b.lhs, 1),
        _ => return false,
    };
    if !matches!(unparenthesize(number), Expression::Literal(Literal::Integer(i)) if i.value == Some(value))
    {
        return false;
    }
    let Some(call) = call_to(count, "count") else {
        return false;
    };
    args(call)
        .first()
        .is_some_and(|a| same_expression(source, a.value(), items))
        && native(facts, items).is_some_and(|t| is_array(t) || t.is_never())
}

/// `isMatchingEmptyAndForeachedExpr` with `$items` natively an array: `if (empty($items))` with no statement or a
/// bare `return;`.
fn is_empty_guard(
    source: &str,
    tokens: &[Token],
    facts: &Facts,
    r#if: &mago_syntax::cst::If<'_>,
    items: &Expression<'_>,
) -> bool {
    let parts = if_parts(tokens, r#if);
    if parts.has_else || parts.has_elseifs {
        return false;
    }
    let Expression::Construct(Construct::Empty(empty)) = unparenthesize(r#if.condition) else {
        return false;
    };
    if !same_expression(source, empty.value, items) {
        return false;
    }
    let body = match (&parts.stmts.stmts[..], parts.stmts.nop) {
        ([], false) => true,
        ([Statement::Return(r)], false) => r.value.is_none(),
        _ => false,
    };
    body && native(facts, empty.value).is_some_and(is_array)
}

fn native<'f>(facts: &'f Facts, expr: &Expression<'_>) -> Option<&'f TUnion> {
    facts.native_type(node_span(unparenthesize(expr)))
}

/// `isArray()->yes()`.
fn is_array(t: &TUnion) -> bool {
    !t.types.is_empty() && t.types.iter().all(|a| matches!(a, TAtomic::Array(_)))
}
