//! PHPStan's DumpTypeRule and DumpPhpDocTypeRule: the same inferred type, with precise or PHPDoc serialization.

use mago_codex::metadata::CodebaseMetadata;
use mago_names::ResolvedNames;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{Level, to_phpdoc};

use super::{Checker, declared, function_checks};

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

/// Every syntactic argument, including named and unpacked arguments, uses the shared per-file inference run.
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
            if dump(self.names, self.codebase, call.function).is_some() {
                self.spans.extend(
                    call.argument_list
                        .arguments
                        .iter()
                        .map(|a| a.value().span()),
                );
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
    let Some(dump) = dump(c.names, c.codebase, call.function) else {
        return;
    };
    let args = &call.argument_list.arguments;
    for (i, arg) in args.iter().enumerate() {
        if let Some(ty) = c.phpstan_type(arg.value()) {
            let (identifier, ty) = declared::with_classes(c, || match dump {
                Dump::Type => ("phpstan.dumpType", ty.describe(Level::Precise)),
                Dump::PhpDoc => ("phpstan.dumpPhpDocType", to_phpdoc(&ty)),
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
