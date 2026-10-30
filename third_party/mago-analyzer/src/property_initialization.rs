//! Opt-in PHPStan PropertyInitializationExpr facts from the live semantic walk.
//!
//! These are virtual expression holders, not PHP's runtime initializedness or Mago's
//! readonly-property state. In particular, unset does not remove them. Constructor
//! facts use raw return scopes (before finally), exclude throwing exits, and are
//! remembered only for the same class's instance methods. CalledMethodProcessor
//! gathers bounded, non-transitive helper scopes through the real semantic walker.

use foldhash::HashMap;
use mago_allocator::Arena;
use mago_codex::context::ScopeContext;
use mago_codex::identifier::method::MethodIdentifier;
use mago_codex::reference::ReferenceOrigin;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::{TAtomic, object::TObject};
use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::{ClassLikeMember, Method, MethodBody, Node, Statement};
use mago_word::{Word, WordMap};

use crate::artifacts::AnalysisArtifacts;
use crate::context::{Context, block::BlockContext};
use crate::error::AnalysisError;
use crate::statement::function_like::{FunctionLikeBody, analyze_function_like};

/// Certainty of PHPStan's virtual PropertyInitializationExpr, independently of value/nullability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyInitializationCertainty {
    Yes,
    Maybe,
    No,
}

impl PropertyInitializationCertainty {
    pub(crate) fn join(self, other: Self) -> Self {
        if self == other { self } else { Self::Maybe }
    }
}

/// Observe a named virtual marker immediately before a real operand is evaluated.
/// A request enables tracking and constructor-before-instance-method scheduling for
/// this analysis. Isolated helper gathers never publish operand observations.
/// Missing operands produce no answer; a visited operand with no holder produces `No`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropertyInitializationRequest {
    pub expression: Span,
    /// Bare, case-sensitive property name, without `$`.
    pub property: Word,
}

pub(crate) type Markers = WordMap<PropertyInitializationCertainty>;

/// Per-file analysis cache, never Mago's transitive runtime-initialization summary.
#[derive(Debug, Default)]
pub(crate) struct CalledMethods {
    results: HashMap<MethodIdentifier, Option<Markers>>,
    pub(crate) processing: bool,
}

/// TypeUtils::findThisType searches unions/intersections, not generic bounds or `static`.
pub(crate) fn has_this_type(ty: &TUnion) -> bool {
    fn atomic_has_this_type(atomic: &TAtomic) -> bool {
        matches!(atomic, TAtomic::Object(TObject::Named(object)) if object.is_this)
            || atomic
                .get_intersection_types()
                .is_some_and(|types| types.iter().any(atomic_has_this_type))
    }
    ty.types.iter().any(atomic_has_this_type)
}

/// ScopeOps::mergeInitializedProperties copies only virtual holders and ORs certainty.
pub(crate) fn merge_initialized(ours: &mut Markers, helper: &Markers) {
    for (property, certainty) in helper {
        ours.entry(*property)
            .and_modify(|previous| {
                if *certainty == PropertyInitializationCertainty::Yes {
                    *previous = *certainty;
                }
            })
            .or_insert(*certainty);
    }
}

/// Port of CalledMethodProcessor: a fresh method entry, not the constructor's locals,
/// arguments or remembered markers. Its collector/artifacts/events stay in the fork.
pub(crate) fn called_method<'ctx, 'arena, A: Arena>(
    context: &mut Context<'ctx, 'arena, A>,
    id: MethodIdentifier,
) -> Result<Option<Markers>, AnalysisError> {
    let class = context
        .codebase
        .get_class_like(id.get_class_name().as_bytes())
        .unwrap();
    if class.name_span.is_none() || class.span.file_id != context.source_file.id {
        return Ok(None);
    }
    if let Some(result) = context.called_method_properties.results.get(&id) {
        return Ok(result.clone());
    }
    if context.called_method_properties.processing {
        return Ok(None);
    }
    let program = context
        .observation_program
        .expect("property requests retain the program");
    let method = find_method(
        Node::Program(program),
        context.codebase.declaration_span(class.span),
        id.get_method_name(),
    );
    let result = if let Some(method) = method
        && context
            .source_file
            .line_number(method.span().end.offset - 1)
            - context.source_file.line_number(method.span().start.offset)
            <= 50
        && let MethodBody::Concrete(body) = &method.body
    {
        let metadata = context.codebase.get_method_by_id(&id).unwrap();
        let mut scope = ScopeContext::new(ReferenceOrigin::Symbol((class.name, metadata.name)));
        scope.set_class_like(Some(class));
        scope.set_function_like(Some(metadata));
        scope.set_static(method.is_static());
        let mut block = BlockContext::new(scope, context.settings.register_super_globals);
        let mut fork = context.fork_for_observation();
        fork.ensured_non_null_types.clear();
        fork.consume_stored_expression_results = false;
        fork.observations.calls.clear();
        fork.called_method_properties
            .results
            .clone_from(&context.called_method_properties.results);
        fork.called_method_properties.processing = true;
        let mut discarded = AnalysisArtifacts::new();
        let artifacts = crate::plugin::registry::with_isolated_type_pricing(|| {
            analyze_function_like(
                &mut fork,
                &mut discarded,
                &mut block,
                metadata,
                &method.parameter_list,
                FunctionLikeBody::Statements(body.statements.as_slice(), body.span()),
                None,
            )
        })?;
        // StatementsHandler emits no execution end when the last body statement
        // was unreachable. IfHandler's branch-result fallback is different.
        let mut ends = if body
            .statements
            .last()
            .is_none_or(|last| artifacts.property_statement_ends.contains_key(&last.span()))
        {
            statement_ends(body.statements.as_slice(), &block, &artifacts)
        } else {
            Vec::new()
        };
        ends.extend(artifacts.constructor_property_returns.values().cloned());
        (!ends.is_empty()).then(|| join(ends.iter()))
    } else {
        None
    };
    context
        .called_method_properties
        .results
        .insert(id, result.clone());
    Ok(result)
}

/// Locate only the declaring named class. PHPStan stops at class/function-like
/// boundaries; a nested class inside a function is not eligible for this gather.
fn find_method<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    class: Span,
    method: Word,
) -> Option<&'ast Method<'arena>> {
    match node {
        Node::Class(declaration) => {
            if declaration.span() != class {
                return None;
            }
            declaration.members.iter().find_map(|member| match member {
                ClassLikeMember::Method(declaration)
                    if declaration
                        .name
                        .value
                        .eq_ignore_ascii_case(method.as_bytes()) =>
                {
                    Some(declaration)
                }
                _ => None,
            })
        }
        Node::Interface(_)
        | Node::Trait(_)
        | Node::Enum(_)
        | Node::AnonymousClass(_)
        | Node::Function(_)
        | Node::Closure(_)
        | Node::ArrowFunction(_) => None,
        _ => node
            .children()
            .into_iter()
            .find_map(|child| find_method(child, class, method)),
    }
}

/// Read already-produced live statement results, never evaluate syntax or types here.
pub(crate) fn statement_ends(
    statements: &[Statement<'_>],
    block: &BlockContext<'_>,
    artifacts: &AnalysisArtifacts,
) -> Vec<Markers> {
    statements
        .last()
        .and_then(|statement| artifacts.property_statement_ends.get(&statement.span()))
        .cloned()
        .unwrap_or_else(|| vec![block.variable_state.property_initializations.clone()])
}

/// StatementsHandler's last statement result; IfHandler and BlockHandler forward
/// their actual child ends. Explicit-never expression ends do not contribute.
pub(crate) fn statement_end(
    statement: &Statement<'_>,
    block: &BlockContext<'_>,
    artifacts: &mut AnalysisArtifacts,
) {
    let ends = match statement {
        Statement::If(_) => return,
        Statement::Block(block_statement) => {
            statement_ends(block_statement.statements.as_slice(), block, artifacts)
        }
        Statement::Expression(expression)
            if artifacts
                .get_expression_type(expression.expression)
                .is_some_and(|ty| ty.is_explicit_never()) =>
        {
            Vec::new()
        }
        _ => vec![block.variable_state.property_initializations.clone()],
    };
    artifacts
        .property_statement_ends
        .insert(statement.span(), ends);
}

pub(crate) fn remember(
    scopes: &mut foldhash::HashMap<Span, Markers>,
    span: Span,
    incoming: &Markers,
) {
    scopes
        .entry(span)
        .and_modify(|previous| *previous = join([&*previous, incoming]))
        .or_insert_with(|| incoming.clone());
}

/// processFinallyScopeVariableTypeHolders compares types, not certainties. All
/// virtual markers have MixedType: only a newly introduced marker is transferred.
pub(crate) fn apply_finally(ours: &mut Markers, after: &Markers, original: &Markers) {
    ours.extend(
        after
            .iter()
            .filter(|(name, _)| !original.contains_key(name))
            .map(|(name, certainty)| (*name, *certainty)),
    );
}

/// ScopeOps::mergeWith: a holder absent on any incoming edge is only Maybe.
pub(crate) fn join<'a>(scopes: impl IntoIterator<Item = &'a Markers>) -> Markers {
    let scopes: Vec<_> = scopes.into_iter().collect();
    let mut joined = Markers::default();
    for scope in &scopes {
        for name in scope.keys() {
            joined.entry(*name).or_insert_with(|| {
                if scopes
                    .iter()
                    .all(|scope| scope.get(name) == Some(&PropertyInitializationCertainty::Yes))
                {
                    PropertyInitializationCertainty::Yes
                } else {
                    PropertyInitializationCertainty::Maybe
                }
            });
        }
    }
    joined
}
