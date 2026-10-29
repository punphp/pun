use mago_allocator::Arena;
use std::borrow::Cow;
use std::rc::Rc;

use mago_algebra::AssertionMap;
use mago_codex::assertion::Assertion;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::combine_union_types;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::union::TUnion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;
use mago_syntax::cst::Binary;
use mago_syntax::cst::Expression;
use mago_syntax::cst::Literal;
use mago_syntax::cst::UnaryPrefixOperator;
use mago_syntax::cst::Variable;
use mago_text_edit::Safety;
use mago_text_edit::TextEdit;
use mago_word::Word;
use mago_word::WordSet;
use mago_word::concat_word;
use mago_word::word;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::context::non_nullability::may_be_null;
use crate::error::AnalysisError;
use crate::reconciler::reconcile_keyed_types;
use crate::reconciler::simple_negated_assertion_reconciler::offset_is_required;
use crate::utils::expression::get_block_expression_id;
use crate::utils::expression::get_literal_array_key;
use crate::utils::expression::get_root_expression_id;
use crate::utils::misc::unwrap_expression;

/// Analyzes the null coalescing operator (`??`).
///
/// The result type is determined as follows:
/// - If the left-hand side (LHS) is definitely `null`, the result type is the type of the right-hand side (RHS).
///   A hint is issued about the LHS always being `null`.
/// - If the LHS is definitely not `null`, the result type is the type of the LHS. The RHS is still analyzed
///   for potential errors but does not contribute to the result type. A hint is issued about the RHS being redundant.
/// - If the LHS is nullable (can be `null` or other types), the result type is the union of the
///   non-null parts of the LHS and the type of the RHS.
/// - If the LHS type is unknown (`mixed`), the result type is `mixed`.
#[allow(clippy::unwrap_used)]
pub fn analyze_null_coalesce_operation<'ctx, 'arena, A>(
    binary: &Binary<'arena>,
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
) -> Result<(), AnalysisError>
where
    A: Arena,
{
    let was_inside_isset = block_context.flags.inside_isset();
    block_context.flags.set_inside_isset(matches!(
        binary.lhs,
        Expression::Variable(_) | Expression::Access(_) | Expression::ArrayAccess(_)
    ));
    binary.lhs.analyze(context, block_context, artifacts)?;
    block_context.flags.set_inside_isset(was_inside_isset);

    let lhs_type_option = artifacts.get_rc_expression_type(&binary.lhs);

    let Some(lhs_type) = lhs_type_option else {
        binary.rhs.analyze(context, block_context, artifacts)?;

        artifacts.set_expression_type(binary, get_mixed());

        return Ok(());
    };

    // CoalesceHandler joins the isset-success scope with the RHS scope, even for a statically known LHS.
    // Narrow without walking the LHS again: its assignments and observer events must occur only once.
    // An isset-allowed absent variable has a mixed placeholder read, but only the RHS supplies its value.
    let left_is_absent = get_variable_name(binary.lhs)
        .is_some_and(|name| block_context.variable_certainty(name).is_absent())
        || offset_is_absent(binary.lhs, artifacts);
    let is_static_var = is_rooted_in_static_local(binary.lhs, &block_context.static_locals);
    let lhs_id = evaluated_offset_id(binary.lhs, context, block_context, artifacts);
    // Array refinement uses evaluated, coerced keys. ScopeOps::nodeKey instead
    // remembers the expression itself, for arrays as well as string receivers.
    let mut offsets = Vec::new();
    let mut link = unwrap_expression(binary.lhs);
    while let Expression::ArrayAccess(access) = link {
        if let Some(evaluated) = evaluated_offset_id(link, context, block_context, artifacts)
            && let Some(value) = artifacts.get_rc_expression_type(link)
        {
            offsets.push((
                evaluated,
                offset_expression_id(link, context, block_context, artifacts),
                Rc::clone(value),
            ));
        }
        link = unwrap_expression(access.array);
    }
    let required_offset = if let Expression::ArrayAccess(access) = unwrap_expression(binary.lhs)
        && let Some(id) = lhs_id
        && let Some(parent) = evaluated_offset_id(access.array, context, block_context, artifacts)
        && let Some(parent_type) = artifacts.get_rc_expression_type(access.array)
    {
        // A nested parent may be an evaluated read, not a specified holder.
        // Query its actual read type without publishing a parent existence fact.
        let mut presence = BlockContext::new(block_context.scope.clone(), false);
        presence.locals.insert(parent, Rc::clone(parent_type));
        offset_is_required(id, &presence)
    } else {
        false
    };
    let mut lhs_context = block_context.clone();
    if let Some(root) = get_root_expression_id(binary.lhs)
        && !block_context.variable_certainty(root).is_absent()
    {
        let subject = match unwrap_expression(binary.lhs) {
            Expression::ArrayAccess(access)
                if artifacts.get_expression_type(access.array).is_some_and(TUnion::is_array) =>
            {
                lhs_id.unwrap_or(root)
            }
            _ => root,
        };
        let mut assertions = AssertionMap::default();
        assertions.insert(subject, vec![vec![Assertion::IsIsset]]);
        reconcile_keyed_types(
            context,
            &assertions,
            AssertionMap::default(),
            &mut lhs_context,
            &mut WordSet::default(),
            &WordSet::default(),
            &binary.lhs.span(),
            false,
            false,
        );
    }
    if matches!(unwrap_expression(binary.lhs), Expression::ArrayAccess(_))
        && !lhs_type.is_error()
        && let Some(id) = lhs_id
    {
        let mut present_value = lhs_type.to_non_nullable();
        present_value.set_possibly_undefined(false, Some(false));
        lhs_context.locals.insert(id, Rc::new(present_value));
    }
    // CoalesceCompositionHelper reads the LHS on its isset-success scope.
    // Removing an explicit null from the raw read alone does not narrow mixed.
    let non_null_lhs_type = lhs_id
        .and_then(|id| lhs_context.locals.get(&id))
        .unwrap_or(lhs_type)
        .remove_null();
    restore_offset_holders(&offsets, block_context, &mut lhs_context);
    let lhs_scope = lhs_context.variable_scope();
    let rhs_scope;
    let mut result_type: TUnion;

    let left_is_set = !left_is_absent
        && !may_be_null(lhs_type)
        && !lhs_type.possibly_undefined()
        && !lhs_type.possibly_undefined_from_try()
        && !is_static_var;
    // Assignment::analyze preserves the three-byte `??=` operator span on its synthetic Binary.
    // AssignOpHandler narrows its RHS even counterfactually; CoalesceHandler skips that when surely set.
    let is_coalesce_assignment = binary.operator.span().length() == 3;
    let mut rhs_context = block_context.clone();
    let mut assertions = AssertionMap::default();
    if let Expression::ArrayAccess(access) = unwrap_expression(binary.lhs) {
        if let Some(id) = lhs_id
            && required_offset
        {
            // Only actual required entries can be pinned to null. Absent-read
            // placeholders and maybe-absent values do not specify RHS holders.
            // Seed evaluated parents too, so a nested null pin refines the chain.
            for (evaluated, _, value) in offsets.iter().rev() {
                rhs_context.locals.insert(*evaluated, Rc::clone(value));
            }
            assertions.insert(id, vec![vec![Assertion::IsType(TAtomic::Null)]]);
        } else if !matches!(unwrap_expression(access.array), Expression::ArrayAccess(_))
            && artifacts.get_expression_type(access.array).is_some_and(|parent| !parent.is_mixed())
            && let Some(parent) = evaluated_offset_id(access.array, context, block_context, artifacts)
            && let Some(key) = get_literal_array_key(access.index, artifacts)
        {
            // DefaultNarrowingHelper preserves maybe-absent values and filters only
            // top-level constant-array parents, without leaking nested-key existence.
            assertions.insert(parent, vec![vec![Assertion::DoesNotHaveNonnullEntryForKey(key)]]);
        }
    } else if (!left_is_set || is_coalesce_assignment)
        && let Some(id) = lhs_id
    {
        assertions.insert(id, vec![vec![Assertion::IsNotIsset]]);
    }
    if !assertions.is_empty() {
        reconcile_keyed_types(
            context,
            &assertions,
            AssertionMap::default(),
            &mut rhs_context,
            &mut WordSet::default(),
            &WordSet::default(),
            &binary.lhs.span(),
            false,
            false,
        );
    }
    restore_offset_holders(&offsets, block_context, &mut rhs_context);

    if (lhs_type.is_null() || left_is_absent) && !is_static_var {
        context.collector.propose_with_code(
            IssueCode::RedundantNullCoalesce,
            Issue::help("Redundant null coalesce: left-hand side is always `null`.")
                .with_annotation(Annotation::primary(binary.lhs.span()).with_message("This is always `null`"))
                .with_annotation(
                    Annotation::secondary(binary.rhs.span())
                        .with_message("This right-hand side will always be evaluated"),
                )
                .with_note("The right-hand side of `??` will always be evaluated.")
                .with_help("Consider directly using the right-hand side expression."),
            |edits| {
                edits.push(
                    TextEdit::delete(binary.lhs.span().join(binary.operator.span()))
                        .with_safety(Safety::PotentiallyUnsafe),
                );
            },
        );

        *block_context = rhs_context;
        binary.rhs.analyze(context, block_context, artifacts)?;
        result_type = artifacts.get_expression_type(&binary.rhs).cloned().unwrap_or_else(get_mixed); // Fallback if RHS analysis fails
        rhs_scope = block_context.variable_scope();
    } else if left_is_set {
        context.collector.propose_with_code(
            IssueCode::RedundantNullCoalesce,
            Issue::help(
                "Redundant null coalesce: left-hand side can never be `null` or undefined."
            )
            .with_annotation(Annotation::primary(binary.lhs.span()).with_message(format!(
                "This expression (type `{}`) is never `null` or undefined",
                lhs_type.get_id()
            )))
            .with_annotation(
                Annotation::secondary(binary.rhs.span()).with_message("This right-hand side will never be evaluated"),
            )
            .with_note(
                "The null coalesce operator `??` only evaluates the right-hand side if the left-hand side is `null` or not set.",
            )
            .with_help("Consider removing the `??` operator and the right-hand side expression."),
            |edits| {
                edits.push(TextEdit::delete(
                    binary.operator.span().join(binary.rhs.span())
                ).with_safety(Safety::PotentiallyUnsafe));
            },
        );

        result_type = non_null_lhs_type;

        // Analyze the unreachable right-hand side in a separate disposable context
        // Prevents its terminating control flow and recorded thrown exceptions from affecting the remainder of the block
        let mut dead_rhs_context = rhs_context;
        dead_rhs_context.flags.set_has_returned(true);
        binary.rhs.analyze(context, &mut dead_rhs_context, artifacts)?;
        rhs_scope = dead_rhs_context.variable_scope();
    } else {
        *block_context = rhs_context;
        let has_returned = block_context.flags.has_returned();
        block_context.flags.set_has_returned(false);
        binary.rhs.analyze(context, block_context, artifacts)?;
        block_context.flags.set_has_returned(block_context.flags.has_returned() && has_returned);
        rhs_scope = block_context.variable_scope();

        let rhs_type =
            artifacts.get_expression_type(&binary.rhs).map_or_else(|| Cow::Owned(get_mixed()), Cow::Borrowed);

        result_type =
            combine_union_types(&non_null_lhs_type, &rhs_type, context.codebase, context.settings.combiner_options());
    }

    let mut continuing_scopes = vec![lhs_scope];
    if !artifacts.get_expression_type(&binary.rhs).is_some_and(TUnion::is_never) {
        continuing_scopes.push(rhs_scope);
    }
    block_context.merge_variable_scope_types(&continuing_scopes, context.codebase);

    // Check if clauses guarantee non-null result from OR assertion
    // If we have assert($x !== null || $y !== null), then $x ?? $y cannot be null
    if result_type.is_nullable()
        && let (Some(lhs_var), Some(rhs_var)) = (get_variable_name(binary.lhs), get_variable_name(binary.rhs))
    {
        for clause in &block_context.clauses {
            if clause.possibilities.len() == 2
                && clause.possibilities.contains_key(&lhs_var)
                && clause.possibilities.contains_key(&rhs_var)
            {
                let lhs_not_null = clause.possibilities.get(&lhs_var).is_some_and(|assertions| {
                    assertions.values().any(|a| {
                        matches!(a, Assertion::IsNotIdentical(TAtomic::Null) | Assertion::IsNotType(TAtomic::Null))
                    })
                });

                let rhs_not_null = clause.possibilities.get(&rhs_var).is_some_and(|assertions| {
                    assertions.values().any(|a| {
                        matches!(a, Assertion::IsNotIdentical(TAtomic::Null) | Assertion::IsNotType(TAtomic::Null))
                    })
                });

                if lhs_not_null && rhs_not_null {
                    result_type = result_type.remove_null();
                    break;
                }
            }
        }
    }

    if is_coalesce_assignment
        && matches!(unwrap_expression(binary.lhs), Expression::ArrayAccess(_))
        && let Some(id) = offset_expression_id(binary.lhs, context, block_context, artifacts)
    {
        // AssignHandler publishes a written target with Yes certainty even for
        // MixedType/ErrorType receivers. Parent-array refinement is separate;
        // array_assignment commits it after this synthetic ??= result.
        block_context.locals.insert(id, Rc::new(result_type.clone()));
    }

    artifacts.set_rc_expression_type(binary, Rc::new(result_type));

    Ok(())
}

fn get_variable_name(expr: &Expression<'_>) -> Option<Word> {
    match unwrap_expression(expr) {
        Expression::Variable(Variable::Direct(var)) => Some(word(var.name)),
        _ => None,
    }
}

/// IssetabilityResolution checks offset existence separately from the read's value.
/// A prepared missing read may be ErrorType, just like a required error-valued entry.
/// Query evaluated array metadata, not that sentinel, and never evaluate an index twice.
fn offset_is_absent(expression: &Expression<'_>, artifacts: &AnalysisArtifacts) -> bool {
    let Expression::ArrayAccess(access) = unwrap_expression(expression) else {
        return false;
    };
    if offset_is_absent(access.array, artifacts) {
        return true;
    }
    artifacts.get_expression_type(access.array).zip(get_literal_array_key(access.index, artifacts)).is_some_and(
        |(parent, key)| {
            parent.types.iter().all(|atomic| match atomic {
                TAtomic::Array(TArray::Keyed(array)) => {
                    array.parameters.is_none() && !array.known_items.as_ref().is_some_and(|items| items.contains_key(&key))
                }
                TAtomic::Array(TArray::List(list)) => {
                    key.get_integer().and_then(|index| usize::try_from(index).ok()).is_none_or(|index| {
                        list.known_count.is_some_and(|count| index >= count)
                            || (list.element_type.is_never()
                                && !list.known_elements.as_ref().is_some_and(|items| items.contains_key(&index)))
                    })
                }
                _ => false,
            })
        },
    )
}

/// Publish specified expressions, not the temporary paths used for parent-array refinement.
fn restore_offset_holders(
    offsets: &[(Word, Option<Word>, Rc<TUnion>)],
    before: &BlockContext<'_>,
    scope: &mut BlockContext<'_>,
) {
    for (evaluated, expression, _) in offsets {
        if Some(*evaluated) == *expression {
            continue;
        }
        if let Some(value) = scope.locals.remove(evaluated)
            && let Some(expression) = expression
        {
            scope.locals.insert(*expression, value);
        }
        if let Some(original) = before.locals.get(evaluated) {
            scope.locals.insert(*evaluated, Rc::clone(original));
        }
    }
}

/// ScopeOps keys a fetch by its expression, not by the index's value or toArrayKey().
fn offset_expression_id<'arena, A: Arena>(
    expression: &Expression<'arena>,
    context: &Context<'_, 'arena, A>,
    block_context: &BlockContext<'_>,
    artifacts: &AnalysisArtifacts,
) -> Option<Word> {
    match unwrap_expression(expression) {
        Expression::ArrayAccess(access) => {
            let parent = offset_expression_id(access.array, context, block_context, artifacts)?;
            let index = offset_expression_id(access.index, context, block_context, artifacts)?;
            Some(concat_word!(parent.as_bytes(), b"[", index.as_bytes(), b"]"))
        }
        Expression::Literal(Literal::String(_)) => {
            let value = artifacts.get_expression_type(expression)?.get_single_literal_string_value()?;
            Some(ArrayKey::String(word(value)).id_word())
        }
        Expression::Literal(Literal::Integer(_)) => {
            Some(artifacts.get_expression_type(expression)?.get_single_array_key()?.id_word())
        }
        Expression::Literal(Literal::True(keyword) | Literal::False(keyword) | Literal::Null(keyword)) => {
            Some(word(keyword.value))
        }
        Expression::UnaryPrefix(prefix)
            if matches!(prefix.operator, UnaryPrefixOperator::Plus(_) | UnaryPrefixOperator::Negation(_)) =>
        {
            let operand = offset_expression_id(prefix.operand, context, block_context, artifacts)?;
            Some(concat_word!(prefix.operator.as_bytes(), operand.as_bytes()))
        }
        Expression::UnaryPostfix(postfix) => {
            let operand = offset_expression_id(postfix.operand, context, block_context, artifacts)?;
            Some(concat_word!(operand.as_bytes(), postfix.operator.as_str().as_bytes()))
        }
        _ => get_block_expression_id(expression, context, block_context),
    }
}

/// Refer to an evaluated offset, never to the changed operand of a postfix index.
fn evaluated_offset_id<'arena, A: Arena>(
    expression: &Expression<'arena>,
    context: &Context<'_, 'arena, A>,
    block_context: &BlockContext<'_>,
    artifacts: &AnalysisArtifacts,
) -> Option<Word> {
    match unwrap_expression(expression) {
        Expression::ArrayAccess(access) => {
            let parent = evaluated_offset_id(access.array, context, block_context, artifacts)?;
            let index = get_literal_array_key(access.index, artifacts)
                .map(|key| key.id_word())
                .or_else(|| evaluated_offset_id(access.index, context, block_context, artifacts))?;
            Some(concat_word!(parent.as_bytes(), b"[", index.as_bytes(), b"]"))
        }
        Expression::UnaryPostfix(_) => None,
        _ => get_block_expression_id(expression, context, block_context),
    }
}

fn is_rooted_in_static_local(expr: &Expression<'_>, static_locals: &WordSet) -> bool {
    match unwrap_expression(expr) {
        Expression::Variable(Variable::Direct(var)) => static_locals.contains(&word(var.name)),
        Expression::ArrayAccess(access) => is_rooted_in_static_local(access.array, static_locals),
        _ => false,
    }
}
