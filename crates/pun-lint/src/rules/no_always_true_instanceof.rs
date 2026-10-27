//! `lint/dead-code/no-always-true-instanceof`, ported from Rector 2.6.1's `RemoveDeadInstanceOfRector`
//! (rules/DeadCode/Rector/If_).
//!
//! An `if` without `elseif` or `else` whose condition is `$value instanceof SomeClass` where the value's native type
//! is already that class (or a subclass of it) always runs: its statements take its place. `!$value instanceof
//! SomeClass` never runs, so the `if` goes, and `$value instanceof SomeClass && $other` becomes `$other`. A value
//! assigned in the condition (`($value = new SomeClass()) instanceof SomeClass`) stays as a statement before the
//! `if`'s statements, negated or not. The value must be no property, call or `new`, and the `if` not in a trait; the
//! class must be written as a name, and the native type one class (`$this`, a nullable type or a union are not).
//!
//! php-parser's printer decides the bytes: see [`super::stmt_splice`]. Under a node other than a function, method,
//! closure or `foreach`, an `if` that is its list's only statement and becomes several stays (php-parser prints the
//! node anew there).

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_syntax::cst::{
    Access, BinaryOperator, Expression, Node, NodeKind, Statement, UnaryPrefixOperator,
};
use pun_problem::{Edit, Fix, Safety, Span};

use super::php_parser::{Token, tokens};
use super::rector_batch5::resolve_class_name;
use super::rector_conditions::{if_parts, is_stmts_aware};
use super::stmt_splice::{Change, Code, NewStatement, run_edits};
use super::types::facts::{Facts, facts};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoAlwaysTrueInstanceof;

impl Rule for NoAlwaysTrueInstanceof {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-always-true-instanceof"
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
        if !matches!(node, Node::If(_)) {
            return;
        }
        let source = ctx.source;
        let tokens = tokens(source);
        let facts = facts(ctx);
        let edits = run_edits(source, &tokens, ancestors, &|chain| {
            change(source, &tokens, &facts, chain)
        });
        if let Some(edits) = edits {
            out.push(report(node_span(&ancestors[ancestors.len() - 1]), edits));
        }
    }
}

/// What `refactor()` does to the `if` statement `chain` ends with.
fn change(source: &str, tokens: &[Token], facts: &Facts, chain: &[Node<'_, '_>]) -> Option<Change> {
    let n = chain.len();
    let Node::Statement(Statement::If(r#if)) = chain[n - 1] else {
        return None;
    };
    if !is_stmts_aware(chain[n - 2], &chain[..n - 2]) {
        return None;
    }
    let parts = if_parts(tokens, r#if);
    if parts.has_else || parts.has_elseifs {
        return None;
    }
    let cond = unparenthesize(r#if.condition);
    let (instanceof, negated) = match cond {
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
            match unparenthesize(u.operand) {
                Expression::Binary(b) if matches!(b.operator, BinaryOperator::Instanceof(_)) => {
                    (b, true)
                }
                _ => return None,
            }
        }
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::And(_)) => {
            let Expression::Binary(left) = unparenthesize(b.lhs) else {
                return None;
            };
            if !matches!(left.operator, BinaryOperator::Instanceof(_))
                || !is_same_type(facts, chain, left.lhs, left.rhs)
            {
                return None;
            }
            // `$if->cond = $booleanAnd->right`
            let right = node_span(unparenthesize(b.rhs));
            let edit = Edit::new(node_span(cond), &source[right.start..right.end]);
            return Some(Change::Edits(vec![edit]));
        }
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::Instanceof(_)) => (b, false),
        _ => return None,
    };
    if !is_same_type(facts, chain, instanceof.lhs, instanceof.rhs) {
        return None;
    }
    let value = unparenthesize(instanceof.lhs);
    let mut new = Vec::new();
    if let Expression::Assignment(_) = value {
        let span = node_span(value);
        new.push(NewStatement {
            comments: Vec::new(),
            code: Code::Text(format!("{};", &source[span.start..span.end])),
        });
    } else if negated || parts.stmts.is_empty() {
        return Some(Change::Remove);
    }
    new.extend(NewStatement::all(source, tokens, &parts.stmts));
    Some(Change::Replace(new))
}

fn report(span: Span, edits: Vec<Edit>) -> Report {
    Report {
        span,
        message: "This `instanceof` is always true for the value's type.".to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Safe,
            edits,
        }),
    }
}

/// `isInstanceofTheSameType() === true`: `$value instanceof Name` where the value is no property, call or `new`,
/// outside a trait, and the value's native type is one class that is the named class or a subclass of it.
fn is_same_type(
    facts: &Facts,
    ancestors: &[Node<'_, '_>],
    value: &Expression<'_>,
    class: &Expression<'_>,
) -> bool {
    let Expression::Identifier(name) = class else {
        return false;
    };
    let value = unparenthesize(value);
    if matches!(
        value,
        Expression::Access(Access::Property(_) | Access::StaticProperty(_))
            | Expression::Call(_)
            | Expression::Instantiation(_)
    ) {
        return false;
    }
    if ancestors.iter().any(|a| matches!(a, Node::Trait(_))) {
        return false;
    }
    let Some(t) = facts.native_type(node_span(value)) else {
        return false;
    };
    let own = match t.types.as_ref() {
        [TAtomic::Object(TObject::Named(n))] if !n.is_this => n.name,
        [TAtomic::Object(TObject::Enum(e))] => e.get_name(),
        _ => return false,
    };
    let wanted = resolve_class_name(ancestors, name);
    facts.codebase.get_class_like(own.as_bytes()).is_some()
        && facts
            .codebase
            .is_instance_of(own.as_bytes(), wanted.as_bytes())
}
