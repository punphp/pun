//! Rector's `NodeTypeResolver::getType` on Mago's types: PHPStan's type of an expression with docblocks read
//! ([`Facts::full_type`]), except for the two shapes Rector types itself. A ternary whose branches have one type each
//! (the second not `null`) and a `??` whose sides do are a `UnionType` of the two, whatever they are, even the same
//! type twice.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::union::TUnion;
use mago_syntax::cst::{BinaryOperator, Expression};

use super::facts::Facts;
use crate::syntax::{node_span, unparenthesize};

/// What `getType` gives an expression.
pub(crate) enum RectorType<'f> {
    /// The `UnionType` Rector builds of a ternary's or a coalesce's two sides.
    Built,
    /// PHPStan's type, from Mago; `None` where the analyser did not reach the expression.
    Scope(Option<&'f TUnion>),
}

/// What `isUnionTypeable` reads of a type: whether it is a `UnionType`, and whether it is `null`.
#[derive(Clone, Copy)]
struct Shape {
    union: bool,
    null: bool,
}

impl RectorType<'_> {
    fn shape(&self) -> Shape {
        match self {
            RectorType::Built => Shape {
                union: true,
                null: false,
            },
            RectorType::Scope(t) => Shape {
                union: t.is_some_and(is_union),
                null: t.is_some_and(|t| {
                    !t.types.is_empty() && t.types.iter().all(|a| matches!(a, TAtomic::Null))
                }),
            },
        }
    }
}

/// Whether PHPStan's type is a `UnionType`: more than one member, `true` with `false` counting as one `bool`.
pub(crate) fn is_union(t: &TUnion) -> bool {
    let bools = (t.types.iter())
        .filter(|a| matches!(a, TAtomic::Scalar(s) if s.is_bool()))
        .count();
    t.types.len() - bools.saturating_sub(1) > 1
}

/// `isUnionTypeable`: neither is a union and the second is not `null`.
fn union_typeable(first: Shape, second: Shape) -> bool {
    !first.union && !second.union && !second.null
}

/// `NodeTypeResolver::getType` of `expr`.
pub(crate) fn get_type<'f>(facts: &'f Facts, expr: &Expression<'_>) -> RectorType<'f> {
    let expr = unparenthesize(expr);
    let scope = || RectorType::Scope(facts.full_type(node_span(expr)));
    let built = match expr {
        // `resolveTernaryType`
        Expression::Conditional(c) => {
            let branches = c.then.is_some_and(|then| {
                union_typeable(
                    get_type(facts, then).shape(),
                    get_type(facts, c.r#else).shape(),
                )
            });
            // a nullable condition (a union with `null`): its first member, which is no union, and the `else`
            let nullable = || match get_type(facts, c.condition) {
                RectorType::Scope(Some(t)) => {
                    is_union(t) && t.types.iter().any(|a| matches!(a, TAtomic::Null))
                }
                _ => false,
            };
            branches || (nullable() && union_typeable(SINGLE, get_type(facts, c.r#else).shape()))
        }
        // PHPStan 2.3 types the left side with its `null` (`?Foo` is a union), so `$nullable ?? $other` is PHPStan's
        // own type of the whole
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::NullCoalesce(_)) => {
            union_typeable(
                get_type(facts, b.lhs).shape(),
                get_type(facts, b.rhs).shape(),
            )
        }
        _ => false,
    };
    if built { RectorType::Built } else { scope() }
}

const SINGLE: Shape = Shape {
    union: false,
    null: false,
};
