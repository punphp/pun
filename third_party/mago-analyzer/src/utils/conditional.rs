use mago_allocator::Arena;
use std::cell::RefCell;
use std::rc::Rc;

use mago_algebra::AssertionMap;
use mago_codex::assertion::Assertion;
use mago_codex::ttype::TType;
use mago_codex::ttype::union::TUnion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;
use mago_syntax::cst::BinaryOperator;
use mago_syntax::cst::Expression;
use mago_syntax::cst::UnaryPrefixOperator;
use mago_word::Word;
use mago_word::WordMap;
use mago_word::WordSet;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::artifacts::BooleanExpressionScope;
use crate::assertion::get_class_type_relations;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::context::scope::conditional_scope::IfConditionalScope;
use crate::context::scope::if_scope::IfScope;
use crate::error::AnalysisError;
use crate::reconciler::reconcile_keyed_types;

pub(crate) fn boolean_scopes<'a>(
    expression: &Expression,
    artifacts: &'a AnalysisArtifacts,
) -> Option<&'a crate::artifacts::BooleanExpressionScopes> {
    let expression = crate::utils::misc::unwrap_expression(expression);
    let span = expression.span();
    artifacts
        .boolean_expression_scopes
        .get(&(span.start.offset, span.end.offset))
}

/// Transfer the actual operand scope after specifications, which must not re-select
/// a pre-evaluation value when a short-circuiting expression has changed its type.
pub(crate) fn apply_boolean_branch(
    expression: &Expression,
    truthy: bool,
    artifacts: &AnalysisArtifacts,
    block_context: &mut BlockContext<'_>,
) -> WordSet {
    let mut changed = WordSet::default();
    if let Some(scopes) = boolean_scopes(expression, artifacts) {
        let scope = if truthy {
            &scopes.truthy
        } else {
            &scopes.falsey
        };
        for (key, value) in &scope.locals {
            if block_context.locals.get(key) != Some(value) {
                changed.insert(*key);
                block_context.locals.insert(*key, Rc::clone(value));
            }
        }
        scope.restore_certainty(block_context);
        BlockContext::retain_unreconciled_clauses(&mut block_context.clauses, &changed);
    }
    changed
}

pub(crate) fn record_boolean_scopes<'ctx, A: Arena>(
    context: &mut Context<'ctx, '_, A>,
    artifacts: &mut AnalysisArtifacts,
    binary: &mago_syntax::cst::Binary<'_>,
    left: &BlockContext<'ctx>,
    right: &BlockContext<'ctx>,
) -> BooleanExpressionScope {
    // BooleanAndHandler/BooleanOrHandler join the actual walked scopes. A false &&
    // (or true ||) can have evaluated either one or both operands.
    let mut post = left.clone();
    if let Some(scopes) = boolean_scopes(binary.lhs, artifacts) {
        post.locals.clone_from(&scopes.post.locals);
        scopes.post.restore_certainty(&mut post);
    }
    let mut right = right.clone();
    if let Some(scopes) = boolean_scopes(binary.rhs, artifacts) {
        right.locals.clone_from(&scopes.post.locals);
        scopes.post.restore_certainty(&mut right);
    }
    let operand_scopes = [post.variable_scope(), right.variable_scope()];
    for (key, value) in &right.locals {
        let value = match post.locals.get(key) {
            Some(left) => mago_codex::ttype::combine_union_types_rc(
                left,
                value,
                context.codebase,
                context.settings.combiner_options(),
            ),
            None => Rc::clone(value),
        };
        post.locals.insert(*key, value);
    }
    // Join certainty from the incoming edges, before the RHS-only type cache can
    // make an output-reference variable appear definitely assigned on the LHS.
    post.merge_variable_scopes(&operand_scopes, context.codebase);
    let expression = Expression::Binary(binary.clone());
    let is_and = matches!(
        binary.operator,
        BinaryOperator::And(_) | BinaryOperator::LowAnd(_)
    );
    let branch = |context: &mut Context<'ctx, '_, A>, truthy: bool| {
        let (subject, mut scope) = if is_and == truthy {
            (binary.rhs, right.clone())
        } else {
            (&expression, post.clone())
        };
        let mut clauses = crate::formula::get_formula(
            subject.span(),
            subject.span(),
            subject,
            context.get_assertion_context_from_block(&scope),
            artifacts,
            &context.settings.algebra_thresholds(),
            context.settings.formula_size_threshold,
        )
        .unwrap_or_default();
        if !truthy {
            clauses = crate::formula::negate_or_synthesize(
                clauses,
                subject,
                context.get_assertion_context_from_block(&scope),
                artifacts,
                &context.settings.algebra_thresholds(),
                context.settings.formula_size_threshold,
            );
        }
        let (assertions, _) =
            mago_algebra::find_satisfying_assignments(&clauses, None, &mut WordSet::default());
        reconcile_keyed_types(
            context,
            &assertions,
            AssertionMap::default(),
            &mut scope,
            &mut WordSet::default(),
            &WordSet::default(),
            &subject.span(),
            false,
            false,
        );
        if is_and == truthy {
            apply_boolean_branch(subject, truthy, artifacts, &mut scope);
        }
        BooleanExpressionScope {
            locals: scope.locals,
            variable_state: scope.variable_state,
            variables_possibly_in_scope: scope.variables_possibly_in_scope,
        }
    };
    let truthy = branch(context, true);
    let falsey = branch(context, false);
    let post = BooleanExpressionScope {
        locals: post.locals,
        variable_state: post.variable_state,
        variables_possibly_in_scope: post.variables_possibly_in_scope,
    };
    let span = binary.span();
    artifacts.boolean_expression_scopes.insert(
        (span.start.offset, span.end.offset),
        crate::artifacts::BooleanExpressionScopes {
            post: post.clone(),
            truthy,
            falsey,
        },
    );
    post
}

pub(crate) fn analyze<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    outer_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    if_scope: &mut IfScope,
    condition: &Expression<'arena>,
    check_for_paradoxes: bool,
) -> Result<IfConditionalScope<'ctx>, AnalysisError>
where
    A: Arena,
{
    if !crate::native::condition_shape(condition, true) {
        outer_context.native_locals = None;
    }
    let mut entry_clauses = vec![];

    if !if_scope.negated_clauses.is_empty() {
        entry_clauses.extend(if_scope.negated_clauses.iter().cloned());

        let mut changed_var_ids = WordSet::default();

        if !if_scope.negated_types.is_empty() {
            let mut tmp_context = outer_context.clone();

            reconcile_keyed_types(
                context,
                &if_scope.negated_types,
                AssertionMap::default(),
                &mut tmp_context,
                &mut changed_var_ids,
                &WordSet::default(),
                &condition.span(),
                check_for_paradoxes,
                false,
            );

            if !changed_var_ids.is_empty() {
                *outer_context = tmp_context;
            }
        }
    }

    let externally_applied_if_cond_expr = get_definitely_evaluated_expression_after_if(condition);
    let internally_applied_if_cond_expr = get_definitely_evaluated_expression_inside_if(condition);
    let externally_applied_context = outer_context;

    let pre_condition_locals = externally_applied_context.locals.clone();
    let pre_referenced_var_ids = std::mem::take(&mut externally_applied_context.conditionally_referenced_variable_ids);
    let pre_assigned_var_ids = std::mem::take(&mut externally_applied_context.assigned_variable_ids);

    let mut if_body_context = None;
    if expressions_differ(externally_applied_if_cond_expr, internally_applied_if_cond_expr) {
        if_body_context = Some(externally_applied_context.clone());
    }

    let was_inside_conditional = externally_applied_context.flags.inside_conditional();

    // When the second pass below runs it re-covers everything this pass touches, so record and
    // discard this pass's diagnostics to avoid reporting them twice; its type/data-flow effects
    // are kept regardless.
    let must_reanalyze_full_condition = expressions_differ(internally_applied_if_cond_expr, condition)
        || expressions_differ(externally_applied_if_cond_expr, condition);

    externally_applied_context.flags.set_inside_conditional(true);
    let tmp_if_body_context = std::mem::take(&mut externally_applied_context.if_body_context);
    if must_reanalyze_full_condition {
        context.collector.start_recording();
    }
    externally_applied_if_cond_expr.analyze(context, externally_applied_context, artifacts)?;
    if must_reanalyze_full_condition {
        context.collector.finish_recording();
    }
    externally_applied_context.if_body_context = tmp_if_body_context;

    let first_cond_assigned_var_ids = externally_applied_context.assigned_variable_ids.clone();
    let first_cond_referenced_var_ids = externally_applied_context.conditionally_referenced_variable_ids.clone();

    externally_applied_context.assigned_variable_ids.extend(pre_assigned_var_ids);
    externally_applied_context.conditionally_referenced_variable_ids.extend(pre_referenced_var_ids);
    externally_applied_context.flags.set_inside_conditional(was_inside_conditional);

    let if_body_context = if_body_context.unwrap_or_else(|| externally_applied_context.clone());
    let mut post_if_context = externally_applied_context.clone();
    let mut conditionally_referenced_variable_ids;
    let assigned_in_conditional_variable_ids;
    let newish_var_ids;
    let collect_newish_var_ids = |locals: &WordMap<Rc<TUnion>>, referenced: &WordSet, assigned: &WordMap<u32>| {
        locals
            .keys()
            .filter(|k| {
                !pre_condition_locals.contains_key(*k) && !referenced.contains(*k) && !assigned.contains_key(*k)
            })
            .copied()
            .collect::<WordSet>()
    };
    let mut if_body_context = if must_reanalyze_full_condition {
        let mut if_body_context = if_body_context;
        let tmp_if_body_context_nested = if_body_context.if_body_context.take();
        let mut if_conditional_context = if_body_context.clone();
        if_conditional_context.if_body_context = Some(Rc::new(RefCell::new(if_body_context)));
        if_conditional_context.assigned_variable_ids = WordMap::default();
        if_conditional_context.conditionally_referenced_variable_ids.clear();

        let was_inside_conditional = if_conditional_context.flags.inside_conditional();
        if_conditional_context.flags.set_inside_conditional(true);
        condition.analyze(context, &mut if_conditional_context, artifacts)?;
        if_conditional_context.flags.set_inside_conditional(was_inside_conditional);

        if_conditional_context.conditionally_referenced_variable_ids.extend(first_cond_referenced_var_ids);
        if_conditional_context.assigned_variable_ids.extend(first_cond_assigned_var_ids);

        conditionally_referenced_variable_ids = if_conditional_context.conditionally_referenced_variable_ids.clone();
        assigned_in_conditional_variable_ids = if_conditional_context.assigned_variable_ids.clone();

        newish_var_ids = collect_newish_var_ids(
            &if_conditional_context.locals,
            &conditionally_referenced_variable_ids,
            &assigned_in_conditional_variable_ids,
        );

        let mut if_body_context = {
            // SAFETY: We set this field above.
            let rc = unsafe { if_conditional_context.if_body_context.unwrap_unchecked() };
            // SAFETY: The Rc has a strong count of one.
            let ref_cell = unsafe { Rc::try_unwrap(rc).unwrap_unchecked() };
            ref_cell.into_inner()
        };
        if_body_context.if_body_context = tmp_if_body_context_nested;
        if_body_context
    } else {
        conditionally_referenced_variable_ids = first_cond_referenced_var_ids;
        assigned_in_conditional_variable_ids = first_cond_assigned_var_ids;
        newish_var_ids = collect_newish_var_ids(
            &if_body_context.locals,
            &conditionally_referenced_variable_ids,
            &assigned_in_conditional_variable_ids,
        );
        if_body_context
    };

    if let Some(scopes) = boolean_scopes(condition, artifacts) {
        post_if_context.locals.clone_from(&scopes.post.locals);
        scopes.post.restore_certainty(&mut post_if_context);
        externally_applied_context.locals.clone_from(&scopes.post.locals);
        scopes.post.restore_certainty(externally_applied_context);
    }

    if check_for_paradoxes && let Some(condition_type) = artifacts.get_rc_expression_type(condition) {
        handle_paradoxical_condition(context, condition, condition_type);
    }

    conditionally_referenced_variable_ids.retain(|k| !assigned_in_conditional_variable_ids.contains_key(k));
    conditionally_referenced_variable_ids.extend(newish_var_ids);

    let condition_span = condition.span();
    let condition_range = (condition_span.start_offset(), condition_span.end_offset());
    if let Some(assertions) = artifacts.if_true_assertions.get(&condition_range) {
        for (key, assertion_set) in assertions {
            if key.as_bytes().ends_with(b"()") {
                if_body_context.active_method_call_assertions.entry(*key).or_default().extend(assertion_set.clone());
            }
        }
    }

    if let Some(assertions) = artifacts.true_branch_only_assertions.get(&condition_range) {
        let mut var_assertions: AssertionMap<Word, Vec<Vec<Assertion>>> = AssertionMap::default();
        for (key, assertion_set) in assertions {
            var_assertions.entry(*key).or_default().extend(assertion_set.clone());
        }

        if !var_assertions.is_empty() {
            let mut changed_var_ids = WordSet::default();
            reconcile_keyed_types(
                context,
                &var_assertions,
                AssertionMap::default(),
                &mut if_body_context,
                &mut changed_var_ids,
                &WordSet::default(),
                &condition_span,
                false,
                false,
            );
        }
    }

    for (source, target) in
        get_class_type_relations(condition, context.get_assertion_context_from_block(&if_body_context))
    {
        if_body_context.add_class_type_relation(source, target);
    }

    if_body_context.native_locals =
        crate::native::branch(condition, &post_if_context.native_locals, artifacts, true, context.codebase);

    Ok(IfConditionalScope {
        if_body_context,
        post_if_context,
        conditionally_referenced_variable_ids,
        assigned_in_conditional_variable_ids,
        entry_clauses,
    })
}

#[inline]
fn expressions_differ(left: &Expression<'_>, right: &Expression<'_>) -> bool {
    !std::ptr::eq(left, right) && left != right
}

fn get_definitely_evaluated_expression_after_if<'ast, 'arena>(
    condition: &'ast Expression<'arena>,
) -> &'ast Expression<'arena> {
    match &condition {
        Expression::Parenthesized(p) => {
            return get_definitely_evaluated_expression_after_if(p.expression);
        }
        Expression::Binary(binary) => {
            if let BinaryOperator::Or(_)
            | BinaryOperator::LowOr(_)
            | BinaryOperator::And(_)
            | BinaryOperator::LowAnd(_) = binary.operator
            {
                return get_definitely_evaluated_expression_after_if(binary.lhs);
            }

            return condition;
        }
        Expression::UnaryPrefix(unary) => {
            if let UnaryPrefixOperator::Not(_) = unary.operator {
                let inner_expression = get_definitely_evaluated_expression_inside_if(unary.operand);

                if expressions_differ(inner_expression, unary.operand) {
                    return inner_expression;
                }
            }
        }
        _ => {}
    }

    condition
}

fn get_definitely_evaluated_expression_inside_if<'ast, 'arena>(
    condition: &'ast Expression<'arena>,
) -> &'ast Expression<'arena> {
    match &condition {
        Expression::Parenthesized(p) => {
            return get_definitely_evaluated_expression_inside_if(p.expression);
        }
        Expression::Binary(binary) => {
            if let BinaryOperator::Or(_) | BinaryOperator::LowOr(_) = binary.operator {
                return get_definitely_evaluated_expression_inside_if(binary.lhs);
            }

            return condition;
        }
        Expression::UnaryPrefix(unary) => {
            if let UnaryPrefixOperator::Not(_) = unary.operator {
                let inner_expression = get_definitely_evaluated_expression_inside_if(unary.operand);

                if expressions_differ(inner_expression, unary.operand) {
                    return inner_expression;
                }
            }
        }
        _ => {}
    }

    condition
}

pub fn handle_paradoxical_condition<T, A>(context: &mut Context<'_, '_, A>, expression: &T, expression_type: &TUnion)
where
    T: HasSpan,
    A: Arena,
{
    if expression_type.is_always_falsy() {
        let type_id = expression_type.get_id();
        context.collector.report_with_code(
            IssueCode::ImpossibleCondition,
            Issue::warning(format!(
                "This condition (type `{type_id}`) will always evaluate to false."
            ))
            .with_annotation(
                Annotation::primary(expression.span())
                    .with_message(format!("Expression of type `{type_id}` is always falsy")),
            )
            .with_note(
                "Because this condition is always false, the code block it controls will never be executed."
            )
            .with_help(
                "Check the logic of this expression. If the code block is intended to be unreachable, consider removing it. Otherwise, revise the condition.",
            ),
        );
    } else if expression_type.is_always_truthy() {
        let type_id = expression_type.get_id();
        context.collector.report_with_code(
            IssueCode::RedundantCondition,
            Issue::warning(format!(
                "This condition (type `{type_id}`) will always evaluate to true."
            ))
            .with_annotation(
                Annotation::primary(expression.span())
                    .with_message(format!("Expression of type `{type_id}` is always truthy")),
            )
            .with_note(
                "Because this condition is always true, the code block it controls will always execute if this part of the code is reached."
            ).with_note(
                "The explicit condition might be redundant."
            )
            .with_help(
                "Consider simplifying or removing the conditional check if the guarded code should always execute, or verify the expression's logic if a conditional check is truly needed.",
            ),
        );
    }
}
