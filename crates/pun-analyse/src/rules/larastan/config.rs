//! ConfigCollectionRule: `Config::collection('key')` where the config value is no array
//! ([`crate::larastan::config`]), and the config extensions' return types ([`Types`]).

use std::sync::Arc;

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::function::{FunctionReturnTypeProvider, FunctionTarget};
use mago_analyzer::plugin::provider::method::{MethodReturnTypeProvider, MethodTarget};
use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{get_array_parameters, get_error};
use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::super::{Checker, FunctionScopeKind, declared};
use super::{extensions, helpers};
use crate::larastan::config::{Configs, IsArray, union_is_array};

const CONFIG: &[u8] = b"Illuminate\\Support\\Facades\\Config";

/// The arguments the rule reads the types of, for a `Config::collection()` call.
pub(super) fn collection<'a, 'arena>(
    names: &mago_names::ResolvedNames<'arena>,
    call: &'a StaticMethodCall<'arena>,
) -> Option<&'a [Argument<'arena>]> {
    let ClassLikeMemberSelector::Identifier(m) = &call.method else {
        return None;
    };
    let Expression::Identifier(class) = call.class else {
        return None;
    };
    (m.value == b"collection" && names.get(&class.span()) == CONFIG)
        .then_some(call.argument_list.arguments.as_slice())
}

pub(super) fn check<'arena>(c: &mut Checker<'_, 'arena>, call: &StaticMethodCall<'arena>) {
    let Some(larastan) = c.larastan.filter(|l| l.check_config_types) else {
        return;
    };
    let Some(args @ [first, ..]) = collection(c.names, call) else {
        return;
    };
    // getConstantStrings(): every part of the type is a constant string
    let keys: Vec<String> = c.type_of(first.value()).map_or_else(Vec::new, |t| {
        t.types
            .iter()
            .map(|a| {
                a.get_literal_string_value()
                    .map(|v| String::from_utf8_lossy(v).into_owned())
            })
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default()
    });
    // `$returnTypes + $configTypes`: the default's type takes the first key
    let class = c.class_scope();
    let mut scope = declared::scope_at(c, call.span(), class);
    let function = c.functions.last().and_then(|function| match function.kind {
        FunctionScopeKind::Function => {
            let name = scope.names.namespace_name().map_or_else(
                || function.name.clone(),
                |namespace| format!("{}\\{}", String::from_utf8_lossy(namespace), function.name),
            );
            c.codebase.get_function(name.as_bytes())
        }
        FunctionScopeKind::Method { .. } => class.and_then(|class| {
            c.codebase
                .get_method(class.name.as_bytes(), function.name.as_bytes())
        }),
    });
    if let Some(function) = function {
        scope = declared::value_scope(c, function, class);
    }
    let mut types =
        declared::with_classes(c, || larastan.configs.types(&keys, &scope)).unwrap_or_default();
    if let Some(default) = args.get(1) {
        let default = c
            .type_of(default.value())
            .map_or(IsArray::Maybe, |t| union_is_array(t, true));
        match types.first_mut() {
            Some(t) => *t = default,
            None => types.push(default),
        }
    }
    if types.iter().any(|t| *t != IsArray::No) {
        return;
    }
    let Some(key) = keys.first() else {
        return;
    };
    c.report(
        call,
        "larastan.configCollection",
        format!("Config key '{key}' is not an array."),
        None,
    );
}

/// The spans [`check`] reads the types of.
pub(super) fn typed(
    names: &mago_names::ResolvedNames,
    call: &StaticMethodCall,
) -> Vec<mago_span::Span> {
    collection(names, call)
        .map(|args| args.iter().take(2).map(|a| a.value().span()).collect())
        .unwrap_or_default()
}

static META: ProviderMeta =
    ProviderMeta::new("pun-larastan-config", "pun-larastan-config", "Larastan's config extensions");

/// ConfigFunctionDynamicFunctionReturnTypeExtension (`config()`), ConfigRepositoryDynamicMethodReturnTypeExtension
/// (`get()` and `collection()` of `Illuminate\Config\Repository`) and
/// ConfigFacadeCollectionDynamicStaticMethodReturnTypeExtension (`Config::collection()`), all through
/// ConfigHelper::getReturnTypes(); registered with `checkConfigTypes`.
pub(crate) struct Types(pub(crate) Arc<Configs>);

impl Provider for Types {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl Types {
    /// ConfigHelper::getReturnTypes(): the default's type (its callables called, generalized) in place of the first
    /// key's; `None` when Pun does not infer a type.
    fn return_types(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<Vec<TUnion>> {
        let codebase = context.codebase();
        let argument = |i: usize| invocation.arguments().get(i).and_then(|a| a.value());
        let keys: Vec<String> = context
            .get_expression_type(argument(0)?)?
            .types
            .iter()
            .map(|a| a.get_literal_string_value().map(|v| String::from_utf8_lossy(v).into_owned()))
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let mut types = self.0.value_types(&keys).map_or(Some(Vec::new()), |t| t)?;
        if invocation.argument_count() > 1 {
            let default = context.get_expression_type(argument(1)?)?;
            let default = helpers::generalize(codebase, &helpers::call_callables(codebase, default));
            match types.first_mut() {
                Some(t) => *t = default,
                None => types.push(default),
            }
        }
        Some(types)
    }

    /// `get()`: the union, `None` without types.
    fn get(&self, context: &ProviderContext<'_, '_, '_>, invocation: &InvocationInfo<'_, '_, '_>) -> Option<TUnion> {
        let types = self.return_types(context, invocation)?;
        (!types.is_empty()).then(|| helpers::union(context.codebase(), types))
    }

    /// `collection()`: a Collection of the union's keys and values when it is an array, else an ErrorType.
    fn collection(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let codebase = context.codebase();
        let types = self.return_types(context, invocation)?;
        let ty = helpers::union(codebase, types);
        let arrays: Option<Vec<_>> = ty
            .types
            .iter()
            .map(|a| match a {
                // a sealed list's keys are its indices (Mago adds `int<0, max>` once it has values)
                TAtomic::Array(TArray::List(list)) if list.element_type.is_never() => {
                    let known = list.known_elements.as_deref()?;
                    Some((
                        TUnion::from_vec(known.keys().map(|i| TAtomic::Scalar(TScalar::literal_int(*i as i64))).collect()),
                        helpers::union(codebase, known.values().map(|(_, t)| t.clone())),
                    ))
                }
                TAtomic::Array(array) => Some(get_array_parameters(array, codebase)),
                _ => None,
            })
            .collect();
        match arrays {
            Some(arrays) if !arrays.is_empty() => {
                let (keys, values): (Vec<_>, Vec<_>) = arrays.into_iter().unzip();
                Some(extensions::generic(
                    codebase,
                    "Illuminate\\Support\\Collection",
                    vec![helpers::union(codebase, keys), helpers::union(codebase, values)],
                ))
            }
            _ => Some(get_error()),
        }
    }
}

impl FunctionReturnTypeProvider for Types {
    fn targets() -> FunctionTarget {
        FunctionTarget::Exact(b"config")
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        if invocation.has_no_arguments() {
            return None;
        }
        self.get(context, invocation)
    }
}

impl MethodReturnTypeProvider for Types {
    fn targets() -> &'static [MethodTarget] {
        static TARGETS: [MethodTarget; 2] = [MethodTarget::any_class(b"get"), MethodTarget::any_class(b"collection")];
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
        let is = |parent: &[u8]| receiver.eq_ignore_ascii_case(parent) || codebase.is_instance_of(receiver, parent);
        if invocation.has_no_arguments() {
            return None;
        }
        let repository = is(b"Illuminate\\Config\\Repository");
        if method.eq_ignore_ascii_case(b"get") && repository {
            self.get(context, invocation)
        } else if method.eq_ignore_ascii_case(b"collection") && (repository || is(CONFIG)) {
            self.collection(context, invocation)
        } else {
            None
        }
    }
}
