//! UndefinedArgumentOrOptionRule: `$this->argument('x')` and `$this->option('x')` in a command, against the
//! definitions of the registered commands that are that class or extend it ([`crate::larastan::console`]).

use mago_syntax::cst::*;

use super::super::Checker;
use super::{class_is, is_object_of};

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
