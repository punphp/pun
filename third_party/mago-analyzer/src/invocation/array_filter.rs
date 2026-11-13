//! PHPStan's ArrayFilterFunctionReturnTypeHelper for a callback whose body is one expression: an arrow function, or a
//! closure whose only statement is `return <expr>;`. Each key and value of the array is assigned to the callback's
//! parameters and narrowed by the expression's truthy specification (`filterByTruthyValue()`).

use std::rc::Rc;
use std::sync::Arc;

use mago_allocator::Arena;
use mago_codex::identifier::function_like::FunctionLikeIdentifier;
use mago_codex::ttype::add_union_type;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::KnownItems;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::get_array_parameters;
use mago_codex::ttype::get_empty_keyed_array;
use mago_codex::ttype::get_keyed_array;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::wrap_atomic;
use mago_span::HasSpan;
use mago_codex::assertion::Assertion;
use mago_syntax::cst::Expression;
use mago_syntax::cst::PartialApplication;
use mago_syntax::cst::Statement;
use mago_word::Word;
use mago_word::WordSet;
use mago_word::word;
use mago_algebra::AssertionMap;
use mago_algebra::assertion_set::AssertionSet;

use crate::artifacts::AnalysisArtifacts;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::context::scope::var_has_root;
use crate::invocation::Invocation;
use crate::plugin::context::InvocationInfo;
use crate::reconciler::reconcile_keyed_types;

/// `array_filter($array, <single-expression callback>[, $mode])`'s return type, `None` where PHPStan's helper takes
/// another route (no such callback, a non-array argument, a non-constant mode).
pub(crate) fn return_type<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
    invocation: &Invocation<'ctx, '_, 'arena>,
) -> Option<TUnion>
where
    A: Arena,
{
    let Some(FunctionLikeIdentifier::Function(name)) = invocation.target.get_function_like_identifier() else {
        return None;
    };
    if !name.as_bytes().eq_ignore_ascii_case(b"array_filter") {
        return None;
    }
    let info = InvocationInfo::new(invocation);
    let array_argument = info.get_argument(0, &[b"array"])?;
    // a missing or `null` callback removes the falsey values (`filterArrayRemovingFalsey()`)
    let callback = info
        .get_argument(1, &[b"callback"])
        .filter(|callback| !artifacts.get_expression_type(callback).is_some_and(TUnion::is_null));
    // determineMode(): ARRAY_FILTER_USE_BOTH is 1 and ARRAY_FILTER_USE_KEY 2; anything else uses the item
    let (use_key, use_item) = match info.get_argument(2, &[b"mode"]).filter(|_| callback.is_some()) {
        None => (false, true),
        Some(mode) => match artifacts.get_expression_type(mode)?.get_single_literal_int_value()? {
            1 => (true, true),
            2 => (true, false),
            _ => (false, true),
        },
    };
    // createDummyArgs(): a function named by a constant string or a first-class callable is called with `$item`
    // (and `$key`); its assertions on those arguments narrow them
    let dummies = || match (use_key, use_item) {
        (false, _) => vec![word("$item")],
        (true, false) => vec![word("$key")],
        (true, true) => vec![word("$item"), word("$key")],
    };
    let called = |context: &Context<'ctx, 'arena, A>, name: &[u8]| {
        let function = context.codebase.get_function(name)?;
        let dummies = dummies();
        let rename = |assertions: &std::collections::BTreeMap<Word, Vec<mago_codex::assertion::Assertion>>| {
            let mut map: AssertionMap<Word, AssertionSet> = AssertionMap::default();
            for (parameter, dummy) in function.parameters.iter().zip(&dummies) {
                if let Some(found) = assertions.get(&parameter.get_name().0) {
                    map.insert(*dummy, found.iter().map(|a| vec![a.clone()]).collect());
                }
            }
            map
        };
        let body = Body::Assertions(rename(&function.if_true_assertions), rename(&function.if_false_assertions));
        Some((dummies, body))
    };
    let (names, body): (Vec<Word>, Body<'_, '_>) = match callback {
        None => {
            let only = |assertion| {
                let mut map: AssertionMap<Word, AssertionSet> = AssertionMap::default();
                map.insert(word("$item"), vec![vec![assertion]]);
                map
            };
            (vec![word("$item")], Body::Assertions(only(Assertion::Truthy), only(Assertion::Falsy)))
        }
        Some(Expression::ArrowFunction(arrow)) => {
            (arrow.parameter_list.parameters.iter().map(|p| word(p.variable.name)).collect(), Body::Expression(arrow.expression))
        }
        Some(Expression::Closure(closure)) => match closure.body.statements.as_slice() {
            [Statement::Return(ret)] => (
                closure.parameter_list.parameters.iter().map(|p| word(p.variable.name)).collect(),
                Body::Expression(ret.value?),
            ),
            _ => return None,
        },
        Some(Expression::PartialApplication(PartialApplication::Function(partial)))
            if partial.argument_list.is_first_class_callable() =>
        {
            let Expression::Identifier(identifier) = partial.function else { return None };
            let name = context.resolve_function_name(identifier);
            called(context, name)?
        }
        Some(callback) => {
            let callback_type = artifacts.get_expression_type(callback)?;
            let [TAtomic::Scalar(mago_codex::ttype::atomic::scalar::TScalar::String(name))] = callback_type.types.as_ref() else {
                return None;
            };
            let name = name.get_known_literal_atom()?;
            called(context, name.as_bytes().strip_prefix(b"\\").unwrap_or(name.as_bytes()))?
        }
    };
    let (item, key) = match (use_key, use_item) {
        (false, _) => (Some(*names.first()?), None),
        (true, false) => (None, Some(*names.first()?)),
        (true, true) => (Some(*names.first()?), names.get(1).copied()),
    };
    let array_type = artifacts.get_expression_type(array_argument)?.clone();
    if array_type.types.is_empty() || !array_type.types.iter().all(|t| matches!(t, TAtomic::Array(_))) {
        return None;
    }

    let mut filter = Filter { block_context, artifacts, body, span: callback.map_or_else(|| array_argument.span(), HasSpan::span), item, key };
    let mut result: Option<TUnion> = None;
    for atomic in array_type.types.iter() {
        let TAtomic::Array(array) = atomic else { return None };
        let filtered = filter.array(context, array);
        result = Some(match result {
            Some(acc) => add_union_type(acc, &filtered, context.codebase, CombinerOptions::default()),
            None => filtered,
        });
    }
    result
}

/// The callback's returned expression, or a called function's assertions when true and when false.
enum Body<'ast, 'arena> {
    Expression(&'ast Expression<'arena>),
    Assertions(AssertionMap<Word, AssertionSet>, AssertionMap<Word, AssertionSet>),
}

struct Filter<'b, 'ctx, 'ast, 'arena> {
    block_context: &'b BlockContext<'ctx>,
    artifacts: &'b AnalysisArtifacts,
    body: Body<'ast, 'arena>,
    span: mago_span::Span,
    item: Option<Word>,
    key: Option<Word>,
}

impl<'ctx> Filter<'_, 'ctx, '_, '_> {
    /// `filterByTruthyValue()` of one array: a constant array entry by entry, any other by its key and value types.
    fn array<A: Arena>(&mut self, context: &mut Context<'ctx, '_, A>, array: &TArray) -> TUnion {
        let (known, rest) = match array {
            TArray::Keyed(keyed) if keyed.known_items.is_some() => (
                keyed.known_items.as_deref().cloned().unwrap_or_default(),
                keyed.parameters.as_ref().map(|(k, v)| ((**k).clone(), (**v).clone())),
            ),
            TArray::List(list) if list.known_elements.is_some() => (
                list.known_elements
                    .as_deref()
                    .into_iter()
                    .flatten()
                    .map(|(i, entry)| (ArrayKey::Integer(*i as i64), entry.clone()))
                    .collect(),
                (!list.element_type.is_never()).then(|| {
                    (get_array_parameters(array, context.codebase).0, (*list.element_type).clone())
                }),
            ),
            _ => {
                let (key, value) = get_array_parameters(array, context.codebase);
                let (key, value, _) = self.entry(context, key, value);
                return if key.is_never() || value.is_never() {
                    get_empty_keyed_array()
                } else {
                    get_keyed_array(key, value)
                };
            }
        };
        let mut items = KnownItems::default();
        for (array_key, (optional, value)) in known {
            let key_type = array_key.to_union();
            let (new_key, new_value, filtered_optional) = self.entry(context, key_type.clone(), value.clone());
            if new_key.is_never() || new_value.is_never() {
                continue;
            }
            // ConstantArrayTypeBuilder: an entry the callback narrows becomes optional
            let optional = if new_value == value && new_key == key_type { optional || filtered_optional } else { true };
            items.insert(array_key, (optional, new_value));
        }
        let mut keyed = TKeyedArray::new();
        keyed.non_empty = items.values().any(|(optional, _)| !optional);
        if !items.is_empty() {
            keyed = keyed.with_known_items(items);
        }
        if let Some((key, value)) = rest {
            let (key, value, _) = self.entry(context, key, value);
            if !key.is_never() && !value.is_never() {
                keyed = keyed.with_parameters(Arc::new(key), Arc::new(value));
            }
        }
        wrap_atomic(TAtomic::Array(TArray::Keyed(keyed)))
    }

    /// `processKeyAndItemType()`: the key and value types in the truthy scope, and whether the falsey scope is
    /// possible (the entry may be removed).
    fn entry<A: Arena>(&mut self, context: &mut Context<'ctx, '_, A>, key: TUnion, value: TUnion) -> (TUnion, TUnion, bool) {
        let mut scope = self.block_context.clone();
        scope.clauses.clear();
        for (variable, ty) in [(self.item, &value), (self.key, &key)] {
            let Some(variable) = variable else { continue };
            scope.locals.retain(|name, _| !var_has_root(*name, variable));
            scope.invalidate_variable_certainty(variable);
            scope.locals.insert(variable, Rc::new(ty.clone()));
            scope.variables_possibly_in_scope.insert(variable);
        }
        let read = |scope: &BlockContext<'_>, variable: Option<Word>, otherwise: &TUnion| {
            variable.and_then(|v| scope.locals.get(&v).map(|t| (**t).clone())).unwrap_or_else(|| otherwise.clone())
        };
        let truthy = self.specify(context, &scope, true);
        let falsey = self.specify(context, &scope, false);
        let optional = !(read(&falsey, self.item, &value).is_never() || read(&falsey, self.key, &key).is_never());
        (read(&truthy, self.key, &key), read(&truthy, self.item, &value), optional)
    }

    /// MutatingScope::filterByTruthyValue() / filterByFalseyValue() of the callback's expression.
    fn specify<A: Arena>(&self, context: &mut Context<'ctx, '_, A>, scope: &BlockContext<'ctx>, truthy: bool) -> BlockContext<'ctx> {
        let mut scope = scope.clone();
        let (assertions, span) = match &self.body {
            Body::Assertions(when_true, when_false) => {
                (if truthy { when_true.clone() } else { when_false.clone() }, self.span)
            }
            Body::Expression(expression) => {
                let span = expression.span();
                let thresholds = context.settings.algebra_thresholds();
                let size = context.settings.formula_size_threshold;
                let assertion_context = context.get_assertion_context_from_block(&scope);
                let Some(mut clauses) =
                    crate::formula::get_formula(span, span, expression, assertion_context, self.artifacts, &thresholds, size)
                else {
                    return scope;
                };
                if !truthy {
                    clauses = crate::formula::negate_or_synthesize(
                        clauses,
                        expression,
                        context.get_assertion_context_from_block(&scope),
                        self.artifacts,
                        &thresholds,
                        size,
                    );
                }
                (mago_algebra::find_satisfying_assignments(&clauses, None, &mut WordSet::default()).0, span)
            }
        };
        reconcile_keyed_types(
            context,
            &assertions,
            Default::default(),
            &mut scope,
            &mut WordSet::default(),
            &WordSet::default(),
            &span,
            false,
            false,
        );
        scope
    }
}
