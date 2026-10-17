//! `pun analyse` (Phase 2, M2.3): PHPStan 2.2.7's rules, level by level, on Mago's parser, name resolution and
//! codex. Errors carry PHPStan's identifier and message so baselines and `ignoreErrors` keep matching.

mod autoload;
mod codebase;
mod laravel;
mod mago;
mod project;
mod rules;

use std::path::{Path, PathBuf};

use pun_phpstan::config::Resolved;
use rayon::prelude::*;

pub use laravel::Laravel;
pub use mago::{Dumped, MagoIssue, MagoRun, run_mago};
pub use project::Project;

/// One reported error, as `phpstan analyse --error-format=json` prints it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error {
    pub file: String,
    pub line: u32,
    pub identifier: String,
    pub message: String,
    pub tip: Option<String>,
}

/// Identifiers Pun reports. Errors with other identifiers are PHPStan's alone for now.
pub const IDENTIFIERS: &[&str] = rules::IDENTIFIERS;

/// Analyses the project the resolved `phpstan.neon` describes.
pub fn analyse(project_root: &Path, config: &Resolved) -> Vec<Error> {
    run(&Project::from_config(project_root, &config.parameters))
}

/// Analyses `project.analysed`, knowing the symbols of `project.scanned` too, then drops what
/// `@phpstan-ignore*` comments and `ignoreErrors` ignore, as `phpstan analyse` reports it.
pub fn run(project: &Project) -> Vec<Error> {
    let (codebase, sources) = codebase::build_with_sources(project);
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
    let settings = rules::Checks::of(project);
    let errors: Vec<_> = project
        .analysed
        .par_iter()
        .flat_map_iter(|f| rules::check_file(f, &codebase, &settings, &files, &sources))
        .collect();
    // the codebase is large: freed off the critical path
    std::thread::spawn(move || drop((codebase, sources)));
    let analysed: Vec<String> = project.analysed.iter().map(|p| display(p)).collect();
    let processed = project
        .ignore_errors
        .process(errors, false, &analysed, false);
    let mut errors: Vec<Error> = processed
        .errors
        .into_iter()
        .map(|e| Error {
            file: e.file,
            line: e.line.map_or(0, |l| l as u32),
            identifier: e.identifier.unwrap_or_default(),
            message: e.message,
            tip: e.tip,
        })
        .collect();
    errors.sort();
    errors
}

fn display(p: &Path) -> String {
    p.to_string_lossy().into_owned()
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
