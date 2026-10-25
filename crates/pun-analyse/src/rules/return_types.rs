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
use mago_codex::ttype::cast::cast_atomic_to_callable;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::expander::get_parameter_dependent_signature_of_function_like_identifier;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    combine_union_types, get_array_parameters, get_false, get_int, get_int_or_string,
    get_int_range, get_iterable_parameters, get_keyed_array, get_literal_int, get_mixed, get_never,
    get_non_negative_int, get_null, intersect_union_types,
};
use mago_span::{HasSpan, Span};

/// The registry the rules' inference runs with; `contents` is the analysed file's (`bindTo()` finds its receiver in it).
pub(super) fn registry(contents: &[u8]) -> PluginRegistry {
    let mut registry = PluginRegistry::new();
    registry.register_function_provider(Benevolent);
    registry.register_function_provider(BuiltinFunctions);
    registry.register_method_provider(ClosureMethods(contents.to_vec()));
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

/// PHPStan 2.3.1's ArrayRandFunctionReturnTypeExtension, RandomIntFunctionReturnTypeExtension,
/// ArraySliceFunctionReturnTypeExtension and the array/list/non-empty sliceArray methods.
/// Mago 1.57's prelude only has a conditional generic return type: it neither slices shapes nor turns integer-keyed
/// non-lists back into lists. Keep this in the production registry, so assignments and subsequent calls see it too.
struct BuiltinFunctions;

impl Provider for BuiltinFunctions {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for BuiltinFunctions {
    fn targets() -> FunctionTarget {
        FunctionTarget::ExactMultiple(&[
            b"array_rand",
            b"array_slice",
            b"rand",
            b"mt_rand",
            b"random_int",
        ])
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        if invocation.function_name().eq_ignore_ascii_case("rand")
            || invocation.function_name().eq_ignore_ascii_case("mt_rand")
            || invocation
                .function_name()
                .eq_ignore_ascii_case("random_int")
        {
            return RandomInt::get_return_type(context, invocation);
        }
        let array = context.get_expression_type(invocation.get_argument(0, &[b"array"])?)?;
        if invocation
            .function_name()
            .eq_ignore_ascii_case("array_rand")
        {
            let (keys, empty) = Self::iterable_keys(array, context.codebase());
            // Picking from an empty array throws, but PHPStan deliberately returns int|string, not never.
            // Fix the actual return type, rather than suppressing the resulting false dead-code diagnostic.
            let keys = if empty {
                get_int_or_string()
            } else {
                intersect_union_types(&keys, &get_int_or_string(), context.codebase())
                    .unwrap_or_else(get_never)
            };
            let Some(num) = invocation.get_argument(1, &[b"num"]) else {
                return Some(keys);
            };
            let num = context.get_expression_type(num)?;
            if Self::count_in_range(num, 1, Some(1)) {
                return Some(keys);
            }
            let list = TUnion::from_atomic(TAtomic::Array(TArray::List(TList::new_non_empty(
                Arc::new(keys.clone()),
            ))));
            return Some(if Self::count_in_range(num, 2, None) {
                list
            } else {
                combine_union_types(&keys, &list, context.codebase(), CombinerOptions::default())
            });
        }
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
        } else if preserve.types.iter().all(|a| match a {
            TAtomic::Scalar(TScalar::Bool(b)) => b.is_false(),
            TAtomic::Mixed(_) | TAtomic::GenericParameter(_) | TAtomic::Reference(_) => false,
            // PHPStan asks isTrue(), not PHP truthiness: even a literal 1 is definitely not the true type.
            _ => true,
        }) {
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

impl BuiltinFunctions {
    /// getIterableKeyType and isIterableAtLeastOnce()->no(), before the empty-input fallback. An empty arm
    /// contributes no keys to a union that can be non-empty; applying the fallback per arm would widen it.
    fn iterable_keys(array: &TUnion, codebase: &CodebaseMetadata) -> (TUnion, bool) {
        let mut keys = get_never();
        let mut empty = true;
        for atomic in array.types.iter() {
            let (part, part_empty) = match atomic {
                TAtomic::GenericParameter(template) => {
                    Self::iterable_keys(&template.constraint, codebase)
                }
                TAtomic::Array(TArray::List(list)) => {
                    if let Some(items) = &list.known_elements
                        && list.element_type.is_never()
                    {
                        let keys = items.keys().fold(get_never(), |keys, key| {
                            combine_union_types(
                                &keys,
                                &get_literal_int(*key as i64),
                                codebase,
                                Default::default(),
                            )
                        });
                        (keys, items.is_empty() && !list.non_empty)
                    } else {
                        // A generic list<never> is maybe empty, not ConstantArrayType's empty array.
                        (get_non_negative_int(), false)
                    }
                }
                TAtomic::Array(array) => {
                    (get_array_parameters(array, codebase).0, array.is_empty())
                }
                TAtomic::Mixed(_) => (get_int_or_string(), false),
                _ => get_iterable_parameters(atomic, codebase)
                    .map_or_else(|| (get_never(), true), |(keys, _)| (keys, false)),
            };
            keys = combine_union_types(&keys, &part, codebase, CombinerOptions::default());
            empty &= part_empty;
        }
        (keys, empty)
    }

    /// PHPStan checks the count's type, not its truthiness or integer cast. TemplateIntegerType is not an
    /// IntegerRangeType, even with a range bound; TemplateConstantIntegerType retains its literal instead.
    fn count_in_range(count: &TUnion, min: i64, max: Option<i64>) -> bool {
        count.types.iter().all(|atomic| match atomic {
            TAtomic::Never => true,
            TAtomic::GenericParameter(template) => {
                if template.constraint.types.len() > 1 {
                    Self::count_in_range(&template.constraint, min, max)
                } else {
                    template
                        .constraint
                        .get_single_literal_int_value()
                        .is_some_and(|n| n >= min && max.is_none_or(|max| n <= max))
                }
            }
            _ => {
                atomic.get_minimum_int_value().is_some_and(|n| n >= min)
                    && max
                        .is_none_or(|max| atomic.get_maximum_int_value().is_some_and(|n| n <= max))
            }
        })
    }

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

/// RandomIntFunctionReturnTypeExtension. PHPStan deliberately uses the same bounds and implicit-never
/// invalid-interval result before and after PHP 8; rand alone swaps reversed bounds.
struct RandomInt;

impl RandomInt {
    fn get_return_type(
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let min = invocation.get_argument(0, &[b"min"]);
        let max = invocation.get_argument(1, &[b"max"]);
        if invocation.has_no_arguments()
            && !invocation
                .function_name()
                .eq_ignore_ascii_case("random_int")
        {
            return Some(get_non_negative_int());
        }
        let min = Self::to_integer(context.get_expression_type(min?)?);
        let max = Self::to_integer(context.get_expression_type(max?)?);
        let (min, max) = if invocation.function_name().eq_ignore_ascii_case("rand") {
            let both = combine_union_types(&min, &max, context.codebase(), Default::default());
            Self::bounds(&both)
        } else {
            (Self::bounds(&min).0, Self::bounds(&max).1)
        };
        // IntegerRangeType::fromInterval, including equal and machine-limit normalization that Mago's
        // get_int_range does not perform. Do not turn invalid mt_rand/random_int bounds into a swapped range.
        Some(match (min, max) {
            (Some(min), Some(max)) if min > max => get_never(),
            (Some(min), Some(max)) if min == max => get_literal_int(min),
            (None, Some(i64::MIN)) => get_literal_int(i64::MIN),
            (Some(i64::MAX), None) => get_literal_int(i64::MAX),
            _ => get_int_range(min, max),
        })
    }
    /// createRange iterates a top-level UnionType's members. A TemplateUnionType is such a union, but when
    /// nested in rand's union with the other bound it is not itself an integer/range endpoint.
    fn bounds(ty: &TUnion) -> (Option<i64>, Option<i64>) {
        let ty = if let [TAtomic::GenericParameter(template)] = ty.types.as_ref()
            && template.constraint.types.len() > 1
        {
            &template.constraint
        } else {
            ty
        };
        (ty.get_minimum_int_value(), ty.get_maximum_int_value())
    }

    /// PHPStan casts the argument types before extracting bounds, not just literal numeric expressions.
    fn to_integer(ty: &TUnion) -> TUnion {
        TUnion::from_vec(
            ty.types
                .iter()
                .flat_map(|atomic| {
                    let converted = match atomic {
                        TAtomic::Scalar(TScalar::Integer(_)) => TUnion::from_atomic(atomic.clone()),
                        TAtomic::GenericParameter(template) => {
                            if template.constraint.is_int()
                                && template.constraint.get_single_literal_int_value().is_none()
                            {
                                // toInteger keeps TemplateIntegerType itself; createRange recognizes only
                                // IntegerRangeType/ConstantIntegerType, not the template's narrower bound.
                                TUnion::from_atomic(atomic.clone())
                            } else if template.constraint.is_bool() {
                                // TemplateBooleanType inherits BooleanType, including for a literal bound.
                                get_int_range(Some(0), Some(1))
                            } else {
                                Self::to_integer(&template.constraint)
                            }
                        }
                        TAtomic::Null => get_literal_int(0),
                        TAtomic::Scalar(TScalar::Bool(value)) => value.value.map_or_else(
                            || get_int_range(Some(0), Some(1)),
                            |value| get_literal_int(i64::from(value)),
                        ),
                        TAtomic::Scalar(TScalar::String(value)) => value
                            .get_known_literal_value()
                            .map_or_else(get_int, |value| {
                                get_literal_int(pun_tokens::php_compat::intval(
                                    &String::from_utf8_lossy(value),
                                ))
                            }),
                        TAtomic::Scalar(TScalar::Float(value)) => {
                            value.get_literal_value().map_or_else(get_int, |value| {
                                // PHP's float cast truncates and wraps modulo 2^64; Rust's `as i64` saturates instead.
                                let value = value.trunc() % 18446744073709551616.0;
                                let value = if value >= 9223372036854775808.0 {
                                    value - 18446744073709551616.0
                                } else if value < -9223372036854775808.0 {
                                    value + 18446744073709551616.0
                                } else {
                                    value
                                };
                                get_literal_int(value as i64)
                            })
                        }
                        TAtomic::Array(array) => {
                            let constant_empty = match array {
                                TArray::List(list) => {
                                    list.known_elements.is_some() && array.is_empty()
                                }
                                _ => array.is_empty(),
                            };
                            if constant_empty {
                                get_literal_int(0)
                            } else if array.is_non_empty() {
                                get_literal_int(1)
                            } else {
                                get_int_range(Some(0), Some(1))
                            }
                        }
                        // General numeric/string/mixed inputs cast to int. ErrorType from an invalid cast also
                        // contributes an unbounded endpoint in createRange, rather than a fabricated literal 1.
                        _ => get_int(),
                    };
                    converted.types.into_owned()
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

/// Closure method return types: `bind()`, `bindTo()`, and `fromCallable()`.
struct ClosureMethods(Vec<u8>);

impl Provider for ClosureMethods {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl MethodReturnTypeProvider for ClosureMethods {
    fn targets() -> &'static [MethodTarget] {
        static TARGETS: [MethodTarget; 3] = [
            MethodTarget::exact(b"closure", b"bind"),
            MethodTarget::exact(b"closure", b"bindto"),
            MethodTarget::exact(b"closure", b"fromcallable"),
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
        if method.eq_ignore_ascii_case(b"fromcallable") {
            return Self::from_callable(context, invocation);
        }
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

impl ClosureMethods {
    /// Mago 1.59.0's ClosureFromCallableProvider (3ea59634580e5986c267d99e7fb3ccbf6ee26587), using 1.57's
    /// existing callable cast and parameter-dependent signature APIs. Keep the signature, changing only its kind.
    fn from_callable(
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let callback = invocation.get_argument(0, &[b"callback"])?;
        let callback_type = context.get_expression_type(callback)?;
        let codebase = context.codebase();
        let mut closures = Vec::with_capacity(callback_type.types.len());
        for atomic in callback_type.types.as_ref() {
            // As upstream, one unresolved alternative keeps the declared Closure fallback for the whole call.
            let callable = cast_atomic_to_callable(atomic, codebase, None)?;
            let mut signature = match callable.as_ref() {
                TCallable::Signature(signature) => signature.clone(),
                TCallable::Alias(identifier) => {
                    get_parameter_dependent_signature_of_function_like_identifier(
                        identifier, codebase,
                    )?
                }
            };
            signature.is_closure = true;
            closures.push(TAtomic::Callable(TCallable::Signature(signature)));
        }
        Some(TUnion::from_vec(closures))
    }

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
