//! `pun analyse` (Phase 2, M2.3): PHPStan 2.3.1's rules, level by level, on Mago's parser, name resolution and
//! codex. Errors carry PHPStan's identifier and message so baselines and `ignoreErrors` keep matching.

mod autoload;
mod codebase;
mod larastan;
mod laravel;
mod mago;
mod parser;
mod project;
mod relative_path;
mod rules;

use std::path::{Path, PathBuf};

use pun_phpstan::config::Resolved;
use rayon::prelude::*;

pub use codebase::CouldNotReadFile;
pub use laravel::Laravel;
pub use mago::{Dumped, MagoIssue, MagoRun, run_mago};
pub use project::{Exceptions, MethodRules, Project, Reported, StrictRules};
pub use relative_path::RelativePath;
pub use rules::Larastan;

/// One reported error, as `phpstan analyse --error-format=json` prints it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    pub file: String,
    /// PHPStan's `-1` for a node without a position (a synthetic `break`).
    pub line: i64,
    pub identifier: String,
    pub message: String,
    pub tip: Option<String>,
    /// Whether ignore comments, configured ignores and baselines may suppress this error.
    pub can_be_ignored: bool,
}

/// Identifiers Pun reports. Errors with other identifiers are PHPStan's alone for now.
pub const IDENTIFIERS: &[&str] = rules::IDENTIFIERS;

/// Whether Pun reports `identifier` at some level: [`IDENTIFIERS`], the level-6 missingType rules, the level-2
/// generics rules and Larastan's.
pub fn reports(identifier: &str) -> bool {
    rules::reports(identifier)
}

/// Analyses the project the resolved `phpstan.neon` describes.
pub fn analyse(project_root: &Path, config: &Resolved) -> Result<Vec<Error>, CouldNotReadFile> {
    run(&Project::from_config(project_root, &config.parameters))
}

/// Analyses `project.analysed`, knowing the symbols of `project.scanned` too, then drops what
/// `@phpstan-ignore*` comments and `ignoreErrors` ignore, as `phpstan analyse` reports it. A file that exists but
/// can't be read stops the run, as it stops PHPStan's.
pub fn run(project: &Project) -> Result<Vec<Error>, CouldNotReadFile> {
    Ok(analyse_project(project)?.errors)
}

/// What [`analyse_project`] reports.
pub struct Analysis {
    /// The file-specific errors, sorted.
    pub errors: Vec<Error>,
    /// PHPStan's not-file-specific errors: `ignoreErrors` entries without a file path nothing matched.
    pub general: Vec<String>,
}

/// [`run`], with PHPStan's not-file-specific errors.
pub fn analyse_project(project: &Project) -> Result<Analysis, CouldNotReadFile> {
    let (codebase, sources) = codebase::build(project, None)?;
    let files: std::collections::HashMap<_, _> = project
        .analysed
        .iter()
        .chain(&project.scanned)
        .chain(project.autoloaded.iter().map(|a| &a.path))
        .map(|p| {
            (
                mago_database::file::FileId::new(display(p).as_bytes()),
                p.clone(),
            )
        })
        .collect();
    let settings = rules::Checks::of(project, &codebase);
    let errors: Vec<_> = project
        .analysed
        .par_iter()
        .flat_map_iter(|f| rules::check_file(f, &codebase, &settings, &files, &sources))
        .chain(rules::unused_traits(project, &codebase, &sources))
        .collect();
    let errors = rules::renamed_named_arguments(
        errors,
        &sources,
        project.report_ignores_without_comments,
        project.method_rules.renamed_named_arguments,
    );
    // the codebase is large: freed off the critical path
    std::thread::spawn(move || drop((codebase, sources)));
    let analysed: Vec<String> = project.analysed.iter().map(|p| display(p)).collect();
    let errors = rules::conditions_in_traits(errors);
    let errors = rules::impure_points(errors);
    let mut errors = rules::larastan_finish(project.larastan.as_ref(), errors);
    settings.keep_reported(&mut errors);
    let processed = project
        .ignore_errors
        .process(errors, false, &analysed, false);
    let mut errors: Vec<Error> = processed
        .errors
        .into_iter()
        .map(|e| Error {
            file: e.file,
            line: e.line.unwrap_or(0),
            identifier: e.identifier.unwrap_or_default(),
            message: e.message,
            tip: e.tip,
            can_be_ignored: e.can_be_ignored,
        })
        .collect();
    // PHPStan's AnalysisResult orders diagnostics by file, line and message, not identifier.
    errors.sort_by(|a, b| (&a.file, a.line, &a.message).cmp(&(&b.file, b.line, &b.message)));
    Ok(Analysis {
        errors,
        general: processed.string_errors,
    })
}

fn display(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

/// `b` as text, invalid UTF-8 replaced.
pub(crate) fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

/// Files below `dir` with one of `extensions`, sorted. Subdirectories are walked in parallel.
fn php_files(dir: &Path, extensions: &[String]) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    // the entry's own type (readdir's d_type) saves a stat per entry; a symlink is followed as before
    let mut entries: Vec<_> = entries
        .flatten()
        .map(|e| {
            let p = e.path();
            let dir = match e.file_type() {
                Ok(t) if !t.is_symlink() => t.is_dir(),
                _ => p.is_dir(),
            };
            (p, dir)
        })
        .collect();
    entries.sort();
    entries
        .into_par_iter()
        .flat_map_iter(|(p, dir)| {
            if dir {
                php_files(&p, extensions)
            } else if p
                .extension()
                .is_some_and(|e| extensions.iter().any(|x| e == x.as_str()))
            {
                vec![p]
            } else {
                Vec::new()
            }
        })
        .collect()
}
