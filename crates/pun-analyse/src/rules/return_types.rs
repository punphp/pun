//! PHPStan's dynamic return types that Mago's own types lack, as Mago return type providers for the rules' inference.

use std::sync::Arc;

use mago_analyzer::plugin::PluginRegistry;
use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::Provider;
use mago_analyzer::plugin::provider::ProviderMeta;
use mago_analyzer::plugin::provider::function::{FunctionReturnTypeProvider, FunctionTarget};
use mago_analyzer::plugin::provider::method::{MethodReturnTypeProvider, MethodTarget};
use mago_codex::identifier::function_like::FunctionLikeIdentifier;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::atomic::array::list::TList;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    get_array_parameters, get_false, get_keyed_array, get_mixed, get_never, get_non_negative_int,
    get_null,
};
use mago_span::{HasSpan, Span};

/// The registry the rules' inference runs with; `contents` is the analysed file's (`bindTo()` finds its receiver in it).
pub(super) fn registry(contents: &[u8]) -> PluginRegistry {
    let mut registry = PluginRegistry::new();
    registry.register_function_provider(Benevolent);
    registry.register_function_provider(ArraySlice);
    registry.register_method_provider(Bind(contents.to_vec()));
    registry
}

/// Marks a Mago type as one of PHPStan's `BenevolentUnionType`s (`to_phpstan()` reads both flags together).
pub(super) fn benevolent(mut t: TUnion) -> TUnion {
    t.set_ignore_nullable_issues(true);
    t.set_ignore_falsable_issues(true);
    t
}

pub(super) fn is_benevolent(t: &TUnion) -> bool {
    t.ignore_nullable_issues() && t.ignore_falsable_issues()
}

static META: ProviderMeta = ProviderMeta::new("pun", "pun", "PHPStan's dynamic return types");

/// PHPStan 2.3.1's `ArraySliceFunctionReturnTypeExtension` and the array/list/non-empty `sliceArray` methods.
/// Mago 1.57's prelude only has a conditional generic return type: it neither slices shapes nor turns integer-keyed
/// non-lists back into lists. Keep this in the production registry, so assignments and subsequent calls see it too.
struct ArraySlice;

impl Provider for ArraySlice {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for ArraySlice {
    fn targets() -> FunctionTarget {
        FunctionTarget::Exact(b"array_slice")
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let array = context.get_expression_type(invocation.get_argument(0, &[b"array"])?)?;
        let offset = context.get_expression_type(invocation.get_argument(1, &[b"offset"])?)?;
        let length = match invocation.get_argument(2, &[b"length"]) {
            Some(e) => context.get_expression_type(e)?.clone(),
            None => get_null(),
        };
        let preserve = match invocation.get_argument(3, &[b"preserve_keys"]) {
            Some(e) => context.get_expression_type(e)?.clone(),
            None => get_false(),
        };
        // MixedType::sliceArray deliberately does not use ArrayType's zero-length/list shortcuts.
        if array.is_mixed() {
            return Some(get_keyed_array(get_mixed(), get_mixed()));
        }
        // These may contain arrays, but Mago cannot represent PHPStan's ErrorType alternative here.
        if array.types.iter().any(|a| {
            matches!(
                a,
                TAtomic::GenericParameter(_) | TAtomic::Iterable(_) | TAtomic::Reference(_)
            )
        }) {
            return None;
        }
        // PHPStan first normalizes unions of general arrays (merging their key/value domains). Slicing Mago's
        // alternatives independently would incorrectly narrow int|string keys to non-negative-int|string.
        let [TAtomic::Array(array)] = array.types.as_ref() else {
            return if array.types.iter().any(|a| matches!(a, TAtomic::Array(_))) {
                None
            } else {
                // Pun's PHP floor is 8.4: array functions throw on non-arrays, rather than returning null.
                Some(get_never())
            };
        };
        let preserve = if preserve.is_true() {
            Some(true)
        } else if preserve.is_false() {
            Some(false)
        } else {
            None
        };
        Some(TUnion::from_atomic(Self::slice(
            context.codebase(),
            array,
            offset,
            &length,
            preserve,
        )?))
    }
}

impl ArraySlice {
    fn slice(
        codebase: &CodebaseMetadata,
        array: &TArray,
        offset: &TUnion,
        length: &TUnion,
        preserve: Option<bool>,
    ) -> Option<TAtomic> {
        let empty = || TAtomic::Array(TArray::Keyed(TKeyedArray::default()));
        if array.is_empty() || length.get_single_literal_int_value() == Some(0) {
            return Some(empty());
        }
        if array.is_sealed() {
            let literal_length = if length.is_null() {
                Some(i64::MAX)
            } else {
                length.get_single_literal_int_value()
            };
            if let (Some(offset), Some(length)) =
                (offset.get_single_literal_int_value(), literal_length)
            {
                let items: Vec<_> = match array {
                    TArray::List(list) => list
                        .known_elements
                        .iter()
                        .flat_map(|m| m.iter())
                        .map(|(key, value)| (ArrayKey::Integer(*key as i64), value.clone()))
                        .collect(),
                    // Mago stores keyed shapes in a BTreeMap, losing PHP's insertion order. A singleton is safe;
                    // larger keyed shapes need ordered inference artifacts, not an invented sorted slice.
                    TArray::Keyed(keyed) => {
                        if keyed.known_items.as_ref().is_some_and(|m| m.len() > 1) {
                            return None;
                        }
                        keyed
                            .known_items
                            .iter()
                            .flat_map(|m| m.iter())
                            .map(|(k, v)| (*k, v.clone()))
                            .collect()
                    }
                };
                // Optional slots need ConstantArrayType's optional-key builder; do not silently make them required.
                if items.iter().any(|(_, (optional, _))| *optional) {
                    return None;
                }
                let items = Self::slice_known(&items, offset, length, preserve)?;
                return Some(TAtomic::Array(TArray::Keyed(TKeyedArray {
                    non_empty: !items.is_empty(),
                    known_items: Some(Arc::new(items.into_iter().collect())),
                    ..TKeyedArray::default()
                })));
            }
        }
        // ArrayType::sliceArray, plus AccessoryArrayListType and NonEmptyArrayType's intersection behavior.
        let (keys, values) = get_array_parameters(array, codebase);
        let from_start = offset.get_single_literal_int_value() == Some(0);
        let non_empty = array.is_non_empty()
            && from_start
            && (length.is_null()
                || (array.is_list()
                    && length.types.iter().all(
                        |a| matches!(a, TAtomic::Scalar(TScalar::Integer(i)) if i.is_positive()),
                    )));
        if (preserve == Some(false) && keys.is_int()) || (array.is_list() && from_start) {
            return Some(TAtomic::Array(TArray::List(TList {
                non_empty,
                ..TList::new(Arc::new(values))
            })));
        }
        Some(TAtomic::Array(TArray::Keyed(TKeyedArray {
            parameters: Some((
                Arc::new(if array.is_list() {
                    get_non_negative_int()
                } else {
                    keys
                }),
                Arc::new(values),
            )),
            non_empty,
            ..TKeyedArray::default()
        })))
    }

    /// ConstantArrayType::sliceArray on required, ordered slots. Negative lengths first remove trailing slots,
    /// then slice the shortened array with the original offset (including PHPStan's negative-offset semantics).
    fn slice_known(
        items: &[(ArrayKey, (bool, TUnion))],
        offset: i64,
        length: i64,
        preserve: Option<bool>,
    ) -> Option<Vec<(ArrayKey, (bool, TUnion))>> {
        let count = items.len() as i64;
        let length = if count.saturating_add(length) <= 0 {
            0
        } else {
            length
        };
        let normalized_offset = if count.saturating_add(offset) <= 0 {
            0
        } else {
            offset
        };
        if normalized_offset < 0 && length < 0 && normalized_offset >= length {
            return Some(Vec::new());
        }
        if length < 0 {
            return Self::slice_known(
                &items[..(count + length) as usize],
                offset,
                i64::MAX,
                preserve,
            );
        }
        let offset = normalized_offset;
        let start = if offset < 0 {
            count + offset
        } else {
            offset.min(count)
        };
        let end = start.saturating_add(length).min(count);
        // PHPStan reverses, reindexes, then reverses again for negative offsets. More than one reindexed key
        // consequently has descending insertion order, which Mago's BTreeMap cannot encode. Defer, do not sort it.
        if offset < 0 && preserve != Some(true) && end - start > 1 {
            return None;
        }
        let mut next = 0;
        Some(
            items[start as usize..end as usize]
                .iter()
                .map(|(key, value)| {
                    let key = if preserve != Some(true) && key.is_integer() {
                        let key = ArrayKey::Integer(next);
                        next += 1;
                        key
                    } else {
                        *key
                    };
                    (key, value.clone())
                })
                .collect(),
        )
    }
}

/// The functions whose functionMap return type is a `__benevolent<…>` union.
struct Benevolent;

impl Provider for Benevolent {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for Benevolent {
    fn targets() -> FunctionTarget {
        FunctionTarget::ExactMultiple(&[
            b"apcu_cache_info",
            b"apcu_sma_info",
            b"curl_init",
            b"get_include_path",
            b"getopt",
            b"imagecreate",
            b"imagecreatetruecolor",
            b"php_sapi_name",
            b"scandir",
            b"stream_get_contents",
            b"tempnam",
            b"tmpfile",
        ])
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let function = context
            .codebase()
            .get_function(invocation.function_name().as_bytes())?;
        Some(benevolent(
            function.return_type_metadata.as_ref()?.type_union.clone(),
        ))
    }
}

/// `ClosureBindDynamicReturnTypeExtension`, `ClosureBindToDynamicReturnTypeExtension`
struct Bind(Vec<u8>);

impl Provider for Bind {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl MethodReturnTypeProvider for Bind {
    fn targets() -> &'static [MethodTarget] {
        static TARGETS: [MethodTarget; 2] = [
            MethodTarget::exact(b"closure", b"bind"),
            MethodTarget::exact(b"closure", b"bindto"),
        ];
        &TARGETS
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        _: &[u8],
        method: &[u8],
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let (closure, new_this) = if method.eq_ignore_ascii_case(b"bind") {
            let closure =
                context.get_expression_type(invocation.get_argument(0, &[b"closure"])?)?;
            (closure, invocation.get_argument(1, &[b"newThis"]))
        } else {
            let closure = context.get_expression_type(&self.receiver(invocation)?)?;
            (closure, invocation.get_argument(0, &[b"newThis"]))
        };
        // Mago expands a closure alias when a variable holding it is read (see `to_phpstan()`)
        let [atom @ TAtomic::Callable(TCallable::Signature(signature))] = closure.types.as_ref()
        else {
            return None;
        };
        if !signature.is_closure() {
            return None;
        }
        let source = signature.get_source();
        let maybe_null = || benevolent(TUnion::from_vec(vec![atom.clone(), TAtomic::Null]));
        if !self.is_static(context.codebase(), source) {
            return Some(maybe_null());
        }
        let Some(new_this) = new_this.and_then(|e| context.get_expression_type(e)) else {
            return Some(closure.clone());
        };
        Some(if new_this.is_null() {
            closure.clone()
        } else if !new_this.has_null() && !new_this.has_mixed() {
            TUnion::from_vec(vec![TAtomic::Null])
        } else {
            maybe_null()
        })
    }
}

impl Bind {
    /// `$closure` of `$closure->bindTo(…)`: from the call's start to the `->` before the method name.
    fn receiver(&self, invocation: &InvocationInfo<'_, '_, '_>) -> Option<Span> {
        let call = invocation.span();
        let arguments = invocation.inner().arguments_source.span();
        let before = self
            .0
            .get(call.start.offset as usize..arguments.start.offset as usize)?
            .trim_ascii_end();
        let before = before[..before.len() - b"bindTo".len()]
            .trim_ascii_end()
            .strip_suffix(b"->")?;
        let before = before.strip_suffix(b"?").unwrap_or(before).trim_ascii_end();
        Some(Span::new(
            call.file_id,
            call.start,
            call.start.forward(before.len() as u32),
        ))
    }

    /// `ClosureType::isStaticClosure()`: Mago's metadata has no static closures, its source does (`static fn`).
    fn is_static(
        &self,
        codebase: &CodebaseMetadata,
        source: Option<FunctionLikeIdentifier>,
    ) -> bool {
        source
            .and_then(|id| codebase.get_function_like(&id))
            .and_then(|f| self.0.get(f.span.start.offset as usize..))
            .is_some_and(|text| text.starts_with(b"static"))
    }
}
