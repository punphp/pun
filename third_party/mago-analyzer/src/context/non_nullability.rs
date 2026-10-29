//! PHPStan's shallow nullsafe receiver specification and its matching revert.
//! These devices are scope facts, not assignments or truthy conditions.

use std::rc::Rc;

use mago_algebra::AssertionMap;
use mago_allocator::Arena;
use mago_codex::assertion::Assertion;
use mago_codex::ttype::{atomic::TAtomic, get_mixed, union::TUnion};
use mago_span::HasSpan;
use mago_syntax::cst::{Access, Expression, Variable};
use mago_word::{Word, WordMap, WordSet};

use crate::{
    artifacts::AnalysisArtifacts,
    context::scope::var_has_root,
    context::{
        Context,
        block::{BlockContext, VariableState},
    },
    reconciler::reconcile_keyed_types,
    utils::{
        expression::{array::get_array_target_type_given_index, get_block_expression_id},
        misc::unwrap_expression,
    },
};

pub(crate) struct EnsuredNonNullability<'arena> {
    pub original_type: TUnion,
    pub receiver_type: TUnion,
    pub scope_authoritative: bool,
    restore: Vec<(Expression<'arena>, Word, Option<Rc<TUnion>>)>,
}

/// Type::isNull()->maybe() includes mixed and nullable template bounds, not void.
/// Unlike containsNull(), this controls the short-circuit's scope merge.
pub(crate) fn may_be_null(ty: &TUnion) -> bool {
    ty.types.iter().any(|atomic| match atomic {
        TAtomic::Null => true,
        TAtomic::Mixed(mixed) => !mixed.is_non_null(),
        TAtomic::GenericParameter(parameter) => may_be_null(&parameter.constraint),
        _ => false,
    })
}

pub(crate) fn merge_short_circuit<'ctx, A: Arena>(
    scope: &mut BlockContext<'ctx>,
    before: &BlockContext<'ctx>,
    context: &Context<'ctx, '_, A>,
) {
    let shared: WordSet = scope
        .locals
        .keys()
        .filter(|id| before.locals.contains_key(id))
        .copied()
        .collect();
    let branches = [scope.variable_scope(), before.variable_scope()];
    scope.merge_variable_scope_types(&branches, context.codebase);
    // ScopeOps::finishMerge discards Maybe holders for fetches, after deriving
    // conditional holders. A fact introduced on just the call or skip edge is
    // not a property/offset override on the joined scope. Variable and function
    // call holders have different lookup contracts and remain in the join.
    scope.locals.retain(|id, _| {
        let bytes = id.as_bytes();
        !(bytes.contains(&b'[') || bytes.windows(2).any(|part| part == b"->" || part == b"::"))
            || shared.contains(id)
    });
}

fn tracked_id<'arena, A: Arena>(
    expression: &Expression<'arena>,
    context: &Context<'_, 'arena, A>,
    scope: &BlockContext<'_>,
) -> Option<Word> {
    // get_block_expression_id aliases assignments to their LHS for other
    // callers. PHPStan's expression holder does not share that identity.
    let mut root = unwrap_expression(expression);
    loop {
        root = match root {
            Expression::Assignment(_) => return None,
            Expression::ArrayAccess(access) => unwrap_expression(access.array),
            Expression::Access(Access::Property(access)) => unwrap_expression(access.object),
            Expression::Access(Access::NullSafeProperty(access)) => {
                unwrap_expression(access.object)
            }
            _ => return get_block_expression_id(expression, context, scope),
        };
    }
}

fn scope_type<'ctx, 'arena, A: Arena>(
    expression: &Expression<'arena>,
    context: &mut Context<'ctx, 'arena, A>,
    scope: &BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
) -> TUnion {
    if let Some(ty) = tracked_id(expression, context, scope).and_then(|id| scope.locals.get(&id)) {
        return (**ty).clone();
    }
    // resolveScopeStateType reads current parents, not an offset result stored
    // before a later argument wrote to the array.
    if let Expression::ArrayAccess(access) = unwrap_expression(expression) {
        let container = scope_type(access.array, context, scope, artifacts);
        let index = scope_type(access.index, context, scope, artifacts);
        let parent = tracked_id(access.array, context, scope);
        return context
            .record(|context| {
                get_array_target_type_given_index(
                    context,
                    scope,
                    access.span(),
                    access.array.span(),
                    Some(access.index.span()),
                    &container,
                    &index,
                    false,
                    parent,
                    None,
                    false,
                )
            })
            .0;
    }
    artifacts
        .get_expression_type(expression)
        .cloned()
        .unwrap_or_else(get_mixed)
}

// specifyExpressionTypeInPlace intersects parent arrays with HasOffsetValueType
// before installing the exact child holder. Use an isolated reconciliation
// context: specifying a device must not run alias or conditional transfers.
fn specify<'ctx, 'arena, A: Arena>(
    expression: &Expression<'arena>,
    id: Word,
    ty: TUnion,
    context: &mut Context<'ctx, 'arena, A>,
    scope: &mut BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
) {
    let mut parents = Vec::new();
    let mut parent = unwrap_expression(expression);
    while let Expression::ArrayAccess(access) = parent {
        parent = unwrap_expression(access.array);
        if let Some(parent_id) = tracked_id(parent, context, scope) {
            parents.push((parent_id, parent));
        }
    }
    if !parents.is_empty() {
        let mut specified = scope.clone();
        specified.variable_state = VariableState::default();
        specified.references_in_scope.clear();
        specified.derived_local_sources.clear();
        for (id, expression) in parents.iter().rev() {
            let ty = scope_type(expression, context, &specified, artifacts);
            specified.locals.insert(*id, Rc::new(ty));
        }
        let mut assertions = AssertionMap::default();
        let mut link = unwrap_expression(expression);
        while let Expression::ArrayAccess(access) = link {
            if let Some(parent) = tracked_id(access.array, context, &specified)
                && let Some(key) =
                    scope_type(access.index, context, &specified, artifacts).get_single_array_key()
            {
                assertions.insert(parent, vec![vec![Assertion::HasArrayKey(key)]]);
            }
            link = unwrap_expression(access.array);
        }
        assertions.insert(
            id,
            vec![ty.types.iter().cloned().map(Assertion::IsType).collect()],
        );
        reconcile_keyed_types(
            context,
            &assertions,
            AssertionMap::default(),
            &mut specified,
            &mut WordSet::default(),
            &WordSet::default(),
            &expression.span(),
            false,
            false,
        );
        for (parent, _) in parents {
            if let Some(ty) = specified.locals.get(&parent) {
                scope.locals.insert(parent, Rc::clone(ty));
            }
        }
    }
    scope.locals.insert(id, Rc::new(ty));
}

pub(crate) fn ensure<'ctx, 'arena, A: Arena>(
    expression: &Expression<'arena>,
    context: &mut Context<'ctx, 'arena, A>,
    scope: &mut BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
) -> EnsuredNonNullability<'arena> {
    let id = tracked_id(expression, context, scope);
    let walk_type = artifacts
        .get_expression_type(expression)
        .cloned()
        .unwrap_or_else(get_mixed);
    let current = id
        .and_then(|id| scope.locals.get(&id).map(|ty| (**ty).clone()))
        .unwrap_or_else(|| walk_type.clone());
    let original_type = id
        .and_then(|id| {
            context
                .ensured_non_null_types
                .iter()
                .rev()
                .find_map(|frame| frame.get(&id))
        })
        .cloned()
        .unwrap_or(walk_type);
    // A definitely null receiver is not deviced. removeNull removes only
    // explicit null alternatives, leaving mixed, void and template bounds alone.
    let mut receiver_type = if current.is_null() {
        current.clone()
    } else {
        current.remove_null()
    };
    if current != receiver_type
        && !matches!(
            unwrap_expression(expression),
            Expression::Variable(Variable::Direct(_))
        )
    {
        receiver_type.set_possibly_undefined(false, Some(false));
    }
    let scope_authoritative = current != receiver_type || id.is_some_and(|id| scope.locals.contains_key(&id));
    let mut restore = Vec::new();
    let mut originals = WordMap::default();
    if current != receiver_type
        && let Some(id) = id
    {
        // Save only the immediate array parent. Its later specification
        // propagates recursively while retaining unrelated ancestor writes.
        if let Expression::ArrayAccess(access) = unwrap_expression(expression)
            && let Some(parent_id) = tracked_id(access.array, context, scope)
        {
            let original = scope.locals.get(&parent_id).cloned();
            if let Some(ty) = original
                .as_deref()
                .or_else(|| artifacts.get_expression_type(access.array))
            {
                originals.insert(parent_id, ty.clone());
            }
            restore.push((access.array.clone(), parent_id, original));
        }
        restore.push((expression.clone(), id, Some(Rc::new(current.clone()))));
        originals.insert(id, current);
        specify(
            expression,
            id,
            receiver_type.clone(),
            context,
            scope,
            artifacts,
        );
    }
    context.ensured_non_null_types.push(originals);
    EnsuredNonNullability {
        original_type,
        receiver_type,
        scope_authoritative,
        restore,
    }
}

impl<'arena> EnsuredNonNullability<'arena> {
    pub(crate) fn revert<'ctx, A: Arena>(
        self,
        context: &mut Context<'ctx, 'arena, A>,
        scope: &mut BlockContext<'ctx>,
        artifacts: &AnalysisArtifacts,
    ) {
        context.ensured_non_null_types.pop();
        for (expression, name, original) in self.restore {
            if let Some(ty) = original {
                specify(&expression, name, (*ty).clone(), context, scope, artifacts);
            } else {
                scope.locals.retain(|id, _| !var_has_root(*id, name));
            }
        }
    }
}
