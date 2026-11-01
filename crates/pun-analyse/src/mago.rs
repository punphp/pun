//! Mago's own analyser over a project (analyse.md Build 0, the engine spike): the issues it reports, by code, and the
//! types it infers for the arguments of `\PHPStan\dumpType()`, optionally with Pun's Laravel plugin.

use mago_allocator::LocalArena;
use mago_analyzer::Analyzer;
use mago_analyzer::analysis_result::AnalysisResult;
use mago_analyzer::plugin::create_registry_with_plugins;
use mago_analyzer::settings::Settings;
use mago_codex::reference::SymbolReferences;
use mago_codex::ttype::TType;
use mago_database::file::{File, FileType};
use mago_names::ResolvedNames;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use crate::codebase::{self, CouldNotReadFile, read};
use crate::laravel::{self, Laravel};
use crate::{Project, display};

/// One issue as Mago reports it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MagoIssue {
    pub file: String,
    pub line: u32,
    pub code: String,
    pub message: String,
}

/// The type Mago infers for the argument of a `\PHPStan\dumpType()` call, in Mago's own notation.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dumped {
    pub file: String,
    pub line: u32,
    pub type_id: String,
}

#[derive(Debug, Default)]
pub struct MagoRun {
    pub issues: Vec<MagoIssue>,
    pub dumped: Vec<Dumped>,
}

/// Runs Mago's analyser over `project.analysed` with Mago's default plugins, plus Pun's Laravel plugin for the given
/// Laravel app (its migrations and models).
pub fn run_mago(project: &Project, laravel: Option<Laravel>) -> Result<MagoRun, CouldNotReadFile> {
    let (codebase, _) = codebase::build(project, laravel.as_ref().map(|app| app.root))?;
    let version = codebase::php_version(project.php_version);
    let mut registry = create_registry_with_plugins(&[], false);
    if let Some(app) = laravel {
        laravel::register(&mut registry, app, project, &codebase);
    }
    let mut out = MagoRun::default();
    for path in &project.analysed {
        let Some(file) = read(path, FileType::Host)? else {
            continue;
        };
        let arena = LocalArena::new();
        let (program, names) = crate::codebase::parse(&arena, &file);
        let analyzer = Analyzer::new(
            &arena,
            &file,
            &names,
            &codebase,
            &registry,
            Settings::new(version),
        );
        let mut result = AnalysisResult::new(SymbolReferences::new());
        // ponytail: Mago's analyzer trips debug assertions on some code (as in rules::infer); the file is skipped
        let analysed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            analyzer.analyze_with_artifacts(program, &mut result)
        }));
        let Ok(Ok(artifacts)) = analysed else {
            continue;
        };
        let name = display(path);
        for issue in result.issues {
            // an issue without a primary span (none of Mago's issues so far) has no line, as in PHPStan's JSON
            let primary = issue.annotations.iter().find(|a| a.kind.is_primary());
            out.issues.push(MagoIssue {
                file: name.clone(),
                line: primary.map_or(0, |a| line(&file, a.span)),
                code: issue.code.unwrap_or_default(),
                message: issue.message.into_owned(),
            });
        }
        let mut dumps = DumpTypeCalls {
            names: &names,
            args: Vec::new(),
        };
        dumps.walk_program(program, &mut ());
        for span in dumps.args {
            let type_id = artifacts
                .get_expression_type(&span)
                .map_or_else(|| "mixed".to_string(), |t| t.get_id().to_string());
            out.dumped.push(Dumped {
                file: name.clone(),
                line: line(&file, span),
                type_id,
            });
        }
    }
    out.issues.sort();
    out.dumped.sort();
    Ok(out)
}

fn line(file: &File, span: Span) -> u32 {
    file.line_number(span.start.offset) + 1
}

/// The first argument of every `\PHPStan\dumpType()` call.
struct DumpTypeCalls<'n, 'arena> {
    names: &'n ResolvedNames<'arena>,
    args: Vec<Span>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for DumpTypeCalls<'_, 'arena> {
    fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
        if let Expression::Identifier(id) = call.function
            && self
                .names
                .get(&id.span())
                .eq_ignore_ascii_case(b"PHPStan\\dumpType")
            && let Some(Argument::Positional(a)) = call.argument_list.arguments.first()
        {
            self.args.push(a.value.span());
        }
    }
}
