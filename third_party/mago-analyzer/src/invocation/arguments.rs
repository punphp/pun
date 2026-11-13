use std::borrow::Cow;
use std::collections::hash_map::Entry;
use std::rc::Rc;

use foldhash::HashMap;

use mago_allocator::Arena;
use mago_bytes::BytesDisplay;
use mago_codex::identifier::function_like::FunctionLikeIdentifier;
use mago_codex::ttype::TType;
use mago_codex::ttype::add_union_type;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::cast::cast_atomic_to_callable;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::comparator::ComparisonResult;
use mago_codex::ttype::comparator::union_comparator::can_expression_types_be_identical;
use mago_codex::ttype::comparator::union_comparator::is_contained_by;
use mago_codex::ttype::expander::get_signature_of_function_like_identifier;
use mago_codex::ttype::get_iterable_value_parameter;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::get_never;
use mago_codex::ttype::union::TUnion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_reporting::IssueCollection;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::Expression;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;
use crate::invocation::InvocationTarget;
use crate::utils::get_type_diff;
use crate::utils::misc::unwrap_expression;

/// Checks if an argument can be passed by reference.
pub(super) fn is_argument_referenceable(argument_expression: &Expression, argument_type: &TUnion) -> bool {
    argument_expression.is_referenceable(false)
        || (argument_expression.is_referenceable(true) && argument_type.by_reference())
}

pub(super) fn is_argument_mutated_by_reference(
    invocation_target: &InvocationTarget<'_>,
    argument_offset: usize,
    argument_type: &TUnion,
    declared_by_reference: bool,
) -> bool {
    if declared_by_reference && argument_offset > 0 && is_array_multisort(invocation_target) {
        return !argument_type.is_int();
    }

    declared_by_reference
}

fn requires_referenceable_argument(invocation_target: &InvocationTarget<'_>, declared_by_reference: bool) -> bool {
    declared_by_reference && !is_array_multisort(invocation_target)
}

pub(super) fn is_array_multisort(invocation_target: &InvocationTarget<'_>) -> bool {
    matches!(
        invocation_target.get_function_like_identifier(),
        Some(FunctionLikeIdentifier::Function(name)) if name.as_bytes().eq_ignore_ascii_case(b"array_multisort")
    )
}

fn is_empty_container_construction(expression: &Expression) -> bool {
    if let Expression::Instantiation(instantiation) = expression
        && let Some(argument_list) = &instantiation.argument_list
        && argument_list.arguments.len() == 1
        && let Some(first_arg) = argument_list.arguments.iter().next()
    {
        let argument_value = first_arg.value();

        match argument_value {
            Expression::Array(array_expr) => array_expr.elements.is_empty(),
            Expression::LegacyArray(array_expr) => array_expr.elements.is_empty(),
            _ => false,
        }
    } else {
        false
    }
}

/// The parameter types of a callable parameter type's signatures, joined by position: a closure argument's inferred
/// parameter types.
pub(crate) fn callable_parameter_types<A: Arena>(
    context: &Context<'_, '_, A>,
    callable_type: &TUnion,
) -> HashMap<usize, TUnion> {
    let mut inferred_parameters = HashMap::default();

    callable_type
        .types
        .as_ref()
        .iter()
        .filter_map(|atomic| match atomic {
            TAtomic::Callable(TCallable::Signature(callable)) => Some(callable),
            _ => None,
        })
        .flat_map(|callable| callable.parameters.iter().enumerate())
        .filter_map(|(parameter_index, parameter)| {
            parameter.get_type_signature().map(|param_type| (parameter_index, param_type.clone()))
        })
        .for_each(|(parameter_index, parameter_type)| match inferred_parameters.entry(parameter_index) {
            Entry::Occupied(occupied_entry) => {
                let existing_type: TUnion = occupied_entry.remove();
                let updated_type =
                    add_union_type(existing_type, &parameter_type, context.codebase, CombinerOptions::default());

                inferred_parameters.insert(parameter_index, updated_type);
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(parameter_type);
            }
        });

    inferred_parameters
}

/// ArrayHandler::getExpectedArrayType() and getExpectedValueType(): the value type the array members of a passed-to
/// type expect of an array literal's items, `None` without an array member.
pub(crate) fn expected_value_type<A: Arena>(context: &Context<'_, '_, A>, passed_to_type: &TUnion) -> Option<TUnion> {
    passed_to_type
        .types
        .iter()
        .filter(|atomic| matches!(atomic, TAtomic::Array(_) | TAtomic::Iterable(_)))
        .filter_map(|atomic| get_iterable_value_parameter(atomic, context.codebase))
        .reduce(|left, right| add_union_type(left, &right, context.codebase, CombinerOptions::default()))
}

/// Analyzes an argument expression and stores its inferred type.
pub fn analyze_and_store_argument_type<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    invocation_target: &InvocationTarget<'ctx>,
    argument_expression: &Expression<'arena>,
    argument_offset: usize,
    analyzed_argument_types: &mut [Option<(Rc<TUnion>, Span)>],
    referenced_parameter: bool,
    closure_parameter_type: Option<&TUnion>,
    call_context: Option<crate::observations::CallContext>,
    physical_reference_context: Option<bool>,
    expression_issues: Option<&mut IssueCollection>,
) -> Result<(), AnalysisError>
where
    A: Arena,
{
    if argument_offset != usize::MAX
        && let Some((argument_type, _)) = &analyzed_argument_types[argument_offset]
    {
        verify_referenceable_argument(
            context,
            invocation_target,
            argument_expression,
            argument_offset,
            referenced_parameter,
            argument_type,
        );
        return Ok(());
    }

    let is_array_literal =
        matches!(unwrap_expression(argument_expression), Expression::Array(_) | Expression::LegacyArray(_));
    let inferred_parameter_types = closure_parameter_type
        .filter(|_| !is_array_literal)
        .map(|closure_parameter_type| callable_parameter_types(context, closure_parameter_type))
        .map(|mut types| {
            // array_reduce()'s carry is its initial value and what the callback returns (`0|1`), which the initial
            // value alone (`0`) is not; ponytail: a literal carry keeps the callback's declared type
            if matches!(
                invocation_target.get_function_like_identifier(),
                Some(FunctionLikeIdentifier::Function(name)) if name.as_str().is_some_and(|n| n.eq_ignore_ascii_case("array_reduce"))
            ) && types.get(&0).is_some_and(|carry| {
                carry.types.iter().any(|t| matches!(t, TAtomic::Scalar(s) if s.is_literal_value()))
            }) {
                types.remove(&0);
            }
            types
        });
    // ArgumentsHandler passes an array literal its parameter's type (enterPassedToType())
    let passed_to_type = closure_parameter_type
        .filter(|_| is_array_literal)
        .and_then(|parameter_type| expected_value_type(context, parameter_type));
    let passed_to_type = std::mem::replace(&mut artifacts.array_passed_to_type, passed_to_type);

    let inferred_parameter_types = std::mem::replace(&mut artifacts.inferred_parameter_types, inferred_parameter_types);

    let was_inside_general_use = block_context.flags.inside_general_use();
    let was_inside_call = block_context.flags.inside_call();
    let was_inside_variable_reference = block_context.flags.inside_variable_reference();

    block_context.flags.set_inside_general_use(true);
    block_context.flags.set_inside_call(true);
    block_context.flags.set_inside_variable_reference(physical_reference_context.unwrap_or(referenced_parameter));

    // Unpacked/fallback arguments have a callee but no selected parameter.
    let call_context = call_context.or_else(|| {
        crate::observations::argument_context(context, Some(invocation_target), None, argument_expression.span())
    });
    let pushed = call_context.is_some();
    if let Some(call_context) = call_context {
        context.observations.calls.push(call_context);
    }
    let result = if let Some(expression_issues) = expression_issues {
        let (result, issues) = context.record(|context| argument_expression.analyze(context, block_context, artifacts));
        expression_issues.extend(issues);
        result
    } else {
        argument_expression.analyze(context, block_context, artifacts)
    };
    if pushed {
        context.observations.calls.pop();
    }
    result?;

    if referenced_parameter {
        crate::variable_flow::reference_argument(argument_expression, context);
    }
    block_context.flags.set_inside_general_use(was_inside_general_use);
    block_context.flags.set_inside_call(was_inside_call);
    block_context.flags.set_inside_variable_reference(was_inside_variable_reference);
    artifacts.inferred_parameter_types = inferred_parameter_types;
    artifacts.array_passed_to_type = passed_to_type;

    let argument_type =
        artifacts.get_rc_expression_type(argument_expression).cloned().unwrap_or_else(|| Rc::new(get_mixed()));

    verify_referenceable_argument(
        context,
        invocation_target,
        argument_expression,
        argument_offset,
        referenced_parameter,
        &argument_type,
    );

    if argument_offset != usize::MAX {
        analyzed_argument_types[argument_offset] = Some((argument_type, argument_expression.span()));
    }

    Ok(())
}

fn verify_referenceable_argument<A: Arena>(
    context: &mut Context<'_, '_, A>,
    invocation_target: &InvocationTarget<'_>,
    argument_expression: &Expression<'_>,
    argument_offset: usize,
    referenced_parameter: bool,
    argument_type: &TUnion,
) {
    if requires_referenceable_argument(invocation_target, referenced_parameter)
        && !is_argument_referenceable(argument_expression, argument_type)
    {
        let target_kind_str = invocation_target.guess_kind();
        let target_name_str = invocation_target.guess_name(context);
        let parameter_label = invocation_target
            .get_effective_parameter_name(argument_offset)
            .map_or_else(|| format!("#{}", argument_offset + 1), |name| format!("`{}`", BytesDisplay(name.as_bytes())));

        context.collector.report_with_code(
            IssueCode::InvalidPassByReference,
            Issue::error(format!(
                "Invalid argument for by-reference parameter {parameter_label} in call to {target_kind_str} `{target_name_str}`.",
            ))
            .with_annotation(
                Annotation::primary(argument_expression.span())
                    .with_message("This expression cannot be passed by reference."),
            )
            .with_note(
                "You can only pass variables, properties, array elements, or the result of another function that itself returns a reference."
            )
            .with_help("To fix this, assign this value to a variable first, and then pass that variable to the function."),
        );
    }
}

/// Verifies an argument's type against the expected parameter type.
pub fn verify_argument_type<'arena, A>(
    context: &mut Context<'_, 'arena, A>,
    input_type: &TUnion,
    parameter_type: &TUnion,
    argument_offset: usize,
    input_expression: &Expression<'arena>,
    invocation_target: &InvocationTarget<'_>,
) where
    A: Arena,
{
    // ErrorType preserves the failed read; its primary diagnostic is not an argument type mismatch.
    if input_type.is_error() {
        return;
    }

    let target_kind_str = invocation_target.guess_kind();
    let call_site_message = || match target_kind_str {
        "function" => Cow::Borrowed("Arguments to this function are incorrect"),
        "method" => Cow::Borrowed("Arguments to this method are incorrect"),
        "closure" => Cow::Borrowed("Arguments to this closure are incorrect"),
        "callable" => Cow::Borrowed("Arguments to this callable are incorrect"),
        _ => Cow::Owned(format!("Arguments to this {target_kind_str} are incorrect")),
    };
    let effective_parameter_name = invocation_target.get_effective_parameter_name(argument_offset);
    let argument_subject = || {
        effective_parameter_name.map_or_else(
            || format!("Argument #{}", argument_offset + 1),
            |name| format!("Argument `{}`", BytesDisplay(name.as_bytes())),
        )
    };

    if input_type.is_never() {
        let argument_subject = argument_subject();
        let target_name_str = invocation_target.guess_name(context);
        context.collector.report_with_code(
            IssueCode::NoValue,
            Issue::error(format!(
                "{argument_subject} passed to {target_kind_str} `{target_name_str}` has type `never`, meaning it cannot produce a value.",
            ))
            .with_annotation(
                Annotation::primary(input_expression.span())
                    .with_message("This argument expression results in type `never`")
            )
            .with_note(
                "The `never` type means no value can reach this point at runtime - this code path is unreachable."
            )
            .with_note(
                "This often occurs in unreachable code, due to impossible conditional logic, or if an expression always exits (e.g., `throw`, `exit()`)."
            )
            .with_help(
                "Review preceding logic to ensure this argument can receive a value, or remove if unreachable."
            ),
        );

        return;
    }

    if !parameter_type.accepts_null() {
        if input_type.is_null() {
            let argument_subject = argument_subject();
            let target_name_str = invocation_target.guess_name(context);
            let parameter_type_str = parameter_type.get_id();
            let call_site = Annotation::secondary(invocation_target.span()).with_message(call_site_message());
            context.collector.report_with_code(
                IssueCode::NullArgument,
                Issue::error(format!(
                    "{argument_subject} of {target_kind_str} `{target_name_str}` is `null`, but parameter type `{parameter_type_str}` does not accept it.",
                ))
                .with_annotation(Annotation::primary(input_expression.span()).with_message("This argument is `null`"))
                .with_annotation(call_site)
                .with_help(format!(
                    "Provide a non-null value, or declare the parameter as nullable (e.g., `{parameter_type_str}|null`)."
                )),
            );

            return;
        }

        if input_type.is_nullable() && !input_type.ignore_nullable_issues() {
            let argument_subject = argument_subject();
            let target_name_str = invocation_target.guess_name(context);
            let input_type_str = input_type.get_id();
            let parameter_type_str = parameter_type.get_id();
            let call_site = Annotation::secondary(invocation_target.span()).with_message(call_site_message());
            context.collector.report_with_code(
                IssueCode::PossiblyNullArgument,
                Issue::error(format!(
                    "{argument_subject} of {target_kind_str} `{target_name_str}` is possibly `null`, but parameter type `{parameter_type_str}` does not accept it.",
                ))
                .with_annotation(
                    Annotation::primary(input_expression.span())
                        .with_message(format!("This argument of type `{input_type_str}` might be `null`")),
                )
                .with_annotation(call_site)
                .with_help("Add a `null` check before this call to ensure the value is not `null`."),
            );
        }
    }

    if !parameter_type.accepts_false() {
        if input_type.is_false() {
            let argument_subject = argument_subject();
            let target_name_str = invocation_target.guess_name(context);
            let parameter_type_str = parameter_type.get_id();
            let call_site = Annotation::secondary(invocation_target.span()).with_message(call_site_message());
            context.collector.report_with_code(
                IssueCode::FalseArgument,
                Issue::error(format!(
                    "{argument_subject} of {target_kind_str} `{target_name_str}` is `false`, but parameter type `{parameter_type_str}` does not accept it.",
                ))
                .with_annotation(Annotation::primary(input_expression.span()).with_message("This argument is `false`"))
                .with_annotation(call_site)
                .with_help(format!(
                    "Provide a different value, or update the parameter type to accept false (e.g., `{parameter_type_str}|false`)."
                )),
            );

            return;
        }

        if input_type.is_falsable() && !input_type.ignore_falsable_issues() {
            let argument_subject = argument_subject();
            let target_name_str = invocation_target.guess_name(context);
            let input_type_str = input_type.get_id();
            let parameter_type_str = parameter_type.get_id();
            let call_site = Annotation::secondary(invocation_target.span()).with_message(call_site_message());
            context.collector.report_with_code(
                IssueCode::PossiblyFalseArgument,
                Issue::error(format!(
                    "{argument_subject} of {target_kind_str} `{target_name_str}` is possibly `false`, but parameter type `{parameter_type_str}` does not accept it.",
                ))
                .with_annotation(
                    Annotation::primary(input_expression.span())
                        .with_message(format!("This argument of type `{input_type_str}` might be `false`")),
                )
                .with_annotation(call_site)
                .with_help("Add a check to ensure the value is not `false` before this call."),
            );
        }
    }

    let mut union_comparison_result = ComparisonResult::new();
    let type_match_found =
        is_contained_by(context.codebase, input_type, parameter_type, true, true, false, &mut union_comparison_result);

    if type_match_found {
        return;
    }

    let argument_label = effective_parameter_name.map_or_else(
        || format!("argument #{}", argument_offset + 1),
        |name| format!("`{}`", BytesDisplay(name.as_bytes())),
    );
    let target_name_str = invocation_target.guess_name(context);
    let input_type_str = input_type.get_id();
    let parameter_type_str = parameter_type.get_id();
    let call_site = Annotation::secondary(invocation_target.span()).with_message(call_site_message());

    if input_type.is_mixed() {
        context.collector.report_with_code(
            IssueCode::MixedArgument,
            Issue::error(format!(
                "Invalid argument type for {} of `{}`: expected `{}`, but found `{}`.",
                argument_label,
                target_name_str,
                parameter_type_str,
                input_type_str
            ))
            .with_annotation(
                Annotation::primary(input_expression.span()).with_message(if input_type_str.as_bytes() == b"mixed" {
                    Cow::Borrowed("Argument has type `mixed`")
                } else {
                    Cow::Owned(format!("Argument has type `{input_type_str}`"))
                }),
            )
            .with_annotation(call_site)
            .with_note(format!(
                "The type `{input_type_str}` is too general and does not match the expected type `{parameter_type_str}`."
            ))
            .with_help("Add specific type hints or assertions to the argument value."),
        );

        return;
    }

    let is_empty_container = input_type.is_empty_array() || is_empty_container_construction(input_expression);
    if union_comparison_result.type_coerced.unwrap_or(false) && !input_type.is_mixed() && !is_empty_container {
        let (issue_kind, annotation_msg, note_msg) = if union_comparison_result
            .type_coerced_from_nested_mixed
            .unwrap_or(false)
        {
            (
                IssueCode::LessSpecificNestedArgumentType,
                format!("Provided type `{input_type_str}` is too general due to nested `mixed`."),
                "The structure contains `mixed`, making it incompatible.".to_string(),
            )
        } else {
            (
                IssueCode::LessSpecificArgument,
                format!("Provided type `{input_type_str}` is too general."),
                format!(
                    "The provided type `{input_type_str}` can be assigned to `{parameter_type_str}`, but is wider (less specific)."
                ),
            )
        };

        let mut issue = Issue::error(format!(
            "Argument type mismatch for {} of `{}`: expected `{}`, but provided type `{}` is less specific.",
            argument_label, target_name_str, parameter_type_str, input_type_str
        ))
        .with_annotation(Annotation::primary(input_expression.span()).with_message(annotation_msg))
        .with_annotation(call_site)
        .with_note(note_msg)
        .with_help(format!(
            "Provide a value that more precisely matches `{parameter_type_str}` or adjust the parameter type."
        ));

        if let Some(type_diff) = get_type_diff(context, parameter_type, input_type) {
            issue = issue.with_note(type_diff);
        }

        context.collector.report_with_code(issue_kind, issue);
    } else if !union_comparison_result.type_coerced.unwrap_or(false) {
        let parameter_requires_closure = parameter_type
            .types
            .iter()
            .all(|atomic| matches!(atomic, TAtomic::Callable(callable) if callable.is_closure()));

        let input_can_be_closure = input_type
            .types
            .iter()
            .any(|atomic| matches!(atomic, TAtomic::Callable(callable) if callable.is_closure()));

        let types_can_be_identical = (!parameter_requires_closure || input_can_be_closure)
            && can_expression_types_be_identical(context.codebase, input_type, parameter_type, false, false);

        if types_can_be_identical && parameter_type.is_callable() && !parameter_requires_closure {
            let all_inputs_are_resolvable_aliases = input_type.types.iter().all(|atomic| {
                if matches!(atomic, TAtomic::Callable(_)) {
                    false
                } else if let Some(callable) = cast_atomic_to_callable(atomic, context.codebase, None) {
                    match callable.as_ref() {
                        TCallable::Alias(id) => {
                            get_signature_of_function_like_identifier(id, context.codebase).is_some()
                        }
                        TCallable::Signature(_) => false,
                    }
                } else {
                    false
                }
            });

            if all_inputs_are_resolvable_aliases {
                return;
            }
        }

        let kind;
        let mut issue;
        if types_can_be_identical {
            kind = IssueCode::PossiblyInvalidArgument;

            issue = Issue::error(format!(
                "Possible argument type mismatch for {} of `{}`: expected `{}`, but possibly received `{}`.",
                argument_label, target_name_str, parameter_type_str, input_type_str
            ))
            .with_annotation(
                Annotation::primary(input_expression.span())
                    .with_message(format!("This might not be type `{parameter_type_str}`")),
            )
            .with_annotation(call_site)
            .with_note(format!(
                "The provided type `{input_type_str}` overlaps with `{parameter_type_str}` but is not fully contained."
            ))
            .with_help("Ensure the argument always has the expected type using checks or assertions.");
        } else {
            kind = IssueCode::InvalidArgument;
            issue = Issue::error(format!(
                "Invalid argument type for {} of `{}`: expected `{}`, but found `{}`.",
                argument_label, target_name_str, parameter_type_str, input_type_str
            ))
            .with_annotation(
                Annotation::primary(input_expression.span()).with_message(format!("This has type `{input_type_str}`")),
            )
            .with_annotation(call_site)
            .with_note(format!(
                "The provided type `{input_type_str}` is not compatible with the expected type `{parameter_type_str}`."
            ))
            .with_help(format!(
                "Change the argument value to match `{parameter_type_str}`, or update the parameter's type declaration."
            ));
        }

        if let Some(type_diff) = get_type_diff(context, parameter_type, input_type) {
            issue = issue.with_note(type_diff);
        }

        context.collector.report_with_code(kind, issue);
    }
}

/// Gets the element type when unpacking an argument with the spread operator.
pub fn get_unpacked_argument_type<A>(
    context: &mut Context<'_, '_, A>,
    argument_value_type: &TUnion,
    span: Span,
) -> TUnion
where
    A: Arena,
{
    let mut potential_element_types = Vec::new();
    let mut reported_an_error = false;

    for atomic_type in argument_value_type.types.as_ref() {
        if let Some(value_parameter) = get_iterable_value_parameter(atomic_type, context.codebase) {
            potential_element_types.push(value_parameter);

            continue;
        }

        match atomic_type {
            TAtomic::Never => {
                potential_element_types.push(get_never());
            }
            TAtomic::Mixed(_) => {
                if !reported_an_error {
                    context.collector.report_with_code(
                        IssueCode::MixedArgument,
                        Issue::error(format!(
                            "Cannot unpack argument of type `{}` because it is not guaranteed to be iterable.",
                            atomic_type.get_id()
                        ))
                        .with_annotation(Annotation::primary(span).with_message("Expected an `iterable` for unpacking"))
                        .with_note("Argument unpacking `...` requires an `iterable` (e.g., `array` or `Traversable`).")
                        .with_note("The type `mixed` provides no guarantee of iterability.")
                        .with_help("Ensure the value is an `iterable` using type hints, checks, or assertions."),
                    );
                    reported_an_error = true;
                }

                potential_element_types.push(get_mixed());
            }
            _ => {
                if !reported_an_error {
                    let type_str = atomic_type.get_id();
                    context.collector.report_with_code(
                        IssueCode::InvalidArgument,
                        Issue::error(format!(
                            "Cannot unpack argument of type `{type_str}` because it is not an iterable type."
                        ))
                        .with_annotation(
                            Annotation::primary(span).with_message(format!("Type `{type_str}` is not `iterable`")),
                        )
                        .with_note("Argument unpacking `...` requires an `iterable` (e.g., `array` or `Traversable`).")
                        .with_help("Ensure the value being unpacked is an `iterable`."),
                    );
                    reported_an_error = true;
                }
                potential_element_types.push(get_mixed());
            }
        }
    }

    potential_element_types
        .into_iter()
        .reduce(|acc, element_type| add_union_type(acc, &element_type, context.codebase, CombinerOptions::default()))
        .unwrap_or_else(get_never)
}
