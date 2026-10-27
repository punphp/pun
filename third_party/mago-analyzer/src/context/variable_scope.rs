//! PHPStan's variable-holder certainty and conditional holders, alongside Mago's expression types.

use std::rc::Rc;

use foldhash::HashSet;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::combine_union_types;
use mago_codex::ttype::combiner::{CombinerOptions, combine};
use mago_codex::ttype::comparator::ComparisonResult;
use mago_codex::ttype::comparator::union_comparator::{can_expression_types_be_identical, is_contained_by};
use mago_codex::ttype::union::TUnion;
use mago_span::Span;
use mago_word::{Word, WordMap, WordSet};

use super::BlockContext;
use crate::context::scope::var_has_root;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VariableCertainty {
    Defined,
    Possible,
    Absent,
}

impl VariableCertainty {
    pub fn is_defined(self) -> bool {
        self == Self::Defined
    }
    pub fn is_absent(self) -> bool {
        self == Self::Absent
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ConditionalVariable {
    guard: Word,
    guard_type: Rc<TUnion>,
    target: Word,
    certainty: VariableCertainty,
    target_type: Option<Rc<TUnion>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VariableState {
    pub after_extract_call: bool,
    pub absent: WordSet,
    /// An impossible isset edge is still analyzed for diagnostics, but is not a continuing scope.
    pub impossible: bool,
    /// Resolved input-reference contracts; unlike output references these cannot declare a variable.
    pub non_creating_references: HashSet<Span>,
    conditional: Vec<ConditionalVariable>,
}

/// An actual incoming edge, not a reconstruction of the statements on that edge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariableScope {
    locals: WordMap<Rc<TUnion>>,
    possible: WordSet,
    state: VariableState,
    otherwise: VariableCertainty,
}

impl VariableScope {
    fn certainty(&self, name: Word) -> VariableCertainty {
        certainty(&self.locals, &self.possible, &self.state, self.otherwise, name)
    }

    /// Fixed-point discovery is not an incoming holder. Keep untyped possibilities from entry only.
    pub(crate) fn loop_exit_scope(&self, entry: &Self) -> Self {
        let mut exit = self.clone();
        exit.possible.retain(|name| self.locals.contains_key(name) || entry.possible.contains(name));
        exit
    }

    /// MutatingScope::generalizeWith keeps the incoming holders; update-only names do not become exits.
    pub(crate) fn generalize_with(&self, updated: &Self, codebase: &CodebaseMetadata) -> Self {
        let mut generalized = self.clone();
        for (name, ty) in &mut generalized.locals {
            if let Some(other) = updated.locals.get(name) {
                let mut combined = combine_union_types(ty, other, codebase, CombinerOptions::default());
                // MutatingScope::generalizeType widens differing nonempty constant-string buckets,
                // but retains their literal origin and common casing/nonempty accessories.
                let strings = |union: &TUnion| {
                    union.types.iter().filter_map(|atomic| match atomic {
                        TAtomic::Scalar(TScalar::String(string)) => string.get_known_literal_atom(),
                        _ => None,
                    }).collect::<HashSet<_>>()
                };
                let before = strings(ty);
                let after = strings(other);
                if !before.is_empty() && !after.is_empty() && before != after {
                    let atoms = combined.types.iter().map(|atomic| match atomic {
                        TAtomic::Scalar(TScalar::String(string)) if string.get_known_literal_atom().is_some() => {
                            TAtomic::Scalar(TScalar::String(string.with_unspecified_literal()))
                        }
                        _ => atomic.clone(),
                    }).collect();
                    combined.types = combine(atoms, codebase, CombinerOptions::default()).into();
                }
                combined.set_possibly_undefined(ty.possibly_undefined(), None);
                combined.set_possibly_undefined_from_try(ty.possibly_undefined_from_try());
                *ty = Rc::new(combined);
            }
        }
        generalized
    }
}

fn certainty(
    locals: &WordMap<Rc<TUnion>>,
    possible: &WordSet,
    state: &VariableState,
    otherwise: VariableCertainty,
    name: Word,
) -> VariableCertainty {
    if state.absent.contains(&name) {
        VariableCertainty::Absent
    } else {
        match locals.get(&name) {
            Some(ty) if ty.possibly_undefined() || ty.possibly_undefined_from_try() => VariableCertainty::Possible,
            Some(_) => VariableCertainty::Defined,
            None if possible.contains(&name) => VariableCertainty::Possible,
            None => otherwise,
        }
    }
}

impl BlockContext<'_> {
    pub fn can_any_variable_exist(&self) -> bool {
        self.variable_state.after_extract_call
            || (self.scope.get_function_like().is_none() && self.scope.get_property_hook().is_none())
    }

    pub fn variable_certainty(&self, name: Word) -> VariableCertainty {
        let otherwise = if self.can_any_variable_exist() {
            VariableCertainty::Possible
        } else {
            VariableCertainty::Absent
        };
        certainty(
            &self.locals,
            &self.variables_possibly_in_scope,
            &self.variable_state,
            otherwise,
            name,
        )
    }

    pub fn variable_scope(&self) -> VariableScope {
        VariableScope {
            locals: self.locals.clone(),
            possible: self.variables_possibly_in_scope.clone(),
            state: self.variable_state.clone(),
            otherwise: if self.can_any_variable_exist() {
                VariableCertainty::Possible
            } else {
                VariableCertainty::Absent
            },
        }
    }

    /// Assignments invalidate both a holder's target and the conditions under which it was known.
    pub fn invalidate_variable_certainty(&mut self, name: Word) {
        self.variable_state.absent.retain(|target| !var_has_root(*target, name));
        self.variable_state
            .conditional
            .retain(|holder| !var_has_root(holder.guard, name) && !var_has_root(holder.target, name));
    }

    /// TryCatchHandler joins the scopes at the matching throw points, including narrowed guard types.
    pub fn merge_variable_scope_types(&mut self, branches: &[VariableScope], codebase: &CodebaseMetadata) {
        if branches.is_empty() {
            return;
        }
        self.locals.clear();
        self.variables_possibly_in_scope.clear();
        for branch in branches.iter().filter(|branch| !branch.state.impossible) {
            self.variables_possibly_in_scope.extend(&branch.possible);
            for (name, ty) in &branch.locals {
                self.locals
                    .entry(*name)
                    .and_modify(|existing| {
                        *existing = Rc::new(combine_union_types(existing, ty, codebase, CombinerOptions::default()));
                    })
                    .or_insert_with(|| Rc::clone(ty));
            }
        }
        self.merge_variable_scopes(branches, codebase);
    }

    /// ScopeOps::mergeVariableHolders/createConditionalExpressions. Only continuing branches contribute.
    pub fn merge_variable_scopes(&mut self, branches: &[VariableScope], codebase: &CodebaseMetadata) {
        if branches.is_empty() {
            return;
        }
        let branches = branches.iter().filter(|branch| !branch.state.impossible);
        let Some(first) = branches.clone().next() else {
            self.variable_state.impossible = true;
            return;
        };
        self.variable_state.impossible = false;
        self.variable_state.after_extract_call = branches.clone().all(|branch| branch.state.after_extract_call);
        self.variable_state.conditional = first
            .state
            .conditional
            .iter()
            .filter(|holder| branches.clone().all(|branch| branch.state.conditional.contains(holder)))
            .cloned()
            .collect();
        let names: WordSet = branches
            .clone()
            .flat_map(|branch| {
                branch
                    .locals
                    .keys()
                    .chain(branch.possible.iter())
                    .chain(branch.state.absent.iter())
                    .copied()
            })
            .collect();
        for name in names {
            let initial = first.certainty(name);
            let joined = if branches.clone().all(|branch| branch.certainty(name) == initial) {
                initial
            } else {
                VariableCertainty::Possible
            };
            self.set_variable_certainty(name, joined, None);
            for branch in branches.clone() {
                let branch_certainty = branch.certainty(name);
                if branch_certainty == joined {
                    continue;
                }
                for (guard, guard_type) in &branch.locals {
                    if *guard == name || branch.certainty(*guard) != VariableCertainty::Defined {
                        continue;
                    }
                    // A disjoint branch type is a representable type difference. Overlapping types cannot
                    // establish this implication; PHPStan also rejects a non-discriminating guard.
                    if !branches
                        .clone()
                        .filter(|other| !std::ptr::eq(*other, branch))
                        .all(|other| {
                            other.certainty(*guard) == VariableCertainty::Defined
                                && other.locals.get(guard).is_some_and(|other_type| {
                                    !can_expression_types_be_identical(codebase, guard_type, other_type, false, false)
                                })
                        })
                    {
                        continue;
                    }
                    self.variable_state.conditional.push(ConditionalVariable {
                        guard: *guard,
                        guard_type: Rc::clone(guard_type),
                        target: name,
                        certainty: branch_certainty,
                        target_type: branch.locals.get(&name).cloned(),
                    });
                }
            }
        }
    }

    fn set_variable_certainty(&mut self, name: Word, certainty: VariableCertainty, ty: Option<Rc<TUnion>>) {
        if certainty == VariableCertainty::Absent {
            self.variable_state.absent.insert(name);
        } else {
            self.variable_state.absent.remove(&name);
            if let Some(ty) = ty {
                self.locals.insert(name, ty);
            }
            if let Some(ty) = self.locals.get_mut(&name) {
                let mut ty_value = (**ty).clone();
                ty_value.set_possibly_undefined(certainty == VariableCertainty::Possible, None);
                ty_value.set_possibly_undefined_from_try(false);
                *ty = Rc::new(ty_value);
            }
        }
    }

    /// ScopeOps::matchConditionalExpressions: consume actual narrowed guard types to a fixed point.
    pub fn apply_variable_conditions(&mut self, codebase: &CodebaseMetadata) {
        let mut remaining = self.variable_state.conditional.clone();
        loop {
            let mut matched = false;
            remaining.retain(|holder| {
                if self.variable_certainty(holder.guard) != VariableCertainty::Defined {
                    return true;
                }
                let actual = &self.locals[&holder.guard];
                if !is_contained_by(
                    codebase,
                    actual,
                    &holder.guard_type,
                    false,
                    false,
                    false,
                    &mut ComparisonResult::default(),
                ) {
                    return true;
                }
                self.set_variable_certainty(holder.target, holder.certainty, holder.target_type.clone());
                matched = true;
                false
            });
            if !matched {
                break;
            }
        }
    }
}
