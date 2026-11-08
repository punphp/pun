//! PHPStan's DumpTypeRule and DumpPhpDocTypeRule: the same inferred type, with precise or PHPDoc serialization.
//! FileAssertRule: `assertType()` and its siblings, which hold or are reported.

use mago_codex::metadata::CodebaseMetadata;
use mago_names::ResolvedNames;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Level, Tri, is_super_type_of, to_phpdoc_fully_qualified};

use super::{Checker, declared, function_checks, unset};

enum Dump {
    Type,
    PhpDoc,
}

fn dump<'arena>(
    names: &ResolvedNames<'arena>,
    codebase: &CodebaseMetadata,
    function: &Expression<'arena>,
) -> Option<Dump> {
    let Expression::Identifier(id) = function else {
        return None;
    };
    match function_checks::function_name(names, codebase, id).as_str() {
        "phpstan\\dumptype" => Some(Dump::Type),
        "phpstan\\dumpphpdoctype" => Some(Dump::PhpDoc),
        _ => None,
    }
}

enum Assert {
    Type,
    NativeType,
    SuperType,
    VariableCertainty,
}

/// FileAssertRule::processNode()'s function: one of PHPStan's testing functions, called with two arguments.
fn assertion<'arena>(
    names: &ResolvedNames<'arena>,
    codebase: &CodebaseMetadata,
    call: &FunctionCall<'arena>,
) -> Option<Assert> {
    let Expression::Identifier(id) = call.function else {
        return None;
    };
    if call.argument_list.arguments.len() != 2 {
        return None;
    }
    match function_checks::function_name(names, codebase, id).as_str() {
        "phpstan\\testing\\asserttype" => Some(Assert::Type),
        "phpstan\\testing\\assertnativetype" => Some(Assert::NativeType),
        "phpstan\\testing\\assertsupertype" => Some(Assert::SuperType),
        "phpstan\\testing\\assertvariablecertainty" => Some(Assert::VariableCertainty),
        _ => None,
    }
}

/// A call of `PHPStan\Testing\assertVariableCertainty()`, by its resolved name: its scope is observed before the
/// arguments, as compact()'s is.
pub(super) fn is_variable_certainty(call: &FunctionCall, names: &ResolvedNames) -> bool {
    let Expression::Identifier(id) = call.function else {
        return false;
    };
    names
        .get(&id.span())
        .eq_ignore_ascii_case(b"PHPStan\\Testing\\assertVariableCertainty")
}

/// Every syntactic argument, including named and unpacked arguments, uses the shared per-file inference run, and so
/// do an asserted offset's array and dimension.
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
        fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, (): &mut ()) {
            let assert = assertion(self.names, self.codebase, call).is_some();
            if assert || dump(self.names, self.codebase, call.function).is_some() {
                self.spans.extend(
                    call.argument_list
                        .arguments
                        .iter()
                        .map(|a| a.value().span()),
                );
            }
            if assert
                && let Some(Expression::ArrayAccess(a)) =
                    call.argument_list.arguments.get(1).map(|a| a.value())
            {
                self.spans.extend([a.array.span(), a.index.span()]);
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
    if let Some(assert) = assertion(c.names, c.codebase, call) {
        file_assert(c, call, assert);
    }
    let Some(dump) = dump(c.names, c.codebase, call.function) else {
        return;
    };
    let args = &call.argument_list.arguments;
    for (i, arg) in args.iter().enumerate() {
        if let Some(ty) = c.phpstan_type(arg.value()) {
            let (identifier, ty) = declared::with_classes(c, || match dump {
                Dump::Type => ("phpstan.dumpType", ty.describe(Level::Precise)),
                Dump::PhpDoc => ("phpstan.dumpPhpDocType", to_phpdoc_fully_qualified(&ty)),
            });
            let message = if args.len() > 1 {
                format!("Dumped type #{}: {ty}", i + 1)
            } else {
                format!("Dumped type: {ty}")
            };
            c.report_non_ignorable(call, identifier, message, None);
        }
    }
}

/// FileAssertRule::processAssert*(): an assertion that does not hold, or whose expectation is not a literal.
fn file_assert<'arena>(c: &mut Checker<'_, 'arena>, call: &FunctionCall<'arena>, assert: Assert) {
    let args = &call.argument_list.arguments;
    let (expected, value) = (args[0].value(), args[1].value());
    let literal = |c: &Checker<'_, 'arena>, native: bool| {
        let ty = if native {
            c.phpstan_type_from_in(
                expected,
                c.native_types.get(&expected.span())?,
                &c.native_types,
            )
        } else {
            c.phpstan_type(expected)?
        };
        match ty.constant_strings().as_slice() {
            [one] => Some(Ok(String::from_utf8_lossy(one).into_owned())),
            _ => Some(Err(())),
        }
    };
    let found = match assert {
        Assert::Type | Assert::SuperType => {
            let Some(expected) = literal(c, false) else {
                return;
            };
            let Ok(expected) = expected else {
                let what = if matches!(assert, Assert::Type) {
                    "type"
                } else {
                    "super type"
                };
                return unknown(
                    c,
                    call,
                    format!("Expected {what} must be a literal string."),
                );
            };
            let Some(actual) = c.phpstan_type(value) else {
                return;
            };
            declared::with_classes(c, || {
                let described = actual.describe(Level::Precise);
                if matches!(assert, Assert::Type) {
                    (expected != described).then(|| {
                        (
                            "phpstan.type",
                            format!("Expected type {expected}, actual: {described}"),
                        )
                    })
                } else {
                    let resolved = pun_phpstan::resolve::resolve_str(&expected, None);
                    (!is_super_type_of(&resolved, &actual).yes()).then(|| {
                        (
                            "phpstan.superType",
                            format!("Expected subtype of {expected}, actual: {described}"),
                        )
                    })
                }
            })
        }
        Assert::NativeType => {
            let Some(expected) = literal(c, true) else {
                return;
            };
            let Ok(expected) = expected else {
                return unknown(
                    c,
                    call,
                    "Expected native type must be a literal string.".into(),
                );
            };
            let Some(actual) = c.native_types.get(&value.span()) else {
                return;
            };
            let actual = c.phpstan_type_from_in(value, actual, &c.native_types);
            let described = declared::with_classes(c, || actual.describe(Level::Precise));
            (expected != described).then(|| {
                (
                    "phpstan.nativeType",
                    format!("Expected native type {expected}, actual: {described}"),
                )
            })
        }
        Assert::VariableCertainty => match variable_certainty(c, call, expected, value) {
            Ok(found) => found,
            Err(message) => return unknown(c, call, message.into()),
        },
    };
    if let Some((identifier, message)) = found {
        c.report_non_ignorable(call, identifier, message, None);
    }
}

fn unknown(c: &mut Checker, call: &FunctionCall, message: String) {
    c.report_non_ignorable(call, "phpstan.unknownExpectation", message, None);
}

/// FileAssertRule::processAssertVariableCertainty(): `Ok(None)` when it holds or Pun has no observation.
fn variable_certainty<'arena>(
    c: &Checker<'_, 'arena>,
    call: &FunctionCall<'arena>,
    certainty: &Expression<'arena>,
    variable: &Expression<'arena>,
) -> Result<Option<(&'static str, String)>, &'static str> {
    let Expression::Call(Call::StaticMethod(certainty)) = certainty else {
        // sic: PHPStan does not fill in the function name
        return Err("First argument of %s() must be TrinaryLogic call");
    };
    let Expression::Identifier(class) = certainty.class else {
        return Err("Invalid TrinaryLogic call.");
    };
    if c.names.get(&class.span()) != b"PHPStan\\TrinaryLogic" {
        return Err("Invalid TrinaryLogic call.");
    }
    let ClassLikeMemberSelector::Identifier(method) = &certainty.method else {
        return Err("Invalid TrinaryLogic call.");
    };
    // any other method is not a TrinaryLogic factory: PHPStan's own call fails
    let expected = match method.value.to_ascii_lowercase().as_slice() {
        b"createyes" => Tri::Yes,
        b"createmaybe" => Tri::Maybe,
        b"createno" => Tri::No,
        _ => return Ok(None),
    };
    let (actual, description) = match variable {
        Expression::Variable(Variable::Direct(v)) => {
            let name = String::from_utf8_lossy(&v.name[1..]).into_owned();
            let Some(scope) = c.variables.compact_scopes.get(&call.span()) else {
                return Ok(None);
            };
            (scope.get(&name), format!("variable ${name}"))
        }
        Expression::ArrayAccess(a) => {
            let (Some(array), Some(offset)) = (c.phpstan_type(a.array), c.phpstan_type(a.index))
            else {
                return Ok(None);
            };
            declared::with_classes(c, || {
                (
                    unset::has_offset(c, &array, &offset),
                    format!("offset {}", offset.describe(Level::Precise)),
                )
            })
        }
        _ => return Err("Invalid assertVariableCertainty call."),
    };
    let describe = |t: Tri| match t {
        Tri::Yes => "Yes",
        Tri::Maybe => "Maybe",
        Tri::No => "No",
    };
    Ok((expected != actual).then(|| {
        (
            "phpstan.variable",
            format!(
                "Expected {description} certainty {}, actual: {}",
                describe(expected),
                describe(actual)
            ),
        )
    }))
}
