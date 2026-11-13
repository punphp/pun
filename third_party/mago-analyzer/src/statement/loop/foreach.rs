use std::rc::Rc;
use std::sync::Arc;

use mago_allocator::Arena;
use mago_codex::assertion::Assertion;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::combine_union_types;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::get_arraykey;
use mago_codex::ttype::get_empty_keyed_array;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::get_object;
use mago_codex::ttype::union::TUnion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;
use mago_syntax::comments::docblock::PrecedingDocblocks;
use mago_syntax::cst::Expression;
use mago_syntax::cst::Foreach;
use mago_syntax::cst::ForeachTarget;
use mago_syntax::cst::UnaryPrefix;
use mago_syntax::cst::UnaryPrefixOperator;
use mago_syntax::cst::Variable;
use mago_word::Word;
use mago_word::WordMap;
use mago_word::concat_word;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::context::block::BreakContext;
use crate::context::scope::control_action::ControlAction;
use crate::context::scope::loop_scope::LoopScope;
use crate::context::scope::var_has_root;
use crate::error::AnalysisError;
use crate::expression::assignment::PropertyWriteKind;
use crate::expression::assignment::analyze_assignment_to_variable;
use crate::expression::assignment::assign_to_expression;
use crate::reconciler::assertion_reconciler::reconcile;
use crate::statement::analyze_statements;
use crate::statement::r#loop;
use crate::utils::expression::get_block_expression_id;

impl<'ast, 'arena> Analyzable<'ast, 'arena> for Foreach<'arena> {
    fn analyze<'ctx, A>(
        &'ast self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError>
    where
        A: Arena,
    {
        let iterator = self.expression;
        let is_by_reference = match &self.target {
            ForeachTarget::Value(v) => v.value.is_reference(),
            ForeachTarget::KeyValue(kv) => kv.value.is_reference(),
        };

        let iterator_variable_id = get_block_expression_id(iterator, context, block_context);

        let (always_enters_loop, mut key_type, mut value_type) =
            r#loop::analyze_iterator(context, block_context, artifacts, iterator, iterator_variable_id, self)?;

        // ForeachHandler keeps entry after evaluating the iterable: its assignments are unconditional.
        let entry = block_context.clone();
        let never_enters_loop = artifacts.get_expression_type(iterator).is_some_and(|ty| {
            ty.types.iter().all(|atomic| matches!(atomic, TAtomic::Array(array) if array.is_empty()))
        });
        let unrolled_arrays = unroll_array_mutations(context, block_context, artifacts, self, is_by_reference)?;

        // Pun: an empty array's key and value stay never (ForeachHandler's `getIterableValueType()` of `array{}`)
        if key_type.is_never() && !never_enters_loop {
            key_type = get_mixed();
        }

        if value_type.is_never() && !never_enters_loop {
            value_type = get_mixed();
        }

        let mut iterator_type = artifacts.get_expression_type(iterator);
        while let Some(ty) = iterator_type
            && let [TAtomic::GenericParameter(parameter)] = ty.types.as_ref()
        {
            iterator_type = Some(&parameter.constraint);
        }
        let array = iterator_type.and_then(TUnion::get_single_array);
        let callable_array = array.and_then(|array| array.callable_array_view(context.codebase)).map(TArray::Keyed);
        let constant_entries: Option<Vec<_>> = match callable_array.as_ref().or(array) {
            Some(TArray::Keyed(array)) if array.is_constant_shape() => Some(array
                .known_items
                .iter()
                .flat_map(|items| items.iter())
                .map(|(key, (_, value))| (key.to_union(), value.clone()))
                .collect()),
            Some(TArray::List(array)) if array.element_type.is_never() => {
                let entries: Vec<_> = array
                    .known_elements
                    .iter()
                    .flat_map(|items| items.iter())
                    .map(|(key, (_, value))| (ArrayKey::Integer(*key as i64).to_union(), value.clone()))
                    .collect();
                // getIterableKeyType() of a constant list has no unknown integer-key tail.
                // Keep the precise domain on the key assignment, not a later variable read.
                if let Some(keys) = entries.iter().map(|(key, _)| key.clone()).reduce(|left, right| {
                    combine_union_types(&left, &right, context.codebase, context.settings.combiner_options())
                }) {
                    key_type = keys;
                }
                Some(entries)
            }
            _ => None,
        };

        let mut loop_block_context = block_context.clone();
        if context.settings.pollute_scope_with_always_iterable_foreach
            && let Some(id) = iterator_variable_id
            && let Some(ty) = loop_block_context.locals.get(&id).cloned()
        {
            // `iteratee !== []`: retain non-array alternatives rather than asserting truthiness.
            let narrowed = reconcile(context, &Assertion::NonEmptyCountable(true), Some(&ty), None, true, None, false, false);
            loop_block_context.locals.insert(id, Rc::new(narrowed));
        }
        loop_block_context.flags.set_inside_loop(true);
        loop_block_context.break_types.push(BreakContext::Loop(self.span()));
        let mut key_expression_id = None;
        if let Some(key_expression) = self.target.key() {
            key_expression_id = get_block_expression_id(key_expression, context, block_context);

            let assigned = assign_to_expression(
                context,
                &mut loop_block_context,
                artifacts,
                key_expression,
                key_expression_id,
                None,
                Rc::new(key_type),
                false,
                PropertyWriteKind::Direct,
            )?;

            if !assigned {
                context.collector.report_with_code(
                    IssueCode::InvalidForeachKey,
                    Issue::error("The key expression in `foreach` is not assignable.")
                        .with_annotation(
                            Annotation::primary(key_expression.span())
                                .with_message("This expression cannot be assigned to"),
                        )
                        .with_note("The key expression must be writable.")
                        .with_help(
                            "Ensure the key expression is a valid variable, array, or an object property. If using a complex expression, consider assigning it to a variable first.",
                        ),
                );
            }
        }

        let value_expression = match self.target.value() {
            Expression::UnaryPrefix(UnaryPrefix { operator: UnaryPrefixOperator::Reference(_), operand }) => operand,
            value => value,
        };

        let value_expression_id = get_block_expression_id(value_expression, context, block_context);

        value_type.set_by_reference(is_by_reference);

        if is_by_reference && let Expression::Variable(Variable::Direct(direct_variable)) = value_expression {
            loop_block_context.references_to_external_scope.remove(&Word::from(direct_variable.name));
        }

        let source_value_type = value_type.clone();
        let assigned = assign_to_expression(
            context,
            &mut loop_block_context,
            artifacts,
            value_expression,
            value_expression_id,
            None,
            Rc::new(value_type),
            false,
            if is_by_reference { PropertyWriteKind::Mutation } else { PropertyWriteKind::Direct },
        )?;

        if !assigned {
            context.collector.report_with_code(
                IssueCode::InvalidForeachValue,
                Issue::error("The value expression in `foreach` is not assignable.")
                    .with_annotation(
                        Annotation::primary(value_expression.span())
                            .with_message("This expression cannot be assigned to"),
                    )
                    .with_note("The value expression must be writable.")
                    .with_help(
                        "Ensure the value expression is a valid variable, array, or an object property. If using a complex expression, consider assigning it to a variable first.",
                    ),
            );
        }

        if is_by_reference && let Expression::Variable(Variable::Direct(direct_variable)) = value_expression {
            loop_block_context.references_to_external_scope.insert(Word::from(direct_variable.name));
        }

        let foreach_entry = if !is_by_reference
            && matches!(iterator, Expression::Variable(Variable::Direct(_)))
            && self.target.key().is_some_and(|key| matches!(key, Expression::Variable(Variable::Direct(_))))
            && matches!(value_expression, Expression::Variable(Variable::Direct(_)))
            && let (Some(iterator_id), Some(key_id), Some(value_id)) =
                (iterator_variable_id, key_expression_id, value_expression_id)
        {
            let entry_id = concat_word!(iterator_id.as_bytes(), b"[", key_id.as_bytes(), b"]");
            loop_block_context.locals.insert(entry_id, Rc::new(source_value_type.clone()));
            loop_block_context.derived_local_sources.insert(value_id, entry_id);

            Some((iterator_id, key_id, value_id, entry_id, Rc::new(source_value_type)))
        } else {
            None
        };

        // ForeachHandler's conditional holders are independent of entry write-back above:
        // references and literal iterables also correlate each constant key with its value.
        if PrecedingDocblocks::new(context.comments, self.span().start.offset).next().is_none()
            && self.target.key().is_some_and(|key| matches!(key, Expression::Variable(Variable::Direct(_))))
            && matches!(value_expression, Expression::Variable(Variable::Direct(_)))
            && let Some(entries) = constant_entries
            && let (Some(key_id), Some(value_id)) = (key_expression_id, value_expression_id)
        {
            let offset_id = iterator_variable_id
                .filter(|_| matches!(iterator, Expression::Variable(Variable::Direct(_))))
                .map(|iterator_id| concat_word!(iterator_id.as_bytes(), b"[", key_id.as_bytes(), b"]"));
            for (key_type, value_type) in entries {
                loop_block_context.add_parameter_condition(key_id, key_type.clone(), value_id, value_type.clone());
                if let Some(offset_id) = offset_id {
                    loop_block_context.add_parameter_condition(key_id, key_type, offset_id, value_type);
                }
            }
        }

        let mut loop_scope = LoopScope::new(self.span(), block_context.locals.clone(), None);
        loop_scope.foreach_entry_scope = Some(entry.variable_scope());
        if let Some((_, _, _, entry_id, entry_type)) = &foreach_entry {
            loop_scope.parent_context_variables.insert(*entry_id, Rc::clone(entry_type));
            loop_scope.tracks_assignment_targets = true;
        }

        let (inner_loop_block_context, loop_scope) = r#loop::analyze(
            context,
            self.body.statements(),
            &[],
            vec![],
            loop_scope,
            &mut loop_block_context,
            block_context,
            artifacts,
            false,
            always_enters_loop,
        )?;

        // PHPStan joins only reachable body ends and local exits. Terminating-only bodies use
        // their final scope as the fallback before the empty/nonempty selection below.
        let mut exits = loop_scope.break_scopes.clone();
        exits.extend(loop_scope.foreach_continue_scopes.iter().cloned());
        exits.extend(loop_scope.foreach_body_scope.iter().cloned());
        let always_terminates = exits.is_empty();
        let mut final_context = inner_loop_block_context.clone();
        final_context.merge_variable_scope_types(&exits, context.codebase);

        let refined_entry = if let Some((iterator_id, key_id, value_id, entry_id, original_entry_type)) = foreach_entry
            && !loop_scope.final_actions.contains(ControlAction::Break)
            && assignments_only_target_current_entry(
                iterator_id,
                key_id,
                value_id,
                entry_id,
                &loop_scope.assignment_targets,
            )
            && let Some(entry_type) = inner_loop_block_context.locals.get(&entry_id)
            && entry_type != &original_entry_type
            && !entry_type.is_mixed()
        {
            Some((iterator_id, Rc::clone(entry_type)))
        } else {
            None
        };

        r#loop::inherit_loop_block_context(
            context,
            block_context,
            loop_block_context,
            inner_loop_block_context,
            loop_scope,
            always_enters_loop,
            /* infinite_loop = */ false,
        );

        if never_enters_loop || (always_enters_loop && always_terminates) {
            block_context.merge_variable_scope_types(&[entry.variable_scope()], context.codebase);
        } else if !always_enters_loop {
            // The zero-iteration edge is `iteratee === [] || is_object(iteratee)`, not the
            // original array type. A body replacing the array must not retain its old elements.
            let mut empty_entry = entry.clone();
            if let Some(id) = iterator_variable_id
                && let Some(ty) = empty_entry.locals.get(&id).cloned()
            {
                let mut empty = reconcile(context, &Assertion::IsIdentical(get_empty_keyed_array().get_single().clone()), Some(&ty), None, false, None, false, false);
                // Template types intersect with [] without erasing their identity.
                // A standalone [] is not an instantiation of every template argument.
                if let [TAtomic::GenericParameter(parameter)] = ty.types.as_ref()
                    && !empty.is_never()
                {
                    let mut parameter = parameter.clone();
                    let intersection = if parameter.constraint.is_list() {
                        TAtomic::Array(TArray::List(TList::from_known_elements(Default::default())))
                    } else {
                        empty.get_single().clone()
                    };
                    parameter.add_intersection_type(intersection);
                    empty = TUnion::from_atomic(TAtomic::GenericParameter(parameter));
                }
                let object = reconcile(context, &Assertion::IsType(get_object().get_single().clone()), Some(&ty), None, false, None, false, false);
                empty_entry.locals.insert(id, Rc::new(combine_union_types(&empty, &object, context.codebase, CombinerOptions::default())));
            }
            block_context.merge_variable_scope_types(&[empty_entry.variable_scope(), final_context.variable_scope()], context.codebase);
        } else {
            block_context.merge_variable_scope_types(&[final_context.variable_scope()], context.codebase);
        }
        if always_enters_loop && always_terminates {
            block_context.flags.set_has_returned(true);
        }

        // Refine after inheriting the loop context so a non-empty iterator's
        // widened type does not overwrite the fully transformed value type.
        if let Some((iterator_id, entry_type)) = refined_entry
            && let Some(iterator_type) = block_context.locals.get(&iterator_id)
            && let Some(refined) = refine_foreach_array_values(iterator_type, &entry_type)
        {
            block_context.locals.insert(iterator_id, Rc::new(refined));
        }

        if let Some(arrays) = unrolled_arrays {
            block_context.locals.extend(arrays);
        }

        // Array refinements and unrolling supply final types, not permission to create new holders.
        if always_enters_loop && !always_terminates && !context.settings.pollute_scope_with_always_iterable_foreach {
            let final_context = block_context.clone();
            project_without_pollution(context, block_context, &entry, final_context);
        }

        Ok(())
    }
}

/// MutatingScope::processAlwaysIterableForeachScopeWithoutPollute: final types, entry AND final
/// certainty, entry-only holders retained. This is a projection, not a real-edge value union.
fn project_without_pollution<'ctx, A: Arena>(
    context: &Context<'ctx, '_, A>,
    output: &mut BlockContext<'ctx>,
    entry: &BlockContext<'ctx>,
    mut final_context: BlockContext<'ctx>,
) {
    let mut typed_entry = entry.clone();
    for (name, ty) in &entry.locals {
        if final_context.variable_certainty(*name).is_absent() {
            final_context.locals.insert(*name, Rc::clone(ty));
            final_context.variable_state.absent.remove(name);
        }
    }
    for (name, ty) in &final_context.locals {
        if entry.variable_certainty(*name).is_absent() || !entry.locals.contains_key(name) {
            continue;
        }
        let mut ty = (**ty).clone();
        ty.set_possibly_undefined(!entry.variable_certainty(*name).is_defined(), None);
        ty.set_possibly_undefined_from_try(false);
        typed_entry.locals.insert(*name, Rc::new(ty));
    }
    let branches = [typed_entry.variable_scope(), final_context.variable_scope()];
    output.locals = final_context.locals;
    output.variables_possibly_in_scope = typed_entry.variables_possibly_in_scope;
    output.variables_possibly_in_scope.extend(final_context.variables_possibly_in_scope);
    output.variable_state = entry.variable_state.clone();
    output.merge_variable_scopes(&branches, context.codebase);
    // processAlwaysIterableForeachScopeWithoutPollute retains entry holders only.
    // A final-only offset value is not a certain expression in the projected scope.
    output.locals.retain(|name, _| !name.as_bytes().ends_with(b"]") || entry.locals.contains_key(name));
    output.variable_state.after_extract_call = entry.variable_state.after_extract_call;
}

/// ForeachHandler::tryProcessUnrolledConstantArrayForeach's sequential chain for array state.
/// The ordinary walk remains authoritative for observations and other locals. Only a single
/// sealed, required-key list without breaks or returns can replace its array exits.
fn unroll_array_mutations<'ctx, 'ast, 'arena, A: Arena>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
    statement: &'ast Foreach<'arena>,
    by_reference: bool,
) -> Result<Option<WordMap<Rc<TUnion>>>, AnalysisError> {
    if by_reference || context.plugin_registry.has_statement_hooks() {
        return Ok(None);
    }
    let Expression::Variable(Variable::Direct(value)) = statement.target.value() else {
        return Ok(None);
    };
    let key = match statement.target.key() {
        Some(Expression::Variable(Variable::Direct(key))) => Some(key),
        None => None,
        _ => return Ok(None),
    };
    let actions = ControlAction::from_statements(statement.body.statements().iter().collect(), vec![], None, true);
    if actions.iter().any(|action| !matches!(action, ControlAction::None | ControlAction::Continue)) {
        return Ok(None);
    }
    let entries = match artifacts.get_expression_type(statement.expression).map(|t| t.types.as_ref()) {
        Some([TAtomic::Array(TArray::List(array))]) if array.element_type.is_never() => array
            .known_elements
            .iter()
            .flat_map(|items| {
                items.iter().map(|(key, (optional, value))| {
                    (mago_codex::ttype::get_literal_int(*key as i64), *optional, value.clone())
                })
            })
            .collect::<Vec<_>>(),
        _ => return Ok(None),
    };
    if entries.is_empty() || entries.len() > 16 || entries.iter().any(|(_, optional, _)| *optional) {
        return Ok(None);
    }
    let factor = std::iter::successors(artifacts.loop_scope.as_ref(), |scope| scope.parent_loop.as_deref())
        .map(|scope| scope.foreach_unroll_factor)
        .max()
        .unwrap_or(1);
    if factor > 1 && factor * entries.len() > 8 {
        return Ok(None);
    }
    let factor = factor * entries.len();
    let mut chain = block_context.clone();
    chain.if_body_context = None;
    chain.finally_scope = None;
    chain.flags.set_inside_loop(true);
    chain.break_types.push(BreakContext::Loop(statement.span()));
    context.gather_callback(|context| {
        for (key_type, _, value_type) in entries {
            let mut scratch = AnalysisArtifacts::new().with_native_types(context.settings.track_native_types);
            scratch.loop_scope = artifacts.loop_scope.clone();
            let mut iteration = LoopScope::new(statement.span(), chain.locals.clone(), None);
            iteration.foreach_unroll_factor = factor;
            scratch.set_loop_scope(iteration);
            analyze_assignment_to_variable(
                context,
                &mut chain,
                &mut scratch,
                value.span(),
                None,
                Rc::new(value_type),
                value.name.into(),
                false,
            );
            if let Some(key) = key {
                analyze_assignment_to_variable(
                    context,
                    &mut chain,
                    &mut scratch,
                    key.span(),
                    None,
                    Rc::new(key_type),
                    key.name.into(),
                    false,
                );
            }
            analyze_statements(statement.body.statements(), context, &mut chain, &mut scratch)?;
            let mut next = scratch.loop_scope.unwrap().continue_locals;
            if !chain.flags.has_returned() {
                crate::context::scope::loop_scope::merge_exit_locals(&mut next, &chain.locals, context.codebase);
            }
            let Some(next) = next else {
                return Ok(None);
            };
            chain.locals = next;
            chain.flags.set_has_returned(false);
        }
        Ok(Some(
            chain
                .locals
                .into_iter()
                .filter(|(name, ty)| {
                    ty.types.iter().all(|atomic| matches!(atomic, TAtomic::Array(_)))
                        && block_context.locals.get(name) != Some(ty)
                })
                .collect(),
        ))
    })
}

fn assignments_only_target_current_entry(
    iterator_id: Word,
    key_id: Word,
    value_id: Word,
    entry_id: Word,
    assignment_targets: &mago_word::WordSet,
) -> bool {
    for variable_id in assignment_targets {
        if *variable_id == key_id
            || *variable_id == value_id
            || (*variable_id != entry_id && var_has_root(*variable_id, iterator_id))
        {
            return false;
        }
    }

    true
}

fn refine_foreach_array_values(array_type: &TUnion, value_type: &TUnion) -> Option<TUnion> {
    let mut refined = array_type.clone();

    for atomic in refined.types.to_mut() {
        let TAtomic::Array(array) = atomic else {
            return None;
        };

        if array.is_empty() {
            continue;
        }

        match array {
            TArray::List(list) => {
                if !list.element_type.is_never() {
                    list.element_type = Arc::new(value_type.clone());
                }

                if let Some(known_elements) = list.known_elements.as_mut().map(Arc::make_mut) {
                    for (_, element_type) in known_elements.values_mut() {
                        *element_type = value_type.clone();
                    }
                }
            }
            TArray::Keyed(keyed) => {
                if let Some(known_items) = keyed.known_items.as_mut() {
                    for (_, item_type) in Arc::make_mut(known_items).values_mut() {
                        *item_type = value_type.clone();
                    }
                }

                if let Some((key_type, _)) = keyed.parameters.as_ref() {
                    keyed.parameters = Some((Arc::clone(key_type), Arc::new(value_type.clone())));
                } else if keyed.known_items.is_none() {
                    keyed.parameters = Some((Arc::new(get_arraykey()), Arc::new(value_type.clone())));
                }
            }
        }
    }

    Some(refined)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::code::IssueCode;
    use crate::test_analysis;

    test_analysis! {
        name = foreach_basic,
        code = indoc! {"
            <?php

            namespace X;

            /**
             * @template T of int|float
             *
             * @param list<T> $numbers
             *
             * @return ($numbers is non-empty-list<T> ? T : null)
             *
             * @pure
             */
            function min(array $numbers): null|float|int
            {
                $min = null;
                foreach ($numbers as $number) {
                    if (null === $min || $number < $min) {
                        $min = $number;
                    }
                }

                return $min;
            }

            /**
             * @template T
             *
             * @param iterable<T> $numbers
             * @param (callable(T): numeric) $numeric_function
             *
             * @return T|null
             */
            function min_by(iterable $numbers, callable $numeric_function): mixed
            {
                $min = null;
                $min_num = null;
                foreach ($numbers as $value) {
                    $value_num = $numeric_function($value);
                    if (null === $min_num || $value_num <= $min_num) {
                        $min = $value;
                        $min_num = $value_num;
                    }
                }

                return $min;
            }

            /**
             * @template T of int|float
             *
             * @param T $first
             * @param T $second
             * @param T ...$rest
             *
             * @return T
             *
             * @pure
             */
            function minva(int|float $first, int|float $second, int|float ...$rest): int|float
            {
                $min = $first < $second ? $first : $second;
                foreach ($rest as $number) {
                    if ($number < $min) {
                        $min = $number;
                    }
                }

                return $min;
            }

            /**
             * @template T of int|float
             *
             * @param list<T> $numbers
             *
             * @return ($numbers is non-empty-list<T> ? T : null)
             *
             * @pure
             */
            function max(array $numbers): null|int|float
            {
                $max = null;
                foreach ($numbers as $number) {
                    if (null === $max || $number > $max) {
                        $max = $number;
                    }
                }

                return $max;
            }

            /**
             * @template T
             *
             * @param iterable<T> $numbers
             * @param (callable(T): numeric) $numeric_function
             *
             * @return T|null
             */
            function max_by(iterable $numbers, callable $numeric_function): mixed
            {
                $max = null;
                $max_num = null;
                foreach ($numbers as $value) {
                    $value_num = $numeric_function($value);
                    if (null === $max_num || $value_num >= $max_num) {
                        $max = $value;
                        $max_num = $value_num;
                    }
                }

                return $max;
            }

            /**
             * @template T of int|float
             *
             * @param T $first
             * @param T $second
             * @param T ...$rest
             *
             * @return T
             *
             * @pure
             */
            function maxva(int|float $first, int|float $second, int|float ...$rest): int|float
            {
                $max = $first > $second ? $first : $second;
                foreach ($rest as $number) {
                    if ($number > $max) {
                        $max = $number;
                    }
                }

                return $max;
            }
        "}
    }

    test_analysis! {
        name = iterating_over_intersection,
        code = indoc! {r#"
            <?php

            /**
             * @template K
             * @template-covariant V
             */
            interface Traversable
            {
            }

            /**
             * @template K
             * @template-covariant V
             *
             * @extends Traversable<K, V>
             */
            interface IteratorAggregate extends Traversable
            {
                /**
                 * @return Traversable<K, V>
                 */
                public function getIterator(): Traversable;
            }

            class X
            {
            }

            /**
             * @implements IteratorAggregate<int, string>
             */
            class Y extends X implements IteratorAggregate
            {
                /**
                 * @return Traversable<int, string>
                 */
                public function getIterator(): Traversable
                {
                    return $this->getIterator();
                }
            }

            /**
             * @return X&Traversable<int, string>
             */
            function y(): X
            {
                return new Y();
            }

            foreach (y() as $item) {
                echo $item . "\n";
            }
        "#},
        issues = [
            // Traversable: K and V not used in interface body
            IssueCode::UnusedTemplateParameter,
            IssueCode::UnusedTemplateParameter,
        ]
    }
}
