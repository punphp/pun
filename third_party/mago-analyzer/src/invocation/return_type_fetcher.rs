use mago_allocator::Arena;
use mago_codex::identifier::function_like::FunctionLikeIdentifier;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::template::TemplateResult;
use mago_codex::ttype::union::TUnion;
use mago_word::WordMap;
use mago_word::word;

use crate::artifacts::AnalysisArtifacts;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;
use crate::invocation::Invocation;
use crate::invocation::resolver::resolve_invocation_type;
use crate::visibility::is_callable_method_inaccessible;

pub struct InvocationReturnType {
    pub ty: TUnion,
    pub always_terminates: bool,
}

pub fn fetch_invocation_return_type<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
    invocation: &Invocation<'ctx, '_, 'arena>,
    template_result: &TemplateResult,
    parameters: &WordMap<TUnion>,
) -> Result<InvocationReturnType, AnalysisError>
where
    A: Arena,
{
    let declared = fetch_declared_invocation_return_type(context, invocation, template_result, parameters);
    // PHPStan resolves forwarding as a value rewrite after reading the builtin's mixed acceptor.
    // Mago's prelude instead expresses that rewrite as a generic return; it must not become an exit edge.
    let forwarded = matches!(
        invocation.target.get_function_like_identifier(),
        Some(FunctionLikeIdentifier::Function(name))
            if name.as_bytes().eq_ignore_ascii_case(b"call_user_func")
                || name.as_bytes().eq_ignore_ascii_case(b"call_user_func_array")
    );
    let always_terminates = declared.is_explicit_never() && !forwarded;
    // Forwarding uses the callback's scope-aware acceptor too, not its inaccessible generic R bound.
    let declared = if forwarded
        && parameters.get(&word("$callback")).is_some_and(|callback| {
            callback.types.iter().any(|atomic| is_callable_method_inaccessible(context, block_context, atomic))
        })
    {
        get_mixed()
    } else {
        declared
    };
    let priced = match invocation.target.get_function_like_identifier() {
        Some(identifier) => price_callbacks(context, block_context, artifacts, identifier, invocation)?,
        None => None,
    };
    let artifacts = priced.as_ref().unwrap_or(artifacts);
    let ty = fetch_invocation_provider_return_type(context, block_context, artifacts, invocation).unwrap_or(declared);
    Ok(InvocationReturnType { ty, always_terminates })
}

/// The artifacts with the returns of the callbacks the providers of `identifier` ask to type again, `None` when
/// they ask for none.
pub(crate) fn price_callbacks<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
    identifier: &FunctionLikeIdentifier,
    invocation: &Invocation<'ctx, '_, 'arena>,
) -> Result<Option<AnalysisArtifacts>, AnalysisError>
where
    A: Arena,
{
    let requests = context.plugin_registry.callback_invocations(
        context.codebase, context.source_file, block_context, artifacts, identifier, invocation,
    );
    if requests.is_empty() {
        return Ok(None);
    }
    let mut answers = artifacts.clone();
    answers.callback_returns.clear();
    for request in requests {
        let result = super::callback::evaluate(context, block_context, artifacts, &request)?;
        answers.callback_returns.push((mago_span::HasSpan::span(request.callback), request.arguments, result));
    }
    Ok(Some(answers))
}

/// Requests a custom return type from registered providers and reports provider issues.
///
pub(crate) fn fetch_invocation_provider_return_type<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
    invocation: &Invocation<'ctx, '_, 'arena>,
) -> Option<TUnion>
where
    A: Arena,
{
    let identifier = invocation.target.get_function_like_identifier()?;
    if let Some(filtered) = super::array_filter::return_type(context, block_context, artifacts, invocation) {
        return Some(filtered);
    }

    fetch_function_like_provider_return_type(context, block_context, artifacts, identifier, invocation)
}

/// Requests a custom return type for an explicit function-like identifier.
///
/// This permits dynamic method calls to match the method requested by the user
/// while retaining `__call` or `__callStatic` as the invocation's native target.
///
pub(crate) fn fetch_function_like_provider_return_type<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
    identifier: &FunctionLikeIdentifier,
    invocation: &Invocation<'ctx, '_, 'arena>,
) -> Option<TUnion>
where
    A: Arena,
{
    if let Some(result) = context.plugin_registry.get_function_like_return_type(
        context.codebase,
        context.source_file,
        block_context,
        artifacts,
        identifier,
        invocation,
        context.external_analysis_session,
    ) {
        for reported_issue in result.issues {
            context.collector.report_with_code(reported_issue.code, reported_issue.issue);
        }

        if let Some(ty) = result.return_type {
            return Some(ty);
        }
    }

    None
}

/// Resolves the declared return type of an invocation without consulting providers.
pub(crate) fn fetch_declared_invocation_return_type<A>(
    context: &Context<'_, '_, A>,
    invocation: &Invocation<'_, '_, '_>,
    template_result: &TemplateResult,
    parameters: &WordMap<TUnion>,
) -> TUnion
where
    A: Arena,
{
    let target_return_type = if let Some(function_like_metadata) = invocation.target.get_function_like_metadata()
        && let Some(docblock_return) = function_like_metadata.return_type_metadata.as_ref()
        && docblock_return.from_docblock
        && !docblock_return.type_union.is_explicit_never()
        && !docblock_return.type_union.has_template_types()
        && !docblock_return.type_union.is_generic_parameter()
        && !docblock_return.type_union.types.iter().any(|t| {
            t.is_derived()
                || t.is_conditional()
                || matches!(t, TAtomic::Alias(_) | TAtomic::Reference(_) | TAtomic::GenericParameter(_))
        })
        && let Some(native_return) = function_like_metadata.return_type_declaration_metadata.as_ref()
        && ((native_return.type_union.has_object_type()
            && !docblock_return.type_union.has_object_type()
            && !docblock_return.type_union.has_mixed())
            || (native_return.type_union.is_void() && !docblock_return.type_union.is_void())
            || (native_return.type_union.is_array()
                && !docblock_return.type_union.is_array()
                && !docblock_return.type_union.is_list()
                && !docblock_return.type_union.has_iterable()
                && !docblock_return.type_union.has_mixed()))
    {
        Some(native_return.type_union.clone())
    } else {
        invocation.target.get_return_type().cloned()
    };

    let mut resulting_type = if let Some(return_type) = target_return_type {
        let generalized = generalize_template_arguments(context, &return_type, template_result);
        let template_result = generalized.as_ref().unwrap_or(template_result);
        resolve_invocation_type(context, invocation, template_result, parameters, return_type)
    } else {
        get_mixed()
    };

    if let Some(function_like_metadata) = invocation.target.get_function_like_metadata()
        && function_like_metadata.flags.is_by_reference()
    {
        resulting_type.set_by_reference(true);
    }

    resulting_type
}

/// PHPStan's ResolvedFunctionVariantWithOriginal::resolveResolvableTemplateTypes(): a function or method template
/// whose return type references it in a position that is not covariant (an invariant class template's argument, as
/// in `static<TValue, TCombineValue>`) resolves to TemplateTypeHelper::generalizeInferredTemplateType() of its
/// inferred type: a constant value generalized unless the template's bound is scalar, any scalar for an `array-key`
/// bound. `Collection::combine([1])` is `Collection<TValue, int>`; `wrap(1)` returning `list<T>` stays `list<1>`.
/// ponytail: PHPStan 2.3's template argument frame can decide such an argument from the calling body; without a
/// decision it is generalized, which is all this follows.
fn generalize_template_arguments<A: Arena>(
    context: &Context<'_, '_, A>,
    return_type: &TUnion,
    template_result: &TemplateResult,
) -> Option<TemplateResult> {
    use mago_codex::misc::GenericParent;
    use mago_codex::ttype::template::variance::Variance;

    fn walk(
        codebase: &mago_codex::metadata::CodebaseMetadata,
        t: &TUnion,
        covariant: bool,
        out: &mut Vec<(mago_word::Word, GenericParent)>,
    ) {
        for atomic in t.types.iter() {
            match atomic {
                TAtomic::GenericParameter(p) => {
                    if !covariant && matches!(p.defining_entity, GenericParent::FunctionLike(_)) {
                        out.push((p.parameter_name, p.defining_entity));
                    }
                }
                TAtomic::Object(mago_codex::ttype::atomic::object::TObject::Named(n)) => {
                    let variances = codebase.get_class_like(n.name.as_bytes()).map(|k| k.template_variance.clone());
                    for (i, parameter) in n.type_parameters.iter().flat_map(|p| p.iter()).enumerate() {
                        let variance = variances.as_ref().and_then(|v| v.get(i)).copied().unwrap_or(Variance::Invariant);
                        walk(codebase, parameter, covariant && matches!(variance, Variance::Covariant), out);
                    }
                }
                TAtomic::Array(array) => {
                    let (key, value) = mago_codex::ttype::get_array_parameters(array, codebase);
                    walk(codebase, &key, covariant, out);
                    walk(codebase, &value, covariant, out);
                }
                // a conditional return type's branches are in the return's position
                TAtomic::Conditional(conditional) => {
                    walk(codebase, &conditional.then, covariant, out);
                    walk(codebase, &conditional.otherwise, covariant, out);
                }
                _ => {}
            }
        }
    }

    /// `isConstantValue()->yes()` of a type: every part a literal scalar, `true`, `false` or `null`.
    fn constant(t: &TUnion) -> bool {
        !t.types.is_empty()
            && t.types.iter().all(|a| match a {
                TAtomic::Null => true,
                TAtomic::Scalar(TScalar::Bool(b)) => !b.is_general(),
                TAtomic::Scalar(TScalar::Integer(i)) => i.is_literal(),
                TAtomic::Scalar(TScalar::Float(f)) => f.get_literal_value().is_some(),
                TAtomic::Scalar(TScalar::String(s)) => s.get_known_literal_value().is_some(),
                TAtomic::Scalar(TScalar::ClassLikeString(c)) => c.literal_value().is_some(),
                // a ConstantArrayType
                TAtomic::Array(TArray::List(l)) => l.element_type.is_never() && l.known_elements.is_some(),
                TAtomic::Array(TArray::Keyed(k)) => k.parameters.is_none() && k.known_items.is_some(),
                _ => false,
            })
    }

    /// `generalize(GeneralizePrecision::templateArgument())`: a constant scalar's general type, a constant array's
    /// values generalized (ConstantArrayType::traverse()), any other type itself.
    fn generalized(t: &TUnion) -> TUnion {
        let mut types = Vec::new();
        for a in t.types.iter() {
            let general = match a {
                TAtomic::Scalar(TScalar::Bool(_)) => TAtomic::Scalar(TScalar::bool()),
                TAtomic::Scalar(TScalar::Integer(_)) => TAtomic::Scalar(TScalar::int()),
                TAtomic::Scalar(TScalar::Float(_)) => TAtomic::Scalar(TScalar::float()),
                TAtomic::Scalar(TScalar::String(_) | TScalar::ClassLikeString(_)) => TAtomic::Scalar(TScalar::string()),
                TAtomic::Array(TArray::List(l)) if l.element_type.is_never() => {
                    let mut l = l.clone();
                    if let Some(known) = &l.known_elements {
                        l.known_elements = Some(std::sync::Arc::new(
                            known.iter().map(|(i, (optional, v))| (*i, (*optional, generalized(v)))).collect(),
                        ));
                    }
                    TAtomic::Array(TArray::List(l))
                }
                TAtomic::Array(TArray::Keyed(k)) if k.parameters.is_none() => {
                    let mut k = k.clone();
                    if let Some(known) = &k.known_items {
                        k.known_items = Some(std::sync::Arc::new(
                            known.iter().map(|(key, (optional, v))| (*key, (*optional, generalized(v)))).collect(),
                        ));
                    }
                    TAtomic::Array(TArray::Keyed(k))
                }
                a => a.clone(),
            };
            if !types.contains(&general) {
                types.push(general);
            }
        }
        TUnion::from_vec(types)
    }

    if template_result.lower_bounds.is_empty() || !return_type.has_template_types() {
        return None;
    }
    let mut found = Vec::new();
    walk(context.codebase, return_type, true, &mut found);
    if found.is_empty() {
        return None;
    }
    let mut result = template_result.clone();
    let mut changed = false;
    for (name, entity) in found {
        let Some(template) = template_result
            .template_types
            .get(&name)
            .and_then(|templates| templates.iter().find(|t| t.defining_entity == entity))
        else {
            continue;
        };
        let Some(bounds) = result.lower_bounds.get_mut(&name).and_then(|by| by.get_mut(&entity)) else {
            continue;
        };
        let constraint = &template.constraint;
        let array_key = constraint.types.len() == 2
            && constraint.types.iter().any(|a| matches!(a, TAtomic::Scalar(TScalar::Integer(i)) if i.is_unspecified()))
            && constraint.types.iter().any(|a| matches!(a, TAtomic::Scalar(TScalar::String(s)) if s.get_known_literal_value().is_none()))
            || matches!(constraint.types.as_ref(), [TAtomic::Scalar(TScalar::ArrayKey)]);
        let scalar_bound = !constraint.types.is_empty() && constraint.types.iter().all(|a| matches!(a, TAtomic::Scalar(_)));
        let inferred = mago_codex::ttype::template::bounds::get_most_specific_type_from_bounds(bounds, context.codebase);
        let scalar = !inferred.types.is_empty() && inferred.types.iter().all(|a| matches!(a, TAtomic::Scalar(_)));
        let generalize = (scalar && array_key) || (constant(&inferred) && (!scalar_bound || array_key));
        if generalize {
            *bounds = vec![mago_codex::ttype::template::TemplateBound::new(generalized(&inferred), 0, None)];
            changed = true;
        }
    }
    changed.then_some(result)
}

