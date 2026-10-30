//! PHPStan 2.3.1 NodeCallbackScope/ExpressionResult type observations.
//!
//! A real evaluation and a rule's later type question have different scope positions.
//! Only requested calls retain scopes. Counterfactual pricing owns its context,
//! block, collector and artifacts; it cannot apply effects to the real walk.

use std::cell::RefCell;
use std::rc::Rc;

use foldhash::HashMap;
use mago_algebra::{find_satisfying_assignments, saturate_clauses};
use mago_allocator::Arena;
use mago_codex::identifier::function_like::FunctionLikeIdentifier;
use mago_codex::misc::GenericParent;
use mago_codex::ttype::{
    TType, TypeRef,
    atomic::TAtomic,
    comparator::{ComparisonResult, union_comparator::is_contained_by},
    get_literal_string,
    union::TUnion,
};
use mago_span::{HasSpan, Span};
use mago_syntax::cst::{
    Access, Call, ClassLikeMemberSelector, Closure, Expression, Literal, LiteralString, LiteralStringKind, MethodCall,
    NullSafeMethodCall, Variable,
};
use mago_syntax::walker::MutWalker;
use mago_word::{Word, WordMap, WordSet, word};

use crate::{
    analyzable::Analyzable,
    artifacts::{AnalysisArtifacts, ExpressionResult},
    common::synthetic::new_synthetic_identical,
    context::{
        Context,
        block::{BlockContext, VariableCertainty},
    },
    error::AnalysisError,
    formula::get_formula,
    invocation::InvocationTarget,
    reconciler::reconcile_keyed_types,
    utils::expression::get_block_expression_id,
};

mod history;

/// One call and the expressions its consumer will query, including dependencies.
/// An expression not reached by the real walk has no answer, not a mixed answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallScopeRequest {
    pub call: Span,
    pub expressions: Vec<Span>,
}

/// Owned answers for one method name. Identifier calls remain unfiltered.
#[derive(Debug, Clone)]
pub struct NamedCallScope {
    pub name: Word,
    pub filtered: bool,
    pub types: HashMap<Span, TUnion>,
}

/// Owned reflection facts from ArgumentsHandler::pushInFunctionCall.
/// Equal full parameter types also cover PHPStan's same-parameter identity shortcut.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CallContext {
    argument: Span,
    callee: Option<FunctionLikeIdentifier>,
    parameter_type: Option<TUnion>,
}

pub(crate) fn argument_context<A: Arena>(
    context: &Context<'_, '_, A>,
    target: Option<&InvocationTarget<'_>>,
    parameter_type: Option<&TUnion>,
    argument: Span,
) -> Option<CallContext> {
    if context.observation_program.is_none() {
        return None;
    }
    let callee = match target {
        Some(target) => {
            let mut callee = *target.get_function_like_identifier()?;
            if let Some(method) = target.get_method_context()
                && let FunctionLikeIdentifier::Method(class, _) = callee
            {
                callee = FunctionLikeIdentifier::Method(class, method.requested_name);
            }
            Some(callee)
        }
        // gatherClosureArgType pushes its raw parameter without a callee reflection.
        None => None,
    };
    Some(CallContext {
        argument,
        callee,
        parameter_type: parameter_type.cloned(),
    })
}

pub(crate) fn stored_type<A: Arena>(context: &Context<'_, '_, A>, span: Span) -> Option<TUnion> {
    context
        .observations
        .stored
        .get(&span)
        .and_then(|result| result.result.as_ref().map(|result| result.get_type().clone()))
}

/// A resolved PHPStan parameter reflection includes inference from the argument's own type.
/// Finish these owned reflection facts after the real walk, without changing its saved scopes.
pub(crate) fn settle_argument<A: Arena>(context: &mut Context<'_, '_, A>, resolved: &CallContext) {
    for result in context.observations.stored.values_mut() {
        for position in &mut result.calls {
            if position.argument == resolved.argument && position.callee == resolved.callee {
                position.parameter_type.clone_from(&resolved.parameter_type);
            }
        }
    }
}

/// PHPStan 2.3.1 ClosureCallContextMatcher::matches, without pointer-only fast paths.
fn closure_context_matches(ask: &[CallContext], position: &[CallContext]) -> bool {
    if ask == position || ask.is_empty() || ask.len() != position.len() {
        return true;
    }
    let ask = ask.last().expect("nonempty asking stack");
    let position = position.last().expect("equally deep position stack");
    let Some(ask_type) = &ask.parameter_type else {
        return true;
    };
    if position.parameter_type.as_ref() == Some(ask_type) {
        return true;
    }
    let Some(callee) = position.callee else {
        return false;
    };
    let (callee_name, is_method) = match callee {
        FunctionLikeIdentifier::Method(_, name) => (name, true),
        FunctionLikeIdentifier::Function(name) | FunctionLikeIdentifier::Closure(name) => (name, false),
    };
    ask_type.any_child_node(|node| {
        let TypeRef::Atomic(TAtomic::GenericParameter(parameter)) = node else {
            return false;
        };
        let GenericParent::FunctionLike((class, name)) = parameter.defining_entity else {
            return false;
        };
        let template_is_method = !class.is_empty();
        template_is_method == is_method && name.as_bytes().eq_ignore_ascii_case(callee_name.as_bytes())
    })
}

#[derive(Debug, Clone)]
struct StoredResult {
    locals: WordMap<Rc<TUnion>>,
    certainty: WordMap<VariableCertainty>,
    calls: Vec<CallContext>,
    closure: bool,
    reads: WordSet,
    result: Option<ExpressionResult>,
    evaluated: bool,
    /// atAskPosition pins a scope-authoritative nullsafe receiver at its ensured position.
    eager: bool,
}

type StoredResults = HashMap<Span, StoredResult>;
type Price<'ctx> =
    Rc<dyn Fn(Span, Option<Word>, &AnalysisArtifacts, &StoredResults, bool) -> Result<Option<TUnion>, AnalysisError> + 'ctx>;

#[derive(Clone)]
struct PricingScope<'ctx> {
    locals: WordMap<Rc<TUnion>>,
    certainty: Rc<dyn Fn(Word) -> VariableCertainty + 'ctx>,
    calls: Vec<CallContext>,
    price: Price<'ctx>,
}

impl std::fmt::Debug for PricingScope<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PricingScope")
            .field("locals", &self.locals)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
struct EvaluatedExpression<'ctx> {
    before: PricingScope<'ctx>,
}

#[derive(Debug, Clone)]
struct CallObservation<'ctx, 'arena> {
    entry: PricingScope<'ctx>,
    selector: Option<&'arena Expression<'arena>>,
    identifier: Option<Word>,
    receiver: Span,
    request: CallScopeRequest,
    evaluated: HashMap<Span, EvaluatedExpression<'ctx>>,
    consumes_stored: bool,
}

#[derive(Debug, Default)]
pub(crate) struct ObservationState<'ctx, 'arena> {
    stored: StoredResults,
    /// A lexical assignment-preparation handoff, not the historical observation store.
    prepared_operands: Vec<(Span, Option<ExpressionResult>)>,
    preparing: Vec<Vec<(Span, Option<ExpressionResult>)>>,
    pub calls: Vec<CallContext>,
    history: Option<history::History<'arena>>,
    retained: HashMap<Span, HashMap<Span, EvaluatedExpression<'ctx>>>,
    active: HashMap<Span, CallObservation<'ctx, 'arena>>,
    pub completed: HashMap<Span, Vec<NamedCallScope>>,
    pub pricing: bool,
}

// BlockContext's live if/finally sinks are intentionally not type-pricing outputs.
fn snapshot<'ctx>(scope: &BlockContext<'ctx>) -> BlockContext<'ctx> {
    let mut scope = scope.clone();
    scope.if_body_context = None;
    scope.finally_scope = scope
        .finally_scope
        .as_ref()
        .map(|s| Rc::new(RefCell::new(s.borrow().clone())));
    scope
}

#[derive(Default)]
struct Reads(WordSet);
impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Reads {
    fn walk_in_variable(&mut self, variable: &'ast Variable<'arena>, _: &mut ()) {
        if let Variable::Direct(v) = variable
            && v.name != b"$this"
        {
            self.0.insert(word(v.name));
        }
    }

    fn walk_closure(&mut self, closure: &'ast Closure<'arena>, _: &mut ()) {
        if let Some(uses) = &closure.use_clause {
            for capture in &uses.variables {
                self.0.insert(word(capture.variable.name));
            }
        }
    }
}

struct Find<'arena> {
    wanted: Span,
    found: Option<Expression<'arena>>,
}
impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Find<'arena> {
    fn walk_in_expression(&mut self, expression: &'ast Expression<'arena>, _: &mut ()) {
        if expression.span() == self.wanted {
            self.found = Some(expression.clone());
        }
    }
    fn walk_in_variable(&mut self, variable: &'ast Variable<'arena>, _: &mut ()) {
        if variable.span() == self.wanted {
            self.found = Some(Expression::Variable(variable.clone()));
        }
    }
}

// Erasing the scope behind an owned pricing operation preserves Context's lifetime
// covariance. Keeping BlockContext directly there would expose its live RefCell
// lifetime invariance throughout otherwise read-only inference APIs.
fn capture<'ctx, 'arena, A: Arena>(
    root: &Expression<'arena>,
    selector: Option<&'arena Expression<'arena>>,
    context: &Context<'ctx, 'arena, A>,
    scope: &BlockContext<'ctx>,
) -> PricingScope<'ctx> {
    let root = root.clone();
    let historical = Rc::clone(
        &context
            .observations
            .history
            .as_ref()
            .expect("requested expressions are indexed")
            .expressions,
    );
    let before = Rc::new(snapshot(scope));
    let certainty = Rc::clone(&before);
    let prototype = context.fork_for_observation();
    PricingScope {
        locals: scope.locals.clone(),
        certainty: Rc::new(move |name| certainty.variable_certainty(name)),
        calls: context.observations.calls.clone(),
        price: Rc::new(move |span, name, artifacts, stored, consume_stored| {
            let mut finder = Find {
                wanted: span,
                found: None,
            };
            finder.walk_expression(&root, &mut ());
            let expression = finder
                .found
                .or_else(|| historical.get(&span).cloned())
                .expect("evaluated expression belongs to its retained subtree or requested history");
            let mut scope = snapshot(&before);
            let mut sandbox = artifacts.clone();
            sandbox.loop_scope = None;
            sandbox.case_scopes.clear();
            let mut fork = prototype.fork_for_observation();
            fork.observations.stored = stored.clone();
            // The asking callback's plain twin supplies this mode, including
            // historical dependencies captured before the asking call existed.
            fork.consume_stored_expression_results = consume_stored;
            let (answer, _) = crate::plugin::registry::with_isolated_type_pricing(|| {
                fork.record(|context| {
                    if let (Some(selector), Some(name)) = (selector, name) {
                        filter(selector, name, &mut scope, context, &mut sandbox);
                    }
                    expression.analyze(context, &mut scope, &mut sandbox)?;
                    Ok(sandbox.get_expression_type(&expression).cloned())
                })
            });
            answer
        }),
    }
}

fn prepare_history<'ctx, 'arena, A: Arena>(context: &mut Context<'ctx, 'arena, A>) {
    if context.observations.history.is_none() {
        context.observations.history = Some(history::History::new(
            context
                .observation_program
                .expect("requested observations retain the program"),
            &context.settings.call_scope_observations,
            context.arena,
        ));
    }
}

/// Called only for requested subtrees; ordinary analysis never clones a scope.
pub(crate) fn before<'ctx, 'arena, A: Arena>(
    expression: &Expression<'arena>,
    context: &mut Context<'ctx, 'arena, A>,
    scope: &BlockContext<'ctx>,
) {
    if context.observations.pricing || context.raw_callback_gather_depth > 0 || context.observation_program.is_none() {
        return;
    }
    prepare_history(context);
    let span = expression.span();
    let calls: Vec<_> = context
        .observations
        .active
        .values()
        .filter(|c| history::contains(c.request.call, span))
        .map(|c| (c.request.call, c.selector, c.request.expressions.contains(&span)))
        .collect();
    let history = context
        .observations
        .history
        .as_ref()
        .expect("requested expressions are indexed");
    let queries = history.queries.get(&span).cloned().unwrap_or_default();
    if calls.is_empty() && !history.expressions.keys().any(|root| history::contains(*root, span)) {
        return;
    }
    let mut reads = Reads::default();
    reads.walk_expression(expression, &mut ());
    for (call, selector) in queries {
        let result = EvaluatedExpression {
            before: capture(expression, selector, context, scope),
        };
        context
            .observations
            .retained
            .entry(call)
            .or_default()
            .insert(span, result);
    }
    for (call, selector, requested) in calls {
        if requested {
            let result = EvaluatedExpression {
                before: capture(expression, selector, context, scope),
            };
            context
                .observations
                .active
                .get_mut(&call)
                .expect("active requested call")
                .evaluated
                .insert(span, result);
        }
    }
    context.observations.stored.insert(
        span,
        StoredResult {
            locals: scope.locals.clone(),
            certainty: reads
                .0
                .iter()
                .map(|name| (*name, scope.variable_certainty(*name)))
                .collect(),
            calls: context.observations.calls.clone(),
            closure: matches!(expression, Expression::Closure(_) | Expression::ArrowFunction(_)),
            reads: reads.0,
            result: None,
            evaluated: false,
            eager: false,
        },
    );
}

pub(crate) fn after<'ctx, 'arena, A: Arena>(
    expression: &Expression<'arena>,
    context: &mut Context<'ctx, 'arena, A>,
    scope: &BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
) {
    // Retain only completions emitted by this operand's real walk. In particular,
    // a semantic hook that skips a parent does not manufacture child results.
    if context.raw_callback_gather_depth == 0 {
        for operands in &mut context.observations.preparing {
            operands.push((expression.span(), artifacts.get_expression_result(expression)));
        }
    }
    if context.observations.pricing || context.raw_callback_gather_depth > 0 {
        return;
    }
    if let Some(result) = context.observations.stored.get_mut(&expression.span()) {
        result.result = artifacts.get_expression_result(expression);
        result.evaluated = true;
    }
    // A variable member selector bypasses Expression::analyze. Its position is
    // immediately after the receiver, not call entry or the final argument.
    let selectors: Vec<_> = context
        .observations
        .active
        .values()
        .filter(|c| c.receiver == expression.span())
        .filter_map(|c| c.selector)
        .filter(|e| matches!(e, Expression::Variable(_)))
        .collect();
    for selector in selectors {
        before(selector, context, scope);
        let value = state_type(selector, context, scope);
        if let Some(result) = context.observations.stored.get_mut(&selector.span()) {
            result.result = value.map(ExpressionResult::from_value);
            result.evaluated = true;
        }
    }
}

// ExpressionResult::askScopeVariableStateMatches, including its rule-facing
// wider-entry and born-inside-the-expression compatibility guards.
fn state_matches<A: Arena>(
    context: &Context<'_, '_, A>,
    result: &StoredResult,
    locals: &WordMap<Rc<TUnion>>,
    certainty: &dyn Fn(Word) -> VariableCertainty,
    calls: &[CallContext],
    rule_facing: bool,
) -> bool {
    if result.closure {
        return closure_context_matches(calls, &result.calls);
    }
    result.reads.iter().all(|name| {
        let ask = certainty(*name);
        let position = result.certainty[name];
        if rule_facing {
            if ask.is_absent() {
                return true;
            }
            if position.is_absent() {
                return false;
            }
        } else {
            if ask.is_absent() && position.is_absent() {
                return true;
            }
            if ask != position {
                return false;
            }
        }
        match (locals.get(name), result.locals.get(name)) {
            (None, _) if rule_facing => true,
            (None, None) => true,
            (Some(a), Some(b)) if a == b => true,
            (Some(a), Some(b)) if rule_facing => is_contained_by(
                context.codebase,
                b,
                a,
                false,
                false,
                false,
                &mut ComparisonResult::default(),
            ),
            _ => false,
        }
    })
}

fn state_type<'arena, A: Arena>(
    expression: &Expression<'arena>,
    context: &Context<'_, 'arena, A>,
    scope: &BlockContext<'_>,
) -> Option<TUnion> {
    if matches!(
        expression,
        Expression::Variable(_)
            | Expression::ArrayAccess(_)
            | Expression::Access(Access::Property(_) | Access::NullSafeProperty(_) | Access::StaticProperty(_))
    ) {
        get_block_expression_id(expression, context, scope)
            .filter(|id| !scope.variable_certainty(*id).is_absent())
            .and_then(|id| scope.locals.get(&id))
            .map(|t| (**t).clone())
    } else {
        None
    }
}

/// AssignHandler passes the results of its one target walk to the subsequent write.
/// Keep this handoff bounded to that assignment, including nested writes and errors.
pub(crate) fn collect_prepared_operands<'ctx, 'arena, A: Arena, T>(
    context: &mut Context<'ctx, 'arena, A>,
    walk: impl FnOnce(&mut Context<'ctx, 'arena, A>) -> T,
) -> (T, Vec<(Span, Option<ExpressionResult>)>) {
    context.observations.preparing.push(Vec::new());
    let result = walk(context);
    let operands = context.observations.preparing.pop().expect("the operand walk owns its completion list");
    (result, operands)
}

pub(crate) fn with_prepared_operands<'ctx, 'arena, A: Arena, T>(
    context: &mut Context<'ctx, 'arena, A>,
    operands: &[(Span, Option<ExpressionResult>)],
    write: impl FnOnce(&mut Context<'ctx, 'arena, A>) -> T,
) -> T {
    let depth = context.observations.prepared_operands.len();
    context.observations.prepared_operands.extend_from_slice(operands);
    let result = write(context);
    context.observations.prepared_operands.truncate(depth);
    result
}

/// Compatible child results are consumed without replaying their effects.
pub(crate) fn consume<'arena, A: Arena>(
    expression: &Expression<'arena>,
    context: &Context<'_, 'arena, A>,
    scope: &BlockContext<'_>,
    artifacts: &mut AnalysisArtifacts,
) -> bool {
    if let Some((_, result)) = context.observations.prepared_operands.iter().rev()
        .find(|(span, _)| *span == expression.span())
    {
        match result {
            Some(result) => artifacts.set_expression_result(expression, result.clone()),
            None => {
                let span = expression.span();
                artifacts.remove_expression_type(&(span.start.offset, span.end.offset));
            }
        }
        return true;
    }
    if !context.observations.pricing {
        return false;
    }
    let stored = context.observations.stored.get(&expression.span());
    if let Some(result) = stored.filter(|result| result.eager).and_then(|result| result.result.as_ref()) {
        artifacts.set_expression_result(expression, result.clone());
        return true;
    }
    if matches!(expression, Expression::Call(Call::Method(_)))
        && let Some(id) = get_block_expression_id(expression, context, scope)
        && scope.variable_certainty(id).is_defined()
        && let Some(value) = scope.locals.get(&id)
        && let Some(result) = stored.and_then(|result| result.result.as_ref())
    {
        let native = scope.native_locals.as_ref().and_then(|locals| locals.get(&id)).map(Rc::as_ref);
        artifacts.set_expression_result(expression, result.at_ask_position(Some((value, native)), None, None));
        return true;
    }
    if let Some(value) = state_type(expression, context, scope) {
        artifacts.set_expression_type(expression, value);
        return true;
    }
    let Some(result) = stored else {
        return false;
    };
    if !result.evaluated || !(context.consume_stored_expression_results || state_matches(
        context,
        result,
        &scope.locals,
        &|name| scope.variable_certainty(name),
        &context.observations.calls,
        false,
    )) {
        return false;
    }
    match &result.result {
        Some(result) => artifacts.set_expression_result(expression, result.clone()),
        None => {
            let span = expression.span();
            artifacts.remove_expression_type(&(span.start.offset, span.end.offset));
        }
    }
    true
}

fn unfiltered<A: Arena>(
    span: Span,
    call: &CallObservation<'_, '_>,
    context: &Context<'_, '_, A>,
    artifacts: &AnalysisArtifacts,
) -> Result<Option<TUnion>, AnalysisError> {
    // Raw target callbacks retain their entry position without a completed read.
    // Price those asks there instead of mistaking them for evaluated, untyped operands.
    let result = context.observations.stored.get(&span).expect("an observed expression retains its position");
    if result.evaluated
        && state_matches(
            context,
            result,
            &call.entry.locals,
            &*call.entry.certainty,
            &call.entry.calls,
            true,
        )
    {
        Ok(result.result.as_ref().map(|result| result.get_type().clone()))
    } else {
        (call.entry.price)(span, None, artifacts, &context.observations.stored, call.consumes_stored)
    }
}

fn filter<'ctx, 'arena, A: Arena>(
    selector: &Expression<'arena>,
    name: Word,
    scope: &mut BlockContext<'ctx>,
    context: &mut Context<'ctx, 'arena, A>,
    artifacts: &mut AnalysisArtifacts,
) {
    let literal = Expression::Literal(Literal::String(LiteralString {
        kind: LiteralStringKind::SingleQuoted,
        span: Span::zero(),
        raw: name.as_bytes(),
        value: Some(name.as_bytes()),
    }));
    artifacts.set_expression_type(&literal, get_literal_string(name));
    let predicate = new_synthetic_identical(context.arena, selector, &literal);
    let Some(clauses) = get_formula(
        predicate.span(),
        predicate.span(),
        &predicate,
        context.get_assertion_context_from_block(scope),
        artifacts,
        &context.settings.algebra_thresholds(),
        context.settings.formula_size_threshold,
    ) else {
        return;
    };
    let clauses = saturate_clauses(
        scope.clauses.iter().map(Rc::as_ref).chain(&clauses),
        &context.settings.algebra_thresholds(),
    );
    let (types, active) = find_satisfying_assignments(&clauses, None, &mut WordSet::default());
    reconcile_keyed_types(
        context,
        &types,
        active,
        scope,
        &mut WordSet::default(),
        &WordSet::default(),
        &predicate.span(),
        false,
        false,
    );
}

pub(crate) fn begin_call<'ctx, 'arena, A: Arena>(
    call: &MethodCall<'arena>,
    context: &mut Context<'ctx, 'arena, A>,
    scope: &BlockContext<'ctx>,
) -> Option<Span> {
    begin(&Expression::Call(Call::Method(call.clone())), call.object, &call.method, context, scope)
}

pub(crate) fn begin_nullsafe_call<'ctx, 'arena, A: Arena>(
    call: &NullSafeMethodCall<'arena>,
    context: &mut Context<'ctx, 'arena, A>,
    scope: &BlockContext<'ctx>,
) -> Option<Span> {
    begin(&Expression::Call(Call::NullSafeMethod(call.clone())), call.object, &call.method, context, scope)
}

fn begin<'ctx, 'arena, A: Arena>(
    root: &Expression<'arena>,
    receiver: &Expression<'arena>,
    method: &ClassLikeMemberSelector<'arena>,
    context: &mut Context<'ctx, 'arena, A>,
    scope: &BlockContext<'ctx>,
) -> Option<Span> {
    if context.observations.pricing || context.raw_callback_gather_depth > 0 {
        return None;
    }
    let request = context
        .settings
        .call_scope_observations
        .iter()
        .find(|r| r.call == root.span())?
        .clone();
    prepare_history(context);
    let (selector, identifier) = match method {
        ClassLikeMemberSelector::Identifier(n) => (None, Some(word(n.value))),
        ClassLikeMemberSelector::Variable(v) => (Some(&*context.arena.alloc(Expression::Variable(v.clone()))), None),
        ClassLikeMemberSelector::Expression(e) => (Some(e.expression), None),
        ClassLikeMemberSelector::Missing(_) => return None,
    };
    let entry = capture(root, selector, context, scope);
    context.observations.active.insert(
        root.span(),
        CallObservation {
            entry,
            selector,
            identifier,
            receiver: receiver.span(),
            request,
            evaluated: context
                .observations
                .retained
                .get(&root.span())
                .cloned()
                .unwrap_or_default(),
            consumes_stored: context.consume_stored_expression_results,
        },
    );
    Some(root.span())
}

/// NullsafeMethodCallHandler pre-stores the receiver at the ensured position,
/// then emits the plain twin from that same post-receiver scope.
pub(crate) fn enter_nullsafe_twin<'ctx, 'arena, A: Arena>(
    call: &NullSafeMethodCall<'arena>,
    receiver_type: &TUnion,
    authoritative: bool,
    context: &mut Context<'ctx, 'arena, A>,
    scope: &BlockContext<'ctx>,
) {
    if context.observations.pricing || context.raw_callback_gather_depth > 0 || context.observation_program.is_none() {
        return;
    }
    let root = Expression::Call(Call::NullSafeMethod(call.clone()));
    if let Some(selector) = context.observations.active.get(&call.span()).map(|call| call.selector) {
        let entry = capture(&root, selector, context, scope);
        let active = context.observations.active.get_mut(&call.span()).expect("active nullsafe twin");
        active.entry = entry;
        active.consumes_stored = true;
    }
    let mut receiver = call.object;
    loop {
        let span = receiver.span();
        let id = get_block_expression_id(receiver, context, scope);
        let native = id.and_then(|id| scope.native_locals.as_ref().and_then(|locals| locals.get(&id))).map(Rc::as_ref);
        if let Some(result) = context.observations.stored.get_mut(&span) {
            result.locals = scope.locals.clone();
            result.certainty = result.reads.iter().map(|name| (*name, scope.variable_certainty(*name))).collect();
            if let Some(own) = &result.result {
                result.result = Some(if authoritative && !matches!(receiver, Expression::Call(_)) {
                    ExpressionResult::from_value(receiver_type.clone())
                } else {
                    own.at_ask_position(authoritative.then_some((receiver_type, native)), None, None)
                });
            }
            result.eager = authoritative;
        }
        let calls: Vec<_> = context.observations.active.iter()
            .filter(|(_, call)| call.evaluated.contains_key(&span))
            .map(|(span, call)| (*span, call.selector)).collect();
        for (call, selector) in calls {
            let before = capture(receiver, selector, context, scope);
            context.observations.active.get_mut(&call).expect("active receiver observation")
                .evaluated.insert(span, EvaluatedExpression { before });
        }
        let Expression::Parenthesized(parenthesized) = receiver else { break };
        receiver = parenthesized.expression;
    }
    // Variable selectors bypass Expression::analyze. Move their retained
    // position from receiver completion to the twin's ensured entry scope.
    let selectors: Vec<_> = context.observations.active.values()
        .filter(|active| active.receiver == call.object.span())
        .filter_map(|active| active.selector)
        .filter(|selector| matches!(selector, Expression::Variable(_))).collect();
    for selector in selectors {
        before(selector, context, scope);
        let value = state_type(selector, context, scope);
        if let Some(result) = context.observations.stored.get_mut(&selector.span()) {
            result.result = value.map(ExpressionResult::from_value);
            result.evaluated = true;
        }
    }
}

// UnionType::pickFromTypes vetoes an alternative with no constant strings;
// IntersectionType concatenates its members, and template types use their bound.
fn constant_strings(ty: &TUnion) -> Vec<Word> {
    let mut names = Vec::new();
    for atomic in ty.types.iter() {
        let mut strings = match atomic {
            TAtomic::GenericParameter(parameter) => constant_strings(&parameter.constraint),
            _ => atomic.get_literal_string_value().map(word).into_iter().collect(),
        };
        for member in atomic.get_intersection_types().unwrap_or_default() {
            strings.extend(constant_strings(&TUnion::from_atomic(member.clone())));
        }
        if strings.is_empty() {
            return Vec::new();
        }
        names.extend(strings);
    }
    names
}

pub(crate) fn finish_call<A: Arena>(
    call: Option<Span>,
    context: &mut Context<'_, '_, A>,
    artifacts: &mut AnalysisArtifacts,
) -> Result<(), AnalysisError> {
    let Some(call) = call.and_then(|span| context.observations.active.remove(&span)) else {
        return Ok(());
    };
    let names: Vec<_> = if let Some(identifier) = call.identifier {
        vec![identifier]
    } else {
        let selector = call.selector.expect("a dynamic call retains its selector");
        unfiltered(selector.span(), &call, context, artifacts)?
            .into_iter()
            .flat_map(|t| constant_strings(&t))
            .collect()
    };
    let mut observations = Vec::new();
    for name in names {
        let mut types = HashMap::default();
        for span in &call.request.expressions {
            let Some(result) = call.evaluated.get(span) else {
                continue;
            };
            let value = if call.selector.is_some() {
                (result.before.price)(*span, Some(name), artifacts, &context.observations.stored, call.consumes_stored)?
            } else {
                unfiltered(*span, &call, context, artifacts)?
            };
            if let Some(value) = value {
                types.insert(*span, value);
            }
        }
        observations.push(NamedCallScope {
            name,
            filtered: call.selector.is_some(),
            types,
        });
    }
    artifacts
        .call_scope_observations
        .insert(call.request.call, observations.clone());
    context.observations.completed.insert(call.request.call, observations);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mago_codex::ttype::{atomic::generic::TGenericParameter, get_int, get_mixed, get_string};
    use mago_word::empty_word;
    use std::sync::Arc;

    #[test]
    fn raw_and_template_call_contexts_match_phpstans_public_matcher() {
        // Extension/raw contexts are not all constructible by the normal live walk.
        // These expectations come from closure-matcher.php calling PHPStan itself.
        let expected =
            include_str!("../../../crates/pun-analyse/tests/fixtures/call-scope-observations/closure-matcher.tsv");
        let method = FunctionLikeIdentifier::Method(word("Class"), word("read"));
        let function = FunctionLikeIdentifier::Function(word("map"));
        let template = |parent| {
            TUnion::from_atomic(TAtomic::GenericParameter(TGenericParameter::new(
                word("T"),
                Arc::new(get_mixed()),
                parent,
            )))
        };
        let cases = [
            ("no-asking-parameter", Some(method), None, Some(method), Some(get_int())),
            (
                "equal-full-type",
                Some(function),
                Some(get_int()),
                Some(method),
                Some(get_int()),
            ),
            ("no-position-callee", None, Some(get_string()), None, Some(get_int())),
            (
                "no-position-parameter",
                Some(method),
                Some(get_string()),
                Some(method),
                None,
            ),
            (
                "own-method-template",
                Some(method),
                Some(template(GenericParent::FunctionLike((word("Other"), word("READ"))))),
                Some(method),
                Some(get_int()),
            ),
            (
                "own-function-template",
                Some(function),
                Some(template(GenericParent::FunctionLike((empty_word(), word("MAP"))))),
                Some(function),
                Some(get_int()),
            ),
            (
                "foreign-function-template",
                Some(method),
                Some(template(GenericParent::FunctionLike((empty_word(), word("read"))))),
                Some(method),
                Some(get_int()),
            ),
            (
                "foreign-method-template",
                Some(method),
                Some(template(GenericParent::FunctionLike((
                    word("Other"),
                    word("different"),
                )))),
                Some(method),
                Some(get_int()),
            ),
            (
                "class-template",
                Some(method),
                Some(template(GenericParent::ClassLike(word("Other")))),
                Some(method),
                Some(get_int()),
            ),
        ];
        assert_eq!(cases.len(), expected.lines().count());
        for ((name, ask_callee, ask_type, callee, position_type), expectation) in
            cases.into_iter().zip(expected.lines())
        {
            let (expected_name, expected) = expectation.split_once('\t').unwrap();
            assert_eq!(name, expected_name);
            let ask = CallContext {
                argument: Span::zero(),
                callee: ask_callee,
                parameter_type: ask_type,
            };
            let position = CallContext {
                argument: Span::zero(),
                callee,
                parameter_type: position_type,
            };
            assert_eq!(closure_context_matches(&[ask], &[position]), expected == "1", "{name}");
        }
    }

    #[test]
    fn retained_context_debug_does_not_format_or_execute_pricing_closures() {
        let scope = PricingScope {
            locals: WordMap::default(),
            certainty: Rc::new(|_| panic!("formatting must not query state")),
            calls: Vec::new(),
            price: Rc::new(|_, _, _, _, _| panic!("formatting must not price expressions")),
        };
        assert_eq!(format!("{scope:?}"), "PricingScope { locals: {}, .. }");
    }
}
