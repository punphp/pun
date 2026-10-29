use mago_allocator::Arena;
use std::rc::Rc;

use mago_algebra::AssertionMap;

use mago_algebra::clause::Clause;
use mago_algebra::disjoin_clauses;
use mago_codex::assertion::Assertion;
use mago_codex::identifier::function_like::FunctionLikeIdentifier;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::comparator::ComparisonResult;
use mago_codex::ttype::comparator::union_comparator;
use mago_codex::ttype::get_literal_int;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::get_never;
use mago_codex::ttype::get_null;
use mago_codex::ttype::union::TUnion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::Access;
use mago_syntax::cst::ArrayElement;
use mago_syntax::cst::Assignment;
use mago_syntax::cst::AssignmentOperator;
use mago_syntax::cst::Binary;
use mago_syntax::cst::BinaryOperator;
use mago_syntax::cst::Call;
use mago_syntax::cst::ClassLikeMemberSelector;
use mago_syntax::cst::Construct;
use mago_syntax::cst::Expression;
use mago_syntax::cst::Literal;
use mago_syntax::cst::LocalIdentifier;
use mago_syntax::cst::StringPart;
use mago_syntax::cst::UnaryPrefix;
use mago_syntax::cst::UnaryPrefixOperator;
use mago_syntax::cst::Variable;
use mago_syntax::walker::MutWalker;
use mago_word::Word;
use mago_word::WordSet;
use mago_word::word;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::artifacts::CheckpointAction;
use crate::artifacts::ExpressionResult;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::context::block::ReferenceConstraintSource;
use crate::context::block::VariableState;
use crate::context::scope::var_has_root;
use crate::context::scope::var_references_dynamic;
use crate::error::AnalysisError;
use crate::expression::find_expression_logic_issues;
use crate::formula::get_formula;
use crate::utils::docblock::check_docblock_type_incompatibility;
use crate::utils::docblock::get_type_from_var_docblock;
use crate::utils::expression::array::get_array_target_type_given_index;
use crate::utils::expression::expression_has_logic;
use crate::utils::expression::get_block_expression_id;
use crate::utils::expression::get_non_nullsafe_expression_id;
use crate::utils::expression::get_nullsafe_base_expressions;
use crate::utils::expression::get_root_expression_id;
use crate::utils::expression::variable::get_variables_referenced_in_expression;
use crate::utils::misc::unwrap_expression;

pub(crate) mod array_assignment;
mod property_assignment;
mod static_property_assignment;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PropertyWriteKind {
    Direct,
    Coalesce,
    Mutation,
    ArrayAccessMutation,
}

impl<'ast, 'arena> Analyzable<'ast, 'arena> for Assignment<'arena> {
    fn analyze<'ctx, A>(
        &'ast self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError>
    where
        A: Arena,
    {
        analyze_assignment(
            context,
            block_context,
            artifacts,
            Some(self.span()),
            self.lhs,
            Some(&self.operator),
            Some(self.rhs),
            None,
        )
    }
}

pub fn analyze_assignment<'ctx, 'ast, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    assignment_span: Option<Span>,
    target_expression: &'ast Expression<'arena>,
    assignment_operator: Option<&AssignmentOperator>,
    source_expression: Option<&'ast Expression<'arena>>,
    source_type: Option<TUnion>,
) -> Result<(), AnalysisError>
where
    A: Arena,
{
    let read_target = assignment_operator.is_some_and(|operator| !matches!(operator, AssignmentOperator::Assign(_)));
    let operands = analyze_assignment_target(target_expression, context, block_context, artifacts, read_target)?;
    crate::observations::with_prepared_operands(context, &operands, |context| {
        analyze_prepared_assignment(
            context,
            block_context,
            artifacts,
            assignment_span,
            target_expression,
            assignment_operator,
            source_expression,
            source_type,
        )
    })?;
    if read_target {
        // PHPStan 2.3.1 AssignHandler::applyWrite updates the scope holder, not the prepared
        // operand results in storage. Keep the actual completed reads here too:
        // array_assignment uses these same artifact slots for its write chain.
        for (span, result) in operands {
            match result {
                Some(result) => artifacts.set_expression_result(&span, result),
                None => artifacts.remove_expression_type(&(span.start.offset, span.end.offset)),
            }
        }
    }
    Ok(())
}

fn analyze_prepared_assignment<'ctx, 'ast, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    assignment_span: Option<Span>,
    target_expression: &'ast Expression<'arena>,
    mut assignment_operator: Option<&AssignmentOperator>,
    source_expression: Option<&'ast Expression<'arena>>,
    source_type: Option<TUnion>,
) -> Result<(), AnalysisError>
where
    A: Arena,
{
    if let Some(AssignmentOperator::Assign(_)) = assignment_operator {
        assignment_operator = None;
    }

    let target_variable_id = get_block_expression_id(target_expression, context, block_context);
    let mut reads: Vec<_> = if assignment_operator.is_none()
        && matches!(target_expression, Expression::Variable(Variable::Direct(_)))
        && context.raw_callback_gather_depth == 0
        && let Some(source) = source_expression
    {
        get_variables_referenced_in_expression(source, false).into_iter().collect()
    } else {
        Vec::new()
    };
    // NodeFinder visits occurrences in source order, not VariableScanner's hash order.
    reads.sort_by_key(|(_, span)| span.start.offset);
    let before_assignment = if reads.is_empty() { None } else { block_context.assignment_condition_scope() };
    let mut existing_target_type = None;
    if let Some(target_variable_id) = &target_variable_id {
        block_context.conditionally_referenced_variable_ids.remove(target_variable_id);
        block_context.assigned_variable_ids.insert(*target_variable_id, target_expression.start_offset());
        block_context.possibly_assigned_variable_ids.insert(*target_variable_id);

        existing_target_type = block_context.locals.get(target_variable_id).cloned();
    }

    if let (Some(source_expression), Some(target_variable_id)) = (source_expression, &target_variable_id)
        && assignment_operator.is_none()
        && matches!(target_expression, Expression::Variable(_))
        && is_closure_expression(source_expression)
        && let Some(preliminary_type) = get_closure_expression_type(context.source_file, source_expression)
    {
        block_context.locals.insert(*target_variable_id, Rc::new(preliminary_type));
    }

    let mut assignment_operation_type = None;
    if let Some(source_expression) = source_expression {
        let was_inside_general_use = block_context.flags.inside_general_use();
        block_context.flags.set_inside_general_use(true);

        match assignment_operator {
            None => {
                source_expression.analyze(context, block_context, artifacts)?;
            }
            // this rewrites $a += 4 and $a ??= 4 to $a = $a + 4 and $a = $a ?? 4 respectively
            Some(assignment_operator) => {
                artifacts.begin_expression_type_checkpoint();
                block_context.flags.set_inside_assignment_operation(true);

                let binary_expression = Expression::Binary(Binary {
                    lhs: context.arena.alloc(target_expression.clone()),
                    operator: match assignment_operator {
                        AssignmentOperator::Addition(span) => BinaryOperator::Addition(*span),
                        AssignmentOperator::Subtraction(span) => BinaryOperator::Subtraction(*span),
                        AssignmentOperator::Multiplication(span) => BinaryOperator::Multiplication(*span),
                        AssignmentOperator::Division(span) => BinaryOperator::Division(*span),
                        AssignmentOperator::Modulo(span) => BinaryOperator::Modulo(*span),
                        AssignmentOperator::Exponentiation(span) => BinaryOperator::Exponentiation(*span),
                        AssignmentOperator::Concat(span) => BinaryOperator::StringConcat(*span),
                        AssignmentOperator::BitwiseAnd(span) => BinaryOperator::BitwiseAnd(*span),
                        AssignmentOperator::BitwiseOr(span) => BinaryOperator::BitwiseOr(*span),
                        AssignmentOperator::BitwiseXor(span) => BinaryOperator::BitwiseXor(*span),
                        AssignmentOperator::LeftShift(span) => BinaryOperator::LeftShift(*span),
                        AssignmentOperator::RightShift(span) => BinaryOperator::RightShift(*span),
                        AssignmentOperator::Coalesce(span) => BinaryOperator::NullCoalesce(*span),
                        #[allow(clippy::unreachable)]
                        AssignmentOperator::Assign(_) => unreachable!(),
                    },
                    rhs: context.arena.alloc(source_expression.clone()),
                });

                if let Err(error) = binary_expression.analyze(context, block_context, artifacts) {
                    artifacts.finish_expression_type_checkpoint(CheckpointAction::Keep);
                    return Err(error);
                }
                block_context.flags.set_inside_assignment_operation(false);
                assignment_operation_type = if let Some(assignment_span) = assignment_span {
                    artifacts.get_rc_expression_type(&assignment_span).cloned()
                } else {
                    None
                };

                artifacts.finish_expression_type_checkpoint(CheckpointAction::Merge);
            }
        }

        if expression_has_logic(source_expression) {
            find_expression_logic_issues(source_expression, context, block_context, artifacts);
        }

        block_context.flags.set_inside_general_use(was_inside_general_use);
    }

    let source_type: Rc<TUnion> = if let Some(source_type) = source_type {
        Rc::new(source_type)
    } else if let Some(assignment_operation_type) = assignment_operation_type {
        assignment_operation_type
    } else if let Some(source_expression) = source_expression {
        if let Some(source_type) = artifacts.get_rc_expression_type(&source_expression) {
            Rc::clone(source_type)
        } else {
            Rc::new(get_mixed())
        }
    } else {
        Rc::new(get_mixed())
    };

    if let (Some(target_variable_id), None) = (&target_variable_id, assignment_operator)
        && block_context.flags.inside_loop()
        && !block_context.flags.inside_assignment_operation()
        && let Some(Expression::Clone(clone_expression)) = source_expression
        && let Expression::Variable(Variable::Direct(cloned_var)) = clone_expression.object
        && cloned_var.name == target_variable_id.as_bytes()
        && let Some(assignment_span) = assignment_span
    {
        context.collector.report_with_code(
            IssueCode::CloneInsideLoop,
            Issue::warning(format!(
                "Cloning variable `{target_variable_id}` onto itself inside a loop might not have the intended effect."
            ))
            .with_annotation(
                Annotation::primary(assignment_span).with_message("Cloning onto self within loop")
            )
            .with_note(
                "This pattern overwrites the variable with a fresh clone on each loop iteration."
            )
            .with_note(
                "If the intent was to modify a copy of the variable defined *outside* the loop, the clone should happen *before* the loop starts."
            )
            .with_help(
                format!(
                    "Consider cloning `{target_variable_id}` before the loop if you need a copy, or revise the loop logic if cloning onto itself is not the desired behavior."
                )
            ),
        );
    }

    let pending_conditions = if let (Some(before), Some(target), Some(source)) =
        (before_assignment, target_variable_id, source_expression)
        && derived_assignment_rhs_is_pure(source, context, artifacts)
    {
        before.derive_assignment_conditions(target, reads.into_iter().map(|(name, _)| word(name)), &source_type, |scope| {
            scope.if_body_context = None;
            scope.finally_scope = None;
            let mut sandbox = artifacts.clone();
            sandbox.loop_scope = None;
            sandbox.case_scopes.clear();
            crate::plugin::registry::with_isolated_type_pricing(|| {
                let mut price = |context: &mut Context<'ctx, 'arena, A>| context.gather_callback(|context| {
                    source.analyze(context, scope, &mut sandbox)?;
                    Ok::<_, AnalysisError>(sandbox.get_expression_type(source).cloned().unwrap_or_else(get_mixed))
                });
                if context.observation_program.is_some() {
                    // Argument settling may update retained results. An observation-enabled
                    // walk therefore needs its own observation state, not just copied artifacts.
                    price(&mut context.fork_for_observation())
                } else {
                    price(context)
                }
            })
        })?
    } else {
        VariableState::default()
    };

    if let (Some(target_variable_id), Some(existing_target_type)) = (&target_variable_id, &existing_target_type) {
        block_context.remove_descendants(context, *target_variable_id, existing_target_type, Some(&source_type));
    } else {
        let root_var_id = get_root_expression_id(target_expression);

        if let Some(root_var_id) = root_var_id
            && let Some(existing_root_type) = block_context.locals.get(&root_var_id).cloned()
        {
            block_context.remove_variable_from_conflicting_clauses(context, root_var_id, Some(&existing_root_type));
        }
    }

    let property_write_kind = match assignment_operator {
        Some(AssignmentOperator::Coalesce(_)) => PropertyWriteKind::Coalesce,
        Some(_) => PropertyWriteKind::Mutation,
        None if source_expression.is_some_and(Expression::is_reference) => PropertyWriteKind::Mutation,
        None => PropertyWriteKind::Direct,
    };

    let successful = assign_to_expression(
        context,
        block_context,
        artifacts,
        target_expression,
        target_variable_id,
        source_expression,
        Rc::clone(&source_type),
        false,
        property_write_kind,
    )?;

    if !successful {
        if matches!(
            target_expression,
            Expression::Identifier(_) | Expression::ConstantAccess(_) | Expression::Access(Access::ClassConstant(_))
        ) {
            context.collector.report_with_code(
                IssueCode::AssignmentToConstant,
                Issue::error("Cannot assign to a constant.")
                    .with_annotation(
                        Annotation::primary(target_expression.span())
                            .with_message("Attempting assignment to constant here."),
                    )
                    .with_note("Constants cannot be reassigned after definition.")
                    .with_help("Assign the value to a variable instead, or remove the assignment."),
            );
        } else {
            context.collector.report_with_code(
                IssueCode::InvalidAssignment,
                Issue::error("Invalid target for assignment.")
                    .with_annotation(
                        Annotation::primary(target_expression.span())
                            .with_message("This expression cannot be assigned to."),
                    )
                    .with_note(
                        "Assignments require a valid variable, array element, or object property on the left-hand side."
                    )
                    .with_help(
                        "Ensure the left side of the assignment is a valid target (e.g., `$variable`, `$array[key]`, `$object->property`)."
                    ),
            );
        }
    }

    if successful
        && assignment_operator.is_none()
        && let (Some(target_variable_id), Some(source_expression)) = (target_variable_id, source_expression)
        && matches!(target_expression, Expression::Variable(_))
    {
        add_nullsafe_assignment_clauses(target_variable_id, source_expression, context, block_context);
        block_context.install_assignment_conditions(pending_conditions);
    }

    if let Some(assignment_span) = assignment_span {
        artifacts.set_rc_expression_type(&assignment_span, source_type);
    }

    Ok(())
}

/// The live analyzer has no general ImpurePoint result yet. Unknown calls/accessors
/// are not assumed pure, and implicit string calls require scalar inputs. Local
/// writes are not PHPStan impure points (unlike superglobals).
fn derived_assignment_rhs_is_pure<'arena, A: Arena>(
    source: &Expression<'arena>,
    context: &Context<'_, 'arena, A>,
    artifacts: &AnalysisArtifacts,
) -> bool {
    struct PureRhs {
        pure: bool,
    }
    impl<'ast, 'ctx, 'arena, A: Arena> MutWalker<'ast, 'arena, (&Context<'ctx, 'arena, A>, &AnalysisArtifacts)> for PureRhs {
        fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, state: &mut (&Context<'ctx, 'arena, A>, &AnalysisArtifacts)) {
            let (context, artifacts) = *state;
            // ImplicitToStringCallHelper may add an impure __toString invocation.
            // Missing or non-scalar types cannot establish its absence.
            let scalar = |expression: &Expression<'arena>| artifacts.get_expression_type(expression)
                .is_some_and(|ty| ty.types.iter().all(|ty| matches!(ty, TAtomic::Scalar(_) | TAtomic::Null)));
            self.pure &= match expression {
                Expression::Binary(binary) if matches!(binary.operator, BinaryOperator::StringConcat(_)) =>
                    scalar(binary.lhs) && scalar(binary.rhs),
                Expression::UnaryPrefix(prefix) if matches!(prefix.operator,
                    UnaryPrefixOperator::StringCast(_, _) | UnaryPrefixOperator::BinaryCast(_, _)) => scalar(prefix.operand),
                Expression::CompositeString(string) => string.parts().as_slice().iter().all(|part| match part {
                    StringPart::Literal(_) => true,
                    StringPart::Expression(expression) => scalar(expression),
                    StringPart::BracedExpression(part) => scalar(part.expression),
                }),
                Expression::Call(Call::Function(call)) => match call.function {
                    Expression::Identifier(name) => context.codebase
                        .get_function(context.resolve_function_name(name))
                        .is_some_and(|metadata| metadata.flags.is_pure() && !metadata.flags.suspends_fiber()),
                    _ => false,
                },
                Expression::Variable(Variable::Direct(variable)) =>
                    crate::common::global::get_global_variable_type(word(variable.name)).is_none(),
                Expression::Assignment(assignment) =>
                    matches!(assignment.lhs, Expression::Variable(Variable::Direct(_))),
                Expression::Construct(construct) => matches!(construct, Construct::Isset(_) | Construct::Empty(_)),
                Expression::Call(_) | Expression::Access(Access::Property(_) | Access::NullSafeProperty(_) | Access::StaticProperty(_))
                | Expression::Instantiation(_) | Expression::AnonymousClass(_) | Expression::Yield(_) | Expression::Pipe(_) => false,
                _ => true,
            };
        }

        // Creating a callable does not execute its body or any effects inside it.
        fn walk_closure(&mut self, _: &'ast mago_syntax::cst::Closure<'arena>, _: &mut (&Context<'ctx, 'arena, A>, &AnalysisArtifacts)) {}
        fn walk_arrow_function(&mut self, _: &'ast mago_syntax::cst::ArrowFunction<'arena>, _: &mut (&Context<'ctx, 'arena, A>, &AnalysisArtifacts)) {}
    }
    let mut purity = PureRhs { pure: true };
    purity.walk_expression(source, &mut (&*context, artifacts));
    purity.pure
}

fn add_nullsafe_assignment_clauses<'ctx, 'arena, A>(
    target_variable_id: Word,
    source_expression: &Expression<'arena>,
    context: &Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
) where
    A: Arena,
{
    for base in get_nullsafe_base_expressions(source_expression) {
        let Some(base_id) = get_block_expression_id(base, context, block_context) else {
            continue;
        };

        let base_id = get_non_nullsafe_expression_id(base_id).unwrap_or(base_id);
        if var_has_root(base_id, target_variable_id) {
            continue;
        }

        let target_null = Assertion::IsType(TAtomic::Null);
        let base_not_null = Assertion::IsNotType(TAtomic::Null);
        let possibilities = AssertionMap::from_iter([
            (target_variable_id, AssertionMap::from_iter([(target_null.to_hash(), target_null)])),
            (base_id, AssertionMap::from_iter([(base_not_null.to_hash(), base_not_null)])),
        ]);

        block_context.clauses.push(Rc::new(Clause::new(
            possibilities,
            source_expression.span(),
            source_expression.span(),
            Some(false),
            Some(true),
            Some(true),
        )));
    }
}

pub(crate) fn assign_to_expression<'ctx, 'ast, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    target_expression: &'ast Expression<'arena>,
    target_expression_id: Option<Word>,
    source_expression: Option<&'ast Expression<'arena>>,
    mut source_type: Rc<TUnion>,
    destructuring: bool,
    property_write_kind: PropertyWriteKind,
) -> Result<bool, AnalysisError>
where
    A: Arena,
{
    if let Some(target_expression_id) = target_expression_id {
        artifacts.record_loop_assignment_target(target_expression_id);
    }

    if let Some(source_expression) = source_expression {
        if source_expression.is_reference() != source_type.by_reference() {
            Rc::make_mut(&mut source_type).set_by_reference(source_expression.is_reference());
        }

        analyze_reference_assignment(context, block_context, target_expression, source_expression);
    }

    if let Some(root) = get_root_expression_id(target_expression) {
        let belongs_to_root =
            |key: &Word| key.as_bytes().strip_prefix(root.as_bytes()).is_some_and(|suffix| suffix.starts_with(b"->"));

        block_context.stable_method_call_assertions.retain(|key, _| !belongs_to_root(key));
        block_context.stable_method_calls.retain(|key| !belongs_to_root(key));
        block_context.clauses.retain(|clause| !clause.possibilities.keys().any(belongs_to_root));
        block_context.reconciled_expression_clauses.retain(|clause| !clause.possibilities.keys().any(belongs_to_root));
    } else if !matches!(target_expression, Expression::Variable(_)) {
        block_context.stable_method_call_assertions.clear();
        block_context.stable_method_calls.clear();
        block_context.clauses.retain(|clause| {
            !clause.possibilities.keys().any(|key| memchr::memmem::find(key.as_bytes(), b"()").is_some())
        });
        block_context.reconciled_expression_clauses.retain(|clause| {
            !clause.possibilities.keys().any(|key| memchr::memmem::find(key.as_bytes(), b"()").is_some())
        });
    }

    // Assign/AssignRef (including array-root writes) create PHPStan's marker;
    // compound and increment writes do not. This is separate from readonly's write kind.
    let initializes_marker = matches!(property_write_kind, PropertyWriteKind::Direct | PropertyWriteKind::Coalesce)
        || source_expression.is_some_and(Expression::is_reference);
    match target_expression {
        Expression::Variable(target_variable) if target_expression_id.is_some() => analyze_assignment_to_variable(
            context,
            block_context,
            artifacts,
            target_variable.span(),
            source_expression,
            source_type,
            // SAFETY: `target_expression_id` is guaranteed to be `Some` here.
            unsafe { *target_expression_id.as_ref().unwrap_unchecked() },
            destructuring,
        ),
        Expression::Access(Access::Property(property_access)) => property_assignment::analyze(
            context,
            block_context,
            artifacts,
            property_access,
            &source_type,
            source_expression.map(mago_span::HasSpan::span),
            property_write_kind,
            initializes_marker,
        )?,
        Expression::Access(Access::StaticProperty(property_access)) => static_property_assignment::analyze(
            context,
            block_context,
            artifacts,
            property_access,
            &source_type,
            target_expression_id,
        )?,
        Expression::ArrayAccess(array_access) => {
            array_assignment::analyze(context, block_context, artifacts, array_access.into(), &source_type, initializes_marker)?;
        }
        Expression::ArrayAppend(array_append) => {
            array_assignment::analyze(context, block_context, artifacts, array_append.into(), &source_type, initializes_marker)?;
        }
        Expression::Array(array) => {
            analyze_destructuring(
                context,
                block_context,
                artifacts,
                array.span(),
                source_expression,
                &source_type,
                array.elements.as_slice(),
                true,
            )?;
        }
        Expression::List(list) => {
            analyze_destructuring(
                context,
                block_context,
                artifacts,
                list.span(),
                source_expression,
                &source_type,
                list.elements.as_slice(),
                false,
            )?;
        }
        _ => {
            return Ok(false);
        }
    }

    Ok(true)
}

fn analyze_reference_assignment<'ctx, 'ast, 'arena, A>(
    context: &Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    target_expression: &'ast Expression<'arena>,
    source_expression: &'ast Expression<'arena>,
) where
    A: Arena,
{
    let Expression::UnaryPrefix(UnaryPrefix {
        operator: UnaryPrefixOperator::Reference(_),
        operand: referenced_expression,
    }) = source_expression
    else {
        return;
    };

    let target_variable_id = get_block_expression_id(target_expression, context, block_context);
    let referenced_variable_id = get_block_expression_id(referenced_expression, context, block_context);
    let (Some(target_variable_id), Some(referenced_variable_id)) = (target_variable_id, referenced_variable_id) else {
        return;
    };

    block_context.locals.entry(referenced_variable_id).or_insert_with(|| Rc::new(get_mixed()));

    if block_context.references_in_scope.contains_key(&target_variable_id) {
        block_context.decrement_reference_count(target_variable_id.as_bytes());
    }

    block_context.references_possibly_from_confusing_scope.remove(&target_variable_id);
    block_context.add_conditionally_referenced_variable(target_variable_id.as_bytes());
    block_context.references_in_scope.insert(target_variable_id, referenced_variable_id);
    block_context.referenced_counts.entry(referenced_variable_id).and_modify(|count| *count += 1).or_insert(1);

    let referenced_bytes = referenced_variable_id.as_bytes();
    if memchr::memchr(b'[', referenced_bytes).is_some() || memchr::memmem::find(referenced_bytes, b"->").is_some() {
        block_context.references_to_external_scope.insert(target_variable_id);
    }
}

pub fn analyze_assignment_to_variable<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    variable_span: Span,
    source_expression: Option<&Expression<'arena>>,
    mut assigned_type: Rc<TUnion>,
    variable_id: Word,
    destructuring: bool,
) where
    A: Arena,
{
    if let Some(constraint) = block_context.by_reference_constraints.get(&variable_id) {
        if let Some(constraint_type) = constraint.constraint_type.as_ref()
            && !union_comparator::is_contained_by(
                context.codebase,
                &assigned_type,
                constraint_type,
                assigned_type.ignore_nullable_issues(),
                assigned_type.ignore_falsable_issues(),
                false,
                &mut ComparisonResult::default(),
            )
        {
            let assigned_type_str = assigned_type.get_id();
            let constraint_type_str = constraint_type.get_id();
            let primary_error_span = source_expression.map_or(variable_span, mago_span::HasSpan::span);

            let issue = match constraint.source {
                ReferenceConstraintSource::Parameter => {
                    Issue::error(format!(
                        "Invalid assignment to by-reference parameter `{variable_id}`.",
                    ))
                    .with_annotation(Annotation::primary(primary_error_span).with_message(format!(
                        "This value has type `{assigned_type_str}`, but the parameter expects `{constraint_type_str}`.",
                    )))
                    .with_annotation(Annotation::secondary(constraint.constraint_span).with_message(
                        "Parameter is defined with a by-reference type constraint here.",
                    ))
                    .with_note(
                        "Assigning an incompatible type to a by-reference parameter can cause unexpected `TypeError`s in the calling scope.",
                    )
                    .with_help(
                        "If the parameter should have a different type on exit, declare it using a `@param-out` docblock tag.",
                    )
                },
                ReferenceConstraintSource::Argument => {
                    Issue::error(format!(
                        "Potentially invalid assignment to referenced variable `{variable_id}`.",
                    ))
                    .with_annotation(Annotation::primary(primary_error_span).with_message(format!(
                        "This assignment to type `{assigned_type_str}` may violate a reference constraint.",
                    )))
                    .with_annotation(Annotation::secondary(constraint.constraint_span).with_message(
                        format!("Variable was passed as a by-reference argument here, constraining it to type `{constraint_type_str}`."),
                    ))
                    .with_note(
                        "An object may still hold a reference to this variable. Changing its type can lead to a `TypeError` at runtime.",
                    )
                    .with_help(
                        "Ensure this variable's type remains compatible, or refactor to avoid holding onto the external reference.",
                    )
                },
                ReferenceConstraintSource::Static => {
                    Issue::error(format!(
                        "Invalid assignment to constrained static variable `{variable_id}`.",
                    ))
                    .with_annotation(Annotation::primary(primary_error_span).with_message(format!(
                        "This value of type `{assigned_type_str}` is not compatible with the expected type `{constraint_type_str}`.",
                    )))
                    .with_annotation(Annotation::secondary(constraint.constraint_span).with_message(
                        "Static variable's type is constrained here.",
                    ))
                    .with_note(
                        "Static variables maintain their state across function calls. Violating the type constraint can cause errors in subsequent calls.",
                    )
                    .with_help(format!(
                        "Ensure the assigned value is compatible with the `{constraint_type_str}` type.",
                    ))
                },
                ReferenceConstraintSource::Global => {
                    Issue::error(format!(
                        "Invalid assignment to constrained global variable `{variable_id}`.",
                    ))
                    .with_annotation(Annotation::primary(primary_error_span).with_message(format!(
                        "This value of type `{assigned_type_str}` is not compatible with the global's expected type `{constraint_type_str}`.",
                    )))
                    .with_annotation(Annotation::secondary(constraint.constraint_span).with_message(
                        "Global variable is imported with a type constraint here.",
                    ))
                    .with_note(
                        "Global variables are shared across the application. Changing a global to an incompatible type can lead to widespread errors.",
                    )
                    .with_help(format!(
                        "Ensure the assigned value is compatible with the `{constraint_type_str}` type.",
                    ))
                },
            };

            context.collector.report_with_code(IssueCode::ReferenceConstraintViolation, issue);
        }

        if !assigned_type.by_reference() {
            Rc::make_mut(&mut assigned_type).set_by_reference(true);
        }
    }

    if block_context.references_possibly_from_confusing_scope.contains(&variable_id) {
        context.collector.report_with_code(
            IssueCode::ReferenceReusedFromConfusingScope,
            Issue::warning("Potential unintended modification: This variable may still hold a reference to another variable from a preceding scope.")
                .with_annotation(
                    Annotation::primary(variable_span)
                        .with_message("Assigning a new value here may unintentionally modify the variable this reference points to.")
                )
                .with_note("In PHP, a reference assigned within a block (e.g., an `if` statement or a `foreach` loop) remains active after the block has finished executing.")
                .with_note("Reusing the variable without `unset()` can lead to unexpected side effects.")
                .with_help("To safely reuse this variable, first break the reference by calling `unset()`. For example: `unset($value);`"),
        );
    }

    if variable_id.as_bytes() == b"$this" {
        context.collector.report_with_code(
            IssueCode::AssignmentToThis,
            Issue::error("Cannot assign to `$this`.")
                .with_annotation(
                    Annotation::primary(variable_span).with_message("`$this` cannot be used as an assignment target."),
                )
                .with_note("The `$this` variable is read-only and refers to the current object instance.")
                .with_help("Use a different variable name for the assignment."),
        );
    }

    let mut from_docblock = false;
    if let Some((variable_type, variable_type_span)) =
        get_type_from_var_docblock(context, block_context, artifacts, Some(variable_id.as_bytes()), !destructuring)
    {
        check_docblock_type_incompatibility(
            context,
            Some(variable_id.as_bytes()),
            variable_span,
            &assigned_type,
            &variable_type,
            variable_type_span,
            source_expression,
        );

        assigned_type = Rc::new(variable_type);
        from_docblock = true;
    }

    if !from_docblock
        && assigned_type.is_mixed()
        && !assigned_type.is_error()
        && !variable_id.as_bytes().starts_with(b"$_")
    {
        let assigned_type_str = assigned_type.get_id();

        let mut issue = Issue::warning(format!(
            "Assigning `{assigned_type_str}` type to a variable may lead to unexpected behavior."
        ));

        if let Some(source_expression) = source_expression
            && let Expression::Binary(_) = source_expression
        {
            issue = issue.with_annotation(
                Annotation::secondary(source_expression.span())
                    .with_message(format!("This expression has type `{assigned_type_str}`.")),
            );
        }

        context.collector.report_with_code(
            IssueCode::MixedAssignment,
            issue.with_annotation(Annotation::primary(variable_span).with_message(format!("Assigning `{assigned_type_str}` type here.")))
                .with_note(format!("Using `{assigned_type_str}` can lead to runtime errors if the variable is used in a way that assumes a specific type."))
                .with_help("Consider using a more specific type to avoid potential issues."),
        );
    }

    if !from_docblock
        && assigned_type.is_bool()
        && let Some(source_expression) = source_expression
        && match unwrap_expression(source_expression) {
            Expression::Binary(_) => true,
            Expression::Call(call) => {
                let span = call.span();
                let range = (span.start.offset, span.end.offset);

                artifacts.if_true_assertions.contains_key(&range) || artifacts.if_false_assertions.contains_key(&range)
            }
            _ => false,
        }
    {
        handle_assignment_with_boolean_logic(
            context,
            block_context,
            artifacts,
            variable_span,
            source_expression,
            variable_id,
        );
    }

    block_context.locals.retain(|var_id, _| !var_references_dynamic(*var_id, variable_id));
    block_context.derived_local_sources.remove(&variable_id);
    block_context.possibly_undefined_variable_ids.remove(&variable_id);

    // The source may be an optional offset or a conditional local, but this write defines its target.
    Rc::make_mut(&mut assigned_type).set_possibly_undefined(false, None);
    Rc::make_mut(&mut assigned_type).set_possibly_undefined_from_try(false);

    if artifacts.editor.is_some() {
        artifacts.set_rc_expression_type(&variable_span, Rc::clone(&assigned_type));
    }

    if block_context.native_locals.is_some() {
        let native = if from_docblock {
            // VarAnnotationProcessor::processVarAnnotation() intentionally forgets the RHS's native type.
            Some(Rc::new(get_mixed()))
        } else {
            source_expression.and_then(|source| artifacts.get_rc_native_expression_type(source).cloned())
        };
        if !from_docblock && source_expression.is_some_and(|source|
            matches!(source, Expression::Closure(_) | Expression::ArrowFunction(_)))
        {
            // Creating a closure does not execute its body. Its native callable value remains
            // unavailable, while unaffected outer holders survive the separate capture transfer.
            block_context.native_locals.as_mut().unwrap().remove(&variable_id);
        } else {
            block_context.native_locals = block_context.native_locals.take().zip(native).map(|(mut locals, native)| {
                locals.insert(variable_id, native);
                locals
            });
        }
    }

    block_context.invalidate_variable_certainty(variable_id);
    block_context.locals.insert(variable_id, assigned_type);
    block_context.variables_possibly_in_scope.insert(variable_id);
}

fn analyze_destructuring<'ctx, 'ast, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    target_span: Span, // the span of the destructuring target ( list or array )
    source_expression: Option<&'ast Expression<'arena>>, // the expression being destructured
    array_type: &TUnion, // the type of the array being destructured
    target_elements: &'ast [ArrayElement<'arena>], // the elements being destructured
    convert_arrays: bool, // PHPParser::fixupArrayDestructuring stops at an explicit List_.
) -> Result<(), AnalysisError>
where
    A: Arena,
{
    let mut non_array = false;
    let mut add_null = false;
    let mut null_source = false;

    let source_has_null = array_type.has_null();
    let non_nullable_type = if source_has_null { Some(array_type.to_non_nullable()) } else { None };
    let destructure_type: &TUnion = non_nullable_type.as_ref().unwrap_or(array_type);

    let can_be_destructured = destructure_type
        .types
        .iter()
        .all(|atomic| atomic.is_array() || atomic.extends_or_implements(context.codebase, b"ArrayAccess"));

    if array_type.is_null() {
        let mut issue = Issue::error("Destructuring a `null` value.".to_string());

        if let Some(source_expression) = source_expression {
            issue = issue.with_annotation(
                Annotation::primary(source_expression.span()).with_message("This expression is always `null`"),
            );
        } else {
            issue = issue.with_annotation(
                Annotation::primary(target_span).with_message("The value being destructured is always `null`"),
            );
        }

        issue = issue
            .with_note("PHP assigns `null` to every target and emits a warning when destructuring `null`.")
            .with_help("Ensure the value is an array before destructuring it.");

        context.collector.report_with_code(IssueCode::NullDestructuringSource, issue);

        non_array = true;
        null_source = true;
    } else if !can_be_destructured {
        let assigned_type_str = array_type.get_id();

        let mut issue = Issue::error(format!(
            "Invalid destructuring assignment: Cannot unpack type `{assigned_type_str}` into variables.",
        ));

        if let Some(source_expression) = source_expression {
            issue = issue
                .with_annotation(
                    Annotation::primary(source_expression.span())
                        .with_message(format!("This expression has type `{assigned_type_str}`...")),
                )
                .with_annotation(
                    Annotation::secondary(target_span)
                        .with_message("...but this destructuring pattern requires an array."),
                );
        } else {
            issue = issue.with_annotation(Annotation::primary(target_span).with_message(format!(
                "Attempting to destructure a value of type `{assigned_type_str}`, which is not an array."
            )));
        }

        issue = issue
            .with_note("Array destructuring (`[...] = $value;`) requires `$value` to be an array or an object that implements `ArrayAccess`.")
            .with_note(format!(
                "Attempting to destructure a non-array type like `{assigned_type_str}` is an undefined behavior in PHP.",
            ))
            .with_help(
                "Ensure the value on the right-hand side is an array before attempting to destructure it.",
            );

        context.collector.report_with_code(IssueCode::InvalidDestructuringSource, issue);

        non_array = true;
    } else if source_has_null {
        let assigned_type_str = array_type.get_id();

        let mut issue = Issue::warning(format!("Destructuring a possibly null value of type `{assigned_type_str}`."));

        if let Some(source_expression) = source_expression {
            issue = issue.with_annotation(
                Annotation::primary(source_expression.span())
                    .with_message(format!("This expression has type `{assigned_type_str}` and may be null here")),
            );
        } else {
            issue = issue.with_annotation(Annotation::primary(target_span).with_message(format!(
                "The value being destructured has type `{assigned_type_str}` and may be null here"
            )));
        }

        issue = issue
            .with_note("If the value is null at runtime, every target receives `null` and PHP emits a warning.")
            .with_help("Ensure the value is not null before destructuring, for example with a prior null check.");

        context.collector.report_with_code(IssueCode::PossiblyNullDestructuringSource, issue);

        add_null = true;
    }

    let mut last_index: usize = 0;
    let mut impossible = non_array;

    let has_keyed_elements = target_elements.iter().any(|e| matches!(e, ArrayElement::KeyValue(_)));
    let has_non_keyed_elements = target_elements.iter().any(|e| matches!(e, ArrayElement::Value(_)));
    let has_skipped_elements = target_elements.iter().any(|e| matches!(e, ArrayElement::Missing(_)));

    if has_keyed_elements {
        if has_non_keyed_elements {
            let first_keyed_span =
                // SAFETY: we know that there is at least one keyed element, so this is safe.
                unsafe { target_elements.iter().find(|e| matches!(e, ArrayElement::KeyValue(_))).unwrap_unchecked().span() };

            let first_non_keyed_span =
                // SAFETY: we know that there is at least one non-keyed element, so this is safe.
                unsafe { target_elements.iter().find(|e| matches!(e, ArrayElement::Value(_))).unwrap_unchecked().span() };

            let mut issue = Issue::error("Cannot mix keyed and non-keyed elements in array destructuring.")
                .with_annotation(Annotation::primary(target_span).with_message("This destructuring mixes both styles"))
                .with_note("PHP requires destructuring assignments to use either all list-style elements or all keyed elements, but not both.")
                .with_help("Separate the destructuring into two operations or choose one style.")
            ;

            if first_keyed_span.start.offset < first_non_keyed_span.start.offset {
                issue = issue
                    .with_annotation(Annotation::secondary(first_keyed_span).with_message("This is a keyed element..."))
                    .with_annotation(
                        Annotation::secondary(first_non_keyed_span)
                            .with_message("...and this is a non-keyed (list-style) element"),
                    );
            } else {
                issue = issue
                    .with_annotation(
                        Annotation::secondary(first_non_keyed_span)
                            .with_message("This is a non-keyed (list-style) element..."),
                    )
                    .with_annotation(
                        Annotation::secondary(first_keyed_span).with_message("...and this is a keyed element"),
                    );
            }

            context.collector.report_with_code(IssueCode::MixedDestructuringShape, issue);

            impossible = true;
        }

        if has_skipped_elements {
            let first_skipped_span =
                // SAFETY: we know that there is at least one skipped element, so this is safe.
                unsafe { target_elements.iter().find(|e| matches!(e, ArrayElement::Missing(_))).unwrap_unchecked().span() };
            let first_keyed_span =
                // SAFETY: we know that there is at least one keyed element, so this is safe.
                unsafe { target_elements.iter().find(|e| matches!(e, ArrayElement::KeyValue(_))).unwrap_unchecked().span() };

            let mut issue = Issue::error("Cannot use skipped elements (`,,`) in a keyed array destructuring.")
                .with_annotation(Annotation::primary(target_span).with_message("This destructuring is invalid"))
                .with_help(
                    "To get specific keys, access them directly. Do not mix keyed access with list-style skipping.",
                );

            if first_keyed_span.start.offset < first_skipped_span.start.offset {
                issue = issue
                    .with_annotation(Annotation::secondary(first_keyed_span).with_message("This is a keyed element..."))
                    .with_annotation(
                        Annotation::secondary(first_skipped_span)
                            .with_message("...but skipping elements is only allowed in list-style destructuring"),
                    );
            } else {
                issue = issue
                    .with_annotation(Annotation::primary(target_span).with_message("This destructuring is invalid"))
                    .with_annotation(
                        Annotation::secondary(first_skipped_span).with_message("This is a skipped element..."),
                    );
            }

            context.collector.report_with_code(IssueCode::SkipInKeyedDestructuring, issue);

            impossible = true;
        }
    }

    if !has_keyed_elements && has_non_keyed_elements && !impossible {
        check_list_destructure_keys(context, target_span, source_expression, destructure_type);
    }

    for target_element in target_elements {
        match target_element {
            ArrayElement::KeyValue(key_value_element) => {
                key_value_element.key.analyze(context, block_context, artifacts)?;

                let index_type =
                    artifacts.get_expression_type(key_value_element.key).cloned().unwrap_or_else(get_mixed);

                let access_type = if null_source {
                    get_null()
                } else if impossible {
                    get_mixed()
                } else {
                    let access_type = get_array_target_type_given_index(
                        context,
                        block_context,
                        key_value_element.key.span(),
                        if let Some(source_expression) = source_expression {
                            source_expression.span()
                        } else {
                            target_span
                        },
                        None,
                        destructure_type,
                        &index_type,
                        false,
                        None,
                        None,
                        false,
                    );

                    if add_null { access_type.as_nullable() } else { access_type }
                };

                analyze_destructuring_value(
                    context,
                    block_context,
                    artifacts,
                    key_value_element.value,
                    access_type,
                    convert_arrays,
                )?;
            }
            ArrayElement::Value(value_element) => {
                let index_type = get_literal_int(last_index as i64);

                let access_type = if null_source {
                    get_null()
                } else if impossible {
                    get_mixed()
                } else {
                    let access_type = get_array_target_type_given_index(
                        context,
                        block_context,
                        target_span,
                        if let Some(source_expression) = source_expression {
                            source_expression.span()
                        } else {
                            target_span
                        },
                        Some(value_element.value.span()),
                        destructure_type,
                        &index_type,
                        false,
                        None,
                        None,
                        false,
                    );

                    if add_null { access_type.as_nullable() } else { access_type }
                };

                analyze_destructuring_value(
                    context,
                    block_context,
                    artifacts,
                    value_element.value,
                    access_type,
                    convert_arrays,
                )?;
            }
            ArrayElement::Variadic(variadic_element) => {
                context.collector.report_with_code(
                    IssueCode::SpreadInDestructuring,
                    Issue::error("Variadic unpacking (`...`) is not permitted in a destructuring assignment.")
                        .with_annotation(Annotation::primary(variadic_element.span()).with_message("This syntax is not allowed here"))
                        .with_note("The `...` operator can be used for argument unpacking in function calls or for spreading elements into a new array on the right-hand side of an expression, but not on the left-hand side of an assignment.")
                        .with_help("Remove the `...` operator. If you intend to capture remaining array elements, this must be done in a separate step."),
                );

                analyze_assignment(
                    context,
                    block_context,
                    artifacts,
                    None,
                    variadic_element.value,
                    None,
                    None,
                    Some(get_never()),
                )?;

                continue;
            }
            ArrayElement::Missing(_) => {}
        }

        last_index += 1;
    }

    Ok(())
}

fn check_list_destructure_keys<'arena, A>(
    context: &mut Context<'_, 'arena, A>,
    target_span: Span,
    source_expression: Option<&Expression<'arena>>,
    array_type: &TUnion,
) where
    A: Arena,
{
    let mut has_string_signal = false;
    let mut has_negative_signal = false;

    for atomic in array_type.types.iter() {
        let TAtomic::Array(array) = atomic else {
            continue;
        };

        let TArray::Keyed(keyed) = array else {
            continue;
        };

        let mut local_string_signal = false;
        let mut local_negative_signal = false;
        let mut has_non_negative_int_known_item = false;

        if let Some(known_items) = &keyed.known_items {
            for key in known_items.keys() {
                match key {
                    ArrayKey::String(_) => local_string_signal = true,
                    ArrayKey::Integer(i) if *i < 0 => local_negative_signal = true,
                    ArrayKey::Integer(_) => has_non_negative_int_known_item = true,
                    ArrayKey::ClassLikeConstant { .. } => {}
                }
            }
        }

        if let Some((key_type, _)) = &keyed.parameters {
            if key_type.has_string() {
                local_string_signal = true;
            }

            for key_atomic in key_type.types.iter() {
                if let TAtomic::Scalar(TScalar::Integer(int)) = key_atomic
                    && int.is_negative()
                {
                    local_negative_signal = true;
                }
            }
        }

        if !has_non_negative_int_known_item {
            has_string_signal |= local_string_signal;
            has_negative_signal |= local_negative_signal;
        }
    }

    if has_string_signal {
        let assigned_type_str = array_type.get_id();
        let mut issue = Issue::warning(format!(
            "List-style destructuring of a value with non-integer keys (`{assigned_type_str}`).",
        ))
        .with_annotation(
            Annotation::primary(target_span)
                .with_message("This list-style destructuring only reads keys `0`, `1`, `2`, ..."),
        );

        if let Some(source_expression) = source_expression {
            issue = issue.with_annotation(
                Annotation::secondary(source_expression.span())
                    .with_message(format!("...but this expression has type `{assigned_type_str}`")),
            );
        }

        issue = issue
            .with_note(
                "PHP list destructuring (`[$a, $b]`) reads only sequential integer keys; string keys are silently ignored and trigger \"undefined array key\" warnings at runtime.",
            )
            .with_help(
                "Use a keyed pattern like `['name' => $a, 'age' => $b]`, or convert the source to a list with `array_values(...)`.",
            );

        context.collector.report_with_code(IssueCode::ListDestructureStringKey, issue);
    }

    if has_negative_signal {
        let assigned_type_str = array_type.get_id();
        let mut issue = Issue::warning(format!(
            "List-style destructuring of a value with negative integer keys (`{assigned_type_str}`).",
        ))
        .with_annotation(
            Annotation::primary(target_span)
                .with_message("This list-style destructuring only reads keys `0`, `1`, `2`, ..."),
        );

        if let Some(source_expression) = source_expression {
            issue = issue.with_annotation(
                Annotation::secondary(source_expression.span())
                    .with_message(format!("...but this expression has type `{assigned_type_str}`")),
            );
        }

        issue = issue
            .with_note(
                "PHP list destructuring starts at key `0`; negative keys present in the source are silently ignored.",
            )
            .with_help(
                "Use a keyed pattern like `[-1 => $a, 0 => $b]` to capture the negative entry, or convert the source to a list with `array_values(...)`.",
            );

        context.collector.report_with_code(IssueCode::ListDestructureNegativeKey, issue);
    }
}

/// AssignHandler::applyWrite() prepares each List_ value after the RHS and preceding item writes.
/// A literal Array_ uses prepareTarget()'s ordinary expression-read fallback, never a recursive write.
fn analyze_destructuring_value<'ctx, 'ast, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    value: &'ast Expression<'arena>,
    source_type: TUnion,
    convert_arrays: bool,
) -> Result<(), AnalysisError>
where
    A: Arena,
{
    let target = unwrap_expression(value);
    if !convert_arrays && matches!(target, Expression::Array(_) | Expression::LegacyArray(_)) {
        return value.analyze(context, block_context, artifacts);
    }
    if convert_arrays && let Expression::LegacyArray(array) = target {
        return analyze_destructuring(
            context,
            block_context,
            artifacts,
            array.span(),
            None,
            &source_type,
            array.elements.as_slice(),
            true,
        );
    }
    // PHPParser stores a reference on ArrayItem; its value is the bare variable expression.
    let target = match target {
        Expression::UnaryPrefix(reference) if reference.operator.is_reference() => reference.operand,
        target => target,
    };
    analyze_assignment(context, block_context, artifacts, None, target, None, None, Some(source_type))
}

fn analyze_assignment_target<'ctx, 'arena, A>(
    expression: &Expression<'arena>,
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    read_target: bool,
) -> Result<Vec<(Span, Option<ExpressionResult>)>, AnalysisError>
where
    A: Arena,
{
    // prepareTarget's raw callback belongs to entry, not the RHS or the write.
    // A direct write-only variable deliberately has no completed read result.
    crate::observations::before(expression, context, block_context);
    let mut operands = Vec::new();
    match expression {
        Expression::Variable(Variable::Nested(nested)) => {
            let name = context.arena.alloc(Expression::Variable(nested.variable.clone()));
            prepare_assignment_operand(name, context, block_context, artifacts, &mut operands)?;
        }
        Expression::Variable(Variable::Indirect(indirect)) => {
            prepare_assignment_operand(indirect.expression, context, block_context, artifacts, &mut operands)?;
        }
        // List_ children are prepared during applyWrite(), not before the RHS.
        Expression::List(_) | Expression::Array(_) => {}
        Expression::ArrayAccess(_) | Expression::ArrayAppend(_) => {
            let mut links = Vec::new();
            let mut root = expression;
            loop {
                match root {
                    Expression::ArrayAccess(access) => {
                        links.push((root, access.array, Some(access.index)));
                        root = access.array;
                    }
                    Expression::ArrayAppend(append) => {
                        links.push((root, append.array, None));
                        root = append.array;
                    }
                    _ => break,
                }
            }
            prepare_assignment_operand(root, context, block_context, artifacts, &mut operands)?;
            // PHPStan walks the root and dimensions inside-out, composing each
            // link from those results. Never re-run an index to price the target.
            for (link, array, index) in links.into_iter().rev() {
                if link.span() != expression.span() {
                    crate::observations::before(link, context, block_context);
                }
                if let Some(index) = index {
                    prepare_assignment_operand(index, context, block_context, artifacts, &mut operands)?;
                }
                artifacts.remove_expression_type(&(link.span().start.offset, link.span().end.offset));
                let value = if let Some(index) = index {
                    let array_type = artifacts.get_rc_expression_type(array).cloned();
                    let index_type = artifacts.get_rc_expression_type(index).cloned();
                    array_type.zip(index_type).map(|(array_type, index_type)| {
                        // prepareTarget composes getOffsetValueType, not the
                        // writer's promotion of missing offsets into containers.
                        // array_assignment still owns validation of the write.
                        // ConstantArrayType projects a slot's value independently
                        // of optionality. Do not let ordinary-read diagnostics add
                        // null for absence; genuine nullable values stay unchanged.
                        // This projection is private, never the written receiver.
                        let mut value_projection = array_type.as_ref().clone();
                        let mut possibly_undefined = false;
                        for atomic in value_projection.types.to_mut() {
                            match atomic {
                                TAtomic::Array(TArray::Keyed(array)) => {
                                    if let Some(items) = &mut array.known_items {
                                        possibly_undefined |= index_type
                                            .get_single_array_key()
                                            .and_then(|key| items.get(&key))
                                            .is_some_and(|(optional, _)| *optional);
                                        for (optional, _) in std::sync::Arc::make_mut(items).values_mut() {
                                            *optional = false;
                                        }
                                    }
                                }
                                TAtomic::Array(TArray::List(array)) => {
                                    if let Some(items) = &mut array.known_elements {
                                        possibly_undefined |= index_type
                                            .get_single_literal_int_value()
                                            .and_then(|key| usize::try_from(key).ok())
                                            .and_then(|key| items.get(&key))
                                            .is_some_and(|(optional, _)| *optional);
                                        for (optional, _) in std::sync::Arc::make_mut(items).values_mut() {
                                            *optional = false;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        let mut value = context
                            .record(|context| {
                                get_array_target_type_given_index(
                                    context,
                                    block_context,
                                    link.span(),
                                    array.span(),
                                    Some(index.span()),
                                    &value_projection,
                                    &index_type,
                                    false,
                                    get_block_expression_id(array, context, block_context),
                                    None,
                                    false,
                                )
                            })
                            .0;
                        // Coalesce still needs absence independently of the value.
                        if possibly_undefined {
                            value.set_possibly_undefined(true, None);
                        }
                        value
                    })
                } else {
                    Some(get_never())
                };
                if let Some(value) = value {
                    artifacts.set_expression_type(link, value);
                    crate::native::after_expression(link, context, block_context, artifacts);
                }
                crate::observations::after(link, context, block_context, artifacts);
                operands.push((link.span(), artifacts.get_expression_result(link)));
            }
        }
        Expression::Access(Access::Property(access)) => {
            prepare_assignment_property(
                expression,
                access.object,
                &access.property,
                access.arrow.span(),
                false,
                context,
                block_context,
                artifacts,
                &mut operands,
                read_target,
            )?;
        }
        Expression::Access(Access::NullSafeProperty(access)) => {
            prepare_assignment_property(
                expression,
                access.object,
                &access.property,
                access.question_mark_arrow.span(),
                true,
                context,
                block_context,
                artifacts,
                &mut operands,
                read_target,
            )?;
        }
        Expression::Access(Access::StaticProperty(static_property_access)) => {
            operands.extend(analyze_assignment_target(
                static_property_access.class,
                context,
                block_context,
                artifacts,
                true,
            )?);
        }
        _ => {}
    }

    Ok(operands)
}

fn prepare_assignment_property<'ctx, 'arena, A: Arena>(
    expression: &Expression<'arena>,
    object: &Expression<'arena>,
    property: &ClassLikeMemberSelector<'arena>,
    arrow: Span,
    nullsafe: bool,
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    operands: &mut Vec<(Span, Option<ExpressionResult>)>,
    read_target: bool,
) -> Result<(), AnalysisError> {
    prepare_assignment_operand(object, context, block_context, artifacts, operands)?;
    let mut resolved_property = None;
    let variable = match property {
        ClassLikeMemberSelector::Expression(selector) => {
            prepare_assignment_operand(selector.expression, context, block_context, artifacts, operands)?;
            // PHPStan normalizes a literal-string name to the named fetch's holder.
            // A dynamic expression with a constant-string type is not that identity.
            if matches!(unwrap_expression(selector.expression), Expression::Literal(Literal::String(_)))
                && let Some(name) = artifacts.get_expression_type(selector.expression)
                    .and_then(|ty| ty.get_single_literal_string_value())
            {
                resolved_property = Some(ClassLikeMemberSelector::Identifier(LocalIdentifier {
                    span: property.span(),
                    value: context.arena.alloc_slice_copy(name),
                }));
            }
            None
        }
        ClassLikeMemberSelector::Variable(variable) => {
            let name = context.arena.alloc(Expression::Variable(variable.clone()));
            crate::observations::before(name, context, block_context);
            Some(name)
        }
        _ => None,
    };
    artifacts.remove_expression_type(&(expression.span().start.offset, expression.span().end.offset));
    // AssignHandler parks a composed pre-write value without counting a plain
    // assignment target as a property read. Receiver and selector reads above survive.
    let references = (!read_target).then(|| std::mem::take(&mut artifacts.symbol_references));
    let (result, completed) = crate::observations::collect_prepared_operands(context, |context| {
        crate::observations::with_prepared_operands(context, operands, |context| {
            context
                .record(|context| {
                    crate::expression::access::property_access::analyze_property_access(
                        context,
                        block_context,
                        artifacts,
                        expression.span(),
                        object,
                        arrow,
                        resolved_property.as_ref().unwrap_or(property),
                        nullsafe,
                        false,
                    )
                })
                .0
        })
    });
    if let Some(references) = references {
        artifacts.symbol_references = references;
    }
    operands.extend(completed);
    result?;
    if let Some(name) = variable {
        crate::observations::after(name, context, block_context, artifacts);
        operands.push((name.span(), artifacts.get_expression_result(name)));
    }
    crate::native::after_expression(expression, context, block_context, artifacts);
    crate::observations::after(expression, context, block_context, artifacts);
    operands.push((expression.span(), artifacts.get_expression_result(expression)));
    Ok(())
}

fn prepare_assignment_operand<'ctx, 'arena, A: Arena>(
    expression: &Expression<'arena>,
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    operands: &mut Vec<(Span, Option<ExpressionResult>)>,
) -> Result<(), AnalysisError> {
    let (result, completed) = crate::observations::collect_prepared_operands(context, |context| {
        expression.analyze(context, block_context, artifacts)
    });
    operands.extend(completed);
    result
}

fn handle_assignment_with_boolean_logic<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
    variable_expression_id: Span,
    source_expression: &Expression<'arena>,
    variable_id: Word,
) where
    A: Arena,
{
    let Some(right_clauses) = get_formula(
        source_expression.span(),
        source_expression.span(),
        source_expression,
        context.get_assertion_context_from_block(block_context),
        artifacts,
        &context.settings.algebra_thresholds(),
        context.settings.formula_size_threshold,
    ) else {
        // Complex clauses
        return;
    };

    let right_clauses =
        BlockContext::filter_clauses(context, variable_id, right_clauses.into_iter().map(Rc::new).collect(), None);

    if right_clauses.is_empty() {
        return;
    }

    let mut covered_variable_ids: WordSet = WordSet::default();
    covered_variable_ids.insert(variable_id);
    for clause in &right_clauses {
        for var in clause.possibilities.keys() {
            covered_variable_ids.insert(*var);
        }
    }

    block_context.parent_conflicting_clause_variables.retain(|var| !covered_variable_ids.contains(var));

    let mut possibilities = AssertionMap::default();
    possibilities.insert(variable_id, AssertionMap::from_iter([(Assertion::Falsy.to_hash(), Assertion::Falsy)]));

    block_context.clauses.extend(
        disjoin_clauses(
            vec![Clause::new(possibilities, variable_expression_id, variable_expression_id, None, None, None)],
            right_clauses.into_iter().map(|v| (*v).clone()).collect(),
            source_expression.span(),
            &context.settings.algebra_thresholds(),
        )
        .into_iter()
        .map(Rc::new),
    );
}

fn is_closure_expression<'arena>(expression: &'arena Expression<'arena>) -> bool {
    get_closure_expression_span(expression).is_some()
}

fn get_closure_expression_span<'arena>(expression: &'arena Expression<'arena>) -> Option<Span> {
    if let Expression::Parenthesized(parenthesized) = expression {
        return get_closure_expression_span(parenthesized.expression);
    }

    if matches!(expression, Expression::Closure(_)) { Some(expression.span()) } else { None }
}

fn get_closure_expression_type<'arena>(
    file: &mago_database::file::File,
    expression: &'arena Expression<'arena>,
) -> Option<TUnion> {
    let span = get_closure_expression_span(expression)?;

    Some(TUnion::from_atomic(TAtomic::Callable(TCallable::Alias(FunctionLikeIdentifier::for_closure(file, span)))))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::code::IssueCode;
    use crate::settings::Settings;
    use crate::test_analysis;

    test_analysis! {
        name = test_var_docblock,
        code = indoc! {"
            <?php

            namespace Example;

            /**
             * @template T
             */
            class Suspension
            {
                /**
                 * @template S
                 *
                 * @return Suspension<S>
                 */
                public static function create(): Suspension
                {
                    return new self();
                }

                /**
                 * @param T $_value
                 */
                public function resume(mixed $_value): void
                {
                    exit(0);
                }

                /**
                 * @return T
                 *
                 * @psalm-suppress InvalidReturnType
                 */
                public function suspend(): mixed
                {
                    exit(0);
                }
            }

            /** @var Suspension<string> */
            $suspension = Suspension::create();
            $suspension->resume('Hello, World!');
            $value = $suspension->suspend();

            echo $value;
        "}
    }

    test_analysis! {
        name = test_var_docblock_override_narrow,
        code = indoc! {"
            <?php

            namespace Example;

            /**
             * @return scalar
             */
            function get_scalar() {
                return 'Hello, World!';
            }

            /** @var string */
            $scalar = get_scalar();
        "},
    }

    test_analysis! {
        name = test_var_docblock_override_widen,
        code = indoc! {"
            <?php

            /**
             * @return list<int>
             */
            function get_list(): array {
                return [1, 2, 3];
            }

            /** @var list<int|string> */
            $scalar = get_list();
        "},
    }

    test_analysis! {
        name = test_var_docblock_overridei,
        code = indoc! {"
            <?php

            /**
             * @return list<int>
             */
            function get_list(): array {
                return [1, 2, 3];
            }

            /** @var bool */
            $scalar = get_list();
        "},
        issues = [
            IssueCode::DocblockTypeMismatch,
        ]
    }

    test_analysis! {
        name = list_assignment,
        code = indoc! {"
            <?php

            /**
             * @return array{a: int, b: int}
             */
            function get_a_and_b(): array {
                return ['a' => 1, 'b' => 2];
            }

            /**
             * @return array{1, 2}
             */
            function get_tuple(): array {
                return [1, 2];
            }

            function list_assignment(): void {
                list($_a, $_b) = get_tuple();
                list('a' => $_a, 'b' => $_b) = get_a_and_b();
            }

            function array_assignment(): void {
                [$_a, $_b] = get_tuple();
                ['a' => $_a, 'b' => $_b] = get_a_and_b();
            }
        "}
    }

    test_analysis! {
        name = destructuring_shape,
        code = indoc! {"
            <?php

            /**
             * @return array{name: string, age: int, hobbies: list<string>}
             */
            function get_shape(): array
            {
                return [
                    'name' => 'John Doe',
                    'age' => 30,
                    'hobbies' => ['reading', 'gaming', 'hiking'],
                ];
            }

            /**
             * @param string $_string
             */
            function i_take_string(string $_string): void {}

            /**
             * @param int $_int
             */
            function i_take_int(int $_int): void {}

            /**
             * @param list<string> $_list
             */
            function i_take_list_of_strings(array $_list): void {}

            ['name' => $name, 'age' => $age, 'hobbies' => $hobbies] = get_shape();

            i_take_string($name); // OK
            i_take_int($age); // OK
            i_take_list_of_strings($hobbies); // OK
        "},
    }

    test_analysis! {
        name = destructuring_keyed_shape_to_variables,
        code = indoc! {"
            <?php
            /** @return array{name: string, age: int, hobbies: list<string>} */
            function get_user_shape(): array {
                return ['name' => 'John', 'age' => 30, 'hobbies' => ['coding']];
            }
            /** @param string $_s */
            function i_take_string(string $_s): void {}
            /** @param int $_i */
            function i_take_int(int $_i): void {}
            /** @param list<string> $_l */
            function i_take_list_of_strings(array $_l): void {}

            ['name' => $name, 'age' => $age, 'hobbies' => $hobbies] = get_user_shape();

            i_take_string($name);
            i_take_int($age);
            i_take_list_of_strings($hobbies);
        "},
        issues = []
    }

    test_analysis! {
        name = destructuring_list_to_variables,
        code = indoc! {"
            <?php
            /** @return list<string> */
            function get_simple_list(): array { return ['a', 'b', 'c']; }
            /** @param string $_s */
            function i_take_string(string $_s): void {}

            [$a, $b, $c] = get_simple_list();
            i_take_string($a);
            i_take_string($b);
            i_take_string($c);
        "},
        issues = []
    }

    test_analysis! {
        name = destructuring_list_with_skipped_elements,
        code = indoc! {"
            <?php
            /** @param 'one' $_s */
            function i_take_one(string $_s): void {}
            /** @param 'three' $_s */
            function i_take_three(string $_s): void {}

            [$first, , $third] = ['one', 'two', 'three'];
            i_take_one($first);

            i_take_three($third);
        "},
        issues = []
    }

    test_analysis! {
        name = destructuring_list_with_trailing_comma_skip,
        code = indoc! {"
            <?php
            /** @param 10 $_i */
            function i_take_ten(int $_i): void {}
            [$x, , ] = [10, 20, 30];
            i_take_ten($x);
        "},
        issues = []
    }

    test_analysis! {
        name = destructuring_nested_list_within_keyed,
        code = indoc! {"
            <?php
            /** @return array{name: string, data: list<int>} */
            function get_shape_with_list(): array { return ['name' => 'test', 'data' => [10, 20]]; }
            /** @param string $_s */
            function i_take_string(string $_s): void {}
            /** @param int $_i */
            function i_take_int(int $_i): void {}

            ['name' => $name, 'data' => [$val1, $val2]] = get_shape_with_list();
            i_take_string($name);
            i_take_int($val1);
            i_take_int($val2);
        "},
        issues = []
    }

    test_analysis! {
        name = destructuring_nested_keyed_within_list,
        code = indoc! {"
            <?php
            /** @return list<array{id: int}> */
            function get_list_of_shapes(): array { return [['id' => 1], ['id' => 2]]; }
            /** @param int $_i */
            function i_take_int(int $_i): void {}

            [['id' => $firstId], ['id' => $secondId]] = get_list_of_shapes();
            i_take_int($firstId);
            i_take_int($secondId);
        "},
        issues = []
    }

    test_analysis! {
        name = destructuring_empty_array_results_in_null,
        code = indoc! {"
            <?php
            /** @param null $_n */
            function i_take_null($_n): void {}
            [$d, $e] = [];
            i_take_null($d);
            i_take_null($e);
        "},
        issues = [
            IssueCode::MismatchedArrayIndex,
            IssueCode::MismatchedArrayIndex,
        ]
    }

    test_analysis! {
        name = destructuring_missing_keyed_element_results_in_null,
        code = indoc! {"
            <?php
            /** @param null $_n */
            function i_take_null($_n): void {}
            /** @return array{name: string} */
            function get_partial_shape(): array { return ['name' => 'test']; }

            ['name' => $name, 'age' => $age] = get_partial_shape();
            i_take_null($age);
        "},
        issues = [
            IssueCode::UndefinedStringArrayIndex,
        ]
    }

    test_analysis! {
        name = destructuring_list_with_fewer_elements_results_in_null,
        code = indoc! {"
            <?php
            /** @param null $_n */
            function i_take_null($_n): void {}
            /** @param int $_i */
            function i_take_int(int $_i): void {}

            [$a, $b] = [1];
            i_take_int($a);
            i_take_null($b);
        "},
        issues = [
            IssueCode::UndefinedIntArrayIndex,
        ]
    }

    test_analysis! {
        name = destructuring_list_syntax_basic,
        code = indoc! {"
            <?php
            /** @param string $_s */
            function i_take_string(string $_s): void {}
            list($a, $b) = ['A', 'B'];
            i_take_string($a);
            i_take_string($b);
        "},
        issues = []
    }

    test_analysis! {
        name = destructuring_list_syntax_with_skipped_elements,
        code = indoc! {"
            <?php
            /** @param string $_s */
            function i_take_string(string $_s): void {}
            list($a, , $c) = ['A', 'B', 'C'];
            i_take_string($a);
            i_take_string($c);
        "},
        issues = []
    }

    test_analysis! {
        name = destructuring_list_syntax_with_keyed_source,
        code = indoc! {"
            <?php
            /** @param string $_s */
            function i_take_string(string $_s): void {}
            $source = [0 => 'a', 1 => 'b', 'key' => 'c'];
            list($a, $b) = $source;
            i_take_string($a);
            i_take_string($b);
        "},
        issues = []
    }

    test_analysis! {
        name = destructuring_keyed_with_integer_keys,
        code = indoc! {"
            <?php
            /** @param string $_s */
            function i_take_string(string $_s): void {}
            $source = [1 => 'one', 2 => 'two'];
            [1 => $val1, 2 => $val2] = $source;
            i_take_string($val1);
            i_take_string($val2);
        "},
        issues = []
    }

    test_analysis! {
        name = destructuring_empty_target_is_valid,
        code = indoc! {"
            <?php
            [] = [1, 2, 3]; // This is valid syntax, should produce no errors.
        "},
        issues = []
    }

    test_analysis! {
        name = destructuring_keyed_from_list_source,
        code = indoc! {r#"
            <?php
            /** @return list<string> */
            function get_list(): array { return ["a", "b"]; }
            /** @param string $_s */
            function i_take_string(string $_s): void {}
            [0 => $valA, 1 => $valB] = get_list();
            i_take_string($valA);
            i_take_string($valB);
        "#},
        issues = []
    }

    test_analysis! {
        name = destructuring_list_from_typed_array,
        settings = Settings {
            allow_possibly_undefined_array_keys: false,
            ..Default::default()
        },
        code = indoc! {"
            <?php

            /** @param array<int, float> $source */
            function test_typed_source(array $source): void {
                [$a, $b] = $source;
                /** @param float $_f */
                function i_take_float(float $_f): void {}
                i_take_float($a);
                i_take_float($b);
            }
        "},
        issues = [
            IssueCode::PossiblyUndefinedIntArrayIndex,
            IssueCode::PossiblyUndefinedIntArrayIndex,
        ]
    }

    test_analysis! {
        name = destructuring_list_from_typed_array_with_undefined_keys,
        settings = Settings {
            allow_possibly_undefined_array_keys: true,
            ..Default::default()
        },
        code = indoc! {"
            <?php

            /** @param array<int, float> $source */
            function test_typed_source(array $source): void {
                [$a, $b] = $source;
                /** @param float $_f */
                function i_take_float(float $_f): void {}
                i_take_float($a);
                i_take_float($b);
            }
        "},
    }

    test_analysis! {
        name = expression_is_too_complex,
        code = indoc! {"
            <?php

            function is_special_case(int $id, int $count, float $score, float $threshold, bool $is_active, bool $is_admin, string $name, string $role, string $permission, string $category): bool {
                $result = (
                    ($id > 1000 && $count < 5 || $score >= 99.5 && $threshold < $score || $name === 'azjezz' && $role !== 'guest') &&
                    ($is_active && !$is_admin || $permission === 'write' && ($category === 'critical' || $category === 'urgent')) ||
                    !($count === 0 || $id < 0) && (
                        $role === 'admin' && $is_admin ||
                        $name !== 'guest' && $permission !== 'none' ||
                        ($score - $threshold) > 5.0 && $count > 1
                    ) && (
                        $category === 'general' || $category === 'special' ||
                        ($is_active && $is_admin && $id % 2 === 0) ||
                        ($name !== 'system' && $role !== 'user' && $score < 50.0)
                    ) || (
                        $id < 0 && $count > 100 ||
                        ($score < 10.0 && $threshold > 20.0) ||
                        ($is_active && $is_admin && $name === 'root') ||
                        ($role === 'guest' && $permission === 'read' && $category === 'public')
                    )
                );

                return $result;
            }
        "},
        issues = [
            IssueCode::ExpressionIsTooComplex,
        ]
    }
}
