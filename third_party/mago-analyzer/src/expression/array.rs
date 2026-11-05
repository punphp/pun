use mago_allocator::Arena;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;

use foldhash::HashSet;

use mago_codex::ttype::TType;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::KnownItems;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::atomic::mixed::TMixed;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::int::TInteger;
use mago_codex::ttype::atomic::scalar::string::TString;
use mago_codex::ttype::combine_union_types;
use mago_codex::ttype::combiner::combine;
use mago_codex::ttype::comparator::ComparisonResult;
use mago_codex::ttype::comparator::union_comparator;
use mago_codex::ttype::comparator::union_comparator::is_contained_by;
use mago_codex::ttype::get_arraykey;
use mago_codex::ttype::get_empty_keyed_array;
use mago_codex::ttype::get_int;
use mago_codex::ttype::get_iterable_parameters;
use mago_codex::ttype::get_literal_int;
use mago_codex::ttype::get_literal_string;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::get_never;
use mago_codex::ttype::get_string;
use mago_codex::ttype::union::TUnion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::cst::Array;
use mago_syntax::cst::ArrayElement;
use mago_syntax::cst::Expression;
use mago_syntax::cst::LegacyArray;
use mago_syntax::cst::UnaryPrefix;
use mago_syntax::cst::UnaryPrefixOperator;
use mago_syntax::cst::VariadicArrayElement;
use mago_word::WordSet;
use mago_word::ascii_lowercase_word;
use mago_word::empty_word;
use mago_word::word;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;
use crate::utils::expression::get_block_expression_id;
use crate::utils::misc::unwrap_expression;

/// Analyzes array literals and their elements.
///
/// Example:
///
/// ```php
/// $array = [
///    'key1' => 'value1',
///    'key2' => 'value2',
/// ];
/// ```
impl<'ast, 'arena> Analyzable<'ast, 'arena> for Array<'arena> {
    fn analyze<'ctx, A>(
        &'ast self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError>
    where
        A: Arena,
    {
        analyze_array_elements(context, block_context, artifacts, self.span(), self.elements.as_slice())
    }
}

/// Analyzes a legacy array literal.
///
/// Example:
///
/// ```php
/// $array = array('key1' => 'value1', 'key2' => 'value2');
/// ```
impl<'ast, 'arena> Analyzable<'ast, 'arena> for LegacyArray<'arena> {
    fn analyze<'ctx, A>(
        &'ast self,
        context: &mut Context<'ctx, 'arena, A>,
        block_context: &mut BlockContext<'ctx>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError>
    where
        A: Arena,
    {
        analyze_array_elements(context, block_context, artifacts, self.span(), self.elements.as_slice())
    }
}

#[derive(Debug)]
struct ArrayCreationInfo {
    item_key_atomic_types: Vec<TAtomic>,
    item_value_atomic_types: Vec<TAtomic>,
    property_types: KnownItems,
    class_strings: WordSet,
    can_create_objectlike: bool,
    array_keys: HashSet<ArrayKey>,
    int_offset: i64,
    is_list: bool,
    known_non_list: bool,
    can_be_empty: bool,
    known_int_offset: bool,
    finite_key_indexes: Option<Vec<i64>>,
    property_order: Vec<ArrayKey>,
    degraded_finite_keys: bool,
    has_general_unpack: bool,
}

// PHPStan 2.3.1 ConstantArrayTypeBuilder::setOffsetValueType and
// IntegerRangeType::getFiniteTypes: all alternatives must be bounded, and a
// non-singleton offset must have fewer than ARRAY_COUNT_LIMIT (256) scalars.
fn finite_array_keys(key_type: &TUnion) -> Option<Vec<ArrayKey>> {
    let mut keys = Vec::new();
    let ranges =
        key_type.types.iter().all(|atomic| matches!(atomic, TAtomic::Scalar(TScalar::Integer(TInteger::Range(..)))));
    for atomic in key_type.types.iter() {
        if let Some(value) = atomic.get_literal_string_value() {
            keys.push(ArrayKey::from_string(word(value)));
        } else if let Some(value) = atomic.get_literal_int_value() {
            keys.push(ArrayKey::Integer(value));
        } else if ranges && let TAtomic::Scalar(TScalar::Integer(TInteger::Range(min, max))) = atomic {
            if i128::from(*max) - i128::from(*min) > 128 {
                break;
            }
            keys.extend((*min..=*max).map(ArrayKey::Integer));
        } else {
            return None;
        }
    }
    let mut seen: HashSet<_> = HashSet::default();
    keys.retain(|key| seen.insert(*key));
    (!keys.is_empty() && keys.len() < 256).then_some(keys)
}

fn degrade_finite_array(info: &mut ArrayCreationInfo) {
    info.degraded_finite_keys = true;
    info.can_create_objectlike = false;
    for (key, (_, value)) in std::mem::take(&mut info.property_types) {
        info.item_key_atomic_types.extend(key.to_union().types.into_owned());
        info.item_value_atomic_types.extend(value.types.into_owned());
    }
}

fn set_finite_array_offset<A: Arena>(
    context: &Context<'_, '_, A>,
    info: &mut ArrayCreationInfo,
    keys: Option<&[ArrayKey]>,
    value: &TUnion,
    optional: bool,
) {
    let indexes = info.finite_key_indexes.get_or_insert_with(|| {
        let maximum = info.property_types.keys().filter_map(ArrayKey::get_integer).max().unwrap_or(-1).max(-1);
        maximum.checked_add(1).into_iter().collect()
    });
    if keys.is_none() && indexes.is_empty() {
        return;
    }
    if info.degraded_finite_keys {
        let keys = keys.map_or_else(|| indexes.iter().copied().map(ArrayKey::Integer).collect(), <[ArrayKey]>::to_vec);
        for key in keys {
            info.item_key_atomic_types.extend(key.to_union().types.into_owned());
        }
        info.item_value_atomic_types.extend(value.types.iter().cloned());
        return;
    }
    let union = |old: &TUnion| combine_union_types(old, value, context.codebase, context.settings.combiner_options());
    match keys {
        None => {
            // The append branch visits existing slots in insertion order, not
            // key order. Only the first matched optional slot becomes required.
            let maximum = indexes.iter().copied().max().unwrap();
            let mut next = if optional { indexes.clone() } else { Vec::new() };
            let mut has_optional = false;
            for key in &info.property_order {
                if let ArrayKey::Integer(index) = key
                    && indexes.contains(index)
                {
                    let (slot_optional, old) = info.property_types.get_mut(key).unwrap();
                    *old = union(old);
                    if !has_optional && !optional {
                        *slot_optional = false;
                    }
                    next.extend(index.checked_add(1));
                    has_optional = true;
                }
            }
            let key = ArrayKey::Integer(maximum);
            info.property_order.push(key);
            info.property_types.insert(key, (optional || has_optional, value.clone()));
            next.extend(maximum.checked_add(1));
            next.sort_unstable();
            next.dedup();
            *indexes = next;
        }
        Some([key]) => {
            let existed = info.property_types.contains_key(key);
            if !existed {
                info.property_order.push(*key);
            }
            let (slot_optional, next_value) =
                if optional && let Some((was_optional, old)) = info.property_types.get(key) {
                    (*was_optional, union(old))
                } else {
                    (optional, value.clone())
                };
            info.property_types.insert(*key, (slot_optional, next_value));
            if let ArrayKey::Integer(index) = key {
                if existed && !optional {
                    indexes.retain(|next| next > index);
                } else if !existed && indexes.iter().max().is_some_and(|maximum| index >= maximum) {
                    // Unpacked integer slots use the append branch above;
                    // explicit integer offsets in a literal are required.
                    *indexes = index.checked_add(1).into_iter().collect();
                }
            }
        }
        Some(keys) => {
            // A finite union is not a union of singleton arrays: matched
            // optional slots are replaced, matched required slots are widened,
            // and only unmatched offsets become optional slots.
            for key in keys {
                if let Some((was_optional, old)) = info.property_types.get_mut(key) {
                    *old = if *was_optional && !optional { value.clone() } else { union(old) };
                    continue;
                }
                info.property_order.push(*key);
                info.property_types.insert(*key, (true, value.clone()));
                if let ArrayKey::Integer(index) = key
                    && indexes.iter().max().is_some_and(|maximum| index >= maximum)
                {
                    indexes.extend(index.checked_add(1));
                }
                info.known_non_list = true;
            }
        }
    }
    if info.property_types.len() > 256 {
        degrade_finite_array(info);
    }
}

fn report_unpacked_int_key_overflow<A>(context: &mut Context<'_, '_, A>, element_span: Span)
where
    A: Arena,
{
    context.collector.report_with_code(
        IssueCode::InvalidArrayIndex,
        Issue::error("Cannot add an item with an offset beyond `PHP_INT_MAX`.")
            .with_annotation(Annotation::primary(element_span).with_message("Adding this item would result in an invalid integer key."))
            .with_note(format!("PHP automatically assigns integer keys starting from the highest previous integer key. The next key would exceed `PHP_INT_MAX` ({}).", i64::MAX))
            .with_note("This usually happens in very large arrays or after using an explicit integer key close to the maximum.")
            .with_help("Consider using an explicit string key for this item, restructuring the array, or ensuring previous explicit integer keys are smaller."),
    );
}

fn analyze_array_elements<'ctx, 'arena, A>(
    context: &mut Context<'ctx, 'arena, A>,
    block_context: &mut BlockContext<'ctx>,
    artifacts: &mut AnalysisArtifacts,
    expression_span: Span,
    elements: &[ArrayElement<'arena>],
) -> Result<(), AnalysisError>
where
    A: Arena,
{
    if elements.is_empty() {
        artifacts.set_expression_type(&expression_span, get_empty_keyed_array());
        if block_context.native_locals.is_some() {
            artifacts.set_rc_native_expression_type(&expression_span, Rc::new(get_empty_keyed_array()));
        }

        return Ok(());
    }

    let passed_to_type = artifacts.array_passed_to_type.take();
    let mut array_creation_info = ArrayCreationInfo {
        item_key_atomic_types: Vec::new(),
        item_value_atomic_types: Vec::new(),
        property_types: KnownItems::default(),
        class_strings: WordSet::default(),
        can_create_objectlike: true,
        array_keys: HashSet::default(),
        int_offset: -1,
        is_list: true,
        known_non_list: false,
        can_be_empty: true,
        known_int_offset: true,
        finite_key_indexes: None,
        property_order: Vec::new(),
        degraded_finite_keys: false,
        has_general_unpack: false,
    };

    for element in elements {
        let (item_key_value, key_type, item_is_list_item, value) = match element {
            ArrayElement::KeyValue(key_value_array_element) => {
                let was_inside_general_use = block_context.flags.inside_general_use();
                block_context.flags.set_inside_general_use(true);
                key_value_array_element.key.analyze(context, block_context, artifacts)?;
                block_context.flags.set_inside_general_use(was_inside_general_use);

                let (item_key_value, key_type) = artifacts
                    .get_expression_type(key_value_array_element.key)
                    .map_or((None, get_mixed()), |item_key_type| {
                        let key_type = if item_key_type.is_null() {
                            get_literal_string(empty_word())
                        } else if item_key_type.is_true() {
                            get_literal_int(1)
                        } else if item_key_type.is_false() {
                            get_literal_int(0)
                        } else if let Some(f) = item_key_type.get_single_literal_float_value() {
                            get_literal_int(f.trunc() as i64)
                        } else if item_key_type.is_float() {
                            get_int()
                        } else if !item_key_type.is_always_array_key(true) {
                            let item_key_type_id = item_key_type.get_id();

                            context.collector.report_with_code(
                                IssueCode::InvalidArrayElementKey,
                                Issue::error("Invalid array key type.")
                                    .with_annotation(
                                        Annotation::primary(key_value_array_element.key.span()).with_message(format!(
                                            "This has type `{item_key_type_id}`, which cannot be cast to a string or integer.",
                                        )),
                                    )
                                    .with_note(format!(
                                        "In PHP, array keys must be strings or integers. While types like `bool` or `float` are automatically cast, a value of type `{item_key_type_id}` cannot be.",
                                    ))
                                    .with_help("Ensure the array key is either a string or an integer."),
                            );

                            get_arraykey()
                        } else {
                            item_key_type.clone()
                        };

                        let item_key_value =
                            if let Some(item_key_literal_type) = key_type.get_single_literal_string_value() {
                                Some(ArrayKey::from_string(word(item_key_literal_type)))
                            } else if let Some(literal_integer) = key_type.get_single_literal_int_value() {
                                // The most recent integer key becomes the next available integer key
                                array_creation_info.int_offset = literal_integer;

                                Some(ArrayKey::Integer(literal_integer))
                            } else if let Some(class_string) = key_type.get_single_class_string_value() {
                                array_creation_info.class_strings.insert(class_string);

                                Some(ArrayKey::String(class_string))
                            } else {
                                None
                            };

                        (item_key_value, key_type)
                    });

                (item_key_value, key_type, false, key_value_array_element.value)
            }
            ArrayElement::Value(value_array_element) => {
                // check if we have reached PHP_INT_MAX
                if array_creation_info.int_offset == i64::MAX {
                    context.collector.report_with_code(
                        IssueCode::InvalidArrayIndex,
                        Issue::error(
                            "Cannot add array item implicitly; the next available integer key would exceed PHP_INT_MAX."
                        )
                        .with_annotation(
                            Annotation::primary(value_array_element.span())
                                .with_message("Adding this item would result in an invalid integer key.")
                        )
                        .with_note(
                            format!("PHP automatically assigns integer keys starting from the highest previous integer key. The next key would exceed `PHP_INT_MAX` ({}).", i64::MAX)
                        )
                        .with_note(
                            "This usually happens in very large arrays or after using an explicit integer key close to the maximum."
                        )
                        .with_help(
                            "Consider using an explicit string key for this item, restructuring the array, or ensuring previous explicit integer keys are smaller."
                        ),
                    );

                    break;
                }

                if array_creation_info.known_int_offset {
                    array_creation_info.int_offset += 1;

                    (
                        Some(ArrayKey::Integer(array_creation_info.int_offset)),
                        get_literal_int(array_creation_info.int_offset),
                        true,
                        value_array_element.value,
                    )
                } else {
                    (None, get_int(), true, value_array_element.value)
                }
            }
            ArrayElement::Variadic(variadic_array_element) => {
                let was_inside_general_use = block_context.flags.inside_general_use();
                block_context.flags.set_inside_general_use(true);
                variadic_array_element.value.analyze(context, block_context, artifacts)?;
                block_context.flags.set_inside_general_use(was_inside_general_use);

                if let Some(variadic_array_element_type) = artifacts.get_expression_type(&variadic_array_element.value)
                {
                    handle_variadic_array_element(
                        context,
                        &mut array_creation_info,
                        variadic_array_element,
                        variadic_array_element_type,
                    );
                } else {
                    array_creation_info.can_create_objectlike = false;
                    array_creation_info.item_key_atomic_types.push(TAtomic::Scalar(TScalar::ArrayKey));
                    array_creation_info.item_value_atomic_types.push(TAtomic::Mixed(TMixed::new()));
                }

                continue;
            }
            ArrayElement::Missing(missing_array_element) => {
                context.collector.report_with_code(
                    IssueCode::InvalidArrayElement,
                    Issue::error(
                        "Missing array element: skipping elements is only allowed in list assignments (destructuring)."
                    )
                    .with_annotation(
                        Annotation::primary(missing_array_element.span())
                            .with_message("Element expected here.")
                    )
                    .with_note(
                        "Array literals require a value for each position (e.g., `[1, null, 3]`) unless used on the left side of an assignment (e.g., `[$a, , $c] = ...;`)."
                    )
                    .with_help("Provide a value for this element (e.g., `null`) or remove the extra comma."),
                );

                continue;
            }
        };

        // ArrayHandler reads each value before its separate by-reference escape flow.
        // Taking an array element reference is not a variable-creating assignment.
        if let Expression::UnaryPrefix(UnaryPrefix { operator: UnaryPrefixOperator::Reference(_), operand }) =
            unwrap_expression(value)
        {
            block_context.variable_state.non_creating_references.insert(operand.span());
        }

        // ArrayHandler passes a closure or array item the expected value type: a closure's parameters take its
        // callables' parameter types
        if let Some(passed_to_type) = &passed_to_type {
            match unwrap_expression(value) {
                Expression::Closure(_) | Expression::ArrowFunction(_) => {
                    artifacts.inferred_parameter_types =
                        Some(crate::invocation::arguments::callable_parameter_types(context, passed_to_type));
                }
                Expression::Array(_) | Expression::LegacyArray(_) => {
                    artifacts.array_passed_to_type =
                        crate::invocation::arguments::expected_value_type(context, passed_to_type);
                }
                _ => {}
            }
        }

        let was_inside_general_use = block_context.flags.inside_general_use();
        block_context.flags.set_inside_general_use(true);
        value.analyze(context, block_context, artifacts)?;
        block_context.flags.set_inside_general_use(was_inside_general_use);
        artifacts.array_passed_to_type = None;

        array_creation_info.can_be_empty = false;
        array_creation_info.is_list &= item_is_list_item;

        if let Some(item_key_value) = item_key_value {
            let is_duplicate = array_creation_info.array_keys.contains(&item_key_value);
            if !is_duplicate
                && !matches!(
                    item_key_value,
                    ArrayKey::Integer(index)
                        if usize::try_from(index).is_ok_and(|index| index == array_creation_info.array_keys.len())
                )
            {
                array_creation_info.known_non_list = true;
            }

            if is_duplicate {
                context.collector.report_with_code(
                    IssueCode::DuplicateArrayKey,
                    Issue::error(format!(
                        "Duplicate array key `{item_key_value}` detected."
                    ))
                    .with_annotation(
                        Annotation::primary(element.span())
                            .with_message(format!("This key `{item_key_value}` duplicates an earlier key"))
                    )
                    .with_note(
                        "Using the same key multiple times in an array literal will overwrite the previous value associated with that key."
                    )
                    .with_help(
                        "Remove the duplicate entry or use a unique key for this element."
                    ),
                );
            } else {
                array_creation_info.array_keys.insert(item_key_value);
            }
        }

        if let Expression::UnaryPrefix(UnaryPrefix { operator: UnaryPrefixOperator::Reference(_), operand }) =
            unwrap_expression(value)
        {
            let variable_id = get_block_expression_id(operand, context, block_context);
            if let Some(variable_id) = variable_id {
                if let Some(existing_type) = block_context.locals.remove(&variable_id) {
                    block_context.remove_descendants(context, variable_id, &existing_type, None);
                    // Escape invalidates dependent paths, not the root's captured value.
                    block_context.locals.insert(variable_id, existing_type);
                }
            }
        }

        let value_type = artifacts.get_expression_type(value).cloned().unwrap_or_else(get_mixed);
        let keys = item_key_value.map(|key| vec![key]).or_else(|| finite_array_keys(&key_type));
        if item_is_list_item && array_creation_info.finite_key_indexes.is_some() {
            set_finite_array_offset(context, &mut array_creation_info, None, &value_type, false);
        } else if ((array_creation_info.can_create_objectlike && !array_creation_info.has_general_unpack)
            || array_creation_info.degraded_finite_keys)
            && let Some(keys) = keys
        {
            if array_creation_info.finite_key_indexes.is_some() || keys.len() > 1 || item_key_value.is_none() {
                set_finite_array_offset(
                    context,
                    &mut array_creation_info,
                    (!item_is_list_item).then_some(keys.as_slice()),
                    &value_type,
                    false,
                );
            } else {
                let key = keys[0];
                if !array_creation_info.property_types.contains_key(&key) {
                    array_creation_info.property_order.push(key);
                }
                array_creation_info.property_types.insert(key, (false, value_type));
            }
        } else if let Some(key) = item_key_value.filter(|_| !array_creation_info.degraded_finite_keys) {
            array_creation_info.property_types.insert(key, (false, value_type));
        } else {
            if array_creation_info.finite_key_indexes.is_some() && !array_creation_info.degraded_finite_keys {
                degrade_finite_array(&mut array_creation_info);
            }
            array_creation_info.can_create_objectlike = false;
            array_creation_info.item_key_atomic_types.extend(key_type.types.into_owned());
            array_creation_info.item_value_atomic_types.extend(value_type.types.into_owned());
        }
    }

    if array_creation_info.is_list
        && array_creation_info.property_types.len() == 2
        && let (Some((_, first_type)), Some((_, second_type))) = (
            array_creation_info.property_types.get(&ArrayKey::Integer(0)),
            array_creation_info.property_types.get(&ArrayKey::Integer(1)),
        )
    {
        let class_names: Vec<_> = first_type
            .types
            .iter()
            .filter_map(|atomic| {
                if let Some(class_name) = atomic.get_class_string_value() {
                    Some(class_name)
                } else if let Some(literal_str) = atomic.get_literal_string_value() {
                    Some(word(literal_str))
                } else if let TAtomic::Object(obj) = atomic {
                    match obj {
                        TObject::Named(named) => Some(named.name),
                        TObject::Enum(r#enum) => Some(r#enum.name),
                        _ => None,
                    }
                } else {
                    None
                }
            })
            .collect();

        let method_names: Vec<_> = second_type
            .types
            .iter()
            .filter_map(|atomic| atomic.get_literal_string_value().map(ascii_lowercase_word))
            .collect();

        for class_name in &class_names {
            for method_name in &method_names {
                artifacts.symbol_references.add_reference_to_class_member(
                    &block_context.scope,
                    (ascii_lowercase_word(class_name.as_bytes()), *method_name),
                    false,
                );
            }
        }
    }

    // PHPStan's builder degrades the shape, not its finite scalar unions.
    // Mago's ordinary 128-literal combiner budget must not widen those unions.
    let mut combiner_options = context.settings.combiner_options();
    if array_creation_info.degraded_finite_keys {
        combiner_options.integer_combination_threshold = u16::MAX;
        combiner_options.string_combination_threshold = u16::MAX;
    }
    let item_key_type = if array_creation_info.item_key_atomic_types.is_empty() {
        None
    } else {
        Some(TUnion::from_vec(combine(array_creation_info.item_key_atomic_types, context.codebase, combiner_options)))
    };

    let item_value_type = if array_creation_info.item_value_atomic_types.is_empty() {
        None
    } else {
        Some(TUnion::from_vec(combine(array_creation_info.item_value_atomic_types, context.codebase, combiner_options)))
    };

    let array_type = if !array_creation_info.property_types.is_empty() {
        if array_creation_info.is_list {
            TUnion::from_vec(vec![TAtomic::Array(TArray::List(TList {
                callable: None,
                known_count: if array_creation_info.known_int_offset
                    && array_creation_info.property_types.values().all(|(optional, _)| !optional)
                {
                    Some(array_creation_info.property_types.len())
                } else {
                    None
                },
                known_elements: Some(
                    array_creation_info
                        .property_types
                        .into_iter()
                        .enumerate()
                        .map(|(index, (_, value_tuple))| (index, (value_tuple.0, value_tuple.1)))
                        .collect::<BTreeMap<_, _>>()
                        .into(),
                ),
                element_type: Arc::new(match item_value_type {
                    Some(value) => value,
                    None => get_never(),
                }),
                non_empty: !array_creation_info.can_be_empty,
            }))])
        } else {
            TUnion::from_vec(vec![TAtomic::Array(TArray::Keyed(TKeyedArray {
                callable: None,
                known_items: Some(array_creation_info.property_types.into()),
                parameters: if array_creation_info.can_create_objectlike {
                    None
                } else {
                    match (item_key_type, item_value_type) {
                        (Some(key), Some(value)) => Some((Arc::new(key), Arc::new(value))),
                        (Some(key), None) => Some((Arc::new(key), Arc::new(get_mixed()))),
                        (None, Some(value)) => Some((Arc::new(get_arraykey()), Arc::new(value))),
                        _ => Some((Arc::new(get_arraykey()), Arc::new(get_mixed()))),
                    }
                },
                non_empty: !array_creation_info.can_be_empty,
                known_non_list: array_creation_info.known_non_list,
            }))])
        }
    } else if item_key_type.is_none() && item_value_type.is_none() {
        get_empty_keyed_array()
    } else if array_creation_info.is_list {
        TUnion::from_vec(vec![TAtomic::Array(TArray::List(TList {
            callable: None,
            known_elements: None,
            element_type: Arc::new(item_value_type.unwrap_or_else(get_mixed)),
            known_count: None,
            non_empty: !array_creation_info.can_be_empty,
        }))])
    } else {
        TUnion::from_vec(vec![TAtomic::Array(TArray::Keyed(TKeyedArray {
            callable: None,
            known_items: None,
            parameters: match (item_key_type, item_value_type) {
                (Some(key), Some(value)) => Some((Arc::new(key), Arc::new(value))),
                (Some(key), None) => Some((Arc::new(key), Arc::new(get_mixed()))),
                (None, Some(value)) => Some((Arc::new(get_arraykey()), Arc::new(value))),
                _ => Some((Arc::new(get_arraykey()), Arc::new(get_mixed()))),
            },
            non_empty: !array_creation_info.can_be_empty,
            known_non_list: array_creation_info.known_non_list,
        }))])
    };

    artifacts.set_expression_type(&expression_span, array_type);
    if block_context.native_locals.is_some()
        && let Some(native) = native_literal(elements, artifacts)
    {
        artifacts.set_rc_native_expression_type(&expression_span, Rc::new(native));
    }

    Ok(())
}

// InitializerExprTypeResolver::getArrayType reads each operand in the requested
// flavour. This bounded constant builder consumes only completed native reads.
// Unpacking/nonconstant keys remain unavailable, not selected-type substitutes.
fn native_literal(elements: &[ArrayElement<'_>], artifacts: &AnalysisArtifacts) -> Option<TUnion> {
    // InitializerExprTypeResolver degrades larger literals before pricing items.
    if elements.len() > 256 {
        return None;
    }
    let mut items = KnownItems::default();
    let mut next = 0i64;
    for element in elements {
        let (key, value) = match element {
            ArrayElement::KeyValue(item) => (
                artifacts.get_native_expression_type(item.key)?.get_single_array_key()?,
                item.value,
            ),
            ArrayElement::Value(item) => (ArrayKey::Integer(next), item.value),
            _ => return None,
        };
        if let ArrayKey::Integer(index) = key {
            next = next.max(index.checked_add(1)?);
        }
        items.insert(key, (false, artifacts.get_native_expression_type(value)?.clone()));
    }
    if items.keys().enumerate().all(|(index, key)| *key == ArrayKey::Integer(index as i64)) {
        return Some(TUnion::from_atomic(TAtomic::Array(TArray::List(TList::from_known_elements(
            items.into_iter().enumerate().map(|(index, (_, item))| (index, item)).collect(),
        )))));
    }
    Some(TUnion::from_atomic(TAtomic::Array(TArray::Keyed(
        TKeyedArray::new().with_known_items(items).with_non_empty(true),
    ))))
}

fn handle_variadic_array_element<'arena, A>(
    context: &mut Context<'_, 'arena, A>,
    array_creation_info: &mut ArrayCreationInfo,
    variadic_array_element: &VariadicArrayElement<'arena>,
    variadic_array_element_type: &TUnion,
) where
    A: Arena,
{
    // InitializerExprTypeResolver::getArrayType forwards constant unpack slots
    // to the same builder. In particular, optional finite-key slots must not be
    // skipped or promoted to required offsets by the ordinary unpack fallback.
    let constant = match variadic_array_element_type.types.as_ref() {
        [TAtomic::Array(TArray::Keyed(array))] if array.parameters.is_none() => {
            array.known_items.as_ref().map(|items| {
                items
                    .iter()
                    .map(|(key, (optional, value))| (key.is_string().then_some(*key), *optional, value))
                    .collect::<Vec<_>>()
            })
        }
        [TAtomic::Array(TArray::List(array))] if array.element_type.is_never() => array
            .known_elements
            .as_ref()
            .map(|items| items.values().map(|(optional, value)| (None, *optional, value)).collect::<Vec<_>>()),
        _ => None,
    };
    // A later finite key must not reseal parameters from a general or union
    // unpack. Those inputs retain the existing general-array fallback.
    array_creation_info.has_general_unpack |= constant.is_none();
    if ((array_creation_info.can_create_objectlike && !array_creation_info.has_general_unpack)
        || array_creation_info.degraded_finite_keys)
        && let Some(items) = constant
        && (array_creation_info.finite_key_indexes.is_some() || items.iter().any(|(_, optional, _)| *optional))
    {
        array_creation_info.is_list &= items.iter().all(|(key, _, _)| key.is_none());
        array_creation_info.can_be_empty &= items.iter().all(|(_, optional, _)| *optional);
        for (key, optional, value) in items {
            set_finite_array_offset(
                context,
                array_creation_info,
                key.as_ref().map(std::slice::from_ref),
                value,
                optional,
            );
        }
        return;
    }
    if array_creation_info.finite_key_indexes.is_some() && !array_creation_info.degraded_finite_keys {
        degrade_finite_array(array_creation_info);
        if variadic_array_element_type.types.iter().any(|atomic| matches!(atomic, TAtomic::Array(TArray::List(_)))) {
            array_creation_info.item_key_atomic_types.extend(get_int().types.into_owned());
        }
    }
    let mut all_non_empty = true;

    for atomic_type in variadic_array_element_type.types.as_ref() {
        let (key_type, value_type) = match atomic_type {
            TAtomic::Array(array_type) => match array_type {
                TArray::Keyed(keyed_data) => {
                    if let Some(known_items) = &keyed_data.known_items {
                        for (key, (possibly_undefined, value_type)) in known_items.iter() {
                            if *possibly_undefined {
                                continue;
                            }

                            let new_offset_key = match key {
                                ArrayKey::Integer(_) => {
                                    if array_creation_info.int_offset == i64::MAX {
                                        report_unpacked_int_key_overflow(context, variadic_array_element.span());

                                        continue;
                                    }

                                    array_creation_info.int_offset += 1;
                                    array_creation_info
                                        .item_key_atomic_types
                                        .push(TAtomic::Scalar(TScalar::literal_int(array_creation_info.int_offset)));

                                    ArrayKey::Integer(array_creation_info.int_offset)
                                }
                                ArrayKey::String(string_key) => {
                                    array_creation_info.is_list = false;
                                    array_creation_info
                                        .item_key_atomic_types
                                        .push(TAtomic::Scalar(TScalar::String(TString::known_literal(*string_key))));
                                    ArrayKey::String(*string_key)
                                }
                                ArrayKey::ClassLikeConstant { .. } => {
                                    // unresolved class-like constant key; treat as opaque
                                    array_creation_info.is_list = false;
                                    *key
                                }
                            };

                            array_creation_info.property_types.insert(new_offset_key, (false, value_type.clone()));

                            if new_offset_key.is_integer() {
                                array_creation_info.array_keys.insert(new_offset_key);
                            } else {
                                array_creation_info.known_non_list = true;
                            }
                        }
                    }

                    // Update non-empty status
                    if !keyed_data.non_empty {
                        all_non_empty = false;
                    }

                    match keyed_data.get_generic_parameters() {
                        Some(parameters) => (Some(Cow::Borrowed(parameters.0)), Cow::Borrowed(parameters.1)),
                        None => {
                            continue;
                        }
                    }
                }
                TArray::List(list_data) => {
                    let has_unknown_count = list_data.known_count.is_none();
                    if has_unknown_count {
                        array_creation_info.known_int_offset = false;
                    }

                    if let Some(known_elements) = &list_data.known_elements {
                        for (possibly_undefined, value_type) in known_elements.values() {
                            if *possibly_undefined {
                                continue;
                            }

                            if array_creation_info.int_offset == i64::MAX {
                                report_unpacked_int_key_overflow(context, variadic_array_element.span());

                                continue;
                            }

                            array_creation_info.int_offset += 1;
                            let new_key = ArrayKey::Integer(array_creation_info.int_offset);
                            array_creation_info.array_keys.insert(new_key);
                            array_creation_info
                                .item_key_atomic_types
                                .push(TAtomic::Scalar(TScalar::literal_int(array_creation_info.int_offset)));
                            array_creation_info.property_types.insert(new_key, (false, value_type.clone()));
                        }
                    }

                    if !list_data.non_empty {
                        all_non_empty = false;
                    }

                    (None, Cow::Borrowed(list_data.get_element_type()))
                }
            },
            atomic => {
                all_non_empty = false;

                let Some((iterable_key, iterable_value)) = get_iterable_parameters(atomic, context.codebase) else {
                    array_creation_info.can_create_objectlike = false;
                    array_creation_info.item_key_atomic_types.push(TAtomic::Scalar(TScalar::ArrayKey));
                    array_creation_info.item_value_atomic_types.push(TAtomic::Mixed(TMixed::new()));

                    context.collector.report_with_code(
                        IssueCode::InvalidArrayElement,
                        Issue::error(format!(
                            "Cannot use spread operator on non-iterable type `{}`.",
                            atomic.get_id()
                        ))
                        .with_annotation(
                            Annotation::primary(variadic_array_element.span())
                                .with_message("Spread operator requires an iterable type.")
                        )
                        .with_note(
                            "The spread operator (`...`) can only be used with arrays or objects implementing the `Traversable` interface."
                        )
                        .with_help("Consider using an array or a traversable object."),
                    );

                    continue;
                };

                if iterable_value.is_never() {
                    continue;
                }

                array_creation_info.can_create_objectlike = false;

                (Some(Cow::Owned(iterable_key)), Cow::Owned(iterable_value))
            }
        };

        if value_type.is_never() {
            continue;
        }

        if let Some(key_type) = key_type {
            if key_type.is_never() {
                continue;
            }

            let is_string_key = union_comparator::is_contained_by(
                context.codebase,
                &key_type,
                &get_string(),
                false,
                false,
                false,
                &mut ComparisonResult::new(),
            );

            let is_array_key_key = is_string_key
                || union_comparator::is_contained_by(
                    context.codebase,
                    &key_type,
                    &get_arraykey(),
                    false,
                    false,
                    false,
                    &mut ComparisonResult::new(),
                );

            if is_string_key {
                array_creation_info.is_list = false;
            }

            if !is_array_key_key {
                context.collector.report_with_code(
                    IssueCode::InvalidArrayElementKey,
                    Issue::error(format!(
                        "Cannot use spread operator on an iterable with key type `{}`.",
                        key_type.get_id()
                    ))
                    .with_annotation(
                        Annotation::primary(variadic_array_element.span())
                            .with_message("Spread operator requires an iterable type with array-key keys.")
                    )
                    .with_note(
                        "The spread operator (`...`) can only be used with arrays or objects implementing the `Traversable` interface that have keys of type `array-key`."
                    )
                    .with_help("Consider using an array or a traversable object with appropriate key types.")
                );

                continue;
            }

            for (k, v) in &mut array_creation_info.property_types {
                let prop_key = k.to_union();

                if is_contained_by(
                    context.codebase,
                    &prop_key,
                    &key_type,
                    false,
                    false,
                    false,
                    &mut ComparisonResult::new(),
                ) {
                    let new_prop_val =
                        combine_union_types(&v.1, &value_type, context.codebase, context.settings.combiner_options());

                    *v = (v.0, new_prop_val);
                }
            }

            array_creation_info.item_key_atomic_types.extend(key_type.into_owned().types.into_owned());
        }

        array_creation_info.item_value_atomic_types.extend(value_type.into_owned().types.into_owned());
    }

    if all_non_empty {
        array_creation_info.can_be_empty = false;
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::code::IssueCode;
    use crate::test_analysis;

    test_analysis! {
        name = array_literal_empty_square_brackets,
        code = indoc! {"
            <?php

            /**
             * @param array{} $_arr
             */
            function expect_empty_array(array $_arr): void {}

            $empty1 = [];
            expect_empty_array($empty1);
        "}
    }

    test_analysis! {
        name = array_literal_empty_array_keyword,
        code = indoc! {"
            <?php

            /**
             * @param array{} $_arr
             */
            function expect_empty_array(array $_arr): void {}

            $empty2 = array();
            expect_empty_array($empty2);
        "}
    }

    test_analysis! {
        name = array_literal_literal_int_keys_as_list,
        code = indoc! {r#"
            <?php

            /**
             * @param list<string> $_arr
             */
            function expect_list_of_strings(array $_arr): void {}

            $arr = [0 => "a", 1 => "b"];
            expect_list_of_strings($arr);
        "#}
    }

    test_analysis! {
        name = array_literal_literal_string_keys,
        code = indoc! {r#"
            <?php

            /**
             * @param array{"name": string, "age": int} $_arr
             */
            function expect_shape_person(array $_arr): void {}

            $arr = ["name" => "Alice", "age" => 30];
            expect_shape_person($arr);
        "#}
    }

    test_analysis! {
        name = array_literal_numeric_string_keys_coerced_to_int,
        code = indoc! {r#"
            <?php

            /**
             * @param array{123: bool, 456: bool} $_arr
             */
            function expect_numeric_string_keys(array $_arr): void {}

            $arr = ["123" => true, "456" => false];
            expect_numeric_string_keys($arr);
        "#}
    }

    test_analysis! {
        name = array_literal_null_key_becomes_empty_string,
        code = indoc! {r#"
            <?php

            /**
             * @param array{"": string} $_arr
             */
            function expect_null_key(array $_arr): void {}

            $arr = [null => "val_for_empty_string_key"];
            expect_null_key($arr);
        "#}
    }

    test_analysis! {
        name = array_literal_bool_keys_coerced_to_int,
        code = indoc! {r#"
            <?php

            /**
             * @param array{1: string, 0: string} $_arr
             */
            function expect_bool_keys(array $_arr): void {}

            $arr = [true => "is_true", false => "is_false"];
            expect_bool_keys($arr);
        "#}
    }

    test_analysis! {
        name = array_literal_float_keys_truncated_to_int,
        code = indoc! {r#"
            <?php

            /**
             * @param array{1: string, 2: string} $_arr
             */
            function expect_float_keys(array $_arr): void {}

            $arr = [1.7 => "val1", 2.0 => "val2"];
            expect_float_keys($arr);
        "#}
    }

    test_analysis! {
        name = array_literal_duplicate_literal_int_key,
        code = indoc! {"
            <?php

            /**
             * @param array{0: string} $_arr
             */
            function expect_single_string_at_0(array $_arr):void{}

            $arr = [0 => 'a', 0 => 'b'];
            expect_single_string_at_0($arr);
        "},
        issues = [IssueCode::DuplicateArrayKey]
    }

    test_analysis! {
        name = array_literal_duplicate_literal_string_key,
        code = indoc! {r#"
            <?php

            /**
             * @param array{"k":string} $_arr
             */
            function expect_k_string(array $_arr):void{}

            $arr = ["k" => 'a', "k" => 'b'];
            expect_k_string($arr);
        "#},
        issues = [IssueCode::DuplicateArrayKey]
    }

    test_analysis! {
        name = array_literal_duplicate_coerced_key,
        code = indoc! {"
            <?php

            /**
             * @param array{1:string} $_arr
             */
            function expect_one_string(array $_arr):void{}

            $arr = [true => 'a', 1 => 'b']; // true becomes 1, so '1' is duplicated
            expect_one_string($arr);
        "},
        issues = [IssueCode::DuplicateArrayKey]
    }

    test_analysis! {
        name = array_literal_simple_list_implicit_keys,
        code = indoc! {r#"
            <?php

            /** @param list<string> $_arr */
            function expect_list_of_strings(array $_arr): void {}

            $arr = ["alpha", "beta", "gamma"];
            expect_list_of_strings($arr);
        "#}
    }

    test_analysis! {
        name = array_literal_mixed_implicit_explicit_keys_maintaining_list,
        code = indoc! {r#"
            <?php

            /** @param array{0: string, 1: string, 2: string} $_arr */
            function expect_shape_list_three_strings(array $_arr): void {}

            $arr = [0 => "a", "b", 2 => "c"];
            expect_shape_list_three_strings($arr);
        "#}
    }

    test_analysis! {
        name = array_literal_mixed_implicit_explicit_keys_breaking_list,
        code = indoc! {r#"
            <?php

            /** @param array<array-key, string> $_arr */
            function expect_general_keyed_array_string_values(array $_arr): void {}

            $arr = ["key" => "a", "b"]; // "b" gets key 0, becomes keyed array
            expect_general_keyed_array_string_values($arr);
        "#}
    }

    test_analysis! {
        name = array_literal_spread_empty_array,
        code = indoc! {"
            <?php

            /** @param array{} $_arr */
            function expect_empty_array(array $_arr): void {}

            $empty = [];
            $arr = [...$empty];
            expect_empty_array($arr);
        "}
    }

    test_analysis! {
        name = array_literal_spread_list,
        code = indoc! {r#"
            <?php

            /** @param list<string> $_arr */
            function expect_list_of_strings(array $_arr): void {}

            $list = ["a", "b"];
            $arr = [...$list, "c"];
            expect_list_of_strings($arr); // Expects list<string>
        "#}
    }

    test_analysis! {
        name = array_literal_spread_keyed_array,
        code = indoc! {r#"
            <?php

            /** @param array<string, int> $_arr */
            function expect_map_string_to_int(array $_arr): void {}

            $keyed = ["x" => 1, "y" => 2];
            $arr = [...$keyed, "z" => 3];
            expect_map_string_to_int($arr);
        "#}
    }

    test_analysis! {
        name = array_literal_spread_non_iterable_int,
        code = indoc! {"
            <?php

            /** @param array<array-key, mixed> $_arr */
            function expect_empty_array(array $_arr): void {}

            $num = 123;
            $arr = [...$num];
            expect_empty_array($arr);
        "},
        issues = [IssueCode::InvalidArrayElement]
    }

    test_analysis! {
        name = array_literal_missing_element,
        code = indoc! {"
            <?php

            $arr = [1, , 3];
        "},
        issues = [IssueCode::InvalidArrayElement]
    }

    test_analysis! {
        name = array_literal_results_in_list_type,
        code = indoc! {"
            <?php

            /** @param list<int> $_arr */
            function expect_list_of_ints(array $_arr): void {}

            $arr = [10, 20, 30];
            expect_list_of_ints($arr);
        "}
    }

    test_analysis! {
        name = array_literal_results_in_keyed_array_string_keys,
        code = indoc! {r#"
            <?php

            /** @param array<string, bool> $_arr */
            function expect_map_string_to_bool(array $_arr): void {}

            $arr = ["a" => true, "b" => false];
            expect_map_string_to_bool($arr);
        "#}
    }

    test_analysis! {
        name = array_literal_results_in_keyed_array_mixed_keys,
        code = indoc! {r#"
            <?php

            /** @param array<array-key, int> $_arr */
            function expect_general_keyed_array_int_values(array $_arr): void {}

            $arr = [0 => 1, "a" => 2, 1 => 3];
            expect_general_keyed_array_int_values($arr);
        "#}
    }

    test_analysis! {
        name = list_array_comparison,
        code = indoc! {"
            <?php

            /**
             * @return array{0: 1, 1: 2, 2: 3, 3: 4, 4: 5}
             */
            function x(): array {
               exit();
            }

            /**
             * @param list<1|2|3|4|5> $_list
             */
            function take_list_ints(array $_list): void {
            }

            $list = x();
            take_list_ints($list);
        "}
    }

    test_analysis! {
        name = create_array_using_generic_key,
        code = indoc! {"
            <?php

            /**
             * @template K of array-key
             * @template V
             *
             * @param K $k
             * @param V $v
             *
             * @return non-empty-array<K, V>
             */
            function create_array($k, $v): array {
                return [$k => $v];
            }
        "},
    }
}
