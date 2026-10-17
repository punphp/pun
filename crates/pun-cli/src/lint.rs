//! `pun lint --mago`: runs Mago 1.57's lint-only Laravel rules (no auto-fix in Mago) and reports, never edits.
//! Only built with `--features mago-lint`.
use anyhow::Result;
use mago_allocator::LocalArena;
use mago_database::file::File;
use mago_linter::Linter;
use mago_linter::integration::Integration;
use mago_linter::registry::RuleRegistry;
use mago_linter::settings::Settings;
use mago_names::resolver::NameResolver;
use mago_reporting::AnnotationKind;
use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

pub const RULES: &[&str] = &[
    "no-request-all",
    "prefer-casts-method",
    "prefer-anonymous-migration",
    "prefer-view-array",
    "middleware-in-routes",
];

pub fn run(paths: Vec<PathBuf>, config: Option<PathBuf>) -> Result<u8> {
    let root = super::root()?;
    let cfg = pun_config::load(
        &root,
        &pun_config::Options {
            config,
            ..Default::default()
        },
    )?;
    let files = pun_workspace::discover(&cfg, &paths)?;
    let mut settings = Settings::default();
    settings.integrations.insert(Integration::Laravel);
    let only: Vec<String> = RULES.iter().map(|r| r.to_string()).collect();
    let registry = Arc::new(RuleRegistry::build(&settings, Some(&only), true));
    let mut found = 0usize;
    for path in &files {
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(path)
            .to_string_lossy()
            .into_owned();
        if rel.ends_with(".blade.php") {
            continue;
        }
        let file = File::ephemeral(
            Cow::Owned(rel.clone().into_bytes()),
            Cow::Owned(std::fs::read(path)?),
        );
        let arena = LocalArena::new();
        let program = mago_syntax::parser::parse_file(&arena, &file);
        if program.has_errors() {
            continue; // `pun format` reports syntax errors
        }
        let names = NameResolver::new(&arena).resolve(program);
        let linter = Linter::from_registry(&arena, registry.clone(), settings.php_version);
        for issue in linter.lint(&file, program, &names).iter() {
            let span = issue
                .annotations
                .iter()
                .find(|n| n.kind == AnnotationKind::Primary)
                .or(issue.annotations.first())
                .map(|n| n.span);
            let (line, col) = span.map_or((1, 1), |s| {
                (
                    file.line_number(s.start.offset) + 1,
                    file.column_number(s.start.offset) + 1,
                )
            });
            println!(
                "{rel}:{line}:{col}: {} [{}] {}",
                issue.level,
                issue.code.as_deref().unwrap_or("mago"),
                issue.message
            );
            if let Some(help) = &issue.help {
                println!("    help: {help}");
            }
            found += 1;
        }
    }
    eprintln!("pun lint: {found} issue(s) in {} file(s)", files.len());
    Ok(u8::from(found > 0))
}
