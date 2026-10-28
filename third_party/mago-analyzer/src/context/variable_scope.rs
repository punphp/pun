//! PHPStan's variable-holder certainty and conditional holders, alongside Mago's expression types.

use std::rc::Rc;

use foldhash::HashSet;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::combine_union_types;
use mago_codex::ttype::combine_union_types_preserving_array_shapes;
use mago_codex::ttype::combiner::{CombinerOptions, combine};
use mago_codex::ttype::comparator::ComparisonResult;
use mago_codex::ttype::comparator::atomic_comparator;
use mago_codex::ttype::comparator::union_comparator::is_contained_by;
use mago_codex::ttype::{get_never, intersect_union_types};
use mago_codex::ttype::union::TUnion;
use mago_span::Span;
use mago_word::{Word, WordMap, WordSet};

use super::BlockContext;
use crate::context::scope::{var_has_root, var_references_dynamic};

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

    /// TrinaryLogic::extremeIdentity keeps a certainty only when all alternatives agree.
    fn join(self, other: Self) -> Self {
        if self == other {
            self
        } else {
            Self::Possible
        }
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

/// ExpressionTypeHolder::equalTypes does not compare the holder's certainty.
fn equal_types(left: &TUnion, right: &TUnion) -> bool {
    let without_certainty = |ty: &TUnion| {
        let mut ty = ty.clone();
        ty.set_possibly_undefined(false, None);
        ty.set_possibly_undefined_from_try(false);
        ty
    };
    without_certainty(left) == without_certainty(right)
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
            .retain(|holder| {
                !var_has_root(holder.guard, name) && !var_references_dynamic(holder.guard, name)
                    && !var_has_root(holder.target, name) && !var_references_dynamic(holder.target, name)
            });
    }

    /// ScopeOps applies requireMoreCharacters to targets, but not conditional guards.
    pub(crate) fn invalidate_expression_holders(&mut self, invalidates: impl Fn(Word, bool) -> bool) {
        self.variable_state.absent.retain(|target| !invalidates(*target, false));
        self.variable_state.conditional.retain(|holder| !invalidates(holder.target, false) && !invalidates(holder.guard, true));
    }

    /// ScopeOps joins actual continuing edges. Projection and loop widening remain separate.
    pub fn merge_variable_scope_types(&mut self, branches: &[VariableScope], codebase: &CodebaseMetadata) {
        if branches.is_empty() {
            return;
        }
        self.locals.clear();
        self.variables_possibly_in_scope.clear();
        for branch in branches.iter().filter(|branch| !branch.state.impossible) {
            self.variables_possibly_in_scope.extend(&branch.possible);
            for (name, ty) in &branch.locals {
                // An absent holder has no incoming value. Isset's diagnostic-only
                // null placeholder must not make a later assignment nullable.
                if branch.state.absent.contains(name) {
                    continue;
                }
                self.locals
                    .entry(*name)
                    .and_modify(|existing| {
                        // TypeCombinator::processArrayTypes generalizes open arrays. Closed
                        // alternatives retain their distinct values, discriminators and empty arm.
                        let combined = if !existing.types.iter().chain(ty.types.iter())
                            .any(|atomic| matches!(atomic, TAtomic::Array(array) if !array.is_sealed()))
                        {
                            combine_union_types_preserving_array_shapes(existing, ty, codebase, CombinerOptions::default())
                        } else {
                            combine_union_types(existing, ty, codebase, CombinerOptions::default())
                        };
                        *existing = Rc::new(combined);
                    })
                    .or_insert_with(|| Rc::clone(ty));
            }
        }
        self.merge_variable_scopes(branches, codebase);
        for ty in self.locals.values_mut() {
            if ty.types.len() < 2 {
                continue;
            }
            let mut reduced = (**ty).clone();
            let mut types = reduced.types.to_vec();
            let mut i = 0;
            while i < types.len() {
                if matches!(&types[i], TAtomic::Array(array) if array.is_sealed())
                    && types.iter().enumerate().any(|(j, other)| {
                        i != j && matches!(other, TAtomic::Array(array) if array.is_sealed())
                            && atomic_comparator::is_contained_by(
                                codebase, &types[i], other, false, &mut ComparisonResult::default(),
                            )
                    })
                {
                    types.remove(i);
                } else {
                    i += 1;
                }
            }
            // ConstantArrayType::isKeysSupersetOf/mergeWith can fold one extra key
            // into an optional entry without widening any shared value or discriminator.
            if let [TAtomic::Array(TArray::Keyed(left)), TAtomic::Array(TArray::Keyed(right))] = types.as_slice()
                && left.parameters.is_none()
                && right.parameters.is_none()
                && let (Some(left), Some(right)) = (&left.known_items, &right.known_items)
            {
                let (small, large) = if left.len() < right.len() { (left, right) } else { (right, left) };
                if !small.is_empty()
                    && large.len() == small.len() + 1
                    && small.iter().all(|(key, entry)| large.get(key) == Some(entry))
                {
                    types = combine(types, codebase, CombinerOptions::default());
                }
            }
            *reduced.types.to_mut() = types;
            *ty = Rc::new(reduced);
        }
        // get_array_access_id ends an offset holder with ']'. A stored value's
        // possibly-undefined flag is not the certainty of this expression holder.
        self.locals.retain(|name, _| {
            !name.as_bytes().ends_with(b"]")
                || branches.iter().filter(|branch| !branch.state.impossible)
                    .all(|branch| branch.locals.contains_key(name))
        });
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
            .filter_map(|holder| {
                let mut merged = holder.clone();
                for branch in branches.clone().skip(1) {
                    let other = branch.state.conditional.iter().find(|other| {
                        other.guard == holder.guard && other.guard_type == holder.guard_type
                            && other.target == holder.target && other.certainty == holder.certainty
                    })?;
                    if let (Some(ours), Some(theirs)) = (&merged.target_type, &other.target_type) {
                        merged.target_type = Some(Rc::new(combine_union_types_preserving_array_shapes(
                            ours, theirs, codebase, CombinerOptions::default(),
                        )));
                    }
                }
                Some(merged)
            })
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

        // ScopeOps::createConditionalExpressions derives guards once per incoming edge.
        // A branch's set difference may discriminate even when its full type overlaps another edge.
        let guards: Vec<_> = branches.clone().map(|branch| {
            let mut guards = Vec::new();
            for (guard, our_type) in &branch.locals {
                if branch.certainty(*guard) != VariableCertainty::Defined {
                    continue;
                }
                let mut their_type: Option<TUnion> = None;
                let mut defined = true;
                for other in branches.clone().filter(|other| !std::ptr::eq(*other, branch)) {
                    if other.certainty(*guard) != VariableCertainty::Defined {
                        defined = false;
                        break;
                    }
                    let other_type = &other.locals[guard];
                    their_type = Some(match their_type {
                        Some(ty) => combine_union_types_preserving_array_shapes(
                            &ty, other_type, codebase, CombinerOptions::default(),
                        ),
                        None => (**other_type).clone(),
                    });
                }
                let Some(their_type) = their_type.filter(|_| defined) else { continue };
                let Some(merged) = self.locals.get(guard) else { continue };
                if equal_types(merged, &their_type) || equal_types(our_type, &their_type) {
                    continue;
                }
                let remainder = our_type.difference(&their_type, codebase);
                if remainder.is_never() || equal_types(merged, &remainder) {
                    continue;
                }
                guards.push((*guard, Rc::new(remainder), their_type));
            }
            guards
        }).collect();

        for name in names {
            let joined = branches.clone().skip(1).fold(first.certainty(name), |joined, branch| {
                joined.join(branch.certainty(name))
            });
            self.set_variable_certainty(name, joined, None);
            for (branch, guards) in branches.clone().zip(&guards) {
                let branch_certainty = branch.certainty(name);
                if branch_certainty == joined && match (branch.locals.get(&name), self.locals.get(&name)) {
                    (Some(ours), Some(merged)) => equal_types(ours, merged),
                    (None, None) => true,
                    _ => false,
                } {
                    continue;
                }
                for (guard, guard_type, their_guard_type) in guards {
                    if *guard == name {
                        continue;
                    }
                    if branch_certainty != VariableCertainty::Absent
                        && guard_type.types.iter().all(|ty| matches!(ty, TAtomic::Array(array) if array.is_sealed()))
                        && branches.clone().filter(|other| !std::ptr::eq(*other, branch))
                            .any(|other| other.locals.get(&name).zip(self.locals.get(&name))
                                .is_some_and(|(other, merged)| equal_types(other, merged)))
                    {
                        // ScopeOps excludes subtype-absorbed targets paired with constant-array guards.
                        continue;
                    }
                    // The present-target loop rejects supertype/overlapping guards. Absent targets
                    // use ScopeOps' separate loop and are applied only by exact guard matching.
                    if branch_certainty != VariableCertainty::Absent && (
                        is_contained_by(codebase, guard_type, their_guard_type, false, false, false,
                            &mut ComparisonResult::default())
                        || is_contained_by(codebase, their_guard_type, guard_type, false, false, false,
                            &mut ComparisonResult::default())
                        || ((branches.clone().filter(|other| !std::ptr::eq(*other, branch))
                            .any(|other| other.certainty(name) == VariableCertainty::Defined)
                            || branch.locals.get(&name).is_some_and(|ty| equal_types(ty, guard_type)))
                            && intersect_union_types(guard_type, their_guard_type, codebase)
                                .is_some_and(|ty| !ty.is_never()))
                    ) {
                        continue;
                    }
                    let holder = ConditionalVariable {
                        guard: *guard,
                        guard_type: Rc::clone(guard_type),
                        target: name,
                        certainty: branch_certainty,
                        target_type: (branch_certainty != VariableCertainty::Absent)
                            .then(|| branch.locals.get(&name).cloned()).flatten(),
                    };
                    if !self.variable_state.conditional.contains(&holder) {
                        self.variable_state.conditional.push(holder);
                    }
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

    /// Existing clients supply an already-filtered scope rather than reconciliation's specified keys.
    pub fn apply_variable_conditions(&mut self, codebase: &CodebaseMetadata) {
        let specified = self.locals.keys().filter(|key| {
            self.variable_certainty(**key) == VariableCertainty::Defined
        }).copied().collect();
        self.apply_specified_variable_conditions(codebase, &specified);
    }

    /// ScopeOps::matchConditionalExpressions and MutatingScope's consequent intersection.
    pub fn apply_specified_variable_conditions(&mut self, codebase: &CodebaseMetadata, specified: &WordSet) {
        let originally_specified = specified;
        let mut specified: WordMap<_> = specified.iter().filter_map(|key| {
            self.locals.get(key).map(|ty| (*key, (self.variable_certainty(*key), Some(Rc::clone(ty)))))
        }).collect();
        let targets: WordSet = self.variable_state.conditional.iter().map(|holder| holder.target).collect();
        let mut conditions: WordMap<Vec<&ConditionalVariable>> = WordMap::default();
        loop {
            let before = specified.len();
            for target in &targets {
                if conditions.contains_key(target) {
                    continue;
                }
                // Exact matches take precedence; an absence consequent must never override an
                // expression explicitly specified by this condition, or match just a guard subtype.
                let mut matched: Vec<_> = self.variable_state.conditional.iter().filter(|holder| {
                    holder.target == *target
                        && !(holder.certainty == VariableCertainty::Absent && originally_specified.contains(target))
                        && specified.get(&holder.guard).is_some_and(|(certainty, ty)| {
                            *certainty == VariableCertainty::Defined
                                && ty.as_ref().is_some_and(|ty| equal_types(ty, &holder.guard_type))
                        })
                }).collect();
                if matched.is_empty() {
                    matched = self.variable_state.conditional.iter().filter(|holder| {
                        holder.target == *target && holder.certainty != VariableCertainty::Absent
                            && specified.get(&holder.guard).is_some_and(|(certainty, ty)| {
                                *certainty == VariableCertainty::Defined && ty.as_ref().is_some_and(|ty| {
                                    is_contained_by(codebase, ty, &holder.guard_type, false, false, false,
                                        &mut ComparisonResult::default())
                                })
                            })
                    }).collect();
                }
                if let Some(last) = matched.last() {
                    specified.insert(*target, (last.certainty, last.target_type.clone()));
                    conditions.insert(*target, matched);
                }
            }
            if specified.len() == before {
                break;
            }
        }
        let conditions: Vec<_> = conditions.into_iter().map(|(target, holders)| {
            let certainty = holders.iter().skip(1).fold(holders[0].certainty, |certainty, holder| {
                certainty.join(holder.certainty)
            });
            let ty = holders.iter().filter_map(|holder| holder.target_type.as_ref()).fold(
                self.locals.get(&target).map(|ty| (**ty).clone()),
                |ty, consequence| Some(ty.map_or_else(|| (**consequence).clone(), |ty| {
                    intersect_union_types(&ty, consequence, codebase).unwrap_or_else(get_never)
                })),
            );
            let certainty = if certainty != VariableCertainty::Absent
                && self.variable_certainty(target) == VariableCertainty::Defined {
                VariableCertainty::Defined
            } else {
                certainty
            };
            (target, certainty, ty.map(Rc::new))
        }).collect();
        for (target, certainty, ty) in conditions {
            self.set_variable_certainty(target, certainty, ty);
        }
    }
}
