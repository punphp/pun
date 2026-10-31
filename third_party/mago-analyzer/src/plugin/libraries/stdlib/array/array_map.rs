//! `array_map()` return type provider.
//!
//! Preserves array shape (known items/elements) through `array_map` calls
//! by invoking the callback with the input values through the ordinary call
//! engine. This resolves selected acceptors, templates and conditional returns.

use std::collections::BTreeMap;
use std::sync::Arc;

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::wrap_atomic;

use crate::plugin::context::InvocationInfo;
use crate::plugin::context::ProviderContext;
use crate::plugin::provider::Provider;
use crate::plugin::provider::ProviderMeta;
use crate::plugin::provider::function::CallbackInvocation;
use crate::plugin::provider::function::FunctionReturnTypeProvider;
use crate::plugin::provider::function::FunctionTarget;

static META: ProviderMeta =
    ProviderMeta::new("php::array::array_map", "array_map", "Preserves array shape through array_map");

#[derive(Default)]
pub struct ArrayMapProvider;

impl Provider for ArrayMapProvider {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for ArrayMapProvider {
    fn targets() -> FunctionTarget {
        FunctionTarget::Exact(b"array_map")
    }

    fn callback_invocations<'ast, 'arena>(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, 'ast, 'arena>,
    ) -> Vec<CallbackInvocation<'ast, 'arena>> {
        let Some(callback) = invocation.get_argument(0, &[b"callback"]) else {
            return Vec::new();
        };
        let Some(array) = invocation
            .get_argument(1, &[b"array"])
            .and_then(|argument| context.get_expression_type(argument))
            .and_then(TUnion::get_single_array)
        else {
            return Vec::new();
        };
        if invocation.argument_count() != 2 || context.get_expression_type(callback).is_none_or(TUnion::is_null) {
            return Vec::new();
        }
        let values: Vec<_> = match array {
            TArray::Keyed(array) => array
                .known_items
                .iter()
                .flat_map(|items| items.values())
                .map(|(_, value)| value.clone())
                .chain(array.parameters.iter().map(|(_, value)| (**value).clone()))
                .collect(),
            TArray::List(array) => array
                .known_elements
                .iter()
                .flat_map(|items| items.values())
                .map(|(_, value)| value.clone())
                .chain((!array.element_type.is_never()).then(|| (*array.element_type).clone()))
                .collect(),
        };
        values
            .into_iter()
            .map(|value| CallbackInvocation {
                callback,
                arguments: vec![value],
            })
            .collect()
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        // array_map(?callable $callback, array $array, array ...$arrays)
        let argument_count = invocation.argument_count();
        if argument_count < 2 {
            return None;
        }

        let callback_arg = invocation.get_argument(0, &[b"callback"])?;
        let callback_type = context.get_expression_type(callback_arg)?;
        let callback_is_null = callback_type.is_null();

        if callback_is_null && argument_count == 2 {
            let array_arg = invocation.get_argument(1, &[b"array"])?;
            return context.get_expression_type(array_arg).cloned();
        }

        if callback_is_null && argument_count > 2 {
            return zip_input_arrays(context, invocation, argument_count);
        }

        if argument_count != 2 {
            return None;
        }

        let array_arg = invocation.get_argument(1, &[b"array"])?;
        let array_type = context.get_expression_type(array_arg)?;
        let array = array_type.get_single_array()?;

        let resolve_for = |element: &TUnion| -> Option<TUnion> {
            context
                .get_callback_return_type(callback_arg, std::slice::from_ref(element))
                .cloned()
        };

        match array {
            TArray::Keyed(keyed) if keyed.has_known_items() => {
                let known_items = keyed.get_known_items()?;
                let new_items: BTreeMap<_, _> = known_items
                    .iter()
                    .map(|(key, (optional, value))| Some((*key, (*optional, resolve_for(value)?))))
                    .collect::<Option<_>>()?;

                let mut result = TKeyedArray::new()
                    .with_known_items(new_items)
                    .with_non_empty(keyed.is_non_empty());

                if let Some((key_type, value_type)) = keyed.parameters.as_ref() {
                    result = result.with_parameters(Arc::clone(key_type), Arc::new(resolve_for(value_type)?));
                }

                Some(wrap_atomic(TAtomic::Array(TArray::Keyed(result))))
            }
            TArray::List(list) if list.known_elements.is_some() => {
                let known_elements = list.known_elements.as_deref()?;
                let new_elements: BTreeMap<_, _> = known_elements
                    .iter()
                    .map(|(idx, (optional, value))| Some((*idx, (*optional, resolve_for(value)?))))
                    .collect::<Option<_>>()?;

                let result = TList {
                    callable: None,
                    element_type: if list.element_type.is_never() {
                        Arc::clone(&list.element_type)
                    } else {
                        Arc::new(resolve_for(&list.element_type)?)
                    },
                    known_elements: Some(new_elements.into()),
                    known_count: list.known_count,
                    non_empty: list.non_empty,
                };

                Some(wrap_atomic(TAtomic::Array(TArray::List(result))))
            }
            _ => None,
        }
    }
}

fn zip_input_arrays(
    context: &ProviderContext<'_, '_, '_>,
    invocation: &InvocationInfo<'_, '_, '_>,
    argument_count: usize,
) -> Option<TUnion> {
    let array_count = argument_count - 1;
    let mut tuple_items: BTreeMap<ArrayKey, (bool, TUnion)> = BTreeMap::new();
    let mut all_inputs_non_empty = true;

    for offset in 0..array_count {
        let array_arg = invocation.get_argument(offset + 1, &[])?;
        let array_type = context.get_expression_type(array_arg)?;
        let array = array_type.get_single_array()?;

        let value_type = array_value_type(array);
        all_inputs_non_empty &= match array {
            TArray::List(list) => list.non_empty,
            TArray::Keyed(keyed) => keyed.is_non_empty(),
        };

        tuple_items.insert(ArrayKey::Integer(offset as i64), (false, value_type.as_nullable()));
    }

    Some(wrap_atomic(TAtomic::Array(TArray::List(TList {
        callable: None,
        element_type: Arc::new(wrap_atomic(TAtomic::Array(TArray::Keyed(
            TKeyedArray::new().with_known_items(tuple_items).with_non_empty(true),
        )))),
        known_elements: None,
        known_count: None,
        non_empty: all_inputs_non_empty,
    }))))
}

fn array_value_type(array: &TArray) -> TUnion {
    match array {
        TArray::List(list) => (*list.element_type).clone(),
        TArray::Keyed(keyed) => keyed.get_value_type().cloned().unwrap_or_else(get_mixed),
    }
}
