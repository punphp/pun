//! Opt-in PHPStan PropertyInitializationExpr facts from the live semantic walk.
//!
//! These are virtual expression holders, not PHP's runtime initializedness or Mago's
//! readonly-property state. In particular, unset does not remove them. Constructor
//! facts use raw return scopes (before finally), exclude throwing exits, and are
//! remembered only for the same class's instance methods. Direct helper replay is
//! deliberately unsupported: Mago's transitive initializer summary is not this fact.

use mago_span::Span;
use mago_word::{Word, WordMap};

/// Certainty of PHPStan's virtual PropertyInitializationExpr, independently of value/nullability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyInitializationCertainty {
    Yes,
    Maybe,
    No,
}

impl PropertyInitializationCertainty {
    pub(crate) fn join(self, other: Self) -> Self {
        if self == other { self } else { Self::Maybe }
    }
}

/// Observe a named virtual marker immediately before a real operand is evaluated.
/// A request enables tracking and constructor-before-instance-method scheduling for
/// this analysis. It never prices or reanalyzes an expression. Missing operands
/// produce no answer; a visited operand with no holder produces `No`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyInitializationRequest {
    pub expression: Span,
    /// Bare, case-sensitive property name, without `$`.
    pub property: Word,
}

pub(crate) type Markers = WordMap<PropertyInitializationCertainty>;

pub(crate) fn remember(
    scopes: &mut foldhash::HashMap<Span, Markers>,
    span: Span,
    incoming: &Markers,
) {
    scopes
        .entry(span)
        .and_modify(|previous| *previous = join([&*previous, incoming]))
        .or_insert_with(|| incoming.clone());
}

/// processFinallyScopeVariableTypeHolders compares types, not certainties. All
/// virtual markers have MixedType: only a newly introduced marker is transferred.
pub(crate) fn apply_finally(ours: &mut Markers, after: &Markers, original: &Markers) {
    ours.extend(
        after
            .iter()
            .filter(|(name, _)| !original.contains_key(name))
            .map(|(name, certainty)| (*name, *certainty)),
    );
}

/// ScopeOps::mergeWith: a holder absent on any incoming edge is only Maybe.
pub(crate) fn join<'a>(scopes: impl IntoIterator<Item = &'a Markers>) -> Markers {
    let scopes: Vec<_> = scopes.into_iter().collect();
    let mut joined = Markers::default();
    for scope in &scopes {
        for name in scope.keys() {
            joined.entry(*name).or_insert_with(|| {
                if scopes
                    .iter()
                    .all(|scope| scope.get(name) == Some(&PropertyInitializationCertainty::Yes))
                {
                    PropertyInitializationCertainty::Yes
                } else {
                    PropertyInitializationCertainty::Maybe
                }
            });
        }
    }
    joined
}
