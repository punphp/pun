use mago_allocator::Arena;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;

use mago_bytes::BytesDisplay;
use mago_codex::assertion::Assertion;
use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::atomic::mixed::TMixed;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::combine_union_types;
use mago_codex::ttype::combine_union_types_preserving_array_shapes;
use mago_codex::ttype::combiner;
use mago_codex::ttype::comparator::ComparisonResult;
use mago_codex::ttype::comparator::union_comparator::{can_expression_types_be_identical, is_contained_by};
use mago_codex::ttype::get_error;
use mago_codex::ttype::get_int;
use mago_codex::ttype::union::TUnion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::Access;
use mago_syntax::cst::ClassLikeMemberSelector;
use mago_syntax::cst::Expression;
use mago_syntax::cst::Unset;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;
use crate::expression::assignment::array_assignment::update_type_with_key_values;
use crate::reconciler::negated_assertion_reconciler;
use crate::utils::expression::get_block_expression_id;
use crate::utils::misc::unwrap_expression;
use mago_word::concat_word;

impl<'ast, 'arena> Analyzable<'ast, 'arena> for Unset<'arena> {
    fn analyze<'ctx, A>(
        &'ast self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError>
    where
        A: Arena,
    {
        let was_inside_unset = block_context.flags.inside_unset();
        block_context.flags.set_inside_unset(true);

        for value in &self.values {
            let was_inside_general_use = block_context.flags.inside_general_use();
            block_context.flags.set_inside_general_use(true);
            value.analyze(context, block_context, artifacts)?;
            block_context.flags.set_inside_general_use(was_inside_general_use);

            let value_id = get_block_expression_id(value, context, block_context);

            if let Some(value_id) = &value_id {
                artifacts.record_loop_assignment_target(*value_id);

                block_context.remove_variable(value_id.as_bytes(), true, context);
                block_context.possibly_assigned_variable_ids.insert(*value_id);
                block_context.references_possibly_from_confusing_scope.remove(value_id);
                if matches!(value, Expression::Variable(_)) {
                    block_context.invalidate_variable_certainty(*value_id);
                    block_context.variables_possibly_in_scope.remove(value_id);
                    block_context.variable_state.absent.insert(*value_id);
                }
            }

            check_property_unset(context, artifacts, self.unset.span, value);

            'array_access: {
                let Expression::ArrayAccess(array_access) = value else {
                    break 'array_access;
                };

                let Some(key_type) = artifacts.get_expression_type(array_access.index) else {
                    break 'array_access;
                };

                let Some(array_variable) = artifacts.get_rc_expression_type(array_access.array).cloned() else {
                    break 'array_access;
                };

                let mut array_variable = Rc::unwrap_or_clone(array_variable);
                let mut atomics = vec![];

                let array_key = key_type.get_single_array_key();
                for mut atomic in array_variable.types.iter().cloned() {
                    if let TAtomic::Mixed(mixed) = &mut atomic {
                        *mixed = mixed.unset_offset(context.codebase);
                    }
                    if let TAtomic::Scalar(scalar) = &atomic {
                        let scalar_str = scalar.get_id();

                        context.collector.report_with_code(
                            IssueCode::InvalidUnset,
                            Issue::error(format!(
                                "Cannot apply `unset` to an offset of non-array type `{scalar_str}`."
                            ))
                            .with_annotation(
                                Annotation::primary(array_access.array.span())
                                    .with_message(format!("This has type `{scalar_str}`, not an array."))
                            )
                            .with_annotation(
                                Annotation::secondary(self.unset.span)
                                    .with_message("`unset` used here on a non-array type.")
                            )
                            .with_note(
                                "`unset` on an array offset requires the base variable to be an array or an object that implements `ArrayAccess`."
                            )
                            .with_note(
                                format!("Using `unset` on an offset of type `{scalar_str}` will cause a runtime error.")
                            )
                            .with_help(
                                "Ensure the variable is an array before attempting to unset an element."
                            ),
                        );

                        atomics.extend(get_error().types.into_owned());
                        continue;
                    }

                    if let TAtomic::GenericParameter(parameter) = &mut atomic {
                        // TemplateConstantArrayType::recreate preserves the template identity.
                        // ArrayType::unsetOffset returns an ordinary array for a compatible literal
                        // key, including when subtraction cannot express a more precise key type.
                        if let [TAtomic::Array(TArray::Keyed(array))] = parameter.constraint.types.as_ref() {
                            if array.is_constant_shape() {
                                let (mut updated, recreated) = unset_keyed(context, array.clone(), key_type);
                                // TemplateConstantArrayType's parent omits unsealed extras. Its
                                // displayed bound changes only when ConstantArrayType recreates it.
                                if recreated {
                                    updated.parameters = None;
                                }
                                parameter.constraint =
                                    Arc::new(TUnion::from_atomic(TAtomic::Array(TArray::Keyed(updated))));
                            } else if parameter.intersection_types.is_none()
                                && let Some((keys, _)) = &array.parameters
                                && let Some(key) = array_key
                                && can_expression_types_be_identical(
                                    context.codebase,
                                    keys,
                                    &key.to_union(),
                                    true,
                                    false,
                                )
                            {
                                let (array, _) = unset_keyed(context, array.clone(), key_type);
                                atomics.push(TAtomic::Array(TArray::Keyed(array)));
                                continue;
                            }
                        }
                        for intersection in parameter.intersection_types.iter_mut().flatten() {
                            if let TAtomic::Array(TArray::Keyed(array)) = intersection {
                                *array = unset_keyed(context, array.clone(), key_type).0;
                            }
                        }
                    }

                    let TAtomic::Array(array) = atomic else {
                        atomics.push(atomic);

                        continue;
                    };

                    match array {
                        TArray::List(array) => {
                            let TList { callable, element_type, known_elements, known_count, non_empty } = array;

                            let Some(known_elements) = known_elements else {
                                atomics.push(TAtomic::Array(TArray::Keyed(TKeyedArray {
                                    callable,
                                    known_items: None,
                                    parameters: Some((Arc::new(get_int()), element_type)),
                                    non_empty: if let Some(known_count) = known_count {
                                        known_count > 1 // we removed 1, so we are non-empty if we had more than 1
                                    } else {
                                        false
                                    },
                                    known_non_list: true,
                                })));

                                continue;
                            };

                            let mut known_elements = Arc::unwrap_or_clone(known_elements);

                            let Some(ArrayKey::Integer(target_index)) = &array_key else {
                                if array_key.is_none() {
                                    atomics.push(TAtomic::Array(TArray::List(TList {
                                        callable,
                                        known_elements: Some(
                                            // We don't know the key value, so we can't unset it.
                                            // Mark all items as potentially undefined.
                                            known_elements
                                                .into_iter()
                                                .map(|(index, mut element)| {
                                                    // Mark the item as potentially undefined.
                                                    element.0 = true;

                                                    (index, element)
                                                })
                                                .collect::<BTreeMap<_, _>>()
                                                .into(),
                                        ),
                                        element_type,
                                        known_count,
                                        // Mark the list as potentially undefined.
                                        non_empty: true,
                                    })));
                                } else {
                                    // Keep everything as is, attempting to remove a string key from a list
                                    // makes no sense.
                                    // An error will be emitted when we are analyzing the expression above, so ignore it here.
                                    atomics.push(TAtomic::Array(TArray::List(TList {
                                        callable,
                                        known_elements: Some(known_elements.into()),
                                        element_type,
                                        known_count,
                                        non_empty,
                                    })));
                                }

                                continue;
                            };

                            let is_fixed_list = element_type.is_never();
                            let maintain_list = is_fixed_list && {
                                let elements_count = known_elements.len();
                                let last_index = elements_count - 1;

                                *target_index == (last_index as i64)
                            };

                            let mut element_removed = false;
                            known_elements.retain(|index, _| {
                                if *target_index < 0 {
                                    // this is a negative index, which means we can't remove it
                                    // from the list
                                    true
                                } else {
                                    let index = *index as i64;

                                    if index == *target_index {
                                        element_removed = true;

                                        false
                                    } else {
                                        true
                                    }
                                }
                            });

                            if !element_removed {
                                atomics.push(TAtomic::Array(TArray::List(TList {
                                    callable,
                                    known_elements: Some(known_elements.into()),
                                    element_type,
                                    known_count,
                                    non_empty,
                                })));

                                continue;
                            }

                            let non_empty = !known_elements.is_empty();
                            let known_count = Some(known_elements.len());
                            let known_elements = if known_elements.is_empty() {
                                // Completely empty now.
                                None
                            } else {
                                Some(known_elements)
                            };

                            if maintain_list {
                                atomics.push(TAtomic::Array(TArray::List(TList {
                                    callable,
                                    known_count,
                                    non_empty,
                                    known_elements: known_elements.map(Into::into),
                                    element_type,
                                })));
                            } else {
                                // we removed an integer index from a list such as:
                                // `list{1, 2, 3, ...<int>}`, which means the keys are no longer sequential.
                                // this makes the list no longer a list array, but rather a keyed array.
                                // we need to convert the list into a keyed array.
                                atomics.push(TAtomic::Array(TArray::Keyed(TKeyedArray {
                                    callable,
                                    known_items: known_elements.map(|known_elements| {
                                        known_elements
                                            .into_iter()
                                            .map(|(index, element)| (ArrayKey::Integer(index as i64), element))
                                            .collect::<BTreeMap<_, _>>()
                                            .into()
                                    }),
                                    parameters: if element_type.is_never() {
                                        None
                                    } else {
                                        Some((Arc::new(get_int()), element_type))
                                    },
                                    non_empty,
                                    known_non_list: true,
                                })));
                            }
                        }
                        TArray::Keyed(array) => {
                            atomics.push(TAtomic::Array(TArray::Keyed(unset_keyed(context, array, key_type).0)));
                        }
                    }
                }

                array_variable.types = Cow::Owned(combiner::combine_preserving_array_shapes(
                    atomics,
                    context.codebase,
                    context.settings.combiner_options(),
                ));
                write_back(context, block_context, artifacts, array_access.array, array_variable);
            };
        }

        block_context.flags.set_inside_unset(was_inside_unset);

        Ok(())
    }
}

/// UnsetHandler's ExistingArrayDimFetch virtual assignment: consume only stored input results,
/// replace each containing value from the inside out, and leave those input observations intact.
fn write_back<'ctx, 'arena, A: Arena>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &AnalysisArtifacts,
    mut receiver: &Expression<'arena>,
    mut value: TUnion,
) {
    receiver = unwrap_expression(receiver);
    while let Expression::ArrayAccess(access) = receiver
        && let Some(parent) = artifacts.get_expression_type(access.array)
        && let Some(index) = artifacts.get_rc_expression_type(access.index)
    {
        value = set_existing(context, parent, index, &value);
        receiver = unwrap_expression(access.array);
    }
    // A missing stored link cannot be re-evaluated as part of a virtual assignment.
    if !matches!(receiver, Expression::ArrayAccess(_))
        && let Some(root) = get_block_expression_id(receiver, context, block_context)
    {
        let previous = block_context.locals.get(&root).cloned();
        block_context.remove_descendants(context, root, previous.as_deref().unwrap_or(&value), Some(&value));
        block_context.invalidate_variable_certainty(root);
        block_context.locals.insert(root, Rc::new(value));
        block_context.assigned_variable_ids.insert(root, receiver.span().start.offset);
        block_context.possibly_assigned_variable_ids.insert(root);
    }
}

/// ConstantArrayTypeBuilder's finite union offsets are weak writes: join the single-key alternatives
/// instead of assigning every possible offset. Reuse the full assignment analyzer's array updater.
fn set_existing<A: Arena>(
    context: &mut Context<'_, '_, A>,
    parent: &TUnion,
    index: &Rc<TUnion>,
    value: &TUnion,
) -> TUnion {
    let keys: Vec<_> = index.types.iter().filter_map(TAtomic::to_array_key).map(|key| key.to_atomic()).collect();
    let mut result = parent.clone();
    let mut atomics = Vec::new();
    for mut atomic in parent.types.iter().cloned() {
        if atomic.is_null() {
            atomics.push(atomic);
        } else if matches!(atomic, TAtomic::Scalar(_)) {
            atomics.extend(get_error().types.into_owned());
        } else if let TAtomic::Array(TArray::Keyed(array)) = &mut atomic
            && array.known_items.is_none()
            && let Some((_, items)) = &mut array.parameters
        {
            *items = Arc::new(set_existing_items(context, items, value));
            atomics.push(atomic);
        } else if let TAtomic::Array(TArray::List(array)) = &mut atomic
            && array.known_elements.is_none()
            && !array.element_type.is_never()
        {
            array.element_type = Arc::new(set_existing_items(context, &array.element_type, value));
            atomics.push(atomic);
        } else {
            let parent = TUnion::from_atomic(atomic);
            let written = if !keys.is_empty() && keys.len() == index.types.len() {
                keys.iter()
                    .map(|key| {
                        update_type_with_key_values(context, parent.clone(), value, &vec![key.clone()], Some(index))
                    })
                    .reduce(|left, right| {
                        combine_union_types(&left, &right, context.codebase, context.settings.combiner_options())
                    })
                    .unwrap()
            } else {
                update_type_with_key_values(context, parent, value, &vec![], Some(index))
            };
            atomics.extend(written.types.into_owned());
        }
    }
    result.types = Cow::Owned(combiner::combine_preserving_array_shapes(
        atomics,
        context.codebase,
        context.settings.combiner_options(),
    ));
    result
}

/// ArrayType::setExistingOffsetValueType updates values without asserting an offset exists.
/// Written shapes update their present keys in each original shape; they are not replacements.
fn set_existing_items<A: Arena>(context: &mut Context<'_, '_, A>, previous: &TUnion, value: &TUnion) -> TUnion {
    let previous_arrays: Option<Vec<_>> = previous.types.iter().map(constant_array).collect();
    let written_arrays: Option<Vec<_>> = value.types.iter().map(constant_array).collect();
    if let (Some(previous_arrays), Some(written_arrays)) = (previous_arrays, written_arrays) {
        let mut variants = Vec::new();
        let mut changed = false;
        for written in written_arrays {
            let entries = written.known_items.as_deref().cloned().unwrap_or_default();
            if entries.values().any(|(optional, _)| *optional) && previous_arrays.len() == 1 {
                let mut array = previous_arrays[0].clone();
                let items = Arc::make_mut(array.known_items.get_or_insert_default());
                for (key, (optional, value)) in entries {
                    if optional && let Some((_, old)) = items.get_mut(&key) {
                        *old = combine_union_types(old, &value, context.codebase, context.settings.combiner_options());
                    } else {
                        items.insert(key, (optional, value));
                    }
                }
                array.non_empty |= items.values().any(|(optional, _)| !optional);
                let mut updated = previous.clone();
                updated.types = Cow::Owned(vec![TAtomic::Array(TArray::Keyed(array))]);
                variants.push(updated);
                changed = true;
                continue;
            }
            let mut updated = previous.clone();
            let mut optional_keys = Vec::new();
            for (key, (optional, value)) in entries {
                let index = Rc::new(key.to_union());
                updated = set_existing(context, &updated, &index, &value);
                changed = true;
                if optional {
                    optional_keys.push(index);
                }
            }
            variants.push(updated.clone());
            if !optional_keys.is_empty() {
                for index in optional_keys {
                    updated.types = Cow::Owned(
                        updated
                            .types
                            .iter()
                            .map(|atomic| {
                                let array = constant_array(atomic).unwrap();
                                TAtomic::Array(TArray::Keyed(unset_keyed(context, array, &index).0))
                            })
                            .collect(),
                    );
                }
                variants.push(updated);
            }
        }
        if changed {
            return variants
                .into_iter()
                .reduce(|left, right| {
                    combine_union_types_preserving_array_shapes(
                        &left,
                        &right,
                        context.codebase,
                        context.settings.combiner_options(),
                    )
                })
                .unwrap();
        }
        return combine_union_types_preserving_array_shapes(
            previous,
            value,
            context.codebase,
            context.settings.combiner_options(),
        );
    }
    combine_union_types(previous, value, context.codebase, context.settings.combiner_options())
}

/// Constant-array extraction also recognizes TemplateConstantArrayType's bound.
fn constant_array(atomic: &TAtomic) -> Option<TKeyedArray> {
    match atomic {
        TAtomic::Array(TArray::Keyed(array)) if array.parameters.is_none() => Some(array.clone()),
        TAtomic::Array(TArray::List(array)) if array.element_type.is_never() => Some(TKeyedArray {
            callable: array.callable.clone(),
            known_items: array.known_elements.as_ref().map(|items| {
                Arc::new(items.iter().map(|(key, value)| (ArrayKey::Integer(*key as i64), value.clone())).collect())
            }),
            parameters: None,
            non_empty: array.non_empty,
            known_non_list: false,
        }),
        TAtomic::GenericParameter(parameter) if parameter.constraint.is_single() => {
            constant_array(parameter.constraint.get_single())
        }
        _ => None,
    }
}

/// ArrayType/ConstantArrayType::unsetOffset() for already-normalized array keys. Generic arrays
/// subtract a single literal; finite unions and general offsets only weaken matching shape entries.
/// Non-key inputs retain Mago's fallback until shared PHP scalar-to-key conversion is available here.
/// The flag records ConstantArrayType::recreate(), even when matching keys were already optional.
fn unset_keyed<A: Arena>(
    context: &mut Context<'_, '_, A>,
    mut array: TKeyedArray,
    offset: &TUnion,
) -> (TKeyedArray, bool) {
    if !array.is_constant_shape() {
        // A generic array's known entries model HasOffsetValue, not ConstantArrayType. Its
        // ArrayType subtracts literal keys even when an unrelated asserted offset survives.
        if let Some(key) = offset.get_single_array_key()
            && let Some(items) = &mut array.known_items
        {
            Arc::make_mut(items).remove(&key);
        }
        if let Some((keys, _)) = &mut array.parameters
            && let Some(key) = offset.get_single_array_key()
        {
            let remaining = subtract_key(context, keys, key);
            if remaining.is_never() {
                array.parameters = None;
            } else {
                *keys = Arc::new(remaining);
            }
        }
        // NonEmptyArrayType::unsetOffset is ErrorType; surviving HasOffsetValue accessories
        // re-establish non-emptiness. A union/general offset does not remove those accessories.
        array.non_empty = array.known_items.as_ref().is_some_and(|items| items.values().any(|(o, _)| !o));
        return (array, false);
    }
    let Some(items) = array.known_items.as_mut() else {
        return (array, false);
    };
    let literal_keys: Vec<_> = offset.types.iter().filter_map(TAtomic::to_array_key).collect();
    let mut recreated = false;
    if let Some(key) = offset.get_single_array_key() {
        recreated = Arc::make_mut(items).remove(&key).is_some();
    } else {
        for (key, (optional, _)) in Arc::make_mut(items) {
            let matches = if offset.is_always_array_key(true) && !literal_keys.is_empty() {
                literal_keys.contains(key)
            } else {
                !offset.is_always_array_key(true)
                    || is_contained_by(
                        context.codebase,
                        &key.to_union(),
                        offset,
                        false,
                        false,
                        false,
                        &mut ComparisonResult::new(),
                    )
            };
            recreated |= matches;
            *optional |= matches;
        }
    }
    array.non_empty = items.values().any(|(optional, _)| !optional);
    if items.is_empty() && array.parameters.is_none() {
        array.known_items = None;
    }
    (array, recreated)
}

/// TypeCombinator::remove and TemplateTypeTrait::tryRemove: use the existing strict literal
/// reconciler for scalar subtraction, recursively preserving a template key's identity and flags.
fn subtract_key<A: Arena>(context: &mut Context<'_, '_, A>, keys: &TUnion, key: ArrayKey) -> TUnion {
    let mut remaining = Vec::new();
    for atomic in keys.types.as_ref() {
        if let TAtomic::GenericParameter(parameter) = atomic {
            let constraint = subtract_key(context, &parameter.constraint, key);
            if !constraint.is_never() {
                remaining.push(TAtomic::GenericParameter(parameter.with_constraint(constraint)));
            }
        } else if let TAtomic::Mixed(mixed) = atomic {
            remaining.extend(mixed.subtract(&key.to_union(), context.codebase).types.into_owned());
        } else if matches!(atomic, TAtomic::Scalar(TScalar::ArrayKey)) {
            // ArrayType::__construct normalizes the benevolent array-key alias to mixed,
            // unlike an explicit int|string union. Mago preserves that source distinction.
            remaining.extend(TMixed::new().subtract(&key.to_union(), context.codebase).types.into_owned());
        } else {
            let narrowed = negated_assertion_reconciler::reconcile(
                context,
                &Assertion::IsNotIdentical(key.to_atomic()),
                &TUnion::from_atomic(atomic.clone()),
                None,
                keys.get_id(),
                None,
                false,
            );
            remaining.extend(narrowed.types.into_owned().into_iter().filter(|atomic| !atomic.is_never()));
        }
    }
    let mut result = keys.clone();
    if remaining.is_empty() {
        remaining.push(TAtomic::Never);
    }
    result.types = Cow::Owned(remaining);
    result
}

fn check_property_unset<'arena, A>(
    context: &mut Context<'_, 'arena, A>,
    artifacts: &AnalysisArtifacts,
    unset_keyword_span: Span,
    value: &Expression<'arena>,
) where
    A: Arena,
{
    let Expression::Access(Access::Property(property_access)) = value else {
        return;
    };

    let ClassLikeMemberSelector::Identifier(property_name) = &property_access.property else {
        return;
    };

    let Some(object_type) = artifacts.get_expression_type(property_access.object) else {
        return;
    };

    for atomic in object_type.types.as_ref() {
        let TAtomic::Object(object) = atomic else {
            continue;
        };

        let Some(class_name) = object.get_name() else {
            continue;
        };

        let qualified_property_name = concat_word!(b"$", property_name.value);
        let Some(property_metadata) =
            context.codebase.get_declaring_property(class_name.as_bytes(), qualified_property_name.as_bytes())
        else {
            continue;
        };

        let class_is_readonly =
            context.codebase.get_class_like(class_name.as_bytes()).is_some_and(|class| class.flags.is_readonly());

        let qualified = format!("{}::${}", BytesDisplay(class_name.as_bytes()), BytesDisplay(property_name.value));

        if property_metadata.flags.is_readonly() || class_is_readonly {
            context.collector.report_with_code(
                IssueCode::InvalidUnset,
                Issue::error(format!("Cannot unset readonly property `{qualified}`."))
                    .with_annotation(
                        Annotation::primary(value.span()).with_message("This readonly property cannot be unset."),
                    )
                    .with_annotation(Annotation::secondary(unset_keyword_span).with_message("`unset` used here."))
                    .with_note("Unsetting a readonly property throws an `Error` at runtime.")
                    .with_help("Remove this property from the `unset` statement."),
            );

            return;
        }

        if !property_metadata.hooks.is_empty() || property_metadata.flags.is_virtual_property() {
            context.collector.report_with_code(
                IssueCode::InvalidUnset,
                Issue::error(format!("Cannot unset hooked property `{qualified}`."))
                    .with_annotation(
                        Annotation::primary(value.span()).with_message("This hooked property cannot be unset."),
                    )
                    .with_annotation(Annotation::secondary(unset_keyword_span).with_message("`unset` used here."))
                    .with_note("Unsetting a property that declares hooks throws an `Error` at runtime.")
                    .with_help("Remove this property from the `unset` statement."),
            );

            return;
        }
    }
}
