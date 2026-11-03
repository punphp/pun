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
    /// Explicit No holders from refinements and joins, not invalidations such as unset().
    pub absent: WordSet,
    /// An impossible isset edge is still analyzed for diagnostics, but is not a continuing scope.
    pub impossible: bool,
    /// Resolved input-reference contracts; unlike output references these cannot declare a variable.
    pub non_creating_references: HashSet<Span>,
    /// Virtual expression holders: never invalidate these with runtime property values.
    pub(crate) property_initializations: crate::property_initialization::Markers,
    conditional: Vec<ConditionalVariable>,
}

/// An actual incoming edge, not a reconstruction of the statements on that edge.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariableScope {
    locals: WordMap<Rc<TUnion>>,
    possible: WordSet,
    pub(crate) state: VariableState,
    /// Source-owned abrupt edge, used only to apply a finally delta to virtual markers.
    pub(crate) property_exit: Option<Span>,
    otherwise: VariableCertainty,
}

impl VariableScope {
    pub(crate) fn at_property_exit(mut self, span: Span) -> Self {
        self.property_exit = Some(span);
        self
    }

    pub(crate) fn locals(&self) -> &WordMap<Rc<TUnion>> {
        &self.locals
    }

    /// MutatingScope::processFinallyScopeVariableTypeHolders copies changed/new
    /// holders, not every assignment and not the union of before and after.
    pub(crate) fn apply_finally(&mut self, after: &Self, before: &Self) {
        for (name, value) in &after.locals {
            if before.locals.get(name).is_some_and(|original| original.types == value.types) {
                continue;
            }
            self.locals.insert(*name, Rc::clone(value));
            self.possible.insert(*name);
            self.state.absent.remove(name);
        }
        self.state.conditional.retain(|holder| after.state.conditional.contains(holder));
    }

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

// ConstantArrayType::isSuperTypeOf returns Yes/Maybe in both directions when
// same-key values differ only by null, recursively through plain keyed shapes.
// This proves non-No without substituting Mago's coercive containment or its
// identity comparator, which skips known slots in nonempty keyed arrays.
fn equal_nullable_shape_values(left: &TUnion, right: &TUnion) -> bool {
    let left: Vec<_> = left.types.iter().filter(|atomic| !atomic.is_null()).collect();
    let right: Vec<_> = right.types.iter().filter(|atomic| !atomic.is_null()).collect();
    if left == right {
        return true;
    }
    let ([left], [right]) = (left.as_slice(), right.as_slice()) else {
        return false;
    };
    if let (TAtomic::Array(TArray::Keyed(left)), TAtomic::Array(TArray::Keyed(right))) = (*left, *right)
        && left.parameters.is_none() && right.parameters.is_none()
        && left.callable.is_none() && right.callable.is_none()
        && left.known_non_list == right.known_non_list
        && let (Some(left), Some(right)) = (&left.known_items, &right.known_items)
        && left.keys().eq(right.keys())
    {
        return left.iter().all(|(key, (_, value))| equal_nullable_shape_values(value, &right[key].1));
    }
    false
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
            property_exit: None,
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

    /// enterFunctionLike's definite conditional formal, installed after parameter writes.
    pub(crate) fn add_parameter_condition(&mut self, guard: Word, guard_type: TUnion, target: Word, target_type: TUnion) {
        self.variable_state.conditional.push(ConditionalVariable {
            guard,
            guard_type: Rc::new(guard_type),
            target,
            certainty: VariableCertainty::Defined,
            target_type: Some(Rc::new(target_type)),
        });
    }

    /// Anonymous functions require both sides in `use`; arrows retain outer relationships.
    /// Do not copy absence, extract, or reference state along with these holders.
    pub(crate) fn capture_variable_conditions(&mut self, outer: &Self, captured: impl Fn(Word) -> bool) {
        self.variable_state.conditional.extend(outer.variable_state.conditional.iter().filter(|holder| {
            captured(holder.target) && captured(holder.guard)
        }).cloned());
    }

    /// Keep the pre-RHS holders only when there is a condition to propagate.
    pub(crate) fn assignment_condition_scope(&self) -> Option<Self> {
        (!self.variable_state.conditional.is_empty()).then(|| self.clone())
    }

    /// AssignHandler::processDerivedConditionalExpressionsAfterAssign. The caller supplies
    /// pure RHS pricing; the pending holders must be installed after the ordinary target write.
    pub(crate) fn derive_assignment_conditions<E>(
        &self,
        target: Word,
        reads: impl IntoIterator<Item = Word>,
        assigned: &TUnion,
        mut price: impl FnMut(&mut Self) -> Result<TUnion, E>,
    ) -> Result<VariableState, E> {
        let mut pending = VariableState::default();
        let mut seen = WordSet::default();
        let mut evaluations = 0;
        for read in reads {
            if read == target || !seen.insert(read) {
                continue;
            }
            for holder in self.variable_state.conditional.iter().filter(|holder| {
                holder.target == read && holder.certainty == VariableCertainty::Defined && holder.guard != target
            }) {
                evaluations += 1;
                if evaluations > 16 {
                    return Ok(pending);
                }
                let mut scope = self.clone();
                // assignExpression invalidates the previous guard/consequent, then specifies
                // the conditional holder's definite types on both PHPDoc and native scopes.
                for (name, ty) in [(holder.guard, &holder.guard_type), (read, holder.target_type.as_ref().unwrap())] {
                    scope.invalidate_variable_certainty(name);
                    scope.set_variable_certainty(name, VariableCertainty::Defined, Some(Rc::clone(ty)));
                    if let Some(native) = &mut scope.native_locals {
                        native.insert(name, Rc::clone(ty));
                    }
                }
                let derived = price(&mut scope)?;
                if equal_types(&derived, assigned) {
                    continue;
                }
                let derived = ConditionalVariable {
                    guard: holder.guard,
                    guard_type: Rc::clone(&holder.guard_type),
                    target,
                    certainty: VariableCertainty::Defined,
                    target_type: Some(Rc::new(derived)),
                };
                if !pending.conditional.contains(&derived) {
                    pending.conditional.push(derived);
                }
            }
        }
        Ok(pending)
    }

    pub(crate) fn install_assignment_conditions(&mut self, pending: VariableState) {
        self.variable_state.conditional.extend(pending.conditional);
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
                            let mut combined = combine_union_types_preserving_array_shapes(existing, ty, codebase, CombinerOptions::default());
                            // reduceArrays retains incoming array positions before choosing a primary
                            // superset. Keep that order independently of the shared combiner's sorting.
                            let mut arrays: Vec<_> = combined.types.iter().filter(|atomic| atomic.is_array()).cloned().collect();
                            if arrays.len() > 1 {
                                arrays.sort_by_key(|atomic| existing.types.iter().chain(ty.types.iter()).position(|incoming| incoming == atomic));
                                let mut arrays = arrays.into_iter();
                                for atomic in combined.types.to_mut().iter_mut().filter(|atomic| atomic.is_array()) {
                                    *atomic = arrays.next().unwrap();
                                }
                            }
                            combined
                        } else {
                            combine_union_types(existing, ty, codebase, CombinerOptions::default())
                        };
                        *existing = Rc::new(combined);
                    })
                    .or_insert_with(|| Rc::clone(ty));
            }
        }
        // MutatingScope::mergeWith keeps a call's remembered type only with Yes certainty: a call specified on some
        // branches is read again from its declaration after the join
        let live: Vec<_> = branches.iter().filter(|branch| !branch.state.impossible).collect();
        self.locals.retain(|name, _| {
            !name.as_bytes().ends_with(b")")
                || live.iter().all(|branch| branch.locals.contains_key(name) && !branch.state.absent.contains(name))
        });
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
            // TypeCombinator::reduceArrays visits eligible pairs even when other alternatives
            // remain. Equal shared values allow one extra required key. Equal-sized multi-key
            // shapes additionally allow one No value comparison; nullable differences are Maybe.
            let mut i = 0;
            'arrays: while i < types.len() {
                for j in i + 1..types.len() {
                    if let (TAtomic::Array(TArray::Keyed(left_array)), TAtomic::Array(TArray::Keyed(right_array))) = (&types[i], &types[j])
                        && left_array.parameters.is_none()
                        && right_array.parameters.is_none()
                        && let (Some(left), Some(right)) = (&left_array.known_items, &right_array.known_items)
                    {
                        let (small, large, primary, secondary) = if left.len() <= right.len() {
                            (left, right, j, i)
                        } else {
                            (right, left, i, j)
                        };
                        let equal_shared = !small.is_empty()
                            && small.iter().all(|(key, (_, value))| large.get(key).is_some_and(|(_, other)| equal_types(value, other)))
                            && large.iter().filter(|(key, (optional, _))| !*optional && !small.contains_key(key)).count() <= 1;
                        let nullable_shared = left_array.callable.is_none() && right_array.callable.is_none()
                            && left_array.known_non_list == right_array.known_non_list
                            && small.len() >= 2 && small.keys().eq(large.keys())
                            && small.iter().filter(|(key, (_, value))| !equal_nullable_shape_values(value, &large[*key].1))
                                .take(2).count() <= 1;
                        if equal_shared || nullable_shared {
                            // At most one unproved comparison is safe even if its answer is No;
                            // all remaining shared values are proved Yes/Maybe in both directions.
                            // mergeWith retains the superset's slot and unions its values first.
                            let pair = combine(vec![types[primary].clone(), types[secondary].clone()], codebase, CombinerOptions::default());
                            types.remove(secondary);
                            let primary = primary - usize::from(secondary < primary);
                            types.splice(primary..=primary, pair);
                            continue 'arrays;
                        }
                    }
                }
                i += 1;
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
        self.variable_state.property_initializations =
            crate::property_initialization::join(branches.clone().map(|branch| &branch.state.property_initializations));
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
