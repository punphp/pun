//! ConfigCollectionRule: `Config::collection('key')` where the config value is no array
//! ([`crate::larastan::config`]).

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::super::Checker;
use crate::larastan::config::{IsArray, union_is_array};

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
    let mut types = larastan.configs.types(&keys).unwrap_or_default();
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
