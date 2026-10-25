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
use mago_codex::ttype::comparator::ComparisonResult;
use mago_codex::ttype::comparator::union_comparator::{can_expression_types_be_identical, is_contained_by};
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
use crate::reconciler::negated_assertion_reconciler;
use crate::utils::expression::get_block_expression_id;
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

                let Some(array_id) = get_block_expression_id(array_access.array, context, block_context) else {
                    break 'array_access;
                };

                let Some(key_type) = artifacts.get_expression_type(array_access.index) else {
                    break 'array_access;
                };

                let Some(array_variable) = block_context.locals.remove(&array_id) else {
                    break 'array_access;
                };

                let mut atomics = vec![];

                let array_key = key_type.get_single_array_key();
                for mut atomic in Rc::unwrap_or_clone(array_variable).types.into_owned() {
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

                        atomics.push(atomic);
                        continue;
                    }

                    if let TAtomic::GenericParameter(parameter) = &mut atomic {
                        // TemplateConstantArrayType::recreate preserves the template identity.
                        // ArrayType::unsetOffset returns an ordinary array for a compatible literal
                        // key, including when subtraction cannot express a more precise key type.
                        if let [TAtomic::Array(TArray::Keyed(array))] = parameter.constraint.types.as_ref() {
                            if array.parameters.is_none() {
                                let array = unset_keyed(context, array.clone(), key_type);
                                parameter.constraint =
                                    Arc::new(TUnion::from_atomic(TAtomic::Array(TArray::Keyed(array))));
                            } else if array.known_items.is_none()
                                && parameter.intersection_types.is_none()
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
                                let array = unset_keyed(context, array.clone(), key_type);
                                atomics.push(TAtomic::Array(TArray::Keyed(array)));
                                continue;
                            }
                        }
                        for intersection in parameter.intersection_types.iter_mut().flatten() {
                            if let TAtomic::Array(TArray::Keyed(array)) = intersection {
                                *array = unset_keyed(context, array.clone(), key_type);
                            }
                        }
                    }

                    let TAtomic::Array(array) = atomic else {
                        atomics.push(atomic);

                        continue;
                    };

                    match array {
                        TArray::List(array) => {
                            let TList { element_type, known_elements, known_count, non_empty } = array;

                            let Some(known_elements) = known_elements else {
                                atomics.push(TAtomic::Array(TArray::Keyed(TKeyedArray {
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
                            atomics.push(TAtomic::Array(TArray::Keyed(unset_keyed(context, array, key_type))));
                        }
                    }
                }

                let rc = Rc::new(TUnion::from_vec(atomics));

                block_context.locals.insert(array_id, Rc::clone(&rc));
                block_context.remove_variable_from_conflicting_clauses(context, array_id, Some(rc.as_ref()));
            };
        }

        block_context.flags.set_inside_unset(was_inside_unset);

        Ok(())
    }
}

/// ArrayType/ConstantArrayType::unsetOffset() for already-normalized array keys. Generic arrays
/// subtract a single literal; finite unions and general offsets only weaken matching shape entries.
/// Non-key inputs retain Mago's fallback until shared PHP scalar-to-key conversion is available here.
fn unset_keyed<A: Arena>(context: &mut Context<'_, '_, A>, mut array: TKeyedArray, offset: &TUnion) -> TKeyedArray {
    let Some(items) = array.known_items.as_mut() else {
        array.non_empty = false;
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
        return array;
    };
    let literal_keys: Vec<_> = offset.types.iter().filter_map(TAtomic::to_array_key).collect();
    if let Some(key) = offset.get_single_array_key() {
        Arc::make_mut(items).remove(&key);
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
            *optional |= matches;
        }
    }
    array.non_empty = items.values().any(|(optional, _)| !optional);
    if items.is_empty() && array.parameters.is_none() {
        array.known_items = None;
    }
    array
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
