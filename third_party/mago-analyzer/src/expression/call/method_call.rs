use mago_allocator::Arena;
use mago_word::Word;
use mago_word::WordMap;

use mago_codex::identifier::function_like::FunctionLikeIdentifier;
use mago_codex::identifier::method::MethodIdentifier;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::add_optional_union_type;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::expander::StaticClassType;
use mago_codex::ttype::get_error;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::get_never;
use mago_codex::ttype::get_null;
use mago_codex::ttype::template::TemplateResult;
use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::ArgumentList;
use mago_syntax::cst::Call;
use mago_syntax::cst::ClassLikeMemberSelector;
use mago_syntax::cst::Expression;
use mago_syntax::cst::MethodCall;
use mago_syntax::cst::NullSafeMethodCall;
use mago_syntax::cst::Variable;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::context::non_nullability::EnsuredNonNullability;
use crate::error::AnalysisError;
use crate::expression::call::analyze_invocation_targets;
use crate::expression::call::record_external_method_call;
use crate::expression::call::record_external_method_call_targets;
use crate::invocation::Invocation;
use crate::invocation::InvocationArgumentsSource;
use crate::invocation::InvocationTarget;
use crate::invocation::MethodInvocationKind;
use crate::invocation::MethodTargetContext;
use crate::invocation::analyzer::EvaluatedArguments;
use crate::invocation::analyzer::analyze_invocation_with_arguments;
use crate::invocation::analyzer::apply_callable_signature;
use crate::invocation::post_process::post_invocation_process;
use crate::invocation::return_type_fetcher::fetch_declared_invocation_return_type;
use crate::invocation::return_type_fetcher::fetch_function_like_provider_return_type;
use crate::invocation::return_type_fetcher::fetch_invocation_return_type;
use crate::invocation::template_result::populate_template_result_from_invocation;
use crate::plugin::ExpressionHookResult;
use crate::plugin::context::HookContext;
use crate::resolver::method::UndocumentedMethod;
use crate::resolver::method::UnresolvedMethod;
use crate::resolver::method::report_non_documented_method;
use crate::resolver::method::report_non_existent_method;
use crate::resolver::method::resolve_method_targets;
use crate::resolver::method::resolve_method_targets_after_receiver;
use crate::utils::availability;
use crate::utils::expression::get_block_expression_id;
use crate::visibility::check_method_visibility_by_id;

impl<'ast, 'arena> Analyzable<'ast, 'arena> for MethodCall<'arena> {
    fn analyze<'ctx, A>(
        &'ast self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError>
    where
        A: Arena,
    {
        let observation = crate::observations::begin_call(self, context, block_context);
        if context.plugin_registry.has_method_call_hooks() {
            let mut hook_context = HookContext::new(context.codebase, context.source_file, block_context, artifacts);
            let result = context.plugin_registry.before_method_call(self, &mut hook_context)?;
            for reported in hook_context.take_issues() {
                context.collector.report_with_code(reported.code, reported.issue);
            }

            match result {
                ExpressionHookResult::Continue => {}
                ExpressionHookResult::Skip => {
                    return Ok(());
                }
                ExpressionHookResult::SkipWithType(ty) => {
                    artifacts.set_expression_type(&self.span(), ty);
                    return Ok(());
                }
            }
        }

        analyze_method_call(
            context,
            block_context,
            artifacts,
            self.object,
            &self.method,
            &self.argument_list,
            false, // is_nullsafe
            self.span(),
            None,
        )?;

        if context.plugin_registry.has_method_call_hooks() {
            let mut hook_context = HookContext::new(context.codebase, context.source_file, block_context, artifacts);
            context.plugin_registry.after_method_call(self, &mut hook_context)?;
            for reported in hook_context.take_issues() {
                context.collector.report_with_code(reported.code, reported.issue);
            }
        }

        crate::observations::finish_call(observation, context, artifacts)?;
        Ok(())
    }
}

impl<'ast, 'arena> Analyzable<'ast, 'arena> for NullSafeMethodCall<'arena> {
    fn analyze<'ctx, A>(
        &'ast self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError>
    where
        A: Arena,
    {
        artifacts.nullsafe_method_receiver_types.remove(&self.span());
        if context.plugin_registry.has_nullsafe_method_call_hooks() {
            let mut hook_context = HookContext::new(context.codebase, context.source_file, block_context, artifacts);
            let result = context.plugin_registry.before_nullsafe_method_call(self, &mut hook_context)?;
            for reported in hook_context.take_issues() {
                context.collector.report_with_code(reported.code, reported.issue);
            }

            match result {
                ExpressionHookResult::Continue => {}
                ExpressionHookResult::Skip => {
                    return Ok(());
                }
                ExpressionHookResult::SkipWithType(ty) => {
                    artifacts.set_expression_type(&self.span(), ty);
                    return Ok(());
                }
            }
        }

        let observation = crate::observations::begin_nullsafe_call(self, context, block_context);
        // PHPStan's NullsafeMethodCallHandler always returns a non-terminating expression result.
        let has_returned = block_context.flags.has_returned();
        let control_actions = block_context.control_actions.clone();
        let before = block_context.clone();
        let inside_general_use = block_context.flags.inside_general_use();
        block_context.flags.set_inside_general_use(true);
        let receiver_result = self.object.analyze(context, block_context, artifacts);
        block_context.flags.set_inside_general_use(inside_general_use);
        receiver_result?;
        let ensured = crate::context::non_nullability::ensure(self.object, context, block_context, artifacts);
        let receiver_type = ensured.original_type.clone();
        artifacts.nullsafe_method_receiver_types.insert(self.span(), receiver_type.clone());
        let previous_consume = context.consume_stored_expression_results;
        context.consume_stored_expression_results = true;
        crate::observations::enter_nullsafe_twin(
            self, &ensured.receiver_type, ensured.scope_authoritative, context, block_context,
        );
        let result = (|| {
            analyze_method_call(
                context,
                block_context,
                artifacts,
                self.object,
                &self.method,
                &self.argument_list,
                true, // is_nullsafe
                self.span(),
                Some(&ensured),
            )?;
            crate::observations::finish_call(observation, context, artifacts)
        })();
        context.consume_stored_expression_results = previous_consume;
        ensured.revert(context, block_context, artifacts);
        result?;
        if receiver_type.is_null() {
            *block_context = before;
            let mut return_type = get_null();
            return_type.set_nullsafe_null(true);
            artifacts.set_resolved_expression_type(&self.span(), return_type);
        } else if crate::context::non_nullability::may_be_null(&receiver_type) {
            crate::context::non_nullability::merge_short_circuit(block_context, &before, context);
        }
        block_context.flags.set_has_returned(has_returned);
        block_context.control_actions = control_actions;

        if context.plugin_registry.has_nullsafe_method_call_hooks() {
            let mut hook_context = HookContext::new(context.codebase, context.source_file, block_context, artifacts);
            context.plugin_registry.after_nullsafe_method_call(self, &mut hook_context)?;
            for reported in hook_context.take_issues() {
                context.collector.report_with_code(reported.code, reported.issue);
            }
        }

        Ok(())
    }
}

/// Analyzes an implicit method call that doesn't correspond to a direct call expression
/// in the source code.
///
/// This function simulates a method invocation to determine its return type and analyze
/// potential side effects. It is primarily used for operations that trigger "magic methods"
/// at runtime.
///
/// ### Use Cases
///
/// - **String Casting**: Analyzing the `__toString()` method when an object is cast
///   to a string (e.g., `(string) $obj`).
/// - **Cloning**: Analyzing the `__clone()` method when an object is cloned
///   (e.g., `clone $obj`).
///
/// ### Process
///
/// 1. Checks if the method is visible from the current context.
/// 2. Constructs a synthetic `Invocation` to represent the call with no arguments.
/// 3. Resolves template parameters from the target object type.
/// 4. Fetches the method's return type.
/// 5. Performs post-invocation analysis.
///
/// ### Arguments
///
/// - `object_type`: The type of the object on which the method is implicitly called.
/// - `method_identifier`: The identifier for the method (e.g., `__toString`).
/// - `span`: The code span of the expression triggering the call (e.g., the cast expression).
///
/// ### Returns
///
/// A `Result` containing the `TUnion` type of the method's return value. If the method
/// is not visible, it returns the `never` type.
pub fn analyze_implicit_method_call<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    object_type: &TObject,
    object_variable: Option<&[u8]>,
    requested_name: Word,
    method_identifier: MethodIdentifier,
    class_like_metadata: &'ctx ClassLikeMetadata,
    method_metadata: &'ctx FunctionLikeMetadata,
    arguments_source: Option<InvocationArgumentsSource<'_, 'arena>>,
    span: Span,
) -> Result<TUnion, AnalysisError>
where
    A: Arena,
{
    if !check_method_visibility_by_id(
        context,
        block_context.scope.get_class_like_name(),
        &method_identifier,
        span,
        None,
    ) {
        return Ok(get_never()); // Not visible, return never type.
    }

    let mut template_result = TemplateResult::default();

    let method_target_context = MethodTargetContext {
        invocation_kind: MethodInvocationKind::Instance,
        requested_name: requested_name.to_ascii_lowercase(),
        declaring_method_id: Some(method_identifier),
        class_like_metadata,
        class_type: StaticClassType::Object(object_type.clone()),
        declaring_object_type: None,
    };

    let invocation = Invocation::new(
        InvocationTarget::FunctionLike {
            identifier: FunctionLikeIdentifier::Method(
                method_identifier.get_class_name(),
                method_identifier.get_method_name(),
            ),
            metadata: method_metadata,
            inferred_return_type: None,
            effective_signature: None,
            method_context: Some(method_target_context),
            span,
        },
        arguments_source.unwrap_or(InvocationArgumentsSource::None(span)),
        span,
    );

    populate_template_result_from_invocation(context, &invocation, &mut template_result);

    let result = fetch_invocation_return_type(
        context,
        block_context,
        artifacts,
        &invocation,
        &template_result,
        &WordMap::default(),
    )?;

    post_invocation_process(
        context,
        block_context,
        artifacts,
        &invocation,
        object_variable,
        &template_result,
        &WordMap::default(),
        false,
    )?;

    Ok(result.ty)
}

#[allow(clippy::expect_used)]
fn analyze_method_call<'ctx, 'ast, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    object: &'ast Expression<'arena>,
    selector: &'ast ClassLikeMemberSelector<'arena>,
    argument_list: &'ast ArgumentList<'arena>,
    is_null_safe: bool,
    span: Span,
    receiver: Option<&EnsuredNonNullability<'arena>>,
) -> Result<(), AnalysisError>
where
    A: Arena,
{
    let reflection_scope = block_context.clone();
    if block_context.flags.collect_initializations()
        && let ClassLikeMemberSelector::Identifier(method_ident) = selector
        && is_this_or_self_returning_chain(object, context, block_context)
    {
        let method_name = mago_word::ascii_lowercase_word(method_ident.value);
        block_context.definitely_called_methods.insert(method_name);
        block_context.called_methods.insert(method_name);
    }

    let mut method_resolution = if let Some(receiver) = receiver {
        let mut resolution = resolve_method_targets_after_receiver(
            context,
            block_context,
            artifacts,
            object,
            selector,
            is_null_safe,
            span,
            Some(&receiver.receiver_type),
        )?;
        // The plain twin resolves on a non-null device. Only the receiver's
        // original value determines the nullsafe expression's short-circuit.
        resolution.encountered_null = receiver.original_type.is_nullable();
        resolution
    } else {
        resolve_method_targets(context, block_context, artifacts, object, selector, is_null_safe, span)?
    };

    // MethodCallHandler: capture the already-evaluated receiver before arguments
    // can overwrite an alias. Only a same-declaring-class constructor call qualifies.
    let initializing_method = if !context.settings.property_initialization_observations.is_empty()
        && matches!(selector, ClassLikeMemberSelector::Identifier(_))
        && matches!(block_context.scope.get_function_like_identifier(),
            Some(FunctionLikeIdentifier::Method(_, name)) if name.as_bytes().eq_ignore_ascii_case(b"__construct"))
        && artifacts.get_expression_type(object).is_some_and(crate::property_initialization::has_this_type)
    {
        method_resolution.property_initialization_method.or_else(|| {
            // Preserve the old unambiguous boundary for carriers whose compound
            // reflection order is not available (for example array-containing unions).
            method_resolution.resolved_methods.first().map(|method| method.method_identifier)
                .filter(|id| method_resolution.resolved_methods.iter().all(|method| method.method_identifier == *id))
        }).filter(|id| {
            block_context.scope.get_class_like_name().is_some_and(|class| {
                class.as_bytes().eq_ignore_ascii_case(id.get_class_name().as_bytes())
            })
        })
    } else {
        None
    };

    // The ensured nullsafe twin still walks its arguments when the original
    // receiver is null. Its handler discards those effects afterwards.
    if receiver.is_none()
        && artifacts.get_expression_type(object).is_some_and(|ty| ty.is_null() && (is_null_safe || ty.has_nullsafe_null()))
    {
        let mut return_type = get_null();
        return_type.set_nullsafe_null(true);
        artifacts.set_resolved_expression_type(&span, return_type);

        return Ok(());
    }

    let short_circuit = (!is_null_safe
        && receiver.is_none()
        && crate::context::non_nullability::may_short_circuit(object, artifacts))
    .then(|| block_context.clone());

    let undocumented_template_result =
        (!method_resolution.undocumented_methods.is_empty()).then(|| method_resolution.template_result.clone());

    let mut invocation_targets = vec![];
    for resolved_method in method_resolution.resolved_methods {
        let metadata = context
            .codebase
            .get_class_like_by_name(resolved_method.classname)
            .expect("class-like metadata should exist for resolved method");

        let method_metadata = context
            .codebase
            .get_method_by_id(&resolved_method.method_identifier)
            .expect("method metadata should exist for resolved method");

        availability::check_method_availability(
            context,
            method_metadata,
            &format_args!(
                "{}::{}",
                resolved_method.method_identifier.get_class_name(),
                resolved_method.method_identifier.get_method_name(),
            ),
            span,
        );

        let method_target_context = MethodTargetContext {
            invocation_kind: MethodInvocationKind::Instance,
            requested_name: resolved_method.requested_name,
            declaring_method_id: Some(resolved_method.method_identifier),
            class_like_metadata: metadata,
            class_type: resolved_method.static_class_type,
            declaring_object_type: resolved_method.declaring_object,
        };

        invocation_targets.push(InvocationTarget::FunctionLike {
            identifier: FunctionLikeIdentifier::Method(
                resolved_method.method_identifier.get_class_name(),
                resolved_method.method_identifier.get_method_name(),
            ),
            metadata: method_metadata,
            inferred_return_type: None,
            effective_signature: None,
            method_context: Some(method_target_context),
            span,
        });
    }

    if !method_resolution.unresolved_methods.is_empty() {
        method_resolution.has_invalid_target |= prepare_unresolved_method_targets(
            context,
            artifacts,
            std::mem::take(&mut method_resolution.unresolved_methods),
            InvocationArgumentsSource::ArgumentList(argument_list),
            span,
            MethodInvocationKind::Instance,
            &mut invocation_targets,
        )?;
    }

    let has_resolved_methods = !invocation_targets.is_empty();

    record_external_method_call(context, artifacts, &invocation_targets, span);
    if !method_resolution.undocumented_methods.is_empty() {
        record_external_method_call_targets(
            context,
            artifacts,
            method_resolution.undocumented_methods.iter().map(|method| (method.classname, method.method_name)),
            span,
        );
    }

    let this_variable = get_block_expression_id(object, context, block_context);
    if is_null_safe && let Some(var_id) = this_variable.as_ref() {
        artifacts
            .true_branch_only_assertions
            .entry((span.start.offset, span.end.offset))
            .or_default()
            .entry(*var_id)
            .or_default()
            .push(vec![mago_codex::assertion::Assertion::IsNotType(mago_codex::ttype::atomic::TAtomic::Null)]);
    }

    if has_resolved_methods || method_resolution.undocumented_methods.is_empty() {
        analyze_invocation_targets(
            context,
            block_context,
            &reflection_scope,
            artifacts,
            method_resolution.template_result,
            invocation_targets,
            InvocationArgumentsSource::ArgumentList(argument_list),
            span,
            this_variable,
            method_resolution.encountered_mixed,
            is_null_safe && method_resolution.encountered_null,
            artifacts.get_expression_type(object).is_some_and(|t| t.has_nullsafe_null()),
            is_null_safe,
            matches!(selector, ClassLikeMemberSelector::Identifier(_)),
        )?;
    }

    if !method_resolution.undocumented_methods.is_empty() {
        analyze_undocumented_method_return_type(
            context,
            block_context,
            artifacts,
            method_resolution.undocumented_methods,
            undocumented_template_result.as_ref().expect("undocumented calls have a template result"),
            argument_list,
            span,
            MethodInvocationKind::Instance,
            !has_resolved_methods,
        )?;
    }

    // MethodCallHandler::resolveReturnType(): no reflected method is ErrorType, not an absent result.
    // Dynamic selectors remain mixed; resolved, magic and provider results have already been stored.
    if artifacts.get_expression_type(&span).is_none() {
        let return_type = if method_resolution.has_dynamic_selector { get_mixed() } else { get_error() };
        artifacts.set_resolved_expression_type(&span, return_type);
    }

    if let Some(id) = initializing_method
        && let Some(summary) = crate::property_initialization::called_method(context, id)?
    {
        crate::property_initialization::merge_initialized(&mut block_context.variable_state.property_initializations, &summary);
    }

    if let Some(before) = short_circuit {
        crate::context::non_nullability::merge_short_circuit(block_context, &before, context);
    }

    Ok(())
}

/// Gives external callable-signature providers an opportunity to establish
/// otherwise unresolved methods before Mago reports them as non-existent.
pub(super) fn prepare_unresolved_method_targets<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    artifacts: &AnalysisArtifacts,
    unresolved_methods: Vec<UnresolvedMethod>,
    arguments: InvocationArgumentsSource<'_, 'arena>,
    span: Span,
    invocation_kind: MethodInvocationKind,
    targets: &mut Vec<InvocationTarget<'ctx>>,
) -> Result<bool, AnalysisError>
where
    A: Arena,
{
    targets.reserve(unresolved_methods.len());
    let mut has_invalid_target = false;
    for unresolved in unresolved_methods {
        let Some(class_like_metadata) = context.codebase.get_class_like_by_name(unresolved.classname) else {
            report_non_existent_method(
                context,
                unresolved.target_span,
                unresolved.selector_span,
                unresolved.classname,
                unresolved.method_name,
            );

            has_invalid_target = true;
            continue;
        };

        let identifier = FunctionLikeIdentifier::Method(
            class_like_metadata.original_name,
            unresolved.method_name.to_ascii_lowercase(),
        );
        let target = InvocationTarget::ExternalMethod {
            identifier,
            effective_signature: None,
            method_context: MethodTargetContext {
                invocation_kind,
                requested_name: unresolved.method_name.to_ascii_lowercase(),
                declaring_method_id: None,
                class_like_metadata,
                class_type: unresolved.class_type,
                declaring_object_type: None,
            },
            span,
        };

        let mut invocation = Invocation::new(target, arguments, span);
        if apply_callable_signature(context, artifacts, &identifier, &mut invocation) {
            targets.push(invocation.target);
        } else if !class_like_metadata.has_incomplete_hierarchy() {
            report_non_existent_method(
                context,
                unresolved.target_span,
                unresolved.selector_span,
                unresolved.classname,
                unresolved.method_name,
            );

            has_invalid_target = true;
        }
    }

    Ok(has_invalid_target)
}

/// Gives return-type providers the first opportunity to describe an
/// undocumented magic call, then falls back to the magic method's declaration.
#[allow(clippy::expect_used)]
pub(super) fn analyze_undocumented_method_return_type<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    undocumented_methods: Vec<UndocumentedMethod>,
    template_result: &TemplateResult,
    argument_list: &ArgumentList<'arena>,
    span: Span,
    invocation_kind: MethodInvocationKind,
    analyze_arguments: bool,
) -> Result<(), AnalysisError>
where
    A: Arena,
{
    let mut arguments_analyzed = !analyze_arguments;
    let mut evaluated_arguments = EvaluatedArguments::new(argument_list.arguments.len());
    let mut resulting_type: Option<TUnion> = None;
    for undocumented_method in undocumented_methods {
        let magic_call_method = undocumented_method.magic_method;
        let class_like_metadata = context
            .codebase
            .get_class_like_by_name(magic_call_method.classname)
            .expect("class-like metadata should exist for resolved magic call method");

        let method_metadata = context
            .codebase
            .get_method_by_id(&magic_call_method.method_identifier)
            .expect("method metadata should exist for resolved magic call method");

        let requested_identifier = FunctionLikeIdentifier::Method(
            undocumented_method.classname,
            undocumented_method.method_name.to_ascii_lowercase(),
        );
        let target = InvocationTarget::FunctionLike {
            identifier: FunctionLikeIdentifier::Method(
                magic_call_method.method_identifier.get_class_name(),
                magic_call_method.method_identifier.get_method_name(),
            ),
            metadata: method_metadata,
            inferred_return_type: None,
            effective_signature: None,
            method_context: Some(MethodTargetContext {
                invocation_kind,
                requested_name: magic_call_method.requested_name,
                declaring_method_id: Some(magic_call_method.method_identifier),
                class_like_metadata,
                class_type: magic_call_method.static_class_type,
                declaring_object_type: magic_call_method.declaring_object,
            }),
            span,
        };

        let mut invocation = Invocation::new(target, InvocationArgumentsSource::ArgumentList(argument_list), span);
        let signature_handled = if context.external_analysis_session.is_some()
            && context.plugin_registry.may_have_callable_signature_provider(&requested_identifier)
        {
            apply_callable_signature(context, artifacts, &requested_identifier, &mut invocation)
        } else {
            false
        };

        let mut invocation_template_result = template_result.clone();
        let mut parameter_types = WordMap::default();
        if signature_handled && analyze_arguments {
            analyze_invocation_with_arguments(
                context,
                block_context,
                artifacts,
                &mut invocation,
                None,
                None,
                &mut invocation_template_result,
                &mut parameter_types,
                &mut evaluated_arguments,
            )?;

            arguments_analyzed = true;
        } else if !arguments_analyzed {
            argument_list.analyze(context, block_context, artifacts)?;
            evaluated_arguments.remember(&InvocationArgumentsSource::ArgumentList(argument_list), artifacts);
            arguments_analyzed = true;
        }

        let return_type = match fetch_function_like_provider_return_type(
            context,
            block_context,
            artifacts,
            &requested_identifier,
            &invocation,
        ) {
            Some(return_type) => return_type,
            None => {
                if !signature_handled {
                    report_non_documented_method(
                        context,
                        undocumented_method.target_span,
                        undocumented_method.selector_span,
                        undocumented_method.classname,
                        undocumented_method.method_name,
                    );
                }

                fetch_declared_invocation_return_type(
                    context,
                    &invocation,
                    &invocation_template_result,
                    &parameter_types,
                )
            }
        };

        resulting_type = Some(add_optional_union_type(return_type, resulting_type.as_ref(), context.codebase));
    }

    let mut resulting_type = resulting_type.unwrap_or_else(get_mixed);
    if let Some(existing_type) = artifacts.get_raw_expression_type(&span) {
        resulting_type = add_optional_union_type(resulting_type, Some(existing_type), context.codebase);
    }

    artifacts.set_resolved_expression_type(&span, resulting_type);

    Ok(())
}

/// Checks if an expression ultimately derives from `$this` through a chain of method calls
/// where each method returns `self`/`static`/the same class type.
///
/// This enables tracking method calls in chains like `$this->foo()->bar()->baz()`
/// where `foo()` and `bar()` return `self` or `static`.
fn is_this_or_self_returning_chain<'ctx, 'arena, A>(
    expr: &Expression<'arena>,
    context: &Context<'ctx, 'arena, A>,
    block_context: &BlockContext<'ctx>,
) -> bool
where
    A: Arena,
{
    match expr {
        Expression::Variable(Variable::Direct(var)) if var.name == b"$this" => true,
        Expression::Call(Call::Method(method_call)) => {
            if !is_this_or_self_returning_chain(method_call.object, context, block_context) {
                return false;
            }

            let ClassLikeMemberSelector::Identifier(method_ident) = &method_call.method else {
                return false;
            };

            let Some(class_like) = block_context.scope.get_class_like() else {
                return false;
            };

            let method_name = mago_word::ascii_lowercase_word(method_ident.value);
            method_returns_self_or_static(context, class_like.name, method_name)
        }
        Expression::Call(Call::NullSafeMethod(method_call)) => {
            if !is_this_or_self_returning_chain(method_call.object, context, block_context) {
                return false;
            }

            let ClassLikeMemberSelector::Identifier(method_ident) = &method_call.method else {
                return false;
            };

            let Some(class_like) = block_context.scope.get_class_like() else {
                return false;
            };

            let method_name = mago_word::ascii_lowercase_word(method_ident.value);
            method_returns_self_or_static(context, class_like.name, method_name)
        }
        Expression::Parenthesized(paren) => is_this_or_self_returning_chain(paren.expression, context, block_context),
        _ => false,
    }
}

/// Checks if a method returns `self`, `static`, or the same class type.
fn method_returns_self_or_static<A>(context: &Context<'_, '_, A>, class_name: Word, method_name: Word) -> bool
where
    A: Arena,
{
    let method_id = MethodIdentifier::new(class_name, method_name);
    let Some(method_meta) = context.codebase.get_method_by_id(&method_id) else {
        return false;
    };

    if let Some(return_type_meta) = &method_meta.return_type_declaration_metadata {
        for atomic in return_type_meta.type_union.types.iter() {
            match atomic {
                TAtomic::Object(TObject::Named(named_obj)) if named_obj.is_static => return true,
                TAtomic::Object(TObject::Named(named_obj))
                    if named_obj.name.as_bytes().eq_ignore_ascii_case(class_name.as_bytes()) =>
                {
                    return true;
                }
                _ => {}
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::code::IssueCode;
    use crate::test_analysis;

    #[test]
    fn failed_method_fallback_preserves_active_signature_and_return_providers() {
        use std::borrow::Cow;
        use std::num::NonZeroUsize;
        use std::sync::{Arc, Mutex};

        use mago_allocator::LocalArena;
        use mago_codex::populator::populate_codebase;
        use mago_codex::reference::SymbolReferences;
        use mago_codex::scanner::scan_program;
        use mago_codex::ttype::{TType, get_literal_int};
        use mago_database::file::{File, FileType};
        use mago_extension::{WorkerCommand, WorkerPool, WorkerPoolOptions};
        use mago_names::resolver::NameResolver;
        use mago_php_version::PHPVersion;
        use mago_span::{Position, Span};
        use mago_syntax::parser::parse_file;

        use crate::Analyzer;
        use crate::analysis_result::AnalysisResult;
        use crate::external::protocol::{ANALYZER_PROTOCOL_MAJOR, ANALYZER_PROTOCOL_MINOR};
        use crate::external::{ExternalAnalyzer, ExternalAnalyzerHandle};
        use crate::invocation::{InvocationTarget, MethodInvocationKind};
        use crate::plugin::PluginRegistry;
        use crate::plugin::context::{HookContext, InvocationInfo, ProviderContext};
        use crate::plugin::hook::{ExpressionHook, HookResult};
        use crate::plugin::provider::{MethodReturnTypeProvider, MethodTarget, Provider, ProviderMeta};
        use crate::settings::Settings;

        #[derive(Clone, Default)]
        struct LookupNames(Arc<Mutex<Vec<(String, String, String)>>>);
        impl ExpressionHook for LookupNames {
            fn is_observer(&self) -> bool {
                true
            }

            fn before_arguments(
                &self,
                call: &InvocationInfo<'_, '_, '_>,
                _: &mut HookContext<'_, '_>,
            ) -> HookResult<()> {
                if call.function_name() == "SignatureTarget::provided" {
                    let target = &call.inner().target;
                    assert!(matches!(target, InvocationTarget::ExternalMethod { effective_signature: Some(_), .. }));
                    assert!(target.get_function_like_metadata().is_none());
                    let method = target.get_method_context().unwrap();
                    assert_eq!(method.invocation_kind, MethodInvocationKind::Instance);
                    assert!(method.declaring_method_id.is_none());
                    self.0.lock().unwrap().push((
                        call.function_name(),
                        method.requested_name.to_string(),
                        method.class_like_metadata.original_name.to_string(),
                    ));
                }
                Ok(())
            }
        }

        struct Return;
        impl Provider for Return {
            fn meta() -> &'static ProviderMeta {
                static META: ProviderMeta = ProviderMeta::new("failed-method-return", "Return", "Real method return");
                &META
            }
        }
        impl MethodReturnTypeProvider for Return {
            fn targets() -> &'static [MethodTarget] {
                static TARGETS: [MethodTarget; 1] = [MethodTarget::exact(b"ProviderTarget", b"real")];
                &TARGETS
            }

            fn get_return_type(
                &self,
                _: &ProviderContext<'_, '_, '_>,
                _: &[u8],
                _: &[u8],
                _: &InvocationInfo<'_, '_, '_>,
            ) -> Option<mago_codex::ttype::union::TUnion> {
                Some(get_literal_int(91))
            }
        }

        let worker = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/failed-method-provider.php");
        // Keep the existing worker's payloads, but use the current public protocol version in its header.
        let worker = std::fs::read_to_string(worker).unwrap().replace(
            "pack('nnnn', 2, 1, $kind, 0)",
            &format!("pack('nnnn', {ANALYZER_PROTOCOL_MAJOR}, {ANALYZER_PROTOCOL_MINOR}, $kind, 0)"),
        );
        let pool = WorkerPool::spawn(
            WorkerCommand::new("php").with_argument("-r").with_argument(worker.strip_prefix("<?php").unwrap()),
            NonZeroUsize::new(1).unwrap(),
            WorkerPoolOptions::default(),
        ).unwrap();
        let external = ExternalAnalyzer::initialize([Arc::new(pool)], PHPVersion::PHP84, &[], false).unwrap();
        let mut plugins = PluginRegistry::new();
        plugins.set_external_analyzer(Arc::new(ExternalAnalyzerHandle::ready(external)));
        let lookup_names = LookupNames::default();
        plugins.register_expression_hook(lookup_names.clone());
        plugins.register_method_provider(Return);
        plugins.prepare_external_analyzer().unwrap();
        let code = indoc! {r#"
            <?php
            class SignatureTarget {}
            class ProviderTarget {
                public function real(): string { return 'native'; }
                public function __call(string $name, array $arguments): string { return 'magic'; }
            }
            $signature = new SignatureTarget();
            $signature->PrOvIdEd();
            $object = new ProviderTarget();
            $object->provided();
            ProviderTarget::provided();
            $object->magic();
            $object->real();
            $callable = $object->real(...);
            $callable();
            $object->withoutProvider();
        "#};
        let arena = LocalArena::new();
        let file = Arc::new(File::new(
            Cow::Borrowed(b"failed-method-providers.php"), FileType::Host, None, Cow::Borrowed(code.as_bytes()),
        ));
        let program = parse_file(&arena, &file);
        let names = NameResolver::new(&arena).resolve(program);
        let mut codebase = scan_program(&arena, &file, program, &names, PHPVersion::PHP84);
        let mut references = SymbolReferences::new();
        populate_codebase(&mut codebase, &mut references, Default::default(), Default::default());
        let session = plugins.create_external_analysis_session([Arc::clone(&file)]).unwrap();
        let analyzer = Analyzer::new(&arena, &file, &names, &codebase, &plugins, Settings::new(PHPVersion::PHP84))
            .with_external_analysis_session(&session);
        let mut result = AnalysisResult::new(references);
        let artifacts = analyzer.analyze_with_artifacts(program, &mut result).unwrap();
        for (call, expected) in [
            ("$signature->PrOvIdEd()", "int(73)"),
            ("$object->provided()", "int(73)"),
            ("ProviderTarget::provided()", "int(73)"),
            ("$object->magic()", "int(73)"),
            ("$object->real()", "int(91)"),
            ("$callable()", "int(91)"),
            ("$object->withoutProvider()", "string"),
        ] {
            let start = code.find(call).unwrap() as u32;
            let span = Span::new(file.id, Position::new(start), Position::new(start + call.len() as u32));
            let ty = artifacts.get_expression_type(&span).unwrap();
            assert_eq!(ty.get_id().as_str(), Some(expected), "{call}");
            assert!(!ty.is_error(), "the fallback must not replace {call}");
        }
        assert_eq!(
            *lookup_names.0.lock().unwrap(),
            [("SignatureTarget::provided".to_string(), "provided".to_string(), "SignatureTarget".to_string())],
        );
    }

    test_analysis! {
        name = nullsafe_method_call_on_null,
        code = indoc! {r#"
            <?php

            declare(strict_types=1);

            interface WriteInterface
            {
                /**
                 * @param non-empty-string $data
                 */
                public function write(string $data): void;
            }

            function get_writer(): null|WriteInterface
            {
                return null;
            }

            function write_line(string $message): void
            {
                $message = $message . "\n";

                get_writer()?->write($message);
            }
        "#}
    }

    test_analysis! {
        name = possible_method_call_on_null,
        code = indoc! {r#"
            <?php

            declare(strict_types=1);

            interface WriteInterface
            {
                /**
                 * @param non-empty-string $data
                 */
                public function write(string $data): void;
            }

            function get_writer(): null|WriteInterface
            {
                return null;
            }

            function write_line(string $message): void
            {
                $message = $message . "\n";

                get_writer()->write($message);
            }
        "#},
        issues = [
            IssueCode::PossibleMethodAccessOnNull
        ]
    }

    test_analysis! {
        name = method_call_on_mixed,
        code = indoc! {r#"
            <?php

            declare(strict_types=1);

            function get_mixed(): mixed
            {
                return "Hello, World!";
            }

            function call_method_on_mixed(): void
            {
                $mixed = get_mixed();
                $mixed->someMethod();
            }
        "#},
        issues = [
            IssueCode::MixedAssignment,
            IssueCode::MixedMethodAccess
        ]
    }

    test_analysis! {
        name = method_call_on_undefined_variable,
        code = indoc! {"
            <?php

            declare(strict_types=1);

            function method_call_on_undefined_variable(): void
            {
                $mixed->someMethod();
            }
        "},
        issues = [
            IssueCode::UndefinedVariable,
            IssueCode::MixedMethodAccess
        ]
    }

    test_analysis! {
        name = method_call_on_non_object,
        code = indoc! {"
            <?php

            declare(strict_types=1);

            function call_method_on_non_object(): void
            {
                $non_object = 42;
                $non_object->someMethod();
            }
        "},
        issues = [
            IssueCode::InvalidMethodAccess
        ]
    }

    test_analysis! {
        name = method_call_on_generic_parameter,
        code = indoc! {"
            <?php

            class A
            {
                public function getString(): string
                {
                    return 'Hello, world!';
                }
            }

            class B
            {
                public function getString(): string
                {
                    return 'Hello, world!';
                }
            }

            /**
             * @template T of A|B
             *
             * @param T $object
             */
            function foo(A|B $object): string
            {
                return $object->getString();
            }
        "},
    }

    test_analysis! {
        name = ambiguous_object_method_call,
        code = indoc! {"
            <?php

            declare(strict_types=1);

            function call_ambiguous_method(object $obj): void
            {
                $obj->someMethod();
            }
        "},
        issues = [
            IssueCode::AmbiguousObjectMethodAccess
        ]
    }

    test_analysis! {
        name = template_resolution,
        code = indoc! {"
            <?php

            /**
             * @template-covariant T
             */
            interface TypeInterface
            {
                /**
                 * @param mixed $value
                 * @return T
                 */
                public function assert(mixed $value): mixed;
            }

            /**
             * @param TypeInterface<non-empty-string> $type
             *
             * @return string
             */
            function to_string(mixed $value, TypeInterface $type): string
            {
                return $type->assert($value);
            }
        "},
    }

    test_analysis! {
        name = intersection_read_write_calls,
        code = indoc! {"
            <?php

            interface ReadHandle {
                public function read(): string;
            }

            interface WriteHandle {
                public function write(string $data): void;
            }

            /**
             * @template T as array-key
             * @param iterable<T, ReadHandle&WriteHandle> $handles
             * @return array<T, string>
             */
            function task(iterable $handles): array {
                $result = [];
                foreach ($handles as $index => $handle) {
                    $data = $handle->read();
                    $handle->write($data);

                    $result[$index] = $data;
                }
                return $result;
            }
        "},
    }

    test_analysis! {
        name = intersection_template_resolution,
        code = indoc! {"
            <?php

            interface MockObject
            {
            }

            abstract class TestCase
            {
                /**
                 * @template T of object
                 *
                 * @param class-string<T> $className
                 *
                 * @return MockObject&T
                 */
                protected function createMock(string $className): MockObject
                {
                    exit('Not implemented');
                }

                /**
                 * @template T of object
                 *
                 * @param class-string<T> $className
                 *
                 * @return T&MockObject
                 */
                protected function createMockTwo(string $className): MockObject
                {
                    exit('Not implemented');
                }
            }

            interface ServiceInterface
            {
            }

            class MyTestCase extends TestCase
            {
                private null|(MockObject&ServiceInterface) $service = null;

                public function setup(): void
                {
                    $this->service = $this->createMock(ServiceInterface::class);
                    $this->service = $this->createMockTwo(ServiceInterface::class);
                }
            }
        "},
        issues = [
            IssueCode::WriteOnlyProperty,
        ]
    }

    test_analysis! {
        name = trait_method_access,
        code = indoc! {r#"
            <?php

            trait A {
                private function x(): void {
                    echo "hello 1";
                }

                protected function y(): void {
                    echo "hello 2";
                }
            }

            class B {
                use A;

                public function c(): void {
                    $this->x();
                    $this->y();
                }
            }

            new B()->c();
        "#},
    }

    test_analysis! {
        name = calling_method_on_parent_class,
        code = indoc! {"
            <?php

            /**
             * @template TKey of array-key
             * @template-covariant T
             */
            interface ReadableCollection
            {
                /**
                 * @return list<T>
                 */
                public function getValues(): array;
            }

            /**
             * @template TKey of array-key
             * @template T
             *
             * @template-extends ReadableCollection<TKey, T>
             */
            interface Collection extends ReadableCollection
            {
            }

            class Filing
            {
            }

            class Storage
            {
                /**
                 * @var Collection<string, Filing>
                 */
                private $filings;

                /**
                 * @param Collection<string, Filing> $filings
                 */
                public function __construct(Collection $filings)
                {
                    $this->filings = $filings;
                }

                /**
                 * @return list<Filing>
                 */
                public function getFilings(): array
                {
                    return $this->filings->getValues();
                }
            }
        "},
        issues = [
            // ReadableCollection: TKey not used in interface body
            IssueCode::UnusedTemplateParameter,
        ]
    }

    test_analysis! {
        name = where_constraints,
        code = indoc! {"
            <?php

            interface Stringable
            {
                public function __toString(): string;
            }

            function take_string(string $s): void
            {
                take_string($s);
            }

            function take_int(int $i): void
            {
                take_int($i);
            }

            function take_array(array $arr): void
            {
                take_array($arr);
            }

            /** @param scalar|Stringable $value */
            function take_scalar_or_stringable(mixed $value): void
            {
                take_scalar_or_stringable($value);
            }

            final class Message implements Stringable
            {
                public function __construct(
                    private string $message,
                ) {}

                public function __toString(): string
                {
                    return $this->message;
                }
            }

            /**
             * @template-covariant T
             */
            final class Box
            {
                /**
                 * @param T $value
                 */
                public function __construct(
                    public mixed $value,
                ) {}

                /**
                 * @where T is string|int|float|Stringable
                 */
                public function toString(): string
                {
                    take_scalar_or_stringable($this->value);

                    return (string) $this->value;
                }

                /**
                 * @template Y
                 * @template Z
                 *
                 * @where T is list{Y, Z}
                 *
                 * @return list{Box<Y>, Box<Z>}
                 */
                public function unzip(): array
                {
                    take_array($this->value);

                    [$first, $second] = $this->value;

                    return [
                        new Box($first),
                        new Box($second),
                    ];
                }
            }

            $a = new Box('Hello, World!');
            take_string($a->toString()); // OK

            $b = new Box(42);
            take_string($b->toString()); // OK

            $c = new Box(3.14);
            take_string($c->toString()); // OK

            $d = new Box(new Message('This is a message.'));
            take_string($d->toString()); // OK

            $f = new Box(['foo', 123]);
            [$g, $h] = $f->unzip(); // OK

            take_string($g->value); // OK
            take_int($h->value); // OK
        "},
    }

    test_analysis! {
        name = where_constraints_violation,
        code = indoc! {"
            <?php

            /**
             * @template-covariant T
             */
            final class Box
            {
                /**
                 * @param T $value
                 */
                public function __construct(
                    public mixed $value,
                ) {}

                /**
                 * @where T is string|int|float
                 */
                public function toString(): string
                {
                    return (string) $this->value;
                }
            }

            $a = new Box(['foo', 123]);
            $a->toString(); // violation of @where constraint
        "},
        issues = [
            IssueCode::WhereConstraintViolation
        ]
    }
}
