use mago_allocator::Arena;
use std::borrow::Cow;
use std::rc::Rc;

use foldhash::HashMap;
use foldhash::HashSet;
use foldhash::fast::RandomState;

use mago_bytes::BytesDisplay;
use mago_bytes::trim_start_byte;
use mago_codex::identifier::function_like::FunctionLikeIdentifier;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::ttype::TType;
use mago_codex::ttype::TypeRef;
use mago_codex::ttype::add_union_type;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::callable::TCallableSignature;
use mago_codex::ttype::atomic::callable::parameter::TCallableParameter;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
use mago_codex::ttype::atomic::scalar::string::TString;
use mago_codex::ttype::atomic::scalar::string::TStringLiteral;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::expander;
use mago_codex::ttype::expander::StaticClassType;
use mago_codex::ttype::expander::TypeExpansionOptions;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::intersect_union_types;
use mago_codex::ttype::template::TemplateResult;
use mago_codex::ttype::template::inferred_type_replacer;
use mago_codex::ttype::union::TUnion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_reporting::IssueCollection;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::Expression;
use mago_word::Word;
use mago_word::WordMap;
use mago_word::WordSet;
use mago_word::concat_word;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::artifacts::ClosureBindScope;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;
use crate::invocation::Invocation;
use crate::invocation::InvocationArgument;
use crate::invocation::InvocationArgumentsSource;
use crate::invocation::InvocationTarget;
use crate::invocation::InvocationTargetParameter;
use crate::invocation::arguments::analyze_and_store_argument_type;
use crate::invocation::arguments::get_unpacked_argument_type;
use crate::invocation::arguments::verify_argument_type;
use crate::invocation::resolver::resolve_invocation_type;
use crate::invocation::template_inference::infer_parameter_templates_from_argument;
use crate::invocation::template_inference::infer_parameter_templates_from_default;
use crate::invocation::template_result::get_class_template_parameters_from_result;
use crate::invocation::template_result::populate_template_result_from_invocation;
use crate::invocation::template_result::refine_template_result_for_function_like;
use crate::plugin::context::HookContext;
use crate::plugin::context::InvocationInfo;
use crate::utils::expression::get_block_expression_id;

fn narrow_class_related_argument<A>(
    context: &Context<'_, '_, A>,
    block_context: &BlockContext<'_>,
    target: &InvocationTarget<'_>,
    class_related_argument_variables: Option<&WordSet>,
    expression: &Expression<'_>,
    argument_type: &TUnion,
) -> Option<TUnion>
where
    A: Arena,
{
    let class_related_argument_variables = class_related_argument_variables?;
    let argument_variable = get_block_expression_id(expression, context, block_context)?;
    if !class_related_argument_variables.contains(&argument_variable) {
        return None;
    }

    let class_type = &target.get_method_context()?.class_type;
    let receiver_type = match class_type {
        StaticClassType::Object(object) => TUnion::from_atomic(TAtomic::Object(object.clone())),
        StaticClassType::Exact(name) | StaticClassType::Name(name) => {
            TUnion::from_atomic(TAtomic::Object(TObject::new_named(*name)))
        }
        StaticClassType::Generic(parameter) => parameter.constraint.as_ref().clone(),
        StaticClassType::None => return None,
    };

    intersect_union_types(argument_type, &receiver_type, context.codebase)
}

/// Finds the parameter that corresponds to a given argument.
fn get_parameter_of_argument<'target, 'ctx>(
    target: &'target InvocationTarget<'ctx>,
    argument: &InvocationArgument<'_, '_>,
    mut argument_offset: usize,
) -> Option<(usize, InvocationTargetParameter<'target>)>
where
    'ctx: 'target,
{
    // Handle both named arguments and named placeholders
    if let Some(parameter_name) = argument.get_parameter_name() {
        argument_offset = find_named_parameter_offset(target, parameter_name)?;
    }

    argument_offset = adjust_offset_for_variadic(target, argument_offset);
    target.get_parameter(argument_offset).map(|parameter| (argument_offset, parameter))
}

/// Finds the offset of a named parameter in the parameter list.
fn find_named_parameter_offset<'target, 'ctx>(
    target: &'target InvocationTarget<'ctx>,
    argument_name: &[u8],
) -> Option<usize>
where
    'ctx: 'target,
{
    let argument_variable_name = concat_word!("$", argument_name);
    target
        .iter_parameters()
        .position(|parameter| parameter.get_name().is_some_and(|param_name| argument_variable_name == param_name.0))
        .or_else(|| {
            let offset = target.parameter_count().checked_sub(1)?;
            target.get_parameter(offset)?.is_variadic().then_some(offset)
        })
}

/// Adjusts argument offset for variadic parameters.
fn adjust_offset_for_variadic(target: &InvocationTarget<'_>, argument_offset: usize) -> usize {
    let parameter_count = target.parameter_count();
    if parameter_count > 0
        && argument_offset >= parameter_count
        && target.get_parameter(parameter_count - 1).is_some_and(|parameter| parameter.is_variadic())
    {
        parameter_count - 1
    } else {
        argument_offset
    }
}

/// PHPStan gathers a callback's return before sibling arguments specialize its input.
/// The normal walk below remains authoritative for body types and argument verification.
fn gather_raw_callback<'ctx, 'arena, A: Arena>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &BlockContext<'ctx>,
    target: &InvocationTarget<'ctx>,
    expression: &Expression<'arena>,
    parameter_type: &TUnion,
    metadata: &FunctionLikeMetadata,
) -> Result<Option<TUnion>, AnalysisError> {
    // ClosureTypeResolver::getClosureType() stops its speculative body walks at depth two.
    if context.raw_callback_gather_depth >= 2 {
        let mut declared = metadata.clone();
        declared.return_type_metadata.clone_from(&declared.return_type_declaration_metadata);
        for parameter in &mut declared.parameters {
            parameter.type_metadata.clone_from(&parameter.type_declaration_metadata);
        }
        let signature = expander::get_signature_of_function_like_metadata(
            &FunctionLikeIdentifier::for_closure(context.source_file, expression.span()),
            &declared,
            context.codebase,
            &TypeExpansionOptions {
                self_class: block_context.scope.get_class_like_name(),
                static_class_type: block_context
                    .scope
                    .get_class_like_name()
                    .map_or(StaticClassType::None, StaticClassType::Name),
                ..Default::default()
            },
        );
        return Ok(Some(TUnion::from_atomic(TAtomic::Callable(TCallable::Signature(signature)))));
    }

    let mut block = block_context.clone();
    // These are shared RefCells in BlockContext::clone(), not captured callback state.
    block.if_body_context = None;
    block.finally_scope = None;
    let mut scratch = AnalysisArtifacts::new().with_native_types(context.settings.track_native_types);
    context.gather_callback(|context| {
        let call_context =
            crate::observations::argument_context(context, None, Some(parameter_type), expression.span());
        analyze_and_store_argument_type(
            context,
            &mut block,
            &mut scratch,
            target,
            expression,
            usize::MAX,
            &mut [],
            false,
            Some(parameter_type),
            call_context,
            None,
            None,
        )?;
        Ok(scratch.get_expression_type(expression).cloned())
    })
}

/// Analyzes and verifies arguments passed to a function, method, or callable.
pub fn analyze_invocation<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    invocation: &mut Invocation<'ctx, '_, 'arena>,
    calling_class_like: Option<(Word, Option<&TAtomic>)>,
    class_related_argument_variables: Option<&WordSet>,
    template_result: &mut TemplateResult,
    parameter_types: &mut WordMap<TUnion>,
) -> Result<(), AnalysisError>
where
    A: Arena,
{
    let mut arguments = EvaluatedArguments::new(invocation.arguments_source.argument_count());
    analyze_invocation_with_arguments(
        context,
        block_context,
        artifacts,
        invocation,
        calling_class_like,
        class_related_argument_variables,
        template_result,
        parameter_types,
        &mut arguments,
    )
}

/// Physical operand values belong to the source call. Callee-local argument
/// narrowing, binding and template inference must not mutate this cache.
pub(crate) struct EvaluatedArguments<'ctx> {
    types: Vec<Option<(Rc<TUnion>, Span)>>,
    targets: Vec<InvocationTarget<'ctx>>,
    templates: TemplateResult,
    has_walk_reflection: bool,
    walk_target: Option<InvocationTarget<'ctx>>,
    entered: bool,
    expression_issues: Option<IssueCollection>,
    parameter_contexts: Vec<Option<TUnion>>,
    raw_callbacks: Vec<Option<TUnion>>,
    walk_binding: Option<(TemplateResult, WordMap<TUnion>)>,
}

impl<'ctx> EvaluatedArguments<'ctx> {
    pub(crate) fn new(count: usize) -> Self {
        Self {
            types: vec![None; count],
            targets: Vec::new(),
            templates: TemplateResult::default(),
            has_walk_reflection: true,
            walk_target: None,
            entered: false,
            expression_issues: None,
            parameter_contexts: vec![None; count],
            raw_callbacks: vec![None; count],
            walk_binding: None,
        }
    }

    pub(crate) fn for_targets<A: Arena>(
        context: &Context<'ctx, '_, A>,
        count: usize,
        mut targets: Vec<InvocationTarget<'ctx>>,
        templates: TemplateResult,
        has_walk_reflection: bool,
    ) -> Self {
        // Method resolution uses a LIFO receiver stack. PHPStan's combined
        // reflection retains the receiver union's original member order.
        if targets.iter().all(|target| target.get_method_context().is_some()) {
            targets.reverse();
        }
        let walk_target = has_walk_reflection.then(|| combine_walk_reflection(context, &targets)).flatten();
        Self { targets, templates, has_walk_reflection, walk_target, ..Self::new(count) }
    }

    /// ArgumentsHandler owns the physical walk, independently of the semantic
    /// target loop. Only a statically named call supplies a walk reflection.
    /// Combined-signature verification is discarded; real operand diagnostics
    /// are retained, and every original callee is verified below from the cache.
    pub(crate) fn evaluate<'arena, A: Arena>(
        &mut self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
        source: InvocationArgumentsSource<'_, 'arena>,
        span: Span,
        class_related_argument_variables: Option<&WordSet>,
    ) -> Result<(), AnalysisError> {
        if self.targets.is_empty() || (self.has_walk_reflection && self.targets.len() == 1) {
            return Ok(());
        }
        let target = self.walk_target.clone().unwrap_or_else(|| InvocationTarget::Callable {
            source: None,
            signature: TCallableSignature::new(false, false),
            effective_signature: None,
            span,
        });
        let mut invocation = Invocation::new(target, source, span);
        if context.plugin_registry.has_expression_hooks() {
            let mut hook_context = HookContext::new(context.codebase, context.source_file, block_context, artifacts);
            hook_context.raw_callback_gather = context.raw_callback_gather_depth > 0;
            context.plugin_registry.before_arguments(&InvocationInfo::new(&invocation), &mut hook_context)?;
            for reported in hook_context.take_issues() {
                context.collector.report_with_code(reported.code, reported.issue);
            }
        }
        self.entered = true;
        self.expression_issues = Some(IssueCollection::new());
        let mut templates = self.templates.clone();
        let mut parameters = WordMap::default();
        let (result, _) = context.record(|context| {
            analyze_invocation_with_arguments(
                context,
                block_context,
                artifacts,
                &mut invocation,
                None,
                class_related_argument_variables,
                &mut templates,
                &mut parameters,
                self,
            )
        });
        context.collector.extend(self.expression_issues.take().unwrap());
        result?;
        self.walk_binding = Some((templates, parameters));
        Ok(())
    }

    pub(crate) fn has_evaluated(&self, source: &InvocationArgumentsSource<'_, '_>) -> bool {
        source
            .iter_arguments()
            .enumerate()
            .all(|(offset, argument)| argument.value().is_none() || self.types[offset].is_some())
    }

    pub(crate) fn remember(&mut self, source: &InvocationArgumentsSource<'_, '_>, artifacts: &AnalysisArtifacts) {
        for (offset, expression) in source
            .iter_arguments()
            .enumerate()
            .filter_map(|(offset, argument)| argument.value().map(|expression| (offset, expression)))
        {
            let ty = artifacts.get_rc_expression_type(expression).cloned().unwrap_or_else(|| Rc::new(get_mixed()));
            self.types[offset] = Some((ty, expression.span()));
        }
    }

    fn reference_context(&self, argument: &InvocationArgument<'_, '_>, offset: usize) -> Option<bool> {
        (!self.targets.is_empty()).then(|| {
            self.walk_target.as_ref().is_some_and(|target| {
                if argument.is_unpacked() {
                    target
                        .parameter_count()
                        .checked_sub(1)
                        .and_then(|offset| target.get_parameter(offset))
                        .is_some_and(|parameter| parameter.is_variadic() && parameter.is_by_reference())
                } else {
                    get_parameter_of_argument(target, argument, offset)
                        .is_some_and(|(_, parameter)| parameter.is_by_reference())
                }
            })
        })
    }

    fn closure_parameter_type(&self, offset: usize, single_target_type: Option<&TUnion>) -> Option<TUnion> {
        if !self.has_walk_reflection {
            return None;
        }
        self.parameter_contexts[offset].as_ref().or(single_target_type).cloned()
    }

    /// MethodCallReturnTypeHelper selects a dynamic name's generic acceptor on
    /// beforeScope. Those are type queries, not another physical operand walk:
    /// each query owns its scope and artifacts, and publishes no live events.
    pub(crate) fn resolve_return_arguments<'arena, A: Arena>(
        &self,
        context: &mut Context<'ctx, 'arena, A>,
        before: &BlockContext<'ctx>,
        artifacts: &AnalysisArtifacts,
        invocation: &Invocation<'ctx, '_, 'arena>,
        templates: &mut TemplateResult,
        parameters: &mut WordMap<TUnion>,
    ) -> Result<(), AnalysisError> {
        let mut scratch = artifacts.clone();
        scratch.loop_scope = None;
        scratch.case_scopes.clear();
        let mut values = Self::new(invocation.arguments_source.argument_count());
        values.entered = true;
        let saved_observations = std::mem::take(&mut context.observations);
        let constants = context.constant_types.clone();
        let blocks = context.control_body_blocks.clone();
        let ensured = context.ensured_non_null_types.clone();
        let consume = context.consume_stored_expression_results;
        context.observations.pricing = true;
        let result = crate::plugin::registry::with_isolated_type_pricing(|| {
            context.gather_callback(|context| {
                for expression in invocation.arguments_source.iter_arguments().filter_map(|argument| argument.value()) {
                    let mut scope = before.clone();
                    scope.if_body_context = None;
                    scope.finally_scope = None;
                    expression.analyze(context, &mut scope, &mut scratch)?;
                }
                values.remember(&invocation.arguments_source, &scratch);
                let mut scope = before.clone();
                scope.if_body_context = None;
                scope.finally_scope = None;
                let mut invocation = invocation.clone();
                analyze_invocation_with_arguments(
                    context,
                    &mut scope,
                    &mut scratch,
                    &mut invocation,
                    None,
                    None,
                    templates,
                    parameters,
                    &mut values,
                )
            })
        });
        context.observations = saved_observations;
        context.constant_types = constants;
        context.control_body_blocks = blocks;
        context.ensured_non_null_types = ensured;
        context.consume_stored_expression_results = consume;
        result
    }

    /// Resolve the single physical writeback contract from the gathered operands.
    /// This pass binds cached values only. It never evaluates an argument or
    /// dispatches a return provider, and does not apply any candidate's OUT write.
    pub(crate) fn apply_writeback<'arena, A: Arena>(
        &mut self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
        source: InvocationArgumentsSource<'_, 'arena>,
        span: Span,
    ) -> Result<(), AnalysisError> {
        let Some(target) = self
            .walk_target
            .clone()
            .filter(|target| target.iter_parameters().any(|parameter| parameter.is_by_reference()))
        else {
            return Ok(());
        };
        let mut invocation = Invocation::new(target, source, span);
        let positional_writeback = source.iter_arguments().any(|argument| {
            argument
                .get_parameter_name()
                .is_some_and(|name| find_named_parameter_offset(&invocation.target, name).is_none())
        });
        if let Some((templates, parameters)) = &self.walk_binding {
            return crate::invocation::post_process::update_by_reference_argument_types(
                context,
                block_context,
                artifacts,
                &invocation,
                templates,
                parameters,
                positional_writeback,
            );
        }
        let mut templates = self.templates.clone();
        let mut parameters = WordMap::default();
        let (result, _) = context.record(|context| {
            analyze_invocation_with_arguments(
                context,
                block_context,
                artifacts,
                &mut invocation,
                None,
                None,
                &mut templates,
                &mut parameters,
                self,
            )
        });
        result?;
        crate::invocation::post_process::update_by_reference_argument_types(
            context,
            block_context,
            artifacts,
            &invocation,
            &templates,
            &parameters,
            positional_writeback,
        )
    }
}

/// ParametersAcceptorSelector::combineAcceptors. Only the physical argument
/// contract is combined. Every original target keeps its own return provider.
fn combine_walk_reflection<'ctx, A: Arena>(
    context: &Context<'ctx, '_, A>,
    targets: &[InvocationTarget<'ctx>],
) -> Option<InvocationTarget<'ctx>> {
    let mut target = targets.first()?.clone();
    if targets.len() == 1 {
        return Some(target);
    }
    let minimum = targets
        .iter()
        .map(|target| {
            target.iter_parameters().filter(|parameter| !parameter.has_default() && !parameter.is_variadic()).count()
        })
        .min()
        .unwrap();
    let mut parameters: Vec<TCallableParameter> = Vec::new();
    for candidate in targets {
        let method = candidate.get_method_context();
        for (offset, parameter) in candidate.iter_parameters().enumerate() {
            let expand = |ty: &TUnion| {
                resolve_type_in_class_context(
                    context,
                    ty.clone(),
                    method.map(|method| method.class_like_metadata),
                    None,
                    None,
                    method.map(|method| &method.class_type),
                )
            };
            let ty = parameter.get_type().map_or_else(get_mixed, expand);
            let mut combined = TCallableParameter::new(
                Some(std::sync::Arc::new(ty)),
                parameter.is_by_reference(),
                parameter.is_variadic(),
                offset >= minimum,
            )
            .with_name(parameter.get_name().copied())
            .with_out_type(parameter.get_out_type().map(expand).map(std::sync::Arc::new))
            .with_closure_this_type(parameter.get_closure_this_type().map(expand).map(std::sync::Arc::new));
            if let Some(previous) = parameters.get(offset) {
                let union = |left: &TUnion, right: &TUnion| {
                    add_union_type(left.clone(), right, context.codebase, CombinerOptions::default())
                };
                let mut ty = union(previous.get_type_signature().unwrap(), combined.get_type_signature().unwrap());
                let out = previous.get_out_type().zip(combined.get_out_type()).map(|(left, right)| union(left, right));
                let closure_this = previous
                    .get_closure_this_type()
                    .zip(combined.get_closure_this_type())
                    .map(|(left, right)| union(left, right));
                let variadic = previous.is_variadic() || combined.is_variadic();
                if variadic {
                    for dropped in parameters.iter().skip(offset + 1) {
                        ty = union(&ty, dropped.get_type_signature().unwrap());
                    }
                    for dropped in candidate.iter_parameters().skip(offset + 1) {
                        ty = union(&ty, &dropped.get_type().map_or_else(get_mixed, expand));
                    }
                }
                let name = previous.get_name().zip(combined.get_name()).map(|(left, right)| {
                    if left == right {
                        *left
                    } else {
                        mago_codex::misc::VariableIdentifier(concat_word!(
                            left.0,
                            "|",
                            trim_start_byte(right.0.as_bytes(), b'$')
                        ))
                    }
                });
                combined = TCallableParameter::new(
                    Some(std::sync::Arc::new(ty)),
                    previous.is_by_reference() || combined.is_by_reference(),
                    variadic,
                    offset >= minimum,
                )
                .with_name(name)
                .with_out_type(out.map(std::sync::Arc::new))
                .with_closure_this_type(closure_this.map(std::sync::Arc::new));
                parameters[offset] = combined;
                if variadic {
                    parameters.truncate(offset + 1);
                    break;
                }
            } else {
                parameters.push(combined);
            }
        }
    }
    target.set_effective_signature(crate::invocation::EffectiveCallableSignature {
        parameters,
        allows_named_arguments: true,
        display_name: None,
    });
    Some(target)
}

pub(crate) fn analyze_invocation_with_arguments<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    invocation: &mut Invocation<'ctx, '_, 'arena>,
    calling_class_like: Option<(Word, Option<&TAtomic>)>,
    class_related_argument_variables: Option<&WordSet>,
    template_result: &mut TemplateResult,
    parameter_types: &mut WordMap<TUnion>,
    arguments: &mut EvaluatedArguments<'ctx>,
) -> Result<(), AnalysisError>
where
    A: Arena,
{
    let first_analysis = !arguments.entered;
    arguments.entered = true;
    if arguments.targets.is_empty()
        && context.external_analysis_session.is_some()
        && !invocation.target.has_effective_signature()
        && let Some(identifier) = invocation.target.get_function_like_identifier().copied()
        && context.plugin_registry.may_have_callable_signature_provider(&identifier)
    {
        apply_callable_signature(context, artifacts, &identifier, invocation);
    }

    if !context.settings.allow_side_effects_in_conditions
        && block_context.flags.inside_conditional()
        && !invocation.target.is_pure_or_mutation_free()
    {
        context.collector.report_with_code(
            IssueCode::SideEffectsInCondition,
            Issue::warning("Impure function call inside condition.")
                .with_annotation(
                    Annotation::primary(invocation.span)
                        .with_message("This call is not marked `@pure` or `@mutation-free` and may have side effects"),
                )
                .with_note("Side effects inside conditions can silently alter variables used elsewhere in the same expression.")
                .with_note("PHP's evaluation order may not match reading order, leading to surprising behavior.")
                .with_help(
                    "Extract the call into a variable before the condition, or annotate the function with `@pure` or `@mutation-free` if it has no side effects.",
                ),
        );
    }

    populate_template_result_from_invocation(context, invocation, template_result);

    if first_analysis && context.plugin_registry.has_expression_hooks() {
        let mut hook_context = HookContext::new(context.codebase, context.source_file, block_context, artifacts);
        hook_context.raw_callback_gather = context.raw_callback_gather_depth > 0;
        context.plugin_registry.before_arguments(&InvocationInfo::new(invocation), &mut hook_context)?;
        for reported in hook_context.take_issues() {
            context.collector.report_with_code(reported.code, reported.issue);
        }
    }

    let arg_count = invocation.arguments_source.argument_count();
    let mut analyzed_argument_types = arguments.types.clone();

    let mut non_closure_arguments = Vec::with_capacity(arg_count);
    let mut closure_arguments = Vec::new();
    let mut unpacked_arguments = Vec::new();

    // Small calls cost less to scan than to index.
    let named_parameter_offsets = (arg_count >= 32
        && invocation.target.parameter_count() >= 32
        && invocation
            .arguments_source
            .iter_arguments()
            .filter(|argument| argument.get_parameter_name().is_some())
            .take(32)
            .count()
            == 32)
        .then(|| {
            let mut offsets =
                HashMap::with_capacity_and_hasher(invocation.target.parameter_count(), RandomState::default());
            for (offset, parameter) in invocation.target.iter_parameters().enumerate() {
                if let Some(name) = parameter.get_name().and_then(|name| name.0.as_bytes().strip_prefix(b"$")) {
                    offsets.entry(name).or_insert(offset);
                }
            }
            offsets
        });

    for (offset, argument) in invocation.arguments_source.iter_arguments().enumerate() {
        if argument.is_unpacked() {
            unpacked_arguments.push(argument);
        } else {
            let parameter = if let Some(offsets) = &named_parameter_offsets
                && let Some(name) = argument.get_parameter_name()
            {
                offsets
                    .get(name)
                    .and_then(|offset| invocation.target.get_parameter(*offset).map(|parameter| (*offset, parameter)))
            } else {
                get_parameter_of_argument(&invocation.target, &argument, offset)
            };
            if matches!(argument.value(), Some(Expression::Closure(_) | Expression::ArrowFunction(_))) {
                closure_arguments.push((offset, parameter));
            } else {
                non_closure_arguments.push((offset, parameter));
            }
        }
    }

    drop(named_parameter_offsets);

    let has_unpacked_arguments = !unpacked_arguments.is_empty();
    // The callee reads its extra arguments through `func_get_args()` and friends.
    let reads_extra_arguments =
        invocation.target.get_function_like_metadata().is_some_and(|metadata| metadata.flags.uses_func_get_args());

    let calling_class_like_metadata =
        calling_class_like.and_then(|(id, _)| context.codebase.get_class_like(id.as_bytes()));
    let method_call_context = invocation.target.get_method_context();
    let base_class_metadata = method_call_context.map(|ctx| ctx.class_like_metadata).or(calling_class_like_metadata);
    let calling_instance_type = calling_class_like.and_then(|(_, atomic)| atomic);
    let method_class_type = method_call_context.map(|ctx| &ctx.class_type);

    // Like PHPStan's ArgumentsHandler, defer closures but evaluate every other
    // operand once in its written order, including unpacked operands.
    let mut non_closure_parameters = non_closure_arguments.iter().peekable();
    for (offset, argument) in invocation.arguments_source.iter_arguments().enumerate() {
        if argument.is_unpacked()
            && let Some(argument_expression) = argument.value()
        {
            let referenced_parameter = invocation
                .target
                .parameter_count()
                .checked_sub(1)
                .and_then(|offset| invocation.target.get_parameter(offset))
                .is_some_and(|parameter| parameter.is_variadic() && parameter.is_by_reference());
            analyze_and_store_argument_type(
                context,
                block_context,
                artifacts,
                &invocation.target,
                argument_expression,
                offset,
                &mut analyzed_argument_types,
                referenced_parameter,
                None,
                None,
                arguments.reference_context(&argument, offset),
                arguments.expression_issues.as_mut(),
            )?;
            arguments.types[offset].clone_from(&analyzed_argument_types[offset]);
            continue;
        }

        let Some((argument_offset, parameter)) =
            non_closure_parameters.next_if(|(argument_offset, _)| *argument_offset == offset)
        else {
            continue;
        };
        let Some(argument_expression) = argument.value() else {
            continue;
        };

        let parameter = *parameter;

        let observation_type = context.observation_program.and_then(|_| {
            parameter.map(|(_, parameter)| {
                get_parameter_type(
                    context,
                    Some(parameter),
                    base_class_metadata,
                    calling_class_like_metadata,
                    calling_instance_type,
                    method_class_type,
                )
            })
        });
        let call_context = crate::observations::argument_context(
            context,
            Some(&invocation.target),
            observation_type.as_ref(),
            argument_expression.span(),
        );
        // ArgumentsHandler passes an array literal argument its parameter type (enterPassedToType()), whose
        // callables type the closures among the array's items
        let passed_to_type = if matches!(
            crate::utils::misc::unwrap_expression(argument_expression),
            Expression::Array(_) | Expression::LegacyArray(_)
        ) {
            parameter
                .map(|(_, parameter)| {
                    get_parameter_type(
                        context,
                        Some(parameter),
                        base_class_metadata,
                        calling_class_like_metadata,
                        calling_instance_type,
                        method_class_type,
                    )
                })
                .and_then(|parameter_type| closure_parameter_bounds(context, Some(&parameter_type), template_result))
        } else {
            None
        };
        analyze_and_store_argument_type(
            context,
            block_context,
            artifacts,
            &invocation.target,
            argument_expression,
            *argument_offset,
            &mut analyzed_argument_types,
            parameter.is_some_and(|p| p.1.is_by_reference()),
            passed_to_type.as_ref(),
            call_context,
            arguments.reference_context(&argument, *argument_offset),
            arguments.expression_issues.as_mut(),
        )?;
        arguments.types[*argument_offset].clone_from(&analyzed_argument_types[*argument_offset]);

        if let Some((argument_type, _)) = &mut analyzed_argument_types[*argument_offset]
            && let Some(narrowed_type) = narrow_class_related_argument(
                context,
                block_context,
                &invocation.target,
                class_related_argument_variables,
                argument_expression,
                argument_type,
            )
        {
            *argument_type = Rc::new(narrowed_type);
        }

        if let Some(argument_type) = &analyzed_argument_types[*argument_offset]
            && let Some((_, parameter_ref)) = parameter
            && parameter_ref.get_type().is_some_and(|ty| ty.is_expandable() || ty.has_template_types())
        {
            let parameter_type = get_parameter_type(
                context,
                Some(parameter_ref),
                base_class_metadata,
                calling_class_like_metadata,
                calling_instance_type,
                method_class_type,
            );

            if parameter_type.has_template_types() {
                infer_parameter_templates_from_argument(
                    context,
                    &parameter_type,
                    &argument_type.0,
                    template_result,
                    *argument_offset,
                    argument_type.1,
                    false,
                );
            }
        }
    }

    for (argument_offset, parameter) in &non_closure_arguments {
        let Some(argument) = invocation.arguments_source.get_argument(*argument_offset) else {
            continue;
        };
        let Some((_, parameter)) = *parameter else {
            continue;
        };

        let Some(parameter_name) = parameter.get_name() else {
            continue;
        };

        let argument_type = if let Some((argument_type, _)) = &analyzed_argument_types[*argument_offset] {
            (**argument_type).clone()
        } else if argument.is_placeholder() {
            TUnion::from_atomic(TAtomic::Variable(parameter_name.0))
        } else {
            continue;
        };

        parameter_types.insert(parameter_name.0, argument_type);
    }

    if arguments.expression_issues.is_some() && arguments.targets.len() > 1 {
        // GenericParametersAcceptorResolver's combined acceptor keeps the first
        // declaring scope for a colliding template name, not one bound per callee.
        for (name, bounds) in &mut template_result.lower_bounds {
            if let Some(first) = template_result.template_types.get(name).and_then(|types| types.first()) {
                bounds.retain(|scope, _| *scope == first.defining_entity);
            }
        }
    }

    let global_closure_bind_scope = detect_closure_bind_scope(context, artifacts, invocation, &analyzed_argument_types);
    let previous_bind_scope = artifacts.closure_bind_scope.take();

    for (argument_offset, parameter) in &closure_arguments {
        let Some(argument) = invocation.arguments_source.get_argument(*argument_offset) else {
            continue;
        };
        let Some(argument_expression) = argument.value() else {
            continue;
        };

        let parameter = *parameter;
        let mut parameter_type_had_template_types = false;
        // Store the original unreplaced type for template inference, so closure return types
        // can contribute bounds even after other arguments have already inferred some bounds.
        // See: https://github.com/carthage-software/mago/issues/755
        let mut base_parameter_type_for_inference: Option<TUnion> = None;
        let parameter_type = if let Some((_, parameter_ref)) = parameter {
            let base_parameter_type = get_parameter_type(
                context,
                Some(parameter_ref),
                base_class_metadata,
                calling_class_like_metadata,
                calling_instance_type,
                method_class_type,
            );

            let has_parameter_variable = contains_parameter_variable(&base_parameter_type);
            let mut parameter_type = if base_parameter_type.has_template_types() {
                parameter_type_had_template_types = true;
                // Store the original type before replacement for inference
                base_parameter_type_for_inference = Some(base_parameter_type.clone());
                inferred_type_replacer::replace_with_polarity(
                    &base_parameter_type,
                    template_result,
                    context.codebase,
                    mago_codex::ttype::template::variance::Variance::Contravariant,
                )
            } else {
                base_parameter_type
            };

            if has_parameter_variable {
                parameter_type =
                    resolve_invocation_type(context, invocation, template_result, parameter_types, parameter_type);
            } else if parameter_type.is_expandable() {
                expander::expand_union(
                    context.codebase,
                    &mut parameter_type,
                    &TypeExpansionOptions {
                        self_class: base_class_metadata.map(|metadata| metadata.name),
                        ..Default::default()
                    },
                );
            }

            Some(parameter_type)
        } else {
            None
        };

        let parameter_type = if let Some(mut pt) = parameter_type {
            filter_array_filter_callback_type(&invocation.target, &mut pt, &analyzed_argument_types);
            Some(pt)
        } else {
            None
        };
        let parameter_type = parameter
            .and_then(|(_, parameter)| parameter.get_name())
            .and_then(|name| {
                context.plugin_registry.get_function_parameter_type(
                    context.codebase,
                    context.source_file,
                    block_context,
                    artifacts,
                    invocation,
                    name.0.as_bytes(),
                    true,
                )
            })
            .or(parameter_type);

        // A `@param-closure-this T` parameter binds `$this` to `T` inside the closure literal.
        // It takes precedence over any global `Closure::bind` scope for this argument.
        artifacts.closure_bind_scope = parameter
            .and_then(|(_, p)| p.get_closure_this_type())
            .and_then(|closure_this_type| {
                // Resolve `self`/`static` and class-level context, then bind any method templates
                // (e.g. a trait template `T` mapped to `self<...>` via `@use`).
                let mut resolved = resolve_type_in_class_context(
                    context,
                    closure_this_type.clone(),
                    base_class_metadata,
                    calling_class_like_metadata,
                    calling_instance_type,
                    method_class_type,
                );

                if resolved.has_template_types() {
                    resolved = inferred_type_replacer::replace_with_polarity(
                        &resolved,
                        template_result,
                        context.codebase,
                        mago_codex::ttype::template::variance::Variance::Invariant,
                    );
                }

                closure_this_bind_scope(&resolved)
            })
            .or_else(|| global_closure_bind_scope.clone());

        let raw_callback_type = if arguments.types[*argument_offset].is_some() {
            arguments.raw_callbacks[*argument_offset].clone()
        } else if let Some(base_type) = &base_parameter_type_for_inference
            && matches!(&invocation.target, InvocationTarget::FunctionLike { metadata, effective_signature: None, .. }
                if metadata.flags.is_user_defined()
                    && !metadata.return_type_metadata.as_ref().is_some_and(|t| contains_parameter_variable(&t.type_union)))
            && context.external_analysis_session.is_none()
            && !has_unpacked_arguments
            && artifacts.closure_bind_scope.is_none()
            && !contains_parameter_variable(base_type)
            && matches!(base_type.types.as_ref(), [TAtomic::Callable(TCallable::Signature(_))])
            && let Some(metadata) = context.codebase.get_closure_at(context.source_file, argument_expression.span())
            // Mago drops unresolved mixed-bound templates for untyped parameters. Do not
            // pretend that walking those callbacks under mixed preserves PHPStan's T -> T.
            && metadata.parameters.iter().all(|p| p.type_declaration_metadata.as_ref().is_some_and(|t| !t.type_union.is_mixed()))
            && metadata.template_types.is_empty()
        {
            gather_raw_callback(context, block_context, &invocation.target, argument_expression, base_type, metadata)?
        } else {
            None
        };

        // GenericParametersAcceptorResolver selects a reflection from the argument's
        // stored type before processing it. Do the same for the pricing context only.
        let mut observation_type = context.observation_program.and_then(|_| parameter_type.clone());
        if context.observations.pricing
            && let Some(base_type) = &base_parameter_type_for_inference
            && let Some(argument_type) = crate::observations::stored_type(context, argument_expression.span())
        {
            let mut observation_templates = template_result.clone();
            infer_parameter_templates_from_argument(
                context,
                base_type,
                &argument_type,
                &mut observation_templates,
                *argument_offset,
                argument_expression.span(),
                false,
            );
            observation_type = observation_type
                .map(|ty| resolve_invocation_type(context, invocation, &observation_templates, parameter_types, ty));
        }
        let physical_parameter_type = arguments.closure_parameter_type(*argument_offset, parameter_type.as_ref());
        if arguments.types[*argument_offset].is_none() {
            arguments.parameter_contexts[*argument_offset].clone_from(&physical_parameter_type);
            arguments.raw_callbacks[*argument_offset].clone_from(&raw_callback_type);
        }
        let call_context = crate::observations::argument_context(
            context,
            Some(&invocation.target),
            if arguments.targets.len() > 1 { physical_parameter_type.as_ref() } else { observation_type.as_ref() },
            argument_expression.span(),
        );
        analyze_and_store_argument_type(
            context,
            block_context,
            artifacts,
            &invocation.target,
            argument_expression,
            *argument_offset,
            &mut analyzed_argument_types,
            parameter.is_some_and(|p| p.1.is_by_reference()),
            closure_parameter_bounds(context, physical_parameter_type.as_ref(), template_result).as_ref(),
            call_context,
            arguments.reference_context(&argument, *argument_offset),
            arguments.expression_issues.as_mut(),
        )?;
        arguments.types[*argument_offset].clone_from(&analyzed_argument_types[*argument_offset]);

        // Use the original unreplaced type for inference to allow closure return types
        // to contribute bounds that can widen literal types from other arguments.
        if parameter_type_had_template_types
            && let Some(argument_type) = &analyzed_argument_types[*argument_offset]
            && let Some(base_type) = &base_parameter_type_for_inference
        {
            infer_parameter_templates_from_argument(
                context,
                base_type,
                raw_callback_type.as_ref().unwrap_or(argument_type.0.as_ref()),
                template_result,
                *argument_offset,
                argument_type.1,
                true,
            );
        }
    }

    for (argument_offset, parameter) in &closure_arguments {
        let Some((_, parameter)) = *parameter else {
            continue;
        };
        let Some(parameter_name) = parameter.get_name() else {
            continue;
        };
        let Some((argument_type, _)) = &analyzed_argument_types[*argument_offset] else {
            continue;
        };

        parameter_types.insert(parameter_name.0, (**argument_type).clone());
    }

    for parameter in invocation.target.iter_parameters() {
        let Some(parameter_name) = parameter.get_name() else {
            continue;
        };
        if parameter_types.contains_key(&parameter_name.0) || !parameter.has_default() {
            continue;
        }

        parameter_types.insert(parameter_name.0, parameter.get_default_type().cloned().unwrap_or_else(get_mixed));
    }

    // Restore the previous closure bind scope after analyzing closure arguments
    artifacts.closure_bind_scope = previous_bind_scope;

    if !unpacked_arguments.is_empty()
        && invocation
            .target
            .parameter_count()
            .checked_sub(1)
            .and_then(|offset| invocation.target.get_parameter(offset))
            .is_some_and(|parameter| !parameter.is_variadic())
    {
        let mut current_parameter_position = 0;
        for argument in invocation.arguments_source.iter_arguments() {
            if argument.is_unpacked() {
                let Some(argument_expression) = argument.value() else {
                    continue;
                };

                let argument_value_type =
                    artifacts.get_expression_type(argument_expression).cloned().unwrap_or_else(get_mixed);
                populate_parameter_types_from_unpacked(
                    context,
                    &argument_value_type,
                    &invocation.target,
                    current_parameter_position,
                    parameter_types,
                );

                current_parameter_position += argument_value_type
                    .types
                    .iter()
                    .filter_map(|atomic| match atomic {
                        TAtomic::Array(array) => Some(array.get_minimum_size()),
                        _ => None,
                    })
                    .min()
                    .unwrap_or(0);
            } else {
                current_parameter_position += 1;
            }
        }
    }

    if let Some(function_like_metadata) = invocation.target.get_function_like_metadata() {
        let class_generic_parameters = get_class_template_parameters_from_result(template_result, context);
        refine_template_result_for_function_like(
            template_result,
            context,
            method_call_context,
            base_class_metadata,
            calling_class_like_metadata,
            function_like_metadata,
            &class_generic_parameters,
        );
    }

    let mut assigned_parameters_by_name = HashMap::default();
    let mut assigned_parameters_by_position = HashMap::default();
    let mut filled_parameter_offsets: HashSet<usize> = HashSet::default();

    let target_kind_str = invocation.target.guess_kind();
    let mut has_too_many_arguments = false;
    let mut has_named_argument_anomaly = false;
    let mut last_argument_offset: isize = -1;
    let mut non_closure_index = 0;
    let mut closure_index = 0;

    while non_closure_index < non_closure_arguments.len() || closure_index < closure_arguments.len() {
        let (argument_offset, parameter_ref) =
            match (non_closure_arguments.get(non_closure_index), closure_arguments.get(closure_index)) {
                (Some(non_closure), Some(closure)) if non_closure.0 <= closure.0 => {
                    non_closure_index += 1;
                    non_closure
                }
                (Some(_), Some(closure)) => {
                    closure_index += 1;
                    closure
                }
                (Some(non_closure), None) => {
                    non_closure_index += 1;
                    non_closure
                }
                (None, Some(closure)) => {
                    closure_index += 1;
                    closure
                }
                (None, None) => break,
            };

        let Some(argument) = invocation.arguments_source.get_argument(*argument_offset) else {
            continue;
        };

        let Some(argument_expression) = argument.value() else {
            continue;
        };

        let (argument_value_type, _) = analyzed_argument_types[*argument_offset]
            .clone()
            .unwrap_or_else(|| (Rc::new(get_mixed()), argument_expression.span()));

        if let Some((parameter_offset, parameter_ref)) = *parameter_ref {
            filled_parameter_offsets.insert(parameter_offset);

            if let Some(named_argument) = argument.get_named_argument() {
                let named_value_display = BytesDisplay(named_argument.name.value);
                if let Some(previous_span) = assigned_parameters_by_name.get(&parameter_offset) {
                    has_named_argument_anomaly = true;
                    let target_name_str = invocation.target.guess_name(context);
                    context.collector.report_with_code(
                        IssueCode::DuplicateNamedArgument,
                        Issue::error(format!(
                            "Duplicate named argument `${}` in call to {} `{}`.",
                            named_value_display, target_kind_str, target_name_str
                        ))
                        .with_annotation(
                            Annotation::primary(named_argument.name.span()).with_message("Duplicate argument name"),
                        )
                        .with_annotation(
                            Annotation::secondary(*previous_span)
                                .with_message("Argument previously provided by name here"),
                        )
                        .with_help("Remove one of the duplicate named arguments."),
                    );
                } else if let Some(previous_span) = assigned_parameters_by_position.get(&parameter_offset) {
                    has_named_argument_anomaly = true;
                    let target_name_str = invocation.target.guess_name(context);
                    if parameter_ref.is_variadic() {
                        context.collector.report_with_code(
                            IssueCode::NamedArgumentAfterPositional,
                             Issue::warning(format!(
                                "Named argument `${}` for {} `{}` targets a variadic parameter that has already captured positional arguments.",
                                named_value_display, target_kind_str, target_name_str
                            ))
                            .with_annotation(Annotation::primary(named_argument.name.span()).with_message("Named argument for variadic parameter"))
                            .with_annotation(Annotation::secondary(*previous_span).with_message("Positional arguments already captured by variadic here"))
                            .with_note("Mixing positional and named arguments for the same variadic parameter can be confusing and may lead to unexpected behavior depending on PHP version and argument unpacking.")
                            .with_help("Consider providing all arguments for the variadic parameter either positionally or via unpacking a named array."),
                        );
                    } else {
                        context.collector.report_with_code(
                            IssueCode::NamedArgumentOverridesPositional,
                            Issue::error(format!(
                                "Named argument `${}` for {} `{}` targets a parameter already provided positionally.",
                                named_value_display, target_kind_str, target_name_str
                            ))
                            .with_annotation(
                                Annotation::primary(named_argument.name.span()).with_message("This named argument"),
                            )
                            .with_annotation(
                                Annotation::secondary(*previous_span)
                                    .with_message("Parameter already filled by positional argument here"),
                            )
                            .with_help("Provide the argument either positionally or by name, but not both."),
                        );
                    }
                    assigned_parameters_by_name.insert(parameter_offset, named_argument.name.span());
                } else {
                    assigned_parameters_by_name.insert(parameter_offset, named_argument.name.span());
                }
            } else {
                assigned_parameters_by_position.insert(parameter_offset, argument.span());
            }

            let base_parameter_type = get_parameter_type(
                context,
                Some(parameter_ref),
                base_class_metadata,
                calling_class_like_metadata,
                calling_instance_type,
                method_class_type,
            );

            let final_parameter_type = resolve_parameter_type_for_invocation(
                context,
                invocation,
                template_result,
                parameter_types,
                base_class_metadata,
                base_parameter_type,
            );

            let check_is_deferred_to_partial_invocation =
                matches!(invocation.arguments_source, InvocationArgumentsSource::PartialArgumentList(_))
                    && contains_parameter_variable(&final_parameter_type);

            if !check_is_deferred_to_partial_invocation {
                verify_argument_type(
                    context,
                    &argument_value_type,
                    &final_parameter_type,
                    *argument_offset,
                    argument_expression,
                    &invocation.target,
                );
            }

            if let Some(parameter_name) = parameter_ref.get_name() {
                parameter_types.insert(parameter_name.0, (*argument_value_type).clone());
            }
        } else if let Some(named_argument) = argument.get_named_argument() {
            let argument_name = BytesDisplay(named_argument.name.value);

            let has_variadic_parameter = invocation
                .target
                .parameter_count()
                .checked_sub(1)
                .and_then(|idx| invocation.target.get_parameter(idx))
                .is_some_and(|parameter| parameter.is_variadic());

            if !has_variadic_parameter {
                has_named_argument_anomaly = true;
                let target_name_str = invocation.target.guess_name(context);

                context.collector.report_with_code(
                    IssueCode::InvalidNamedArgument,
                    Issue::error(format!(
                        "Invalid named argument `${argument_name}` for {target_kind_str} `{target_name_str}`"
                    ))
                    .with_annotation(
                        Annotation::primary(named_argument.name.span())
                            .with_message(format!("Unknown argument name `${argument_name}`")),
                    )
                    .with_annotation(
                        Annotation::secondary(invocation.target.span())
                            .with_message(format!("Call to {target_kind_str} is here")),
                    )
                    .with_help({
                        if !invocation.target.allows_named_arguments() {
                            format!("The {target_kind_str} `{target_name_str}` does not support named arguments.")
                        } else if invocation.target.parameter_count() == 0 {
                            format!("The {target_kind_str} `{target_name_str}` has no parameters.")
                        } else {
                            let available_params: Vec<String> = invocation
                                .target
                                .iter_parameters()
                                .filter_map(|parameter| parameter.get_name())
                                .map(|n| {
                                    let bytes = trim_start_byte(n.0.as_bytes(), b'$');
                                    BytesDisplay(bytes).to_string()
                                })
                                .collect();
                            format!("Available parameters are: `{}`.", available_params.join("`, `"))
                        }
                    }),
                );
            }

            break;
        } else if *argument_offset >= invocation.target.parameter_count() {
            has_too_many_arguments = true;
            continue;
        }

        last_argument_offset = *argument_offset as isize;
    }

    let function_template_types = invocation.target.get_template_types();

    // Apply @template T2 = T1 defaults for templates not yet bound from actual arguments.
    // This must run before unused-parameter PHP-default inference so that explicit template
    // defaults take priority (e.g., $x = null must not override T2 = T1 when T1 = string).
    if let Some(template_types) = function_template_types {
        for (template_name, template) in template_types {
            if template_result.has_lower_bound(*template_name, &template.defining_entity) {
                continue;
            }
            if let Some(default) = &template.default {
                let resolved = inferred_type_replacer::replace(default, template_result, context.codebase);
                if !resolved.has_template_types() {
                    template_result.add_lower_bound(*template_name, template.defining_entity, resolved);
                }
            }
        }
    }

    let can_infer_templates_from_default = !matches!(
        &invocation.target,
        InvocationTarget::FunctionLike { metadata, effective_signature: None, .. }
            if metadata.template_types.is_empty()
                && !template_result.has_template_types()
                && template_result.lower_bounds.is_empty()
    );

    if !has_too_many_arguments {
        loop {
            last_argument_offset += 1;
            if last_argument_offset as usize >= invocation.target.parameter_count() {
                break;
            }

            let Some(unused_parameter) = invocation.target.get_parameter(last_argument_offset as usize) else {
                break;
            };

            if !unused_parameter.has_default() {
                continue;
            }

            let mut default_type =
                unused_parameter.get_default_type().map_or_else(|| Cow::Owned(get_mixed()), Cow::Borrowed);

            if default_type.is_expandable() {
                expander::expand_union(context.codebase, default_type.to_mut(), &TypeExpansionOptions::default());
            }

            if can_infer_templates_from_default {
                // Resolving a declared parameter type can expand combinatorial types such as `int-mask-of`.
                // A non-generic static target cannot infer templates from omitted defaults, so skip this
                // expansion while still resolving and storing the default value type below.
                let parameter_type = get_parameter_type(
                    context,
                    Some(unused_parameter),
                    base_class_metadata,
                    calling_class_like_metadata,
                    calling_instance_type,
                    method_class_type,
                );

                infer_parameter_templates_from_default(context, &parameter_type, &default_type, template_result);
            }

            let Some(parameter_name) = unused_parameter.get_name() else {
                continue;
            };

            if parameter_types.contains_key(&parameter_name.0) {
                continue;
            }

            parameter_types.insert(parameter_name.0, default_type.into_owned());
        }
    }

    if !unpacked_arguments.is_empty()
        && let Some(last_parameter_offset) = invocation.target.parameter_count().checked_sub(1)
        && let Some(last_parameter_ref) = invocation.target.get_parameter(last_parameter_offset)
        && last_parameter_ref.is_variadic()
    {
        let base_variadic_parameter_type = get_parameter_type(
            context,
            Some(last_parameter_ref),
            base_class_metadata,
            calling_class_like_metadata,
            calling_instance_type,
            method_class_type,
        );

        if base_variadic_parameter_type.has_template_types() {
            for unpacked_argument in &unpacked_arguments {
                let Some(argument_expression) = unpacked_argument.value() else {
                    continue;
                };

                let argument_value_type =
                    artifacts.get_expression_type(argument_expression).cloned().unwrap_or_else(get_mixed);

                let unpacked_element_type =
                    get_unpacked_argument_type(context, &argument_value_type, argument_expression.span());

                infer_parameter_templates_from_argument(
                    context,
                    &base_variadic_parameter_type,
                    &unpacked_element_type,
                    template_result,
                    last_parameter_offset,
                    argument_expression.span(),
                    false,
                );
            }
        }
    }

    if let Some(template_types) = function_template_types {
        for (template_name, template) in template_types {
            if template_result.has_lower_bound(*template_name, &template.defining_entity) {
                continue;
            }

            let fallback = if let Some(default) = &template.default {
                inferred_type_replacer::replace(default, template_result, context.codebase)
            } else {
                template.constraint.clone()
            };

            template_result.add_lower_bound(*template_name, template.defining_entity, fallback);
        }
    }

    infer_generic_callable_arguments(
        context,
        invocation,
        template_result,
        parameter_types,
        &analyzed_argument_types,
        base_class_metadata,
        calling_class_like_metadata,
        calling_instance_type,
        method_class_type,
    );

    let max_params = invocation.target.parameter_count();
    let number_of_required_parameters = invocation
        .target
        .iter_parameters()
        .filter(|parameter| !parameter.has_default() && !parameter.is_variadic())
        .count();
    let number_of_satisfied_required_parameters = invocation
        .target
        .iter_parameters()
        .enumerate()
        .filter(|(offset, parameter)| {
            !parameter.has_default() && !parameter.is_variadic() && filled_parameter_offsets.contains(offset)
        })
        .count();
    let mut number_of_provided_parameters = non_closure_arguments.len() + closure_arguments.len();

    if !unpacked_arguments.is_empty() {
        if let Some(last_parameter_offset) = invocation.target.parameter_count().checked_sub(1)
            && let Some(last_parameter_ref) = invocation.target.get_parameter(last_parameter_offset)
        {
            if last_parameter_ref.is_variadic() {
                let base_variadic_parameter_type = get_parameter_type(
                    context,
                    Some(last_parameter_ref),
                    base_class_metadata,
                    calling_class_like_metadata,
                    calling_instance_type,
                    method_class_type,
                );

                let final_variadic_parameter_type =
                    if template_result.has_template_types() || !template_result.lower_bounds.is_empty() {
                        let mut replaced_type = inferred_type_replacer::replace(
                            &base_variadic_parameter_type,
                            template_result,
                            context.codebase,
                        );

                        if replaced_type.is_expandable() {
                            expander::expand_union(
                                context.codebase,
                                &mut replaced_type,
                                &TypeExpansionOptions {
                                    self_class: base_class_metadata.map(|meta| meta.name),
                                    ..Default::default()
                                },
                            );
                        }

                        replaced_type
                    } else {
                        base_variadic_parameter_type
                    };

                for unpacked_argument in unpacked_arguments {
                    let Some(argument_expression) = unpacked_argument.value() else {
                        continue;
                    };

                    let argument_value_type =
                        artifacts.get_expression_type(argument_expression).cloned().unwrap_or_else(get_mixed); // Get type of the iterable

                    let (minimum_unpacked_size, has_keyed_array_with_named_args) = argument_value_type
                        .types
                        .as_ref()
                        .iter()
                        .fold((None::<usize>, false), |(minimum_size, mut has_named_args), argument_atomic| {
                            let size = if let TAtomic::Array(array) = argument_atomic {
                                if !has_named_args
                                    && let mago_codex::ttype::atomic::array::TArray::Keyed(keyed_array) = array
                                    && let Some(known_items) = &keyed_array.known_items
                                {
                                    has_named_args = known_items.iter().any(|(array_key, _)| {
                                        matches!(array_key, mago_codex::ttype::atomic::array::key::ArrayKey::String(_))
                                    });
                                }

                                array.get_minimum_size()
                            } else {
                                0
                            };

                            (Some(minimum_size.map_or(size, |current| current.min(size))), has_named_args)
                        });

                    number_of_provided_parameters += minimum_unpacked_size.unwrap_or(0);

                    // Skip union-based validation for keyed arrays with named arguments.
                    // Individual elements are properly validated in validate_keyed_array_elements.
                    if !has_keyed_array_with_named_args {
                        let unpacked_element_type =
                            get_unpacked_argument_type(context, &argument_value_type, argument_expression.span());

                        verify_argument_type(
                            context,
                            &unpacked_element_type,
                            &final_variadic_parameter_type,
                            last_parameter_offset,
                            argument_expression,
                            &invocation.target,
                        );
                    }
                }
            } else {
                let mut current_parameter_position = 0;

                for argument in invocation.arguments_source.iter_arguments() {
                    if argument.is_unpacked() {
                        let Some(argument_expression) = argument.value() else {
                            continue;
                        };

                        // Nonvariadic unpacks were evaluated before template inference.
                        // A semantic hook's missing type is not permission to replay it.
                        let argument_value_type =
                            artifacts.get_expression_type(argument_expression).cloned().unwrap_or_else(get_mixed);

                        // Count the number of elements that would be unpacked
                        let unpacked_count = argument_value_type
                            .types
                            .as_ref()
                            .iter()
                            .fold(None::<usize>, |minimum_size, argument_atomic| {
                                let size = if let TAtomic::Array(array) = argument_atomic {
                                    array.get_minimum_size()
                                } else {
                                    0
                                };

                                Some(minimum_size.map_or(size, |current| current.min(size)))
                            })
                            .unwrap_or(0);
                        number_of_provided_parameters += unpacked_count;
                        let target_name_str = invocation.target.guess_name(context);

                        validate_unpacked_argument_elements(
                            context,
                            &argument_value_type,
                            argument_expression,
                            base_class_metadata,
                            calling_class_like_metadata,
                            calling_instance_type,
                            method_class_type,
                            invocation,
                            template_result,
                            parameter_types,
                            current_parameter_position,
                            target_kind_str,
                            &target_name_str,
                        );

                        current_parameter_position += unpacked_count;
                    } else {
                        current_parameter_position += 1;
                    }
                }
            }
        } else if !unpacked_arguments.is_empty() && !reads_extra_arguments {
            context.collector.report_with_code(
                IssueCode::TooManyArguments,
                Issue::error(format!(
                    "Cannot unpack arguments into {} `{}` which expects no arguments.",
                    invocation.target.guess_kind(),
                    invocation.target.guess_name(context)
                ))
                .with_annotation(
                    Annotation::primary(unpacked_arguments[0].span()).with_message("Unexpected argument unpacking"),
                )
                .with_help("Remove the argument unpacking (`...`)."),
            );
        }
    }

    if let InvocationTarget::Callable { signature, .. } = &invocation.target {
        for constraint in signature.constraints.iter() {
            let Some((argument_offset, argument_expression)) =
                invocation.arguments_source.iter_arguments().enumerate().find_map(|(argument_offset, argument)| {
                    let (_, parameter) = get_parameter_of_argument(&invocation.target, &argument, argument_offset)?;
                    let parameter_name = parameter.get_name()?;
                    if !constraint.parameter_names.iter().any(|name| name == parameter_name) {
                        return None;
                    }

                    Some((argument_offset, argument.value()?))
                })
            else {
                continue;
            };

            let input_type = resolve_invocation_type(
                context,
                invocation,
                template_result,
                parameter_types,
                (*constraint.input_type).clone(),
            );
            let parameter_type = resolve_invocation_type(
                context,
                invocation,
                template_result,
                parameter_types,
                (*constraint.parameter_type).clone(),
            );

            verify_argument_type(
                context,
                &input_type,
                &parameter_type,
                argument_offset,
                argument_expression,
                &invocation.target,
            );
        }
    }

    let should_check_argument_count = match invocation.arguments_source {
        InvocationArgumentsSource::PartialArgumentList(_) => {
            (non_closure_arguments.len() + closure_arguments.len()) < number_of_required_parameters
        }
        _ if has_unpacked_arguments || has_named_argument_anomaly => {
            number_of_provided_parameters < number_of_required_parameters
        }
        _ => number_of_satisfied_required_parameters < number_of_required_parameters,
    };

    if should_check_argument_count {
        let primary_annotation_span = invocation.arguments_source.span();
        let target_name_str = invocation.target.guess_name(context);

        let main_message = match invocation.arguments_source {
            InvocationArgumentsSource::PipeInput(_) => format!(
                "Too few arguments for {target_kind_str} `{target_name_str}` when used with the pipe operator `|>`. Pipe provides 1, but at least {number_of_required_parameters} required."
            ),
            _ => format!("Too few arguments provided for {target_kind_str} `{target_name_str}`."),
        };

        let total_positions = non_closure_arguments.len() + closure_arguments.len();
        let mut issue = Issue::error(main_message)
            .with_annotation(Annotation::primary(primary_annotation_span).with_message("More arguments expected here"))
            .with_note(format!(
                "Expected at least {number_of_required_parameters} argument(s) for non-optional parameters, but received {}.",
                if matches!(invocation.arguments_source, InvocationArgumentsSource::PartialArgumentList(_)) {
                    total_positions
                } else if has_unpacked_arguments || has_named_argument_anomaly {
                    number_of_provided_parameters
                } else {
                    number_of_satisfied_required_parameters
                }
            ));

        issue = match invocation.arguments_source {
            InvocationArgumentsSource::ArgumentList(_)
            | InvocationArgumentsSource::PartialArgumentList(_)
            | InvocationArgumentsSource::AttributeArgumentList(_) => issue.with_annotation(
                Annotation::secondary(invocation.target.span())
                    .with_message(format!("For this {target_kind_str} call")),
            ),
            InvocationArgumentsSource::PipeInput(pipe) => issue
                .with_annotation(Annotation::secondary(pipe.callable.span()).with_message(format!(
                    "This {target_kind_str} requires at least {number_of_required_parameters} argument(s)",
                )))
                .with_annotation(
                    Annotation::secondary(pipe.input.span()).with_message("This value is passed as the first argument"),
                ),
            InvocationArgumentsSource::None(constructor_or_attribute_span) => issue.with_annotation(
                Annotation::secondary(constructor_or_attribute_span)
                    .with_message(format!("For this {target_kind_str}")),
            ),
        };

        issue = issue.with_help("Provide all required arguments.");
        context.collector.report_with_code(IssueCode::TooFewArguments, issue);
    } else if !reads_extra_arguments
        && (has_too_many_arguments
            || (!invocation
                .target
                .parameter_count()
                .checked_sub(1)
                .and_then(|idx| invocation.target.get_parameter(idx))
                .is_some_and(|parameter| parameter.is_variadic())
                && number_of_provided_parameters > max_params
                && max_params > 0))
    {
        let target_name_str = invocation.target.guess_name(context);
        let first_extra_arg_span = invocation
            .arguments_source
            .get_argument(max_params)
            .map_or_else(|| invocation.arguments_source.span(), |argument| argument.span());

        let main_message = match invocation.arguments_source {
            InvocationArgumentsSource::PipeInput(_) => format!(
                "The {target_kind_str} `{target_name_str}` used with pipe operator `|>` expects 0 arguments, but 1 (the piped value) is provided."
            ),
            _ => format!("Too many arguments provided for {target_kind_str} `{target_name_str}`."),
        };

        let mut issue = Issue::error(main_message).with_annotation(
            Annotation::primary(first_extra_arg_span).with_message("Unexpected argument provided here"),
        );

        issue = match invocation.arguments_source {
            InvocationArgumentsSource::PipeInput(pipe) => issue
                .with_annotation(
                    Annotation::secondary(pipe.callable.span())
                        .with_message(format!("This {target_kind_str} expects 0 arguments")),
                )
                .with_annotation(
                    Annotation::secondary(pipe.operator).with_message("Pipe operator provides this as an argument"),
                ),
            _ => issue.with_annotation(
                Annotation::secondary(invocation.target.span())
                    .with_message(format!("For this {target_kind_str} call")),
            ),
        };

        issue = issue
            .with_note(format!("Expected {max_params} argument(s), but received {number_of_provided_parameters}."))
            .with_help("Remove the extra argument(s).");

        context.collector.report_with_code(IssueCode::TooManyArguments, issue);
    }

    if context.observation_program.is_some() {
        for (offset, argument) in invocation.arguments_source.iter_arguments().enumerate() {
            let Some(expression) = argument.value() else { continue };
            let parameter =
                get_parameter_of_argument(&invocation.target, &argument, offset).filter(|_| !argument.is_unpacked());
            let parameter_type = parameter.map(|(_, parameter)| {
                let ty = get_parameter_type(
                    context,
                    Some(parameter),
                    base_class_metadata,
                    calling_class_like_metadata,
                    calling_instance_type,
                    method_class_type,
                );
                resolve_invocation_type(context, invocation, template_result, parameter_types, ty)
            });
            let parameter_type = if matches!(expression, Expression::Closure(_) | Expression::ArrowFunction(_)) {
                arguments.closure_parameter_type(offset, parameter_type.as_ref())
            } else {
                parameter_type
            };
            if let Some(resolved) = crate::observations::argument_context(
                context,
                Some(&invocation.target),
                parameter_type.as_ref(),
                expression.span(),
            ) {
                crate::observations::settle_argument(context, &resolved);
            }
        }
    }
    Ok(())
}

/// Installs a provider-supplied signature for an explicit callable identity.
///
pub(crate) fn apply_callable_signature<'ctx, 'arena, A>(
    context: &Context<'ctx, 'arena, A>,
    artifacts: &AnalysisArtifacts,
    identifier: &FunctionLikeIdentifier,
    invocation: &mut Invocation<'ctx, '_, 'arena>,
) -> bool
where
    A: Arena,
{
    let Some(signature) = context.plugin_registry.get_function_like_callable_signature(
        context.codebase,
        context.source_file,
        artifacts,
        identifier,
        invocation,
        context.external_analysis_session,
    ) else {
        return false;
    };

    invocation.target.set_effective_signature(signature);
    true
}

/// Instantiates first-class generic callables from the surrounding call's other arguments.
#[allow(clippy::too_many_arguments)]
fn infer_generic_callable_arguments<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    invocation: &Invocation<'ctx, '_, 'arena>,
    template_result: &mut TemplateResult,
    parameter_types: &WordMap<TUnion>,
    analyzed_argument_types: &[Option<(Rc<TUnion>, Span)>],
    base_class_metadata: Option<&'ctx ClassLikeMetadata>,
    calling_class_like_metadata: Option<&'ctx ClassLikeMetadata>,
    calling_instance_type: Option<&TAtomic>,
    method_class_type: Option<&StaticClassType>,
) where
    A: Arena,
{
    for (argument_offset, argument) in invocation.arguments_source.iter_arguments().enumerate() {
        let Some((argument_type, argument_span)) = &analyzed_argument_types[argument_offset] else {
            continue;
        };

        if !callable_argument_has_template_parameter(context, argument_type) {
            continue;
        }

        let Some((_, parameter)) = get_parameter_of_argument(&invocation.target, &argument, argument_offset) else {
            continue;
        };

        let base_parameter_type = get_parameter_type(
            context,
            Some(parameter),
            base_class_metadata,
            calling_class_like_metadata,
            calling_instance_type,
            method_class_type,
        );

        let mut result_without_callback_bounds = template_result.clone();
        remove_template_bounds_from_argument(&mut result_without_callback_bounds, argument_offset);

        let expected_parameter_type = resolve_parameter_type_for_invocation(
            context,
            invocation,
            &result_without_callback_bounds,
            parameter_types,
            base_class_metadata,
            base_parameter_type.clone(),
        );

        let expected_signatures = expected_parameter_type.types.iter().filter_map(|atomic| match atomic {
            TAtomic::Callable(TCallable::Signature(signature)) => Some(Cow::Borrowed(signature)),
            TAtomic::Callable(TCallable::Alias(identifier)) => {
                expander::get_signature_of_function_like_identifier(identifier, context.codebase).map(Cow::Owned)
            }
            _ => None,
        });

        for expected_signature in expected_signatures {
            for argument_atomic in argument_type.types.iter() {
                let input_signature: Cow<'_, TCallableSignature> = match argument_atomic {
                    TAtomic::Callable(TCallable::Signature(signature)) if signature.get_source().is_some() => {
                        Cow::Owned(signature.clone())
                    }
                    TAtomic::Callable(TCallable::Alias(identifier)) => {
                        let Some(signature) =
                            expander::get_signature_of_function_like_identifier(identifier, context.codebase)
                        else {
                            continue;
                        };

                        Cow::Owned(signature)
                    }
                    _ => continue,
                };

                let parameter_dependent_return =
                    input_signature.get_return_type().is_some_and(contains_parameter_variable);
                let mut callback_parameters = WordMap::default();
                if parameter_dependent_return {
                    // Replace the provisional callback bounds only after its inputs are known.
                    remove_template_bounds_from_argument(template_result, argument_offset);
                }

                for (input_parameter, expected_parameter) in
                    input_signature.get_parameters().iter().zip(expected_signature.get_parameters())
                {
                    let Some(input_type) = input_parameter.get_type_signature() else {
                        continue;
                    };
                    let Some(expected_type) = expected_parameter.get_type_signature() else {
                        continue;
                    };

                    if parameter_dependent_return && let Some(name) = input_parameter.get_name() {
                        callback_parameters.insert(name.0, expected_type.clone());
                    }

                    if !input_type.has_template_types() || expected_type.is_mixed() {
                        continue;
                    }

                    infer_parameter_templates_from_argument(
                        context,
                        input_type,
                        expected_type,
                        template_result,
                        argument_offset,
                        *argument_span,
                        false,
                    );
                }

                if parameter_dependent_return {
                    let return_type = resolve_invocation_type(
                        context,
                        invocation,
                        template_result,
                        &callback_parameters,
                        input_signature.get_return_type().unwrap().clone(),
                    );
                    let mut specialized_signature = input_signature.into_owned();
                    specialized_signature.return_type = Some(std::sync::Arc::new(return_type));
                    infer_parameter_templates_from_argument(
                        context,
                        &base_parameter_type,
                        &TUnion::from_atomic(TAtomic::Callable(TCallable::Signature(specialized_signature))),
                        template_result,
                        argument_offset,
                        *argument_span,
                        false,
                    );
                }
            }
        }
    }
}

fn callable_argument_has_template_parameter<A>(context: &Context<'_, '_, A>, argument_type: &TUnion) -> bool
where
    A: Arena,
{
    argument_type.types.iter().any(|atomic| match atomic {
        TAtomic::Callable(TCallable::Signature(signature)) if signature.get_source().is_some() => signature
            .get_parameters()
            .iter()
            .filter_map(TCallableParameter::get_type_signature)
            .any(TUnion::has_template_types),
        TAtomic::Callable(TCallable::Alias(identifier)) => {
            context.codebase.get_function_like(identifier).is_some_and(|metadata| {
                metadata
                    .parameters
                    .iter()
                    .filter_map(|parameter| parameter.get_type_metadata().map(|metadata| &metadata.type_union))
                    .any(TUnion::has_template_types)
            })
        }
        _ => false,
    })
}

fn remove_template_bounds_from_argument(template_result: &mut TemplateResult, argument_offset: usize) {
    template_result.lower_bounds.retain(|_, bounds_by_parent| {
        bounds_by_parent.retain(|_, bounds| {
            bounds.retain(|bound| bound.argument_offset != Some(argument_offset));
            !bounds.is_empty()
        });

        !bounds_by_parent.is_empty()
    });
}

/// Gets the effective parameter type, expanding class-relative types based on call context.
fn get_parameter_type<'ctx, A>(
    context: &Context<'ctx, '_, A>,
    invocation_target_parameter: Option<InvocationTargetParameter<'_>>,
    base_class_metadata: Option<&'ctx ClassLikeMetadata>,
    calling_class_like_metadata: Option<&'ctx ClassLikeMetadata>,
    calling_instance_type: Option<&TAtomic>,
    method_class_type: Option<&StaticClassType>,
) -> TUnion
where
    A: Arena,
{
    let Some(invocation_target_parameter) = invocation_target_parameter else {
        return get_mixed();
    };

    let parameter_type = invocation_target_parameter.get_type().cloned().unwrap_or_else(get_mixed);

    resolve_type_in_class_context(
        context,
        parameter_type,
        base_class_metadata,
        calling_class_like_metadata,
        calling_instance_type,
        method_class_type,
    )
}

/// Expands `self`/`static`/`parent` and class-level context in a type declared on a function-like,
/// relative to the class the invocation is dispatched against.
fn resolve_type_in_class_context<'ctx, A>(
    context: &Context<'ctx, '_, A>,
    ttype: TUnion,
    base_class_metadata: Option<&'ctx ClassLikeMetadata>,
    calling_class_like_metadata: Option<&'ctx ClassLikeMetadata>,
    calling_instance_type: Option<&TAtomic>,
    method_class_type: Option<&StaticClassType>,
) -> TUnion
where
    A: Arena,
{
    if !ttype.is_expandable() || contains_parameter_variable(&ttype) {
        return ttype;
    }

    let mut resolved_parameter_type = ttype;

    let static_class_type = method_class_type
        .filter(|t| !matches!(t, StaticClassType::None))
        .cloned()
        .or_else(|| {
            calling_class_like_metadata.map(|calling_meta| {
                calling_instance_type
                    .and_then(|instance| match instance {
                        TAtomic::Object(obj) => Some(StaticClassType::Object(obj.clone())),
                        _ => None,
                    })
                    .unwrap_or(StaticClassType::Name(calling_meta.name))
            })
        })
        .unwrap_or(StaticClassType::None);

    expander::expand_union(
        context.codebase,
        &mut resolved_parameter_type,
        &TypeExpansionOptions {
            self_class: base_class_metadata.map(|meta| meta.name),
            static_class_type,
            function_is_final: calling_class_like_metadata.is_some_and(|meta| meta.flags.is_final()),
            ..Default::default()
        },
    );

    resolved_parameter_type
}

fn contains_parameter_variable(union: &TUnion) -> bool {
    union.any_child_node(|node| {
        matches!(
            node,
            TypeRef::Atomic(TAtomic::Variable(variable))
                if !variable.as_bytes().eq_ignore_ascii_case(b"$this")
        )
    })
}

fn resolve_parameter_type_for_invocation<'ctx, 'arena, A>(
    context: &Context<'ctx, 'arena, A>,
    invocation: &Invocation<'ctx, '_, 'arena>,
    template_result: &TemplateResult,
    parameter_types: &WordMap<TUnion>,
    base_class_metadata: Option<&ClassLikeMetadata>,
    base_parameter_type: TUnion,
) -> TUnion
where
    A: Arena,
{
    let has_parameter_variable = contains_parameter_variable(&base_parameter_type);
    let has_template_bindings = template_result.has_template_types() || !template_result.lower_bounds.is_empty();
    let mut parameter_type = if has_template_bindings {
        inferred_type_replacer::replace_with_polarity(
            &base_parameter_type,
            template_result,
            context.codebase,
            mago_codex::ttype::template::variance::Variance::Contravariant,
        )
    } else {
        base_parameter_type
    };

    if has_parameter_variable {
        return resolve_invocation_type(context, invocation, template_result, parameter_types, parameter_type);
    }

    if has_template_bindings {
        expander::expand_union(
            context.codebase,
            &mut parameter_type,
            &TypeExpansionOptions {
                self_class: base_class_metadata.map(|metadata| metadata.name),
                ..Default::default()
            },
        );
    }

    parameter_type
}

/// Adds statically known unpacked elements to the invocation's parameter map.
fn populate_parameter_types_from_unpacked<A>(
    context: &Context<'_, '_, A>,
    argument_type: &TUnion,
    invocation_target: &InvocationTarget<'_>,
    starting_parameter_position: usize,
    parameter_types: &mut WordMap<TUnion>,
) where
    A: Arena,
{
    let alternative_count = argument_type.types.len();
    let mut candidate_types: WordMap<(TUnion, usize)> = WordMap::default();

    for atomic in argument_type.types.as_ref() {
        let mut branch_types: WordMap<(TUnion, bool)> = WordMap::default();
        let TAtomic::Array(array) = atomic else {
            continue;
        };

        match array {
            TArray::List(list) => {
                if let Some(known_elements) = &list.known_elements {
                    for (offset, (is_optional, element_type)) in known_elements.iter() {
                        insert_unpacked_parameter_type(
                            context,
                            invocation_target,
                            starting_parameter_position + offset,
                            element_type,
                            !*is_optional && !element_type.possibly_undefined(),
                            &mut branch_types,
                        );
                    }
                } else if let Some(known_count) = list.known_count {
                    for offset in 0..known_count {
                        insert_unpacked_parameter_type(
                            context,
                            invocation_target,
                            starting_parameter_position + offset,
                            list.get_element_type(),
                            true,
                            &mut branch_types,
                        );
                    }
                }
            }
            TArray::Keyed(keyed) => {
                let Some(known_items) = &keyed.known_items else {
                    continue;
                };

                for (key, (is_optional, element_type)) in known_items.iter() {
                    let parameter_offset = match key {
                        ArrayKey::Integer(offset) => {
                            usize::try_from(*offset).ok().map(|offset| starting_parameter_position + offset)
                        }
                        ArrayKey::String(name) => find_named_parameter_offset(invocation_target, name.as_bytes()),
                        ArrayKey::ClassLikeConstant { .. } => None,
                    };

                    if let Some(parameter_offset) = parameter_offset {
                        insert_unpacked_parameter_type(
                            context,
                            invocation_target,
                            parameter_offset,
                            element_type,
                            !*is_optional && !element_type.possibly_undefined(),
                            &mut branch_types,
                        );
                    }
                }
            }
        }

        for (parameter_name, (branch_type, definitely_supplied)) in branch_types {
            if let Some((candidate_type, supplied_count)) = candidate_types.get_mut(&parameter_name) {
                *candidate_type = add_union_type(
                    std::mem::replace(candidate_type, get_mixed()),
                    &branch_type,
                    context.codebase,
                    CombinerOptions::default(),
                );
                if definitely_supplied {
                    *supplied_count += 1;
                }
            } else {
                candidate_types.insert(parameter_name, (branch_type, usize::from(definitely_supplied)));
            }
        }
    }

    for (parameter_name, (candidate_type, supplied_count)) in candidate_types {
        if supplied_count == alternative_count {
            parameter_types.insert(parameter_name, candidate_type);
        } else if let Some(existing) = parameter_types.remove(&parameter_name) {
            parameter_types.insert(
                parameter_name,
                add_union_type(existing, &candidate_type, context.codebase, CombinerOptions::default()),
            );
        } else {
            parameter_types.insert(parameter_name, candidate_type);
        }
    }
}

fn insert_unpacked_parameter_type<A>(
    context: &Context<'_, '_, A>,
    invocation_target: &InvocationTarget<'_>,
    parameter_offset: usize,
    element_type: &TUnion,
    definitely_supplied: bool,
    parameter_types: &mut WordMap<(TUnion, bool)>,
) where
    A: Arena,
{
    let parameter_offset = adjust_offset_for_variadic(invocation_target, parameter_offset);
    let Some(parameter_name) = invocation_target.get_parameter(parameter_offset).and_then(|p| p.get_name()) else {
        return;
    };

    let mut element_type = element_type.clone();
    element_type.set_possibly_undefined(false, None);

    let parameter_type = if let Some((existing, existing_is_definite)) = parameter_types.remove(&parameter_name.0) {
        (
            add_union_type(existing, &element_type, context.codebase, CombinerOptions::default()),
            existing_is_definite && definitely_supplied,
        )
    } else {
        (element_type, definitely_supplied)
    };

    parameter_types.insert(parameter_name.0, parameter_type);
}

/// Validates individual elements within unpacked arrays against their corresponding parameters.
fn validate_unpacked_argument_elements<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    argument_value_type: &TUnion,
    argument_expression: &Expression<'arena>,
    base_class_metadata: Option<&'ctx ClassLikeMetadata>,
    calling_class_like_metadata: Option<&'ctx ClassLikeMetadata>,
    calling_instance_type: Option<&TAtomic>,
    method_class_type: Option<&StaticClassType>,
    invocation: &Invocation<'ctx, '_, 'arena>,
    template_result: &TemplateResult,
    parameter_types: &WordMap<TUnion>,
    starting_parameter_position: usize,
    target_kind_str: &str,
    target_name_str: &str,
) where
    A: Arena,
{
    let invocation_target = &invocation.target;

    for argument_atomic in argument_value_type.types.as_ref() {
        let TAtomic::Array(array) = argument_atomic else {
            continue;
        };

        match array {
            TArray::List(list) => {
                if let Some(known_elements) = &list.known_elements {
                    for (array_index, (_, element_type)) in known_elements.iter() {
                        let parameter_position = starting_parameter_position + array_index;
                        if parameter_position >= invocation_target.parameter_count() {
                            break;
                        }

                        if let Some(parameter_ref) = invocation_target.get_parameter(parameter_position) {
                            let base_parameter_type = get_parameter_type(
                                context,
                                Some(parameter_ref),
                                base_class_metadata,
                                calling_class_like_metadata,
                                calling_instance_type,
                                method_class_type,
                            );

                            let final_parameter_type = resolve_parameter_type_for_invocation(
                                context,
                                invocation,
                                template_result,
                                parameter_types,
                                base_class_metadata,
                                base_parameter_type,
                            );

                            verify_argument_type(
                                context,
                                element_type,
                                &final_parameter_type,
                                parameter_position,
                                argument_expression,
                                invocation_target,
                            );
                        }
                    }
                } else {
                    let element_type = list.get_element_type();
                    let min_size = list.known_count.unwrap_or(0);
                    let max_parameters_to_check = std::cmp::min(
                        min_size,
                        invocation_target.parameter_count().saturating_sub(starting_parameter_position),
                    );

                    for i in 0..max_parameters_to_check {
                        let parameter_position = starting_parameter_position + i;
                        if let Some(parameter_ref) = invocation_target.get_parameter(parameter_position) {
                            let base_parameter_type = get_parameter_type(
                                context,
                                Some(parameter_ref),
                                base_class_metadata,
                                calling_class_like_metadata,
                                calling_instance_type,
                                method_class_type,
                            );

                            let final_parameter_type = resolve_parameter_type_for_invocation(
                                context,
                                invocation,
                                template_result,
                                parameter_types,
                                base_class_metadata,
                                base_parameter_type,
                            );

                            verify_argument_type(
                                context,
                                element_type,
                                &final_parameter_type,
                                parameter_position,
                                argument_expression,
                                invocation_target,
                            );
                        }
                    }
                }
            }
            TArray::Keyed(keyed_array) => {
                validate_keyed_array_elements(
                    context,
                    keyed_array,
                    argument_expression,
                    base_class_metadata,
                    calling_class_like_metadata,
                    calling_instance_type,
                    method_class_type,
                    invocation,
                    template_result,
                    parameter_types,
                    starting_parameter_position,
                    target_kind_str,
                    target_name_str,
                );
            }
        }
    }
}

fn validate_keyed_array_elements<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    keyed_array: &mago_codex::ttype::atomic::array::keyed::TKeyedArray,
    argument_expression: &Expression<'arena>,
    base_class_metadata: Option<&'ctx ClassLikeMetadata>,
    calling_class_like_metadata: Option<&'ctx ClassLikeMetadata>,
    calling_instance_type: Option<&TAtomic>,
    method_class_type: Option<&StaticClassType>,
    invocation: &Invocation<'ctx, '_, 'arena>,
    template_result: &TemplateResult,
    parameter_types: &WordMap<TUnion>,
    starting_parameter_position: usize,
    target_kind_str: &str,
    target_name_str: &str,
) where
    A: Arena,
{
    use mago_word::concat_word;

    let invocation_target = &invocation.target;
    let Some(known_items) = &keyed_array.known_items else {
        return;
    };

    for (array_key, (_, element_type)) in known_items.iter() {
        let parameter_name = match array_key {
            ArrayKey::String(key_str) => concat_word!(b"$", key_str.as_bytes()),
            ArrayKey::Integer(key_int) => {
                let Ok(key_offset) = usize::try_from(*key_int) else {
                    continue;
                };
                if let Some(parameter_ref) = invocation_target.get_parameter(starting_parameter_position + key_offset) {
                    if let Some(param_name) = parameter_ref.get_name() {
                        param_name.0
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            ArrayKey::ClassLikeConstant { .. } => {
                continue;
            }
        };

        if let Some((parameter_position, parameter_ref)) = invocation_target
            .iter_parameters()
            .enumerate()
            .find(|(_, p)| p.get_name().is_some_and(|param_name| param_name.0 == parameter_name))
        {
            let base_parameter_type = get_parameter_type(
                context,
                Some(parameter_ref),
                base_class_metadata,
                calling_class_like_metadata,
                calling_instance_type,
                method_class_type,
            );

            let final_parameter_type = resolve_parameter_type_for_invocation(
                context,
                invocation,
                template_result,
                parameter_types,
                base_class_metadata,
                base_parameter_type,
            );

            verify_argument_type(
                context,
                element_type,
                &final_parameter_type,
                parameter_position,
                argument_expression,
                invocation_target,
            );
        } else if let ArrayKey::String(key_str) = array_key {
            let argument_name = BytesDisplay(key_str.as_bytes());

            // For variadic functions, allow extra named arguments
            let has_variadic_parameter = invocation_target
                .parameter_count()
                .checked_sub(1)
                .and_then(|idx| invocation_target.get_parameter(idx))
                .is_some_and(|parameter| parameter.is_variadic());

            if !has_variadic_parameter {
                context.collector.report_with_code(
                    IssueCode::InvalidNamedArgument,
                    Issue::error(format!(
                        "Invalid named argument `${argument_name}` for {target_kind_str} `{target_name_str}`"
                    ))
                    .with_annotation(
                        Annotation::primary(argument_expression.span())
                            .with_message(format!("Unknown argument name `${argument_name}` in unpacked array")),
                    )
                    .with_annotation(
                        Annotation::secondary(invocation_target.span())
                            .with_message(format!("Call to {target_kind_str} is here")),
                    )
                    .with_help(if !invocation_target.allows_named_arguments() {
                        format!("The {target_kind_str} `{target_name_str}` does not support named arguments.")
                    } else if invocation_target.parameter_count() > 0 {
                        let available_params: Vec<String> = invocation_target
                            .iter_parameters()
                            .filter_map(|p| {
                                p.get_name().map(|name| {
                                    let stripped = trim_start_byte(name.0.as_bytes(), b'$');
                                    format!("${}", BytesDisplay(stripped))
                                })
                            })
                            .collect();

                        if available_params.is_empty() {
                            format!("The {target_kind_str} `{target_name_str}` does not accept any named arguments.")
                        } else {
                            format!("Available named arguments are: {}.", available_params.join(", "))
                        }
                    } else {
                        format!("The {target_kind_str} `{target_name_str}` does not accept any arguments.")
                    }),
                );
            }
        }
    }
}

/// Detects if the current invocation is a `Closure::bind` or `Closure::bindTo` call
/// and extracts the bound scope information from its arguments.
///
/// Returns `Some(ClosureBindScope)` if this is a Closure::bind/bindTo call with extractable scope,
/// `None` otherwise.
fn detect_closure_bind_scope<'ctx, 'arena, A>(
    context: &Context<'ctx, 'arena, A>,
    artifacts: &AnalysisArtifacts,
    invocation: &Invocation<'ctx, '_, 'arena>,
    analyzed_argument_types: &[Option<(Rc<TUnion>, Span)>],
) -> Option<ClosureBindScope>
where
    A: Arena,
{
    let identifier = invocation.target.get_function_like_identifier()?;

    let FunctionLikeIdentifier::Method(class_id, method_id) = identifier else {
        return None;
    };

    if !class_id.as_bytes().eq_ignore_ascii_case(b"closure") {
        return None;
    }

    let method_name = method_id.as_bytes().to_ascii_lowercase();

    let (new_this_offset, new_scope_offset) = if method_name.eq_ignore_ascii_case(b"bind") {
        (1, 2)
    } else {
        return None;
    };

    let new_this_type = analyzed_argument_types.get(new_this_offset).and_then(Option::as_ref).map(|(t, _)| t.as_ref());
    let has_this = new_this_type.is_some_and(|t| !t.is_null());

    let class_name =
        if let Some((scope_type, _)) = analyzed_argument_types.get(new_scope_offset).and_then(Option::as_ref) {
            extract_class_name_from_scope_arg(context, scope_type)
        } else if has_this {
            new_this_type.and_then(extract_class_name_from_type)
        } else {
            None
        };

    // StaticCallHandler keeps the second argument's native holder, but an
    // explicit object/class scope supplies the full holder. Keep every member
    // of that scope union, independently of the member-access class above.
    let this_type = analyzed_argument_types
        .get(new_scope_offset)
        .and_then(Option::as_ref)
        .map(|(scope, _)| {
            scope
                .types
                .iter()
                .filter_map(|atomic| {
                    extract_class_name_from_scope_arg(context, &TUnion::from_atomic(atomic.clone()))
                        .map(|name| TAtomic::Object(TObject::new_named(name)))
                })
                .collect::<Vec<_>>()
        })
        .filter(|types| !types.is_empty())
        .map(TUnion::from_vec)
        .or_else(|| new_this_type.filter(|ty| !ty.is_null()).cloned())
        .map(Rc::new);

    if class_name.is_some() || this_type.is_some() {
        Some(ClosureBindScope {
            class_name,
            has_this: this_type.is_some(),
            this_type,
            native_this_type: analyzed_argument_types
                .get(new_this_offset)
                .and_then(Option::as_ref)
                .and_then(|(_, span)| artifacts.get_rc_native_expression_type(span))
                .filter(|ty| !ty.is_null())
                .cloned(),
        })
    } else {
        None
    }
}

/// Extracts a class name from a scope argument (typically the 3rd argument to Closure::bind).
/// This handles cases like `Foo::class`, `'Foo'`, or a class instance type.
fn extract_class_name_from_scope_arg<A>(context: &Context<'_, '_, A>, scope_type: &TUnion) -> Option<Word>
where
    A: Arena,
{
    for atomic in scope_type.types.as_ref() {
        match atomic {
            TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { value })) => {
                return Some(*value);
            }
            TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::OfType { constraint, .. }))
                if let Some(name) = extract_class_name_from_atomic(constraint) =>
            {
                return Some(name);
            }
            TAtomic::Scalar(TScalar::String(TString { literal: Some(TStringLiteral::Value(value)), .. }))
                if context.codebase.get_class_like(value.as_bytes()).is_some() =>
            {
                return Some(*value);
            }
            TAtomic::Object(TObject::Named(named)) => {
                return Some(named.name);
            }
            TAtomic::Object(TObject::Enum(enum_obj)) => {
                return Some(enum_obj.name);
            }
            _ => {}
        }
    }

    None
}

/// A PHPDoc binding changes the full holder without replacing the lexical class.
/// PHPStan assigns ObjectWithoutClassType to the corresponding native holder.
fn closure_this_bind_scope(closure_this_type: &TUnion) -> Option<ClosureBindScope> {
    Some(ClosureBindScope {
        class_name: None,
        has_this: true,
        this_type: Some(Rc::new(closure_this_type.clone())),
        native_this_type: Some(Rc::new(mago_codex::ttype::get_object())),
    })
}

/// Extracts a class name from a type union (typically for $this type).
fn extract_class_name_from_type(t: &TUnion) -> Option<Word> {
    for atomic in t.types.as_ref() {
        if let Some(name) = extract_class_name_from_atomic(atomic) {
            return Some(name);
        }
    }
    None
}

/// Extracts a class name from an atomic type.
fn extract_class_name_from_atomic(atomic: &TAtomic) -> Option<Word> {
    match atomic {
        TAtomic::Object(TObject::Named(named)) => Some(named.name),
        TAtomic::Object(TObject::Enum(enum_obj)) => Some(enum_obj.name),
        _ => None,
    }
}

/// Filters the callable parameter type for `array_filter` based on the `mode` argument.
/// For `array_filter`, narrows the callback parameter type to only the callable signature
/// matching the `mode` argument, so closure parameters get the correct inferred type.
///
/// mode 0 (default) → `callable(V): bool`, mode 1 → `callable(V, K): bool`, mode 2 → `callable(K): bool`
fn filter_array_filter_callback_type(
    target: &InvocationTarget<'_>,
    parameter_type: &mut TUnion,
    analyzed_argument_types: &[Option<(Rc<TUnion>, Span)>],
) {
    let is_array_filter = target.get_function_like_identifier().is_some_and(
        |id| matches!(id, FunctionLikeIdentifier::Function(name) if name.as_bytes().eq_ignore_ascii_case(b"array_filter")),
    );

    if !is_array_filter {
        return;
    }

    let mode = analyzed_argument_types
        .get(2)
        .and_then(Option::as_ref)
        .and_then(|(mode_type, _)| mode_type.get_single_literal_int_value())
        .unwrap_or(0);

    let expected_params: usize = if mode == 1 { 2 } else { 1 };

    let value_type_id = if expected_params == 1 {
        parameter_type.types.as_ref().iter().find_map(|atomic| {
            if let TAtomic::Callable(TCallable::Signature(sig)) = atomic
                && sig.parameters.len() == 2
            {
                sig.parameters.first().and_then(|p| p.get_type_signature()).map(|t| t.get_id())
            } else {
                None
            }
        })
    } else {
        None
    };

    let mut kept_one = false;
    parameter_type.types.to_mut().retain(|atomic| {
        let TAtomic::Callable(TCallable::Signature(sig)) = atomic else {
            return true;
        };

        if sig.parameters.len() != expected_params {
            return false;
        }

        if expected_params == 2 {
            return if kept_one {
                false
            } else {
                kept_one = true;
                true
            };
        }

        let is_value = sig.parameters.first().and_then(|p| p.get_type_signature()).map(|t| t.get_id()) == value_type_id;

        let want_value = mode != 2;
        if is_value == want_value && !kept_one {
            kept_one = true;
            true
        } else {
            false
        }
    });
}

/// GenericParametersAcceptorResolver::resolve() maps the callee's templates no argument inferred to an ErrorType,
/// which TemplateTypeHelper::resolveTemplateTypes() replaces with the template's default, else its bound. A closure
/// argument's parameters are typed with that resolved acceptor (`whereHas('posts', fn (Builder $q) => ...)` types
/// `$q` as `Builder<Model>`, not `Builder<TRelatedModel>`).
fn closure_parameter_bounds<A: Arena>(
    context: &Context<'_, '_, A>,
    parameter_type: Option<&TUnion>,
    template_result: &TemplateResult,
) -> Option<TUnion> {
    let parameter_type = parameter_type?;
    if !parameter_type.has_template_types() {
        return Some(parameter_type.clone());
    }
    let mut bounds = template_result.clone();
    for (name, templates) in &template_result.template_types {
        for template in templates {
            let inferred =
                template_result.lower_bounds.get(name).is_some_and(|by| by.contains_key(&template.defining_entity));
            if !inferred {
                let bound = template.default.clone().unwrap_or_else(|| template.constraint.clone());
                bounds
                    .lower_bounds
                    .entry(*name)
                    .or_default()
                    .insert(template.defining_entity, vec![mago_codex::ttype::template::TemplateBound::new(bound, 0, None)]);
            }
        }
    }
    Some(inferred_type_replacer::replace_with_polarity(
        parameter_type,
        &bounds,
        context.codebase,
        mago_codex::ttype::template::variance::Variance::Contravariant,
    ))
}
