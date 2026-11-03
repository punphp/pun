//! Larastan's helper and container extensions, as Mago return type providers for the rules' inference:
//!
//! - Helpers\AppExtension (`app()`, `resolve()`), the container's `make()`, `makeWith()` and `resolve()`
//!   (ApplicationMakeDynamicReturnTypeExtension, ContainerMakeDynamicReturnTypeExtension) and `App::make()`
//!   (AppMakeDynamicReturnTypeExtension), all through AppMakeHelper;
//! - Helpers\ValidatorExtension (`validator()`), Helpers\ValueExtension (`value()`), Helpers\StrExtension (`str()`),
//!   and, with `generalizeEnvReturnType`, EnvFunctionDynamicFunctionReturnTypeExtension
//!   (`env()`);
//! - RequestFileExtension (`$request->file()`), RequestRouteExtension (`$request->route()`) and

use std::collections::BTreeMap;
use std::sync::Arc;

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::function::{FunctionReturnTypeProvider, FunctionTarget};
use mago_analyzer::plugin::provider::method::{MethodReturnTypeProvider, MethodTarget};
use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::object::with_properties::TObjectWithProperties;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::named::TNamedObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    add_union_type, get_bool, get_error, get_float, get_int, get_keyed_array, get_mixed, get_never, get_null,
    get_string,
};
use mago_syntax::cst::Expression;

use crate::larastan::app::App;

static META: ProviderMeta = ProviderMeta::new("pun-larastan-helpers", "pun-larastan-helpers", "Larastan's helper extensions");

const CONTAINER: &str = "Illuminate\\Contracts\\Container\\Container";
const REQUEST: &str = "Illuminate\\Http\\Request";
const UPLOADED_FILE: &str = "Illuminate\\Http\\UploadedFile";

/// `new ObjectType($class)`, spelled as the codebase declares it.
fn object(codebase: &CodebaseMetadata, class: &str) -> TUnion {
    let name = codebase
        .get_class_like(class.as_bytes())
        .map(|k| k.original_name)
        .unwrap_or_else(|| mago_word::word(class.trim_start_matches('\\')));
    TUnion::from_atomic(TAtomic::Object(TObject::Named(TNamedObject::new(name))))
}

/// `TypeCombinator::union()`.
pub(super) fn union(codebase: &CodebaseMetadata, types: impl IntoIterator<Item = TUnion>) -> TUnion {
    let mut types = types.into_iter();
    let first = types.next().unwrap_or_else(get_never);
    types.fold(first, |t, u| add_union_type(t, &u, codebase, CombinerOptions::default()))
}

/// `TypeUtils::toBenevolentUnion()`, which Pun marks with both ignore flags.
fn benevolent(mut t: TUnion) -> TUnion {
    if t.types.len() > 1 {
        t.set_ignore_nullable_issues(true);
        t.set_ignore_falsable_issues(true);
    }
    t
}

/// The return type of a callable part (`getCallableParametersAcceptors($scope)[0]->getReturnType()`): a callable
/// signature's, mixed for a `Closure` object; `None` for a part that is no callable.
fn callable_return(t: &TAtomic) -> Option<TUnion> {
    match t {
        TAtomic::Callable(TCallable::Signature(s)) => Some(s.get_return_type().cloned().unwrap_or_else(get_mixed)),
        TAtomic::Object(TObject::Named(n)) if n.name.as_bytes().eq_ignore_ascii_case(b"Closure") => Some(get_mixed()),
        _ => None,
    }
}

/// `TypeTraverser::map()` replacing each callable part with its return type.
pub(super) fn call_callables(codebase: &CodebaseMetadata, t: &TUnion) -> TUnion {
    union(codebase, t.types.iter().map(|a| callable_return(a).unwrap_or_else(|| TUnion::from_atomic(a.clone()))))
}

/// `$type->generalize(GeneralizePrecision::lessSpecific())`.
pub(super) fn generalize(codebase: &CodebaseMetadata, t: &TUnion) -> TUnion {
    union(
        codebase,
        t.types.iter().map(|a| match a {
            TAtomic::Scalar(TScalar::Integer(_)) => get_int(),
            TAtomic::Scalar(TScalar::String(_)) => get_string(),
            TAtomic::Scalar(TScalar::Float(_)) => get_float(),
            TAtomic::Scalar(TScalar::Bool(_)) => get_bool(),
            TAtomic::Array(TArray::Keyed(k)) if k.known_items.is_some() || k.parameters.is_some() => {
                let (key, value) = k
                    .known_items
                    .iter()
                    .flat_map(|items| items.iter())
                    .map(|(key, (_, value))| (key.to_union(), value.clone()))
                    .chain(k.parameters.iter().map(|(key, value)| ((**key).clone(), (**value).clone())))
                    .fold((None::<TUnion>, None::<TUnion>), |(ks, vs), (key, value)| {
                        (
                            Some(ks.map_or(key.clone(), |ks| union(codebase, [ks, key]))),
                            Some(vs.map_or(value.clone(), |vs| union(codebase, [vs, value]))),
                        )
                    });
                get_keyed_array(
                    generalize(codebase, &key.unwrap_or_else(get_never)),
                    generalize(codebase, &value.unwrap_or_else(get_never)),
                )
            }
            a => TUnion::from_atomic(a.clone()),
        }),
    )
}

/// The constant strings of a type (`getConstantStrings()`), class strings included; empty unless every part is one.
fn constant_strings(t: &TUnion) -> Vec<String> {
    let strings: Vec<String> = t
        .types
        .iter()
        .filter_map(|a| {
            a.get_literal_string_value()
                .map(|v| String::from_utf8_lossy(v).into_owned())
                .or_else(|| a.get_class_string_value().map(|v| String::from_utf8_lossy(v.as_bytes()).into_owned()))
        })
        .collect();
    if strings.len() == t.types.len() { strings } else { Vec::new() }
}

/// AppMakeHelper::resolveTypeFromCall(): for each constant string, the class of what the container makes for it, else
/// the class it names, else an ErrorType; mixed for a type without constant strings.
fn app_make(app: &App, context: &ProviderContext<'_, '_, '_>, argument: Option<&Expression<'_>>) -> Option<TUnion> {
    let Some(argument) = argument else {
        return Some(get_error());
    };
    let strings = constant_strings(context.get_expression_type(argument)?);
    if strings.is_empty() {
        return Some(get_mixed());
    }
    Some(resolve_all(app, context.codebase(), strings))
}

/// HasContainer::resolve() for each constant string: the class of what the container makes for it, else the class
/// it names, else an ErrorType.
fn resolve_all(app: &App, codebase: &CodebaseMetadata, strings: Vec<String>) -> TUnion {
    let mut types = Vec::new();
    for s in strings {
        let class = app
            .resolve(&s)
            .filter(|c| codebase.class_like_exists(c.as_bytes()))
            .or_else(|| codebase.class_like_exists(s.trim_start_matches('\\').as_bytes()).then_some(s.as_str()));
        match class {
            Some(class) => types.push(object(codebase, class)),
            None => return get_error(),
        }
    }
    union(codebase, types)
}

/// ContainerArrayAccessDynamicMethodReturnTypeExtension: `$container['key']` (`offsetGet()`) on a container is what
/// the container makes for each constant string of the key, else an ErrorType. Mago reads an `ArrayAccess` offset
/// without asking the method providers, so the hook replaces the read's type.
pub(crate) struct ContainerOffsets(pub(crate) Arc<App>);

impl Provider for ContainerOffsets {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl mago_analyzer::plugin::ExpressionHook for ContainerOffsets {
    fn after_expression(
        &self,
        expr: &Expression<'_>,
        context: &mut mago_analyzer::plugin::HookContext<'_, '_>,
    ) -> mago_analyzer::plugin::HookResult<()> {
        let Expression::ArrayAccess(access) = expr else { return Ok(()) };
        let codebase = context.codebase();
        let container = context.get_expression_type(access.array).is_some_and(|t| {
            !t.types.is_empty()
                && t.types.iter().all(|a| match a {
                    TAtomic::Object(TObject::Named(n)) => {
                        n.name.as_bytes().eq_ignore_ascii_case(CONTAINER.as_bytes())
                            || codebase.is_instance_of(n.name.as_bytes(), CONTAINER.as_bytes())
                    }
                    _ => false,
                })
        });
        if !container {
            return Ok(());
        }
        let strings = context.get_expression_type(access.index).map(constant_strings).unwrap_or_default();
        if strings.is_empty() {
            return Ok(());
        }
        let ty = resolve_all(&self.0, codebase, strings);
        context.set_expression_type(expr, ty);
        Ok(())
    }
}

pub(crate) struct Functions {
    pub(crate) app: Arc<App>,
    /// `generalizeEnvReturnType`, which registers EnvFunctionDynamicFunctionReturnTypeExtension.
    pub(crate) generalize_env: bool,
}

impl Provider for Functions {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for Functions {
    fn targets() -> FunctionTarget {
        FunctionTarget::ExactMultiple(&[b"app", b"resolve", b"validator", b"value", b"str", b"env", b"literal"])
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let codebase = context.codebase();
        let count = invocation.argument_count();
        let argument = |i: usize| invocation.arguments().get(i).and_then(|a| a.value());
        let type_of = |i: usize| argument(i).and_then(|a| context.get_expression_type(a)).cloned();
        match invocation.function_name().to_ascii_lowercase().trim_start_matches('\\') {
            "app" | "resolve" if count == 0 => Some(object(codebase, "Illuminate\\Foundation\\Application")),
            "app" | "resolve" => app_make(&self.app, context, argument(0)),
            "validator" if count == 0 => Some(object(codebase, "Illuminate\\Contracts\\Validation\\Factory")),
            // TypeCombinator::intersect() of the class and the contract it implements
            "validator" => Some(object(codebase, "Illuminate\\Validation\\Validator")),
            "value" if count == 0 => Some(get_never()),
            "value" => Some(call_callables(codebase, &type_of(0)?)),
            "literal" => literal(context, invocation),
            "str" if count == 1 => Some(object(codebase, "Illuminate\\Support\\Stringable")),
            "str" => Some(get_mixed()),
            "env" if self.generalize_env && count >= 2 => {
                let default = type_of(1)?;
                Some(generalize(codebase, &call_callables(codebase, &default)))
            }
            _ => None,
        }
    }
}

/// `ObjectShapeType($properties)` intersected with `stdClass`.
fn std_class_shape(codebase: &CodebaseMetadata, properties: BTreeMap<mago_word::Word, (bool, TUnion)>) -> TAtomic {
    let TAtomic::Object(TObject::Named(mut std_class)) = object(codebase, "stdClass").types[0].clone() else {
        unreachable!()
    };
    let shape = TObjectWithProperties { known_properties: properties, sealed: false };
    std_class.intersection_types = Some(Box::new(vec![TAtomic::Object(TObject::WithProperties(shape))]));
    TAtomic::Object(TObject::Named(std_class))
}

/// A constant array's items (`getConstantArrays()` of one array), keys as strings.
fn constant_array(a: &TAtomic) -> Option<Vec<(String, TUnion)>> {
    match a {
        TAtomic::Array(TArray::Keyed(k)) if k.parameters.is_none() => Some(
            k.known_items
                .iter()
                .flat_map(|items| items.iter())
                .map(|(key, (_, t))| {
                    let key = match key {
                        ArrayKey::Integer(i) => i.to_string(),
                        ArrayKey::String(s) => String::from_utf8_lossy(s.as_bytes()).into_owned(),
                        _ => return None,
                    };
                    Some((key, t.clone()))
                })
                .collect::<Option<Vec<_>>>()?,
        ),
        TAtomic::Array(TArray::List(l)) if l.element_type.is_never() => Some(
            l.known_elements.iter().flat_map(|e| e.iter()).map(|(i, (_, t))| (i.to_string(), t.clone())).collect(),
        ),
        _ => None,
    }
}

/// Helpers\LiteralExtension (`literal()`): a single unnamed argument's type (an unpacked constant array's as an
/// object shape, a `stdClass` with an empty shape), else an object shape of the arguments by name or position.
fn literal(context: &ProviderContext<'_, '_, '_>, invocation: &InvocationInfo<'_, '_, '_>) -> Option<TUnion> {
    let codebase = context.codebase();
    let arguments = invocation.arguments();
    let shape_of = |items: Vec<(String, TUnion)>| {
        std_class_shape(codebase, items.into_iter().map(|(k, t)| (mago_word::word(k.as_bytes()), (false, t))).collect())
    };
    if let [argument] = arguments.as_slice()
        && argument.get_parameter_name().is_none()
    {
        let ty = context.get_expression_type(argument.value()?)?.clone();
        if argument.is_unpacked() {
            if ty.types.len() > 1 {
                return Some(TUnion::from_vec(
                    ty.types.iter().map(|a| constant_array(a).map_or_else(|| a.clone(), &shape_of)).collect(),
                ));
            }
            if let Some(items) = ty.types.first().and_then(constant_array) {
                return Some(TUnion::from_atomic(shape_of(items)));
            }
        }
        let std_class = |a: &TAtomic| {
            matches!(a, TAtomic::Object(TObject::Named(n)) if n.name.as_bytes().eq_ignore_ascii_case(b"stdClass"))
        };
        if ty.types.iter().all(|a| matches!(a, TAtomic::Object(_))) && ty.types.iter().any(std_class) {
            return Some(TUnion::from_vec(
                ty.types.iter().map(|a| if std_class(a) { shape_of(Vec::new()) } else { a.clone() }).collect(),
            ));
        }
        return Some(ty);
    }
    let mut items = Vec::new();
    for (index, argument) in arguments.iter().enumerate() {
        let name = argument
            .get_parameter_name()
            .map_or_else(|| index.to_string(), |n| String::from_utf8_lossy(n).into_owned());
        items.push((name, context.get_expression_type(argument.value()?)?.clone()));
    }
    Some(TUnion::from_atomic(shape_of(items)))
}

pub(crate) struct Methods(pub(crate) Arc<App>);

impl Provider for Methods {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl MethodReturnTypeProvider for Methods {
    fn targets() -> &'static [MethodTarget] {
        static TARGETS: [MethodTarget; 5] = [
            MethodTarget::any_class(b"make"),
            MethodTarget::any_class(b"makeWith"),
            MethodTarget::any_class(b"resolve"),
            MethodTarget::any_class(b"file"),
            MethodTarget::any_class(b"route"),
        ];
        &TARGETS
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        class: &[u8],
        method: &[u8],
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let codebase = context.codebase();
        let receiver = invocation.inner().target.get_method_context().map_or(class, |t| t.class_like_metadata.name.as_bytes());
        let is = |parent: &str| {
            receiver.eq_ignore_ascii_case(parent.as_bytes()) || codebase.is_instance_of(receiver, parent.as_bytes())
        };
        let argument = |i: usize| invocation.arguments().get(i).and_then(|a| a.value());
        match method.to_ascii_lowercase().as_slice() {
            b"make" | b"makewith" | b"resolve" if is(CONTAINER) => app_make(&self.0, context, argument(0)),
            b"make" | b"makewith" if is("Illuminate\\Support\\Facades\\App") => app_make(&self.0, context, argument(0)),
            b"file" if is(REQUEST) => {
                let file = object(codebase, UPLOADED_FILE);
                let files = get_keyed_array(get_int(), file.clone());
                let third = match invocation.argument_count() {
                    0 => return Some(files),
                    1 => get_null(),
                    _ => context.get_expression_type(argument(1)?)?.clone(),
                };
                Some(benevolent(union(codebase, [files, file, third])))
            }
            b"route" if is(REQUEST) => {
                let route = || object(codebase, "Illuminate\\Routing\\Route");
                let default = match invocation.argument_count() {
                    0 => return Some(union(codebase, [route(), get_null()])),
                    2 => {
                        let t = context.get_expression_type(argument(1)?)?;
                        // `isCallable()->yes()`: every part is a callable
                        match t.types.iter().map(callable_return).collect::<Option<Vec<_>>>() {
                            Some(returns) if !returns.is_empty() => union(codebase, returns),
                            _ => t.clone(),
                        }
                    }
                    _ => get_null(),
                };
                let object = TUnion::from_atomic(TAtomic::Object(TObject::Any));
                Some(benevolent(union(codebase, [object, get_string(), default])))
            }
            _ => None,
        }
    }
}
