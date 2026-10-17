//! Pun's side of the server: Pint formatting (the native pipeline `pun format --stdin` runs), diagnostics (PHP's
//! syntax errors, then the Mago lint rules of `pun lint --mago`) and the fix-all edit behind the code action.
use mago_allocator::LocalArena;
use mago_database::file::File;
use mago_linter::Linter;
use mago_linter::integration::Integration;
use mago_linter::registry::RuleRegistry;
use mago_linter::settings::Settings;
use mago_names::resolver::NameResolver;
use mago_reporting::{AnnotationKind, Level};
use pun_config::Config;
use pun_format::{LintError, Pipeline};
use std::borrow::Cow;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, TextEdit,
};

/// `pun lint --mago`'s rules (crates/pun-cli/src/lint.rs).
const MAGO_RULES: &[&str] = &[
    "no-request-all",
    "prefer-casts-method",
    "prefer-anonymous-migration",
    "prefer-view-array",
    "middleware-in-routes",
];

pub struct Pun {
    root: PathBuf,
    /// The project's pun.toml and its pipeline, or why formatting is unavailable (Pint's message for a
    /// broken config).
    format: Result<(Config, Pipeline), String>,
    lint: (Settings, Arc<RuleRegistry>),
}

impl Pun {
    pub fn load(root: PathBuf) -> Pun {
        let format = pun_config::load(&root, &pun_config::Options::default()).map_err(|e| e.to_string()).and_then(|cfg| {
            pun_fixers::analyzers::active_preset::set(&cfg.preset);
            let p = Pipeline::new(&cfg.rules, &pun_fixers::all).map_err(|e| e.to_string())?;
            // ponytail: no PHP-CS-Fixer delegation here; `pun lsp` gets it once it shares pun-cli's engine.
            match p.not_ported.is_empty() {
                true => Ok((cfg, p)),
                false => Err(format!("pun lsp formats natively only; `pun format` delegates this config to PHP-CS-Fixer (not ported: {})", p.not_ported.join(", "))),
            }
        });
        let mut settings = Settings::default();
        settings.integrations.insert(Integration::Laravel);
        let only: Vec<String> = MAGO_RULES.iter().map(|r| r.to_string()).collect();
        let registry = Arc::new(RuleRegistry::build(&settings, Some(&only), true));
        Pun {
            root,
            format,
            lint: (settings, registry),
        }
    }

    /// One whole-document edit, none when the code is clean, excluded or doesn't parse (Pint echoes it).
    pub fn format(&self, path: &Path, text: &str) -> Result<Vec<TextEdit>, String> {
        let (cfg, p) = self.format.as_ref().map_err(Clone::clone)?;
        if pun_workspace::is_path_excluded(cfg, path) {
            return Ok(vec![]);
        }
        let code = text.as_bytes();
        // Fixers panic where upstream throws; Pint reports that file as an error and leaves it.
        let run = catch_unwind(AssertUnwindSafe(|| p.run(path, code))).unwrap_or(Ok(None));
        // Runner::fixFile analyses annotations after linting the input, before any fixer runs.
        if !matches!(
            run,
            Err(LintError {
                after_fix: false,
                ..
            })
        ) {
            p.check_annotations(path, code).map_err(|e| e.to_string())?;
        }
        Ok(match run {
            Ok(Some(fixed)) => vec![TextEdit {
                range: Range::new(Position::default(), position(text, text.len())),
                new_text: String::from_utf8_lossy(&fixed.code).into_owned(),
            }],
            _ => vec![],
        })
    }

    pub fn diagnostics(&self, path: &Path, text: &str) -> Vec<Diagnostic> {
        let rel = path
            .strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .into_owned();
        if let Err(e) = pun_format::lint(text.as_bytes()) {
            // The message ends "on line N." (TokenizerLintingResult); PHP gives no column.
            let line = e
                .message
                .rsplit_once(" on line ")
                .and_then(|(_, n)| n.trim_end_matches('.').parse::<usize>().ok())
                .unwrap_or(1)
                - 1;
            let start = text
                .split_inclusive('\n')
                .take(line)
                .map(str::len)
                .sum::<usize>();
            let end = start + text[start..].find('\n').unwrap_or(text.len() - start);
            return vec![diagnostic(
                text,
                start,
                end,
                DiagnosticSeverity::ERROR,
                "syntax",
                e.message,
            )];
        }
        let (settings, registry) = &self.lint;
        let file = File::ephemeral(
            Cow::Owned(rel.into_bytes()),
            Cow::Owned(text.as_bytes().to_vec()),
        );
        let arena = LocalArena::new();
        let program = mago_syntax::parser::parse_file(&arena, &file);
        let names = NameResolver::new(&arena).resolve(program);
        let linter = Linter::from_registry(&arena, registry.clone(), settings.php_version);
        let issues = linter.lint(&file, program, &names);
        issues
            .iter()
            .map(|i| {
                let span = i
                    .annotations
                    .iter()
                    .find(|a| a.kind == AnnotationKind::Primary)
                    .or(i.annotations.first())
                    .map(|a| a.span);
                let (start, end) =
                    span.map_or((0, 0), |s| (s.start.offset as usize, s.end.offset as usize));
                // Lint findings are advice, never shown as errors.
                let severity = match i.level {
                    Level::Error | Level::Warning => DiagnosticSeverity::WARNING,
                    Level::Note => DiagnosticSeverity::INFORMATION,
                    Level::Help => DiagnosticSeverity::HINT,
                };
                diagnostic(
                    text,
                    start,
                    end,
                    severity,
                    i.code.as_deref().unwrap_or("mago"),
                    i.message.to_string(),
                )
            })
            .collect()
    }
}

fn diagnostic(
    text: &str,
    start: usize,
    end: usize,
    severity: DiagnosticSeverity,
    code: &str,
    message: String,
) -> Diagnostic {
    Diagnostic {
        range: Range::new(position(text, start), position(text, end)),
        severity: Some(severity),
        code: Some(NumberOrString::String(code.to_owned())),
        source: Some("pun".to_owned()),
        message,
        ..Default::default()
    }
}

/// The LSP position of byte `offset`: lines, then UTF-16 units (the protocol's default encoding).
fn position(text: &str, offset: usize) -> Position {
    let before = &text[..offset];
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    Position::new(
        before.matches('\n').count() as u32,
        before[line_start..].encode_utf16().count() as u32,
    )
}
