use mago_allocator::Arena;
use std::rc::Rc;
use std::sync::Arc;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::combine_union_types;
use mago_codex::ttype::{get_error, get_never};
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::get_arraykey;
use mago_codex::ttype::get_mixed;
use mago_span::HasSpan;
use mago_syntax::cst::ArrayAccess;
use mago_syntax::cst::Expression;
use mago_word::{Word, WordMap};

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;
use crate::utils::expression::get_root_expression_id;
use crate::utils::expression::array::get_array_target_type_given_index;
use crate::utils::expression::expression_is_nullsafe;
use crate::utils::expression::get_array_access_id;
use crate::utils::expression::get_block_expression_id;

impl<'ast, 'arena> Analyzable<'ast, 'arena> for ArrayAccess<'arena> {
    fn analyze<'ctx, A>(
        &'ast self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError>
    where
        A: Arena,
    {
        let keyed_array_var_id = get_array_access_id(
            self,
            block_context.scope.get_class_like_name(),
            context.resolved_names,
            Some(context.codebase),
        );

        let extended_var_id = get_block_expression_id(self.array, context, block_context);
        let saved_narrowed_type = if matches!(self.index, Expression::UnaryPostfix(_)) {
            keyed_array_var_id.as_ref().and_then(|k| block_context.locals.get(k).cloned())
        } else {
            None
        };

        // ArrayDimFetchHandler walks the dimension before the receiver; a nullsafe chain's skipped world is the
        // scope before both.
        let short_circuit_scope =
            crate::utils::expression::expression_is_nullsafe(self.array).then(|| block_context.clone());

        let was_inside_isset = block_context.flags.inside_isset();
        let was_inside_general_use = block_context.flags.inside_general_use();
        let was_inside_unset = block_context.flags.inside_unset();

        block_context.flags.set_inside_isset(false);
        block_context.flags.set_inside_general_use(true);
        block_context.flags.set_inside_unset(false);

        self.index.analyze(context, block_context, artifacts)?;

        block_context.flags.set_inside_isset(was_inside_isset);
        block_context.flags.set_inside_unset(was_inside_unset);
        block_context.flags.set_inside_general_use(was_inside_general_use);

        let index_type =
            artifacts.get_rc_expression_type(&self.index).cloned().unwrap_or_else(|| Rc::new(get_arraykey()));

        let was_inside_general_use = block_context.flags.inside_general_use();
        block_context.flags.set_inside_general_use(true);
        self.array.analyze(context, block_context, artifacts)?;
        block_context.flags.set_inside_general_use(was_inside_general_use);
        if let Some(before) = short_circuit_scope
            && crate::context::non_nullability::may_short_circuit(self.array, artifacts)
        {
            crate::context::non_nullability::merge_short_circuit(block_context, &before, context);
        }

        let cached_type = if let Some(keyed_array_var_id) = &keyed_array_var_id
            && block_context.has_variable_atom(*keyed_array_var_id)
            && let Some(array_access_type) = block_context.locals.get(keyed_array_var_id).cloned()
        {
            Some(array_access_type)
        } else {
            None
        };
        let cached_type = cached_type.into_iter().chain(saved_narrowed_type).find(|cached_type| {
            !cached_type.is_error()
                && (!cached_type.possibly_undefined()
                    || keyed_array_var_id.as_ref().is_some_and(|k| memchr::memmem::find(k.as_bytes(), b"[$").is_some()))
        });

        // Failed reads and null placeholders do not prove offset presence. Isset probes
        // also need current presence flags, independently of a successfully narrowed value.
        if let Some(cached_type) = &cached_type
            && !cached_type.has_null()
            && !was_inside_isset
        {
            artifacts.set_rc_expression_type(self, Rc::clone(cached_type));

            return Ok(());
        }

        let container_type = artifacts.get_rc_expression_type(&self.array).cloned();

        if let Some(container_type) = container_type {
            let mut access_type = get_array_target_type_given_index(
                context,
                block_context,
                self.span(),
                self.array.span(),
                Some(self.index.span()),
                &container_type,
                &index_type,
                false,
                extended_var_id,
                None,
                container_type.can_be_null() && expression_is_nullsafe(self.array),
            );

            if let Some(cached_type) = cached_type
                && !access_type.is_null()
                && !access_type.is_error()
                && (!cached_type.is_null() || access_type.has_nullish())
            {
                let mut narrowed_type =
                    if !access_type.has_nullish() { cached_type.to_non_nullable() } else { (*cached_type).clone() };
                narrowed_type.set_possibly_undefined(
                    access_type.possibly_undefined(),
                    Some(access_type.possibly_undefined_from_try()),
                );
                access_type = narrowed_type;
            }
            let access_type = Rc::new(access_type);

            // A derived read is not a specified scope expression. Retain explicit
            // offset refinements above, but do not manufacture conditional guards
            // merely by observing a variable-key read.
            artifacts.set_rc_expression_type(self, access_type);
        } else {
            artifacts.set_expression_type(self, get_mixed());
        }

        Ok(())
    }
}

/// ArrayDimFetchHandler's native getOffsetValueType: price the already-visited
/// holder and key, never the selected read or a replay of either operand.
pub(crate) fn native_value(
    access: &ArrayAccess<'_>,
    codebase: &CodebaseMetadata,
    artifacts: &AnalysisArtifacts,
) -> Option<TUnion> {
    native_offset(
        artifacts.get_native_expression_type(access.array)?,
        artifacts.get_native_expression_type(access.index)?,
        codebase,
    ).map(|(value, _)| value)
}

/// The value and existence are independent: an optional slot has its when-defined
/// value, whereas an absent slot is ErrorType, not a nullable or PHPDoc placeholder.
pub(crate) fn native_offset(
    holder: &TUnion,
    index: &TUnion,
    codebase: &CodebaseMetadata,
) -> Option<(TUnion, Option<bool>)> {
    let keys: Option<Vec<_>> = index.types.iter().map(TAtomic::to_array_key).collect();
    let mut value = get_never();
    let mut present = Some(true);
    let mut absent = true;
    for atomic in holder.types.iter() {
        let (items, parameters) = match atomic {
            TAtomic::Array(TArray::Keyed(array)) => (
                array.known_items.as_ref().map(|items| items.iter().map(|(key, value)| (*key, value)).collect::<Vec<_>>()).unwrap_or_default(),
                array.parameters.as_ref().map(|(_, value)| value.as_ref()),
            ),
            TAtomic::Array(TArray::List(array)) => (
                array.known_elements.as_ref().map(|items| items.iter().map(|(key, value)| (ArrayKey::Integer(*key as i64), value)).collect()).unwrap_or_default(),
                (!array.element_type.is_never()).then_some(array.element_type.as_ref()),
            ),
            TAtomic::Mixed(mixed) if index.is_int() || index.is_string() => return Some((
                TUnion::from_atomic(TAtomic::Mixed(mago_codex::ttype::atomic::mixed::TMixed::new().with_is_explicit(mixed.is_explicit()))),
                None,
            )),
            TAtomic::Null => {
                present = None;
                continue;
            }
            _ => return None,
        };
        if let Some(keys) = &keys {
            for key in keys {
                if let Some((optional, item)) = items.iter().find_map(|(candidate, value)| (candidate == key).then_some(*value)) {
                    value = combine_union_types(&value, item, codebase, Default::default());
                    absent = false;
                    if *optional { present = None; }
                } else {
                    present = None;
                    if let Some(parameter) = parameters {
                        value = combine_union_types(&value, parameter, codebase, Default::default());
                        absent = false;
                    }
                }
            }
        } else if index.is_always_array_key(true) {
            present = None;
            // ConstantArrayType::getOffsetValueType's maybe-all fallback: a
            // nonconstant string can price an all-integer shape through PHP's
            // decimal-integer string keys, despite StringType::toArrayKey.
            let integer_string_fallback = index.has_string() && items.iter().all(|(key, _)| key.is_integer());
            for (key, (_, item)) in items {
                if (key.is_integer() && (index.has_int() || integer_string_fallback)) || (key.is_string() && index.has_string()) {
                    value = combine_union_types(&value, item, codebase, Default::default());
                    absent = false;
                }
            }
            if let Some(parameter) = parameters {
                value = combine_union_types(&value, parameter, codebase, Default::default());
                absent = false;
            }
        } else {
            return None;
        }
    }
    Some(if absent { (get_error(), Some(false)) } else { (value, present) })
}

/// AssignOpHandler::applyWrite commits the composed value to the prepared target.
/// The ordinary writer owns the selected lane; this transfer owns only native
/// array holders and consumes the original receiver/key observations.
pub(crate) fn native_write(
    target: &Expression<'_>,
    value: &TUnion,
    artifacts: &AnalysisArtifacts,
    locals: &WordMap<Rc<TUnion>>,
    codebase: &CodebaseMetadata,
) -> Option<(Word, TUnion)> {
    // before_expression admits only direct-local roots. Rebuild the prepared
    // chain inside out using the continuation's holder (including RHS writes),
    // but the original evaluated keys. Neither operand is evaluated again.
    let name = get_root_expression_id(target)?;
    let mut links = Vec::new();
    let mut target = target;
    while let Expression::ArrayAccess(access) = target {
        links.push(access);
        target = access.array;
    }
    let mut holder = locals.get(&name)?.as_ref().clone();
    let mut holders = Vec::new();
    for access in links.into_iter().rev() {
        let index = artifacts.get_native_expression_type(access.index)?;
        let key = index.get_single_array_key()?;
        holders.push((key, holder.clone()));
        holder = native_offset(&holder, index, codebase)?.0;
    }
    let mut written = value.clone();
    for (key, holder) in holders.into_iter().rev() {
        let mut arrays = get_never();
        for atomic in holder.types.iter() {
            let mut array = match atomic {
                TAtomic::Array(TArray::Keyed(array)) => array.clone(),
                TAtomic::Array(TArray::List(array)) if array.element_type.is_never() => {
                    TKeyedArray::new().with_known_items(array.known_elements.as_ref()?.iter()
                        .map(|(key, item)| (ArrayKey::Integer(*key as i64), item.clone())).collect())
                }
                // NullType::setOffsetValueType starts with an empty constant array.
                TAtomic::Null => TKeyedArray::new(),
                _ => return None,
            };
            Arc::make_mut(array.known_items.get_or_insert_with(Default::default)).insert(key, (false, written.clone()));
            array.non_empty = true;
            arrays = combine_union_types(&arrays, &TUnion::from_atomic(TAtomic::Array(TArray::Keyed(array))), codebase, Default::default());
        }
        written = arrays;
    }
    Some((name, written))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::code::IssueCode;
    use crate::test_analysis;

    test_analysis! {
        name = using_generic_parameter_as_index,
        code = indoc! {"
            <?php

            /**
             * @template T as string|lowercase-string
             *
             * @param array<T, bool> $old
             * @param array<T, bool> $new
             * @return array<T, bool>
             */
            function mergeThreadData(array $old, array $new): array {
                foreach ($new as $name => $value) {
                    if (!isset($old[$name]) || !$old[$name] && $value) {
                        $old[$name] = $value;
                    }
                }

                return $old;
            }
        "},
    }

    test_analysis! {
        name = negated_isset_narrows_parent_array_type,
        code = indoc! {"
            <?php

            /**
             * @return array{foo?: array{bar?: string}}
             */
            function y(): array { return []; }

            $y = y();

            if (isset($y['foo']) && !isset($y['foo']['bar'])) {
                echo $y['foo']['bar'];
                echo $y['foo']['baz'];
            }
        "},
        issues = [
            IssueCode::UndefinedStringArrayIndex,  // $y['foo']['bar']
            IssueCode::UndefinedStringArrayIndex,  // $y['foo']['baz']
        ],
    }

    test_analysis! {
        name = variable_key_narrowing_not_null,
        code = indoc! {"
            <?php

            /**
             * @param array<string, int|null> $map
             */
            function testNotNullVariableKeyNarrowing(array $map, string $key): int
            {
                if ($map[$key] !== null) {
                    return $map[$key];
                }

                return 0;
            }
        "},
    }
}
