//! PHPStan's PrintfParametersRule, PrintfArrayParametersRule and FilterVarRule.

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
use mago_codex::ttype::union::TUnion;
use mago_names::ResolvedNames;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::{Checker, printf_helper};

/// Resolve the actual function before matching: a namespaced `sprintf` may shadow the built-in.
pub(super) fn function_name<'arena>(
    names: &ResolvedNames<'arena>,
    codebase: &CodebaseMetadata,
    id: &Identifier<'arena>,
) -> String {
    let resolved = names.get(&id.span());
    if codebase.get_function(resolved).is_some() {
        return String::from_utf8_lossy(resolved).to_ascii_lowercase();
    }
    match id {
        Identifier::Local(l) if !names.is_imported(&l.span) => {
            String::from_utf8_lossy(l.value).to_ascii_lowercase()
        }
        _ => String::from_utf8_lossy(resolved).to_ascii_lowercase(),
    }
}

/// Only these arguments need Mago's inference, through the existing per-file inference run.
pub(super) fn typed_expressions<'arena>(
    names: &ResolvedNames<'arena>,
    codebase: &CodebaseMetadata,
    program: &Program<'arena>,
) -> Vec<Span> {
    struct Wanted<'n, 'arena> {
        names: &'n ResolvedNames<'arena>,
        codebase: &'n CodebaseMetadata,
        spans: Vec<Span>,
    }
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted<'_, 'arena> {
        fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
            let Expression::Identifier(id) = call.function else {
                return;
            };
            let args = &call.argument_list.arguments;
            let name = function_name(self.names, self.codebase, id);
            let positions: &[usize] = match name.as_str() {
                "printf" | "sprintf" => &[0],
                "sscanf" | "fscanf" => &[1],
                "vprintf" | "vsprintf" => &[0, 1],
                "filter_var" | "filter_input" | "filter_var_array" | "filter_input_array" => &[0, 1, 2, 3],
                _ => &[],
            };
            self.spans.extend(
                positions
                    .iter()
                    .filter_map(|&i| args.get(i).map(|a| a.value().span())),
            );
        }
        fn walk_in_pipe(&mut self, pipe: &'ast Pipe<'arena>, _: &mut ()) {
            if let Some(id) = piped_function(pipe)
                && matches!(
                    function_name(self.names, self.codebase, id).as_str(),
                    "printf" | "sprintf" | "vprintf" | "vsprintf"
                )
            {
                self.spans.push(pipe.input.span());
            }
        }
    }
    let mut wanted = Wanted {
        names,
        codebase,
        spans: Vec::new(),
    };
    wanted.walk_program(program, &mut ());
    wanted.spans
}

pub(super) fn function_call<'arena>(c: &mut Checker<'_, 'arena>, call: &FunctionCall<'arena>) {
    let Expression::Identifier(id) = call.function else {
        return;
    };
    let name = function_name(c.names, c.codebase, id);
    match name.as_str() {
        "printf" | "sprintf" | "sscanf" | "fscanf" => printf(c, call, &name),
        "vprintf" | "vsprintf" => {
            let args = &call.argument_list.arguments;
            if let Some(first) = args.first() {
                vprintf(
                    c,
                    call,
                    first.value(),
                    args.get(1).map(Argument::value),
                    &name,
                );
            }
        }
        "filter_var" => filter_var(c, call, 2, false),
        "filter_input" => filter_var(c, call, 3, false),
        "filter_var_array" | "filter_input_array" => filter_var(c, call, 1, true),
        _ => {}
    }
}

/// FilterVarRule: flags that hold both FILTER_NULL_ON_FAILURE and FILTER_THROW_ON_FAILURE (PHP 8.5's), in the $options
/// argument at `position` (FilterFunctionFlagsHelper::getFlagsTypes()); the array variants take a filter specification
/// per input key, and an integer there is the filter id, without flags.
fn filter_var(c: &mut Checker, call: &FunctionCall, position: usize, per_key: bool) {
    let Some(options) = super::types::regex::argument(call, position, "options") else {
        return;
    };
    if c.php_version < 80500 {
        return;
    }
    let constant = |name: &str| {
        c.builtins.constants.get(name).and_then(|v| v.parse::<i64>().ok()).expect("a PHP 8.5 filter constant")
    };
    let (null, throw) = (constant("FILTER_NULL_ON_FAILURE"), constant("FILTER_THROW_ON_FAILURE"));
    let flags: Vec<&Expression> = match options {
        Expression::Array(Array { elements, .. }) if per_key => elements.iter().filter_map(ArrayElement::get_value).collect(),
        _ if per_key && c.type_of(options).is_some_and(|t| !t.types.iter().any(|t| matches!(t, TAtomic::Array(_) | TAtomic::Mixed(_)))) => vec![],
        _ => vec![options],
    };
    for flags in flags {
        // hasFlag(): yes when every constant value has the flag, maybe without constant values
        if let Some(values) = flag_values(c, flags)
            && values.iter().all(|v| v & null == null && v & throw == throw)
        {
            c.report(
                call,
                "filterVar.nullOnFailureAndThrowOnFailure",
                "Cannot use both FILTER_NULL_ON_FAILURE and FILTER_THROW_ON_FAILURE.".to_string(),
                None,
            );
            return;
        }
    }
}

/// The constant integer values of `getFlagsValue()` of a flags argument: inferred integers,
/// or an array's `flags` item (0 without one). The codebase supplies missing integer constants before inference,
/// so bitwise expressions use the inferred result rather than a separate syntax evaluator.
fn flag_values(c: &Checker, e: &Expression) -> Option<Vec<i64>> {
    let inferred = c.type_of(e).and_then(|t| {
        t.types
            .iter()
            .map(|t| match t {
                TAtomic::Scalar(TScalar::Integer(i)) => i.get_literal_value(),
                _ => None,
            })
            .collect::<Option<Vec<i64>>>()
    });
    if inferred.is_some() {
        return inferred;
    }
    match e {
        Expression::Array(a) => {
            let flags = a.elements.iter().find_map(|e| match e {
                ArrayElement::KeyValue(kv)
                    if matches!(kv.key, Expression::Literal(Literal::String(s)) if s.value == Some(b"flags".as_slice())) =>
                {
                    Some(kv.value)
                }
                _ => None,
            });
            flags.map_or(Some(vec![0]), |f| flag_values(c, f))
        }
        _ => None,
    }
}

pub(super) fn piped_function<'ast, 'arena>(
    pipe: &'ast Pipe<'arena>,
) -> Option<&'ast Identifier<'arena>> {
    if let Expression::PartialApplication(PartialApplication::Function(f)) = pipe.callable
        && let Expression::Identifier(id) = f.function
        && f.argument_list.arguments.len() == 1
        && let Some(PartialArgument::VariadicPlaceholder(_)) = f.argument_list.arguments.first()
    {
        Some(id)
    } else {
        None
    }
}

/// PipeHandler synthesizes a call with one argument, at the right-hand callable's source location.
pub(super) fn pipe<'arena>(c: &mut Checker<'_, 'arena>, pipe: &Pipe<'arena>) {
    let Some(id) = piped_function(pipe) else {
        return;
    };
    let name = function_name(c.names, c.codebase, id);
    match name.as_str() {
        "printf" | "sprintf" => check_printf(c, pipe.callable, pipe.input, &name, 0),
        "vprintf" | "vsprintf" => vprintf(c, pipe.callable, pipe.input, None, &name),
        _ => {}
    }
}

/// PHPStan's `getConstantStrings()`: general strings and other atomics contribute no constant strings.
fn strings(ty: &TUnion) -> impl Iterator<Item = &[u8]> {
    ty.types.iter().filter_map(|t| match t {
        TAtomic::Scalar(TScalar::String(s)) => s.get_known_literal_value(),
        TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { value })) => {
            Some(value.as_bytes())
        }
        _ => None,
    })
}

fn printf(c: &mut Checker, call: &FunctionCall, name: &str) {
    let args = &call.argument_list.arguments;
    let scanf = matches!(name, "sscanf" | "fscanf");
    let position = usize::from(scanf);
    if args.len() < if scanf { 3 } else { 1 }
        || args
            .iter()
            .any(|a| matches!(a, Argument::Positional(p) if p.ellipsis.is_some()))
    {
        return;
    }
    check_printf(
        c,
        call,
        args[position].value(),
        name,
        args.len() - position - 1,
    );
}

fn check_printf(c: &mut Checker, at: impl HasSpan, format: &Expression, name: &str, values: usize) {
    let scanf = matches!(name, "sscanf" | "fscanf");
    let Some(format) = c.type_of(format) else {
        return;
    };
    let counts: Option<Vec<_>> = strings(format)
        .map(|format| {
            if scanf {
                printf_helper::scanf_count(format)
            } else {
                printf_helper::printf_count(format, c.surely_at_least(80000))
            }
        })
        .collect();
    let Some(counts) = counts else {
        invalid_placeholder(c, at, name);
        return;
    };
    if let Some(count) = counts.into_iter().max()
        && count != values
    {
        mismatch(c, at, name, (count, count), (values, values));
    }
}

/// PHPStan's integer range union: disjoint constant sizes stay a union, not their hull.
fn size_union(mut sizes: Vec<(usize, usize)>) -> Option<(usize, usize)> {
    sizes.sort_unstable();
    let &(minimum, mut maximum) = sizes.first()?;
    for (low, high) in sizes.into_iter().skip(1) {
        if low > maximum.saturating_add(1) {
            return None;
        }
        maximum = maximum.max(high);
    }
    Some((minimum, maximum))
}

/// `getConstantArrays()` and `getArraySize()` (unknown arrays cause PrintfArrayParametersRule to skip).
fn array_size(ty: &TUnion) -> Option<(usize, usize)> {
    let sizes: Vec<_> = ty
        .types
        .iter()
        .filter_map(|t| {
            let TAtomic::Array(a) = t else { return None };
            match a {
                TArray::List(l) => {
                    if let Some(n) = l.known_count {
                        Some((n, n))
                    } else if l.element_type.is_never() {
                        Some((
                            a.get_minimum_size(),
                            l.known_elements.as_ref().map_or(0, |i| i.len()),
                        ))
                    } else {
                        None
                    }
                }
                TArray::Keyed(k) if k.parameters.is_none() => Some((
                    a.get_minimum_size(),
                    k.known_items.as_ref().map_or(0, |i| i.len()),
                )),
                _ => None,
            }
        })
        .collect();
    size_union(sizes)
}

fn vprintf(
    c: &mut Checker,
    at: impl HasSpan,
    format: &Expression,
    values: Option<&Expression>,
    name: &str,
) {
    let Some(ty) = c.type_of(format) else {
        return;
    };
    let counts: Option<Vec<_>> = strings(ty)
        .map(|format| printf_helper::printf_count(format, c.surely_at_least(80000)))
        .collect();
    let Some(counts) = counts else {
        invalid_placeholder(c, at, name);
        return;
    };
    let Some(&low) = counts.iter().min() else {
        return;
    };
    let high = *counts.iter().max().unwrap();
    let (min_args, max_args) = match values {
        None => (0, 0),
        Some(a) => match c.type_of(a).and_then(array_size) {
            Some(size) => size,
            None => return,
        },
    };
    let matches = if low == high {
        if min_args == max_args {
            low == min_args
        } else {
            low == 0
        }
    } else {
        min_args != max_args && low >= 1 && low <= min_args && high >= max_args
    };
    if !matches {
        mismatch(c, at, name, (low, high), (min_args, max_args));
    }
}

fn invalid_placeholder(c: &mut Checker, at: impl HasSpan, name: &str) {
    c.report(
        at,
        &format!("argument.{name}"),
        format!("Call to {name} contains an invalid placeholder."),
        None,
    );
}

fn mismatch(
    c: &mut Checker,
    at: impl HasSpan,
    name: &str,
    placeholders: (usize, usize),
    values: (usize, usize),
) {
    let describe = |(low, high): (usize, usize), singular, plural| {
        if low != high {
            format!("{low}-{high} {plural}")
        } else {
            format!("{low} {}", if low == 1 { singular } else { plural })
        }
    };
    c.report(
        at,
        &format!("argument.{name}"),
        format!(
            "Call to {name} contains {}, {} given.",
            describe(placeholders, "placeholder", "placeholders"),
            describe(values, "value", "values"),
        ),
        None,
    );
}
