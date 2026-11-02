//! PHPStan's dynamic return types that Mago's own types lack, as Mago return type providers for the rules' inference.

use std::sync::{Arc, Mutex};

use foldhash::HashSet;
use mago_analyzer::plugin::HookResult;
use mago_analyzer::plugin::PluginRegistry;
use mago_analyzer::plugin::context::{HookContext, InvocationInfo, ProviderContext};
use mago_analyzer::plugin::hook::{ExpressionHookResult, FunctionCallHook};
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
use mago_codex::ttype::atomic::scalar::string::{TString, TStringCasing};
use mago_codex::ttype::cast::cast_atomic_to_callable;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::comparator::{ComparisonResult, union_comparator};
use mago_codex::ttype::expander::get_parameter_dependent_signature_of_function_like_identifier;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    combine_union_types, get_array_parameters, get_false, get_int, get_int_or_string,
    get_int_range, get_iterable_parameters, get_keyed_array, get_literal_int, get_mixed, get_never,
    get_non_negative_int, get_null, get_truthy_string, intersect_union_types,
};
use mago_span::{HasSpan, Span};
use mago_syntax::cst::{
    CompositeString, Expression, FunctionCall, Literal, PartialApplication, StringPart,
};
use pun_phpstan::types::Tri;

/// The registry the rules' inference runs with; `contents` is the analysed file's (`bindTo()` finds its receiver in it).
/// With Larastan, a model forwards the methods it lacks to its builder.
pub(super) fn registry(contents: &[u8], larastan: bool) -> PluginRegistry {
    let mut registry = PluginRegistry::new();
    if larastan {
        registry.register_method_provider(super::larastan::ModelForwards);
    }
    registry.register_function_provider(Benevolent);
    registry.register_function_provider(
        mago_analyzer::plugin::libraries::stdlib::array::ArrayIsListProvider,
    );
    let builtins = BuiltinFunctions::default();
    registry.register_function_call_hook(builtins.clone());
    registry.register_function_provider(builtins);
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
/// ArraySliceFunctionReturnTypeExtension, StrlenFunctionReturnTypeExtension, StrRepeatFunctionReturnTypeExtension,
/// the integer/IP filter extensions
/// and the array/list sliceArray methods.
/// Mago 1.57's prelude only has a conditional generic return type: it neither slices shapes nor turns integer-keyed
/// non-lists back into lists. Keep this in the production registry, so assignments and subsequent calls see it too.
#[derive(Clone, Default)]
struct BuiltinFunctions(Arc<Mutex<HashSet<Span>>>);

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
            b"strlen",
            b"str_repeat",
            b"filter_input",
            b"filter_var",
            b"count",
            b"sizeof",
        ])
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        if invocation.function_name().eq_ignore_ascii_case("count")
            || invocation.function_name().eq_ignore_ascii_case("sizeof")
        {
            let array = context.get_expression_type(invocation.get_argument(0, &[b"value"])?)?;
            // CountFunctionReturnTypeExtension delegates normal counts to the value's size.
            let arrays = array.types
                .iter()
                .map(|atomic| {
                    if let TAtomic::Array(array) = atomic {
                        Some(array)
                    } else {
                        None
                    }
                })
                .collect::<Option<Vec<_>>>()?;
            if invocation.get_argument(1, &[b"mode"]).is_some_and(|mode| {
                context
                    .get_expression_type(mode)
                    .and_then(TUnion::get_single_literal_int_value)
                    != Some(0)
            }) && arrays.iter().any(|array| {
                let (_, values) = get_array_parameters(array, context.codebase());
                values
                    .types
                    .iter()
                    .any(|value| value.is_array() || value.is_mixed())
            }) {
                return arrays
                    .iter()
                    .all(|array| array.is_non_empty())
                    .then(|| get_int_range(Some(1), None));
            }
            return Some(TArray::get_union_size(&arrays, context.codebase()));
        }
        if invocation.function_name().eq_ignore_ascii_case("str_repeat") {
            // Stored callable signatures do not run PHPStan's dynamic extension. Do not retain the
            // prelude's conditional non-empty refinement on that separate reflection-only path either.
            if !self.0.lock().unwrap().contains(&invocation.span()) {
                return Some(mago_codex::ttype::get_string());
            }
            let (Some(input), Some(count)) = (
                invocation.get_argument(0, &[b"string"]),
                invocation.get_argument(1, &[b"times"]),
            ) else {
                return Some(mago_codex::ttype::get_string());
            };
            return Some(StrRepeat::return_type(
                context.get_expression_type(input)?,
                context.get_expression_type(count)?,
            ));
        }
        if invocation
            .function_name()
            .eq_ignore_ascii_case("filter_input")
            || invocation
                .function_name()
                .eq_ignore_ascii_case("filter_var")
        {
            if !self.0.lock().unwrap().contains(&invocation.span()) {
                return None;
            }
            return FilterFunctions::return_type(context, invocation);
        }
        if invocation.function_name().eq_ignore_ascii_case("strlen") {
            if !self.0.lock().unwrap().contains(&invocation.span()) {
                return None;
            }
            let input = context.get_expression_type(invocation.get_argument(0, &[b"string"])?)?;
            return Strlen::return_type(input);
        }
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

impl FunctionCallHook for BuiltinFunctions {
    fn before_function_call(
        &self,
        call: &FunctionCall<'_>,
        _: &mut HookContext<'_, '_>,
    ) -> HookResult<ExpressionHookResult> {
        // FuncCallHandler::resolveReturnType only applies extensions to named functions, literal-string
        // callees and immediately invoked named first-class functions, not stored callable signatures.
        // Mago retains the source identifier even on those signatures; record the actual CST boundary.
        let eligible = match super::types::flow::unparen(call.function) {
            Expression::Identifier(_) | Expression::Literal(Literal::String(_)) => true,
            Expression::CompositeString(CompositeString::Document(document)) => document
                .parts
                .iter()
                .all(|part| matches!(part, StringPart::Literal(_))),
            Expression::PartialApplication(PartialApplication::Function(function)) => {
                matches!(function.function, Expression::Identifier(_))
                    && function.argument_list.is_first_class_callable()
            }
            _ => false,
        };
        if eligible {
            self.0.lock().unwrap().insert(call.span());
        }
        Ok(ExpressionHookResult::Continue)
    }
}

/// FilterInput/FilterVarDynamicReturnTypeExtension and the integer/IP validation branches of
/// FilterFunctionReturnTypeHelper (PHPStan 2.3.1). Other filters' exact casts/sanitization are not ported here.
struct FilterFunctions;

impl FilterFunctions {
    const REQUIRE_ARRAY: i64 = 16777216;
    const FORCE_ARRAY: i64 = 67108864;
    const NULL_ON_FAILURE: i64 = 134217728;

    fn return_type(
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let input_function = invocation
            .function_name()
            .eq_ignore_ascii_case("filter_input");
        let position = usize::from(input_function);
        let input = context
            .get_expression_type(invocation.get_argument(position, &[b"value", b"var_name"])?)?;
        if input_function {
            let source = context.get_expression_type(invocation.get_argument(0, &[b"type"])?)?;
            // getInputType rejects only a definitely disjoint integer domain, not a general int.
            if !source.is_int()
                || ![0, 1, 2, 4, 5].iter().any(|value| {
                    source.types.iter().any(|a| {
                        a.get_minimum_int_value().is_none_or(|min| min <= *value)
                            && a.get_maximum_int_value().is_none_or(|max| max >= *value)
                    })
                })
            {
                return Some(get_never());
            }
        }
        let filter =
            context.get_expression_type(invocation.get_argument(position + 1, &[b"filter"])?)?;
        let filter = filter.get_single_literal_int_value();
        if filter.is_none() && !input_function {
            return Some(get_mixed());
        }
        if let Some(filter) = filter
            && filter != 257
            && filter != 275
            && matches!(filter, 258 | 259 | 272..=274 | 276 | 277 | 513..=520 | 523)
        {
            // Do not pretend the unported exact conversions for the other known filters are implemented.
            return None;
        }
        let flags = match invocation.get_argument(position + 2, &[b"options"]) {
            Some(flags) => context.get_expression_type(flags)?.clone(),
            None => get_literal_int(0),
        };
        // THROW_ON_FAILURE is PHP-version-dependent (8.5+); ProviderContext does not expose the target
        // version. Leave this unported boundary unavailable rather than imposing 8.5 semantics on 8.4.
        if Self::has_flag(&flags, 268435456, context.codebase()).yes() {
            return None;
        }
        if !input_function {
            return Some(filter.map_or_else(get_mixed, |filter| {
                Self::filter(input, filter, &flags, context.codebase())
            }));
        }
        let missing = match Self::has_flag(&flags, Self::NULL_ON_FAILURE, context.codebase()) {
            Tri::Yes => get_false(),
            Tri::No => get_null(),
            Tri::Maybe => Self::union(&get_false(), &get_null(), context.codebase()),
        };
        // getOffsetValueType on ArrayType(string, mixed): a definitely non-string key cannot exist.
        if Self::query(input, TAtomic::is_any_string).no() {
            return Some(missing);
        }
        Some(Self::union(
            &filter.map_or_else(get_mixed, |filter| {
                Self::filter(&get_mixed(), filter, &flags, context.codebase())
            }),
            &missing,
            context.codebase(),
        ))
    }

    fn union(a: &TUnion, b: &TUnion, codebase: &CodebaseMetadata) -> TUnion {
        combine_union_types(a, b, codebase, CombinerOptions::default())
    }

    /// The query's union arms use extreme identity, not a boolean "any" or "all".
    fn query(input: &TUnion, predicate: fn(&TAtomic) -> bool) -> Tri {
        let mut result = None;
        for atomic in input.types.iter() {
            let part = match atomic {
                TAtomic::Mixed(_) => Tri::Maybe,
                TAtomic::GenericParameter(template) => Self::query(&template.constraint, predicate),
                _ => Tri::from(predicate(atomic)),
            };
            result = Some(match result {
                Some(previous) if previous != part => Tri::Maybe,
                _ => part,
            });
        }
        result.unwrap_or(Tri::Yes)
    }

    /// hasOffsetValueType/getOffsetValueType for the constant option shapes used by the helper.
    fn offset(input: &TUnion, name: &[u8], codebase: &CodebaseMetadata) -> (Tri, TUnion) {
        let mut presence = None;
        let mut value = get_never();
        for atomic in input.types.iter() {
            let (part, ty) = match atomic {
                TAtomic::Array(TArray::Keyed(array)) => {
                    match array.known_items.as_ref().and_then(|items| {
                        items
                            .iter()
                            .find(|(key, _)| key.get_string() == Some(name))
                            .map(|(_, item)| item)
                    }) {
                        Some((optional, ty)) => {
                            (if *optional { Tri::Maybe } else { Tri::Yes }, ty.clone())
                        }
                        None => (Tri::No, get_never()),
                    }
                }
                TAtomic::Array(TArray::List(_)) => (Tri::No, get_never()),
                _ => (Tri::Maybe, get_mixed()),
            };
            presence = Some(match presence {
                Some(previous) if previous != part => Tri::Maybe,
                _ => part,
            });
            value = Self::union(&value, &ty, codebase);
        }
        (presence.unwrap_or(Tri::No), value)
    }

    fn constant_array(input: &TUnion) -> bool {
        input
            .types
            .iter()
            .all(|a| matches!(a, TAtomic::Array(array) if array.is_sealed()))
    }

    /// getConstantScalarValues is all-or-nothing; finite integer ranges are not enumerated here.
    fn scalar_ints(input: &TUnion) -> Option<Vec<i64>> {
        let mut values = Vec::new();
        for atomic in input.types.iter() {
            match atomic {
                TAtomic::Null => values.push(0),
                TAtomic::Scalar(TScalar::Integer(int)) => values.push(int.get_literal_value()?),
                TAtomic::Scalar(TScalar::Float(float)) => {
                    // PHP's float cast wraps modulo 2^64; Rust's signed cast saturates instead.
                    let value = float.get_literal_value()?;
                    let magnitude = (value.trunc() % 18446744073709551616.0).abs() as u64;
                    values.push(if value.is_sign_negative() {
                        magnitude.wrapping_neg()
                    } else {
                        magnitude
                    } as i64);
                }
                TAtomic::Scalar(TScalar::Bool(boolean)) if boolean.is_general() => {
                    values.extend([0, 1])
                }
                TAtomic::Scalar(TScalar::Bool(boolean)) => {
                    values.push(i64::from(boolean.is_true()))
                }
                TAtomic::Scalar(TScalar::String(string)) => {
                    // Extract PHP's ASCII leading numeric prefix before parsing, preserving large integer
                    // prefixes exactly. The remaining bytes (including arbitrary binary bytes) are ignored.
                    let regex = pcre2::bytes::Regex::new(r"\A[ \t\n\r\x0b\x0c]*([+-]?(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?)").unwrap();
                    let captures = regex.captures(string.get_known_literal_value()?).unwrap();
                    values.push(captures.map_or(0, |captures| {
                        let number =
                            std::str::from_utf8(captures.get(1).unwrap().as_bytes()).unwrap();
                        number
                            .parse()
                            .unwrap_or_else(|_| pun_tokens::php_compat::intval(number))
                    }));
                }
                _ => return None,
            }
        }
        Some(values)
    }

    fn has_flag(input: &TUnion, flag: i64, codebase: &CodebaseMetadata) -> Tri {
        let flags = if Self::constant_array(input) {
            let (present, flags) = Self::offset(input, b"flags", codebase);
            match present {
                Tri::No => get_literal_int(0),
                Tri::Yes => flags,
                Tri::Maybe => Self::union(&flags, &get_literal_int(0), codebase),
            }
        } else {
            input.clone()
        };
        let Some(values) = Self::scalar_ints(&flags).filter(|values| !values.is_empty()) else {
            return Tri::Maybe;
        };
        let first = values[0] & flag == flag;
        if values.iter().all(|value| (*value & flag == flag) == first) {
            Tri::from(first)
        } else {
            Tri::Maybe
        }
    }

    /// Preconditions: isArray()->yes(). TemplateArrayType inherits its bound's iterable domains.
    fn array_parameters(input: &TUnion, codebase: &CodebaseMetadata) -> (TUnion, TUnion) {
        input
            .types
            .iter()
            .filter_map(|atomic| {
                if let TAtomic::Array(array) = atomic {
                    let (_, value) = get_array_parameters(array, codebase);
                    let key = BuiltinFunctions::iterable_keys(
                        &TUnion::from_atomic(atomic.clone()),
                        codebase,
                    )
                    .0;
                    Some((key, value))
                } else {
                    atomic.map_generic_parameter_constraint(|constraint| {
                        Self::array_parameters(constraint, codebase)
                    })
                }
            })
            .fold(
                (get_never(), get_never()),
                |(keys, values), (key, value)| {
                    (
                        Self::union(&keys, &key, codebase),
                        Self::union(&values, &value, codebase),
                    )
                },
            )
    }

    fn filter(input: &TUnion, filter: i64, flags: &TUnion, codebase: &CodebaseMetadata) -> TUnion {
        let (options_present, options) = Self::offset(flags, b"options", codebase);
        let has_options = Self::query(flags, TAtomic::is_array).and(options_present);
        let option = |name| {
            (has_options.yes() && Self::constant_array(&options))
                .then(|| Self::offset(&options, name, codebase))
                .filter(|(present, _)| present.yes())
                .map(|(_, ty)| ty)
        };
        let default = option(b"default").unwrap_or_else(|| {
            match Self::has_flag(flags, Self::NULL_ON_FAILURE, codebase) {
                Tri::Yes => get_null(),
                Tri::No => get_false(),
                Tri::Maybe => Self::union(&get_false(), &get_null(), codebase),
            }
        });
        let require = Self::has_flag(flags, Self::REQUIRE_ARRAY, codebase);
        if require.maybe() {
            return get_mixed();
        }
        let array = Self::query(input, TAtomic::is_array);
        if array.no() && require.yes() {
            return default;
        }
        let force = Self::has_flag(flags, Self::FORCE_ARRAY, codebase);
        if require.no() && force.maybe() {
            return get_mixed();
        }
        let (keys, input) = if array.yes() && (require.yes() || force.yes()) {
            Self::array_parameters(input, codebase)
        } else {
            (get_mixed(), input.clone())
        };
        let exact = if Self::query(&input, |a| matches!(a, TAtomic::Scalar(_))).no()
            && Self::query(&input, TAtomic::is_null).no()
        {
            Some(default.clone())
        } else if filter == 257 {
            Self::exact_integer(&input, &default, flags, codebase)
        } else {
            None
        };
        let mut ty = exact.clone().unwrap_or_else(|| {
            if filter == 257 {
                get_int()
            } else if filter == 275 {
                get_truthy_string()
            } else {
                get_mixed()
            }
        });
        if filter == 257 {
            ty = Self::apply_range(
                ty,
                option(b"min_range"),
                option(b"max_range"),
                &default,
                codebase,
            );
        }
        // Validation filters never sanitize strings. IP's non-falsy-string already carries both accessories.
        if exact.is_none()
            || has_options.maybe()
            || (input != ty && Self::contains(&input, &ty, codebase))
        {
            if !Self::contains(&default, &ty, codebase) {
                ty = Self::union(&ty, &default, codebase);
            }
        }
        if require.yes() || force.yes() {
            ty = get_keyed_array(keys, ty);
            if require.yes() && !array.yes() {
                ty = Self::union(&ty, &default, codebase);
            }
        }
        ty
    }

    fn contains(container: &TUnion, input: &TUnion, codebase: &CodebaseMetadata) -> bool {
        union_comparator::is_contained_by(
            codebase,
            input,
            container,
            false,
            false,
            false,
            &mut ComparisonResult::default(),
        )
    }

    fn exact_integer(
        input: &TUnion,
        default: &TUnion,
        flags: &TUnion,
        codebase: &CodebaseMetadata,
    ) -> Option<TUnion> {
        if input.is_int() {
            return Some(input.clone());
        }
        if input.is_true() {
            return Some(get_literal_int(1));
        }
        if input.is_false() || input.is_null() {
            return Some(default.clone());
        }
        if let Some(value) = input.get_single_literal_float_value() {
            let integer = value as i64;
            return Some(
                if value < -(i64::MIN as f64) && value - integer as f64 == 0.0 {
                    get_literal_int(integer)
                } else {
                    default.clone()
                },
            );
        }
        let value = input.get_single_literal_string_value()?;
        let octal = Self::has_flag(flags, 1, codebase);
        let hex = Self::has_flag(flags, 2, codebase);
        if octal.maybe() || hex.maybe() {
            return None;
        }
        // Port the three preg_match calls, including PHPStan's deliberately narrow 0o octal syntax.
        for (enabled, pattern, radix) in [
            (octal.yes(), r"\A0[oO][0-7]+\z", 8),
            (hex.yes(), r"\A0[xX][0-9A-Fa-f]+\z", 16),
            (true, r"\A[+-]?(?:0|[1-9][0-9]*)\z", 10),
        ] {
            if enabled
                && pcre2::bytes::Regex::new(pattern)
                    .unwrap()
                    .is_match(value)
                    .unwrap()
            {
                let text = std::str::from_utf8(value).unwrap(); // the patterns accept ASCII only
                let integer = if radix == 10 {
                    Some(text.parse().unwrap_or(if text.starts_with('-') {
                        i64::MIN
                    } else {
                        i64::MAX
                    }))
                } else {
                    i64::from_str_radix(&text[2..], radix).ok()
                };
                return Some(integer.map_or_else(|| default.clone(), get_literal_int));
            }
        }
        Some(default.clone())
    }

    fn apply_range(
        ty: TUnion,
        min: Option<TUnion>,
        max: Option<TUnion>,
        default: &TUnion,
        codebase: &CodebaseMetadata,
    ) -> TUnion {
        if !ty.is_int() || (min.is_none() && max.is_none()) {
            return ty;
        }
        let bound = |value: &Option<TUnion>, minimum| {
            value.as_ref().and_then(|value| {
                if let Some(values) = Self::scalar_ints(value).filter(|values| values.len() == 1) {
                    Some(values[0])
                } else {
                    // Only IntegerRangeType owns these bounds; a sparse union's extrema are not a range option.
                    value.get_single_int()?;
                    if minimum {
                        value.get_minimum_int_value()
                    } else {
                        value.get_maximum_int_value()
                    }
                }
            })
        };
        let min = bound(&min, true);
        let max = bound(&max, false);
        if min.zip(max).is_some_and(|(min, max)| min > max) {
            return default.clone();
        }
        let range = if min.is_some() && min == max {
            get_literal_int(min.unwrap())
        } else {
            get_int_range(min, max)
        };
        if intersect_union_types(&range, &ty, codebase).is_none_or(|ty| ty.is_never()) {
            return default.clone();
        }
        if range != ty && Self::contains(&range, &ty, codebase) {
            return ty;
        }
        if min.is_none() || max.is_none() {
            Self::union(&range, default, codebase)
        } else {
            range
        }
    }
}

/// PHPStan 2.3.1's StrRepeatFunctionReturnTypeExtension, in its branch order. The size budget is
/// the sum of every constant input's byte length times one constant multiplier, strictly below 100.
struct StrRepeat;

impl StrRepeat {
    fn return_type(input: &TUnion, count: &TUnion) -> TUnion {
        if BuiltinFunctions::count_in_range(count, 0, Some(0)) {
            return mago_codex::ttype::get_literal_string(mago_word::word(b""));
        }
        if BuiltinFunctions::count_in_range(count, i64::MIN, Some(0)) {
            return get_never();
        }
        let strings = Self::strings(input, 0);
        // TemplateConstantIntegerType inherits ConstantIntegerType; TemplateIntegerType does not.
        let multiplier = count.get_single_literal_int_value().or_else(|| {
            if let [TAtomic::GenericParameter(template)] = count.types.as_ref() {
                template.constraint.get_single_literal_int_value()
            } else {
                None
            }
        });
        if let (Some(multiplier), Some(strings)) = (multiplier, &strings) {
            // PHP promotes overflowing integer arithmetic to float. A wider sum gives the same budget
            // decision without overflow or allocating an oversized repetition (including huge counts).
            let length: u128 = strings.iter()
                .map(|string| string.as_bytes().len() as u128 * multiplier as u128).sum();
            if length < 100 {
                let mut values: Vec<_> = strings.iter().map(|string| {
                    TString::known_literal(mago_word::word(&string.as_bytes().repeat(multiplier as usize)))
                }).collect();
                values.sort_unstable();
                values.dedup();
                return TUnion::from_vec(values.into_iter()
                    .map(|string| TAtomic::Scalar(TScalar::String(string))).collect());
            }
        }
        let positive = BuiltinFunctions::count_in_range(count, 1, None);
        let nonempty = positive && Self::property(input, 0, TString::is_non_empty);
        let truthy = nonempty && Self::property(input, 0, TString::is_truthy);
        let literal = Self::property(input, 0, TString::is_literal_origin);
        let numeric = literal && positive && Self::property(input, 0, |string| string.is_numeric)
            && strings.as_ref().is_none_or(|strings| strings.iter().all(|string| {
                // Upstream uses preg_match('#^[0-9]+$#'), not is_numeric(): signs, whitespace,
                // decimal points and exponent markers must not acquire numeric-string. As upstream,
                // `$` also permits one final newline; do not silently replace it with `\z`.
                static DIGITS: std::sync::OnceLock<pcre2::bytes::Regex> = std::sync::OnceLock::new();
                DIGITS.get_or_init(|| pcre2::bytes::Regex::new("^[0-9]+$").unwrap())
                    .is_match(string.as_bytes()).unwrap()
            }));
        let lower = Self::property(input, 0, TString::is_lowercase);
        let upper = Self::property(input, 0, TString::is_uppercase);
        let casing = match (lower, upper) {
            (true, true) => TStringCasing::Both,
            (true, false) => TStringCasing::Lowercase,
            (false, true) => TStringCasing::Uppercase,
            (false, false) => TStringCasing::Unspecified,
        };
        TUnion::from_atomic(TAtomic::Scalar(TScalar::String(if literal {
            TString::unspecified_literal_with_props(numeric, truthy, nonempty, false, casing)
        } else {
            TString::general_with_props(numeric, truthy, nonempty, false, casing)
        })))
    }

    /// getConstantStrings is all-or-nothing for ordinary unions. Benevolent unions may omit
    /// non-string members, but a general string still cancels extraction. A template of a constant
    /// string template becomes TemplateStringType, whose inherited extraction is empty.
    fn strings(input: &TUnion, depth: usize) -> Option<Vec<mago_word::Word>> {
        let depth = if input.types.len() > 1 { 0 } else { depth };
        let mut values = Vec::new();
        for atomic in input.types.iter() {
            match atomic {
                TAtomic::GenericParameter(template) => {
                    values.extend(Self::strings(&template.constraint, depth + 1)?);
                }
                TAtomic::Scalar(TScalar::String(string)) if depth <= 1 => {
                    values.push(string.get_known_literal_atom()?);
                }
                TAtomic::Scalar(TScalar::ClassLikeString(string)) if depth <= 1 => {
                    values.push(string.literal_value()?);
                }
                _ if is_benevolent(input) && !atomic.is_any_string() => {}
                _ => return None,
            }
        }
        (!values.is_empty()).then_some(values)
    }

    /// Queries inherited by the concrete template parent, with independent guarantees for each
    /// union member. Unknown class-string values are truthy, but not literal or case-constrained.
    fn property(input: &TUnion, depth: usize, query: fn(&TString) -> bool) -> bool {
        let depth = if input.types.len() > 1 { 0 } else { depth };
        input.types.iter().all(|atomic| match atomic {
            TAtomic::GenericParameter(template) => Self::property(&template.constraint, depth + 1, query),
            TAtomic::Scalar(TScalar::String(string)) => {
                if depth > 1 && string.is_known_literal() { false } else { query(string) }
            }
            TAtomic::Scalar(TScalar::ClassLikeString(string)) if depth == 0 || (depth == 1 && string.is_literal()) => {
                query(&string.literal_value().map_or_else(TString::truthy, TString::known_literal))
            }
            _ => false,
        })
    }
}

/// StrlenFunctionReturnTypeExtension: scalar *values* are cast, not their types' toString() results.
/// In particular, a negative zero has length two, not the union of both zero signs' lengths.
struct Strlen;

impl Strlen {
    fn return_type(input: &TUnion) -> Option<TUnion> {
        if let Some(mut lengths) = Self::lengths(input, 0) {
            lengths.sort_unstable();
            lengths.dedup();
            let min = lengths[0];
            let max = *lengths.last().unwrap();
            return Some(if min == max {
                // IntegerRangeType::fromInterval collapses equal bounds. Mago's range helper does not;
                // retain an actual literal so subsequent calls can extract its scalar value too.
                get_literal_int(min)
            } else if max - min == lengths.len() as i64 - 1 {
                get_int_range(Some(min), Some(max))
            } else {
                // Neither this extension nor getConstantScalarValues has a 128-value budget. Do not use
                // Mago's default combiner: its unrelated literal threshold would widen a sparse result.
                TUnion::from_vec(
                    lengths
                        .into_iter()
                        .map(|n| get_literal_int(n).get_single().clone())
                        .collect(),
                )
            });
        }
        // PHPStan's boolean and definitely-empty-string branches are already resolved by scalar extraction:
        // BooleanType enumerates true/false (including inside unions), and an empty string is a constant.
        // The remaining positive branch also covers remove(input, int|float)->isNonEmptyString()->yes().
        Self::positive(input, 0).then(|| get_int_range(Some(1), None))
    }

    /// UnionType::getConstantScalarValues is all-or-nothing, even for benevolent unions. Unlike
    /// getFiniteTypes, it does not enumerate bounded integer ranges. Keep strings as bytes until counting.
    fn lengths(input: &TUnion, template_depth: usize) -> Option<Vec<i64>> {
        // TemplateUnionType inherits its concrete members' extraction, not scalar-template widening.
        let depth = if input.types.len() > 1 {
            0
        } else {
            template_depth
        };
        let mut lengths = Vec::new();
        for atomic in input.types.iter() {
            let length = match atomic {
                TAtomic::GenericParameter(parameter) => {
                    lengths.extend(Self::lengths(&parameter.constraint, depth + 1)?);
                    continue;
                }
                TAtomic::Scalar(TScalar::Bool(boolean)) => {
                    if depth > 0 || boolean.is_general() {
                        // TemplateBooleanType inherits BooleanType even when its bound is true or false.
                        lengths.extend([0, 1]);
                        continue;
                    }
                    i64::from(boolean.is_true())
                }
                TAtomic::Null => 0,
                TAtomic::Scalar(TScalar::Integer(integer)) if depth <= 1 => {
                    integer.get_literal_value()?.to_string().len() as i64
                }
                TAtomic::Scalar(TScalar::Float(float)) if depth == 0 => {
                    pun_tokens::php_compat::float_to_string(float.get_literal_value()?).len() as i64
                }
                TAtomic::Scalar(TScalar::String(string)) if depth <= 1 => {
                    string.get_known_literal_value()?.len() as i64
                }
                TAtomic::Scalar(TScalar::ClassLikeString(string)) if depth <= 1 => {
                    string.literal_value()?.as_bytes().len() as i64
                }
                _ => return None,
            };
            lengths.push(length);
        }
        Some(lengths)
    }

    /// TemplateTypeFactory's concrete parent owns queries: a template of a template constant string is
    /// TemplateStringType (not the constant subclass), and a constant-float bound becomes TemplateMixedType.
    fn positive(input: &TUnion, template_depth: usize) -> bool {
        let depth = if input.types.len() > 1 {
            0
        } else {
            template_depth
        };
        input.types.iter().all(|atomic| match atomic {
            TAtomic::GenericParameter(parameter) => {
                Self::positive(&parameter.constraint, depth + 1)
            }
            TAtomic::Scalar(TScalar::Integer(_) | TScalar::Numeric) => true,
            TAtomic::Scalar(TScalar::Float(float)) => depth == 0 || float.is_general(),
            TAtomic::Scalar(TScalar::String(string)) => {
                string.is_non_empty() && (depth <= 1 || !string.is_known_literal())
            }
            TAtomic::Scalar(TScalar::ClassLikeString(string)) => {
                depth == 0 || (depth == 1 && string.is_literal())
            }
            TAtomic::Never => depth == 0,
            _ => false,
        })
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
                (atomic.get_minimum_int_value().is_some_and(|n| n >= min)
                    || (min == i64::MIN && matches!(atomic, TAtomic::Scalar(TScalar::Integer(_)))))
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
