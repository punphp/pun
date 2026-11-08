//! UndefinedArgumentOrOptionRule: `$this->argument('x')` and `$this->option('x')` in a command, against the
//! definitions of the registered commands that are that class or extend it ([`crate::larastan::console`]).

use std::sync::Arc;

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::method::{MethodReturnTypeProvider, MethodTarget};
use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::array::key::ArrayKey;
use mago_codex::ttype::atomic::array::keyed::KnownItems;
use mago_codex::ttype::atomic::array::keyed::TKeyedArray;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::combiner::CombinerOptions;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::{
    add_union_type, get_bool, get_empty_keyed_array, get_false, get_int, get_keyed_array,
    get_literal_float, get_literal_int, get_literal_string, get_mixed, get_never, get_null,
    get_string, get_true,
};
use mago_syntax::cst::*;
use mago_word::word;

use super::super::Checker;
use super::{class_is, is_object_of};
use crate::larastan::console::{Command, InputArgument, InputOption, Key, Value};

const COMMAND: &[u8] = b"illuminate\\console\\command";

/// The calls the rule reads: `->argument()` or `->option()` with one argument.
pub(super) fn input_call<'a, 'arena>(
    call: &'a MethodCall<'arena>,
) -> Option<(&'a [u8], &'a Expression<'arena>)> {
    let ClassLikeMemberSelector::Identifier(m) = &call.method else {
        return None;
    };
    if !matches!(m.value, b"argument" | b"option") {
        return None;
    }
    match call.argument_list.arguments.as_slice() {
        [a] => Some((m.value, a.value())),
        _ => None,
    }
}

pub(super) fn check<'arena>(c: &mut Checker<'_, 'arena>, call: &MethodCall<'arena>) {
    let Some((method, arg)) = input_call(call) else {
        return;
    };
    let Some(k) = c.class_scope() else {
        return;
    };
    if !class_is(c, k, COMMAND)
        || !c
            .type_of(call.object)
            .is_some_and(|t| is_object_of(c, t, COMMAND))
    {
        return;
    }
    let Some(name) = c
        .type_of(arg)
        .and_then(|t| t.get_single_literal_string_value())
        .map(|v| String::from_utf8_lossy(v).into_owned())
    else {
        return;
    };
    let commands = c
        .larastan
        .map(|l| l.commands.as_slice())
        .unwrap_or_default();
    let mut errors = Vec::new();
    for command in commands
        .iter()
        .filter(|command| super::is_a(c, command.class.as_bytes(), k.name.as_bytes()))
    {
        for key in &command.names {
            if method == b"argument" {
                if !command.has_argument(&name) {
                    errors.push((
                        "larastan.console.undefinedArgument",
                        format!("Command \"{key}\" does not have argument \"{name}\"."),
                    ));
                }
            } else if !command.has_option(&name) {
                errors.push((
                    "larastan.console.undefinedOption",
                    format!("Command \"{key}\" does not have option \"{name}\"."),
                ));
            }
        }
    }
    for (identifier, message) in errors {
        c.report(call, identifier, message, None);
    }
}

/// The ConsoleCommand extensions (ArgumentDynamicReturnTypeExtension, OptionDynamicReturnTypeExtension,
/// HasArgumentDynamicReturnTypeExtension, HasOptionDynamicReturnTypeExtension) with ConsoleApplicationHelper's types,
/// for a command's `argument()`, `arguments()`, `option()`, `options()`, `hasArgument()` and `hasOption()`.
pub(crate) struct Types(pub(crate) Arc<Vec<Command>>);

static META: ProviderMeta = ProviderMeta::new(
    "pun-larastan-console",
    "pun-larastan-console",
    "Larastan's console extensions",
);

impl Provider for Types {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl MethodReturnTypeProvider for Types {
    fn targets() -> &'static [MethodTarget] {
        static TARGETS: [MethodTarget; 6] = [
            MethodTarget::any_class(b"argument"),
            MethodTarget::any_class(b"arguments"),
            MethodTarget::any_class(b"option"),
            MethodTarget::any_class(b"options"),
            MethodTarget::any_class(b"hasArgument"),
            MethodTarget::any_class(b"hasOption"),
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
        let is_command = |class: &[u8]| {
            class.eq_ignore_ascii_case(COMMAND) || codebase.is_instance_of(class, COMMAND)
        };
        let receiver = invocation
            .inner()
            .target
            .get_method_context()
            .map_or(class, |t| t.class_like_metadata.name.as_bytes());
        if !is_command(receiver) {
            return None;
        }
        // ConsoleApplicationResolver::findCommands() of the scope's class
        let scope = context.current_class_name()?;
        if !is_command(scope.as_bytes()) {
            return None;
        }
        let commands: Vec<&Command> = self
            .0
            .iter()
            .filter(|c| {
                c.class.as_bytes().eq_ignore_ascii_case(scope.as_bytes())
                    || codebase.is_instance_of(c.class.as_bytes(), scope.as_bytes())
            })
            .collect();
        let method = method.to_ascii_lowercase();
        let argument = invocation.get_argument(0, &[b"key"]);
        let names = || -> Option<Vec<String>> {
            let ty = context.get_expression_type(argument?)?;
            let names: Vec<String> = ty
                .types
                .iter()
                .filter_map(|t| match t {
                    TAtomic::Scalar(TScalar::String(s)) => s
                        .get_known_literal_value()
                        .map(|v| String::from_utf8_lossy(v).into_owned()),
                    _ => None,
                })
                .collect();
            (!names.is_empty() && names.len() == ty.types.len()).then_some(names)
        };
        match method.as_slice() {
            b"hasargument" | b"hasoption" => {
                let [name] = names()?.try_into().ok()?;
                let mut has = commands.iter().map(|c| {
                    if method == b"hasargument" {
                        c.has_argument(&name)
                    } else {
                        c.has_option(&name)
                    }
                });
                let first = has.next()?;
                has.all(|h| h == first)
                    .then(|| if first { get_true() } else { get_false() })
            }
            b"arguments" | b"options" => shapes(codebase, &commands, method == b"options"),
            _ if argument.is_none() => shapes(codebase, &commands, method == b"option"),
            _ => {
                let default = codebase
                    .get_declaring_method_class(receiver, &method)
                    .and_then(|declaring| {
                        codebase
                            .get_method(declaring.as_bytes(), &method)?
                            .return_type_metadata
                            .as_ref()
                            .map(|t| t.type_union.clone())
                    })
                    .unwrap_or_else(get_mixed);
                let mut returns = Vec::new();
                for name in names()? {
                    let mut types = Vec::new();
                    for command in &commands {
                        let d = &command.merged;
                        if method == b"argument" {
                            types.extend(d.argument(&name).map(|a| argument_type(codebase, a)));
                        } else {
                            let option =
                                d.shortcuts.get(&name).map_or(name.as_str(), String::as_str);
                            types.extend(d.option(option).map(|o| option_type(codebase, o)));
                        }
                        returns.push(if types.is_empty() {
                            default.clone()
                        } else {
                            union(codebase, types.clone())
                        });
                    }
                }
                (!returns.is_empty()).then(|| union(codebase, returns))
            }
        }
    }
}

/// `TypeCombinator::union()`.
fn union(codebase: &CodebaseMetadata, types: Vec<TUnion>) -> TUnion {
    let mut types = types.into_iter();
    let first = types.next().unwrap_or_else(get_never);
    types.fold(first, |t, u| {
        add_union_type(t, &u, codebase, CombinerOptions::default())
    })
}

/// ConsoleApplicationHelper::getArguments() or getOptions(): each command's own definition as an array shape.
fn shapes(codebase: &CodebaseMetadata, commands: &[&Command], options: bool) -> Option<TUnion> {
    let shapes: Vec<TUnion> = commands
        .iter()
        .map(|c| {
            let items: KnownItems = if options {
                c.own
                    .options
                    .iter()
                    .map(|o| {
                        (
                            ArrayKey::String(word(o.name.as_bytes())),
                            (false, option_type(codebase, o)),
                        )
                    })
                    .collect()
            } else {
                c.own
                    .arguments
                    .iter()
                    .map(|a| {
                        (
                            ArrayKey::String(word(a.name.as_bytes())),
                            (false, argument_type(codebase, a)),
                        )
                    })
                    .collect()
            };
            let non_empty = !items.is_empty();
            TUnion::from_atomic(TAtomic::Array(TArray::Keyed(
                TKeyedArray::new()
                    .with_known_items(items)
                    .with_non_empty(non_empty),
            )))
        })
        .collect();
    (!shapes.is_empty()).then(|| union(codebase, shapes))
}

/// `array<int, $value>`.
fn int_array(value: TUnion) -> TUnion {
    get_keyed_array(get_int(), value)
}

/// ConsoleApplicationHelper::getArgumentType().
fn argument_type(codebase: &CodebaseMetadata, a: &InputArgument) -> TUnion {
    let Some(mode) = a.mode else {
        return get_mixed();
    };
    let required = mode & InputArgument::REQUIRED != 0;
    if mode & InputArgument::IS_ARRAY != 0 {
        let t = int_array(get_string());
        if !required && a.default != Value::Array(Vec::new()) {
            return union(codebase, vec![t, value_type(&a.default)]);
        }
        t
    } else if !required {
        union(codebase, vec![get_string(), value_type(&a.default)])
    } else {
        get_string()
    }
}

/// ConsoleApplicationHelper::getOptionType().
fn option_type(codebase: &CodebaseMetadata, o: &InputOption) -> TUnion {
    let Some(mode) = o.mode else {
        return get_mixed();
    };
    if mode & (InputOption::VALUE_REQUIRED | InputOption::VALUE_OPTIONAL) == 0 {
        return if mode & InputOption::VALUE_NEGATABLE != 0 {
            union(codebase, vec![get_bool(), get_null()])
        } else {
            get_bool()
        };
    }
    let array = mode & InputOption::VALUE_IS_ARRAY != 0;
    let mut t = if mode & InputOption::VALUE_REQUIRED != 0 && (array || o.default != Value::Null) {
        get_string()
    } else {
        union(codebase, vec![get_string(), get_null()])
    };
    if array {
        t = int_array(t);
    }
    union(codebase, vec![t, value_type(&o.default)])
}

/// `$scope->getTypeFromValue()`.
fn value_type(v: &Value) -> TUnion {
    match v {
        Value::Null => get_null(),
        Value::Bool(true) => get_true(),
        Value::Bool(false) => get_false(),
        Value::Int(i) => get_literal_int(*i),
        Value::Float(f) => get_literal_float(*f),
        Value::String(s) => get_literal_string(word(s.as_bytes())),
        Value::Array(items) if items.is_empty() => get_empty_keyed_array(),
        Value::Array(items) => {
            let known: KnownItems = items
                .iter()
                .map(|(k, v)| {
                    let key = match k {
                        Key::Int(i) => ArrayKey::Integer(*i),
                        Key::String(s) => ArrayKey::String(word(s.as_bytes())),
                    };
                    (key, (false, value_type(v)))
                })
                .collect();
            TUnion::from_atomic(TAtomic::Array(TArray::Keyed(
                TKeyedArray::new()
                    .with_known_items(known)
                    .with_non_empty(true),
            )))
        }
        Value::Unknown => get_mixed(),
    }
}
