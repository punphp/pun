//! Pun's lint engine (lint.md §6, Build 1): rules on Mago's syntax tree that report problems and fix them with text
//! edits.
//!
//! A [`Rule`] is called once for every node of the tree (or of the kinds it targets), parents before children, and
//! returns [`Report`]s: a byte span, a message, an optional help line and an optional [`Fix`] made of text edits on
//! byte ranges. [`Linter::check`] parses a file once and runs every rule over that one tree. [`Linter::fix`] applies
//! the fixes, with [`pun_problem::apply_fixes`], in rounds: a fix that overlaps one applied before it waits for the
//! next round, and the rounds stop when no rule has a fix left. There is no printer: an edit inserts the text the rule
//! wrote, and the format pass that runs after lint (`pun check --fix`) tidies the layout.

use std::borrow::Cow;
use std::fmt;
use std::path::{Path, PathBuf};

use mago_allocator::LocalArena;
use mago_database::file::{File, FileId, FileType};
use mago_span::HasSpan;
use mago_syntax::cst::{Node, NodeKind};
use mago_syntax::settings::ParserSettings;
use mago_syntax::walker::Walker;
use pun_problem::{Fix, Level, Problem, Span, apply_fixes};

pub mod rules;
pub mod syntax;
mod walk;

/// What a rule sees besides the node: the file's source, its path and the PHP version the project targets.
pub struct Context<'a> {
    pub source: &'a str,
    pub path: &'a Path,
    /// `PHP_VERSION_ID` style: 80400 for PHP 8.4.
    pub php_version: u32,
}

/// One finding of a rule, before the engine adds the rule ID, the path and the level.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    pub span: Span,
    pub message: String,
    pub help: Option<String>,
    pub fix: Option<Fix>,
}

/// A lint rule. `visit` is called for every node of the rule's [`Rule::targets`]; `ancestors` lists the node's
/// parents, outermost first, so its last element is the direct parent.
pub trait Rule: Send + Sync {
    /// The full rule ID, `lint/<group>/<name>`.
    fn id(&self) -> &'static str;

    /// The lowest PHP version (as `PHP_VERSION_ID`) the rule's fix is valid for, like Rector's
    /// `MinPhpVersionInterface`. Below it the rule does not run.
    fn min_php_version(&self) -> u32 {
        0
    }

    /// The node kinds `visit` can report on, like Mago's `LintRule::targets`; `None` means every node. The engine
    /// calls `visit` for these kinds only, so a rule that names its kinds costs nothing on the other nodes.
    fn targets(&self) -> Option<&'static [NodeKind]> {
        None
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    );
}

/// Why a file could not be linted or fixed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LintError {
    /// The file does not parse. Format reports syntax errors, so lint only refuses the file.
    Syntax { span: Span, message: String },
    /// The fixes of these rules, applied together in one round, produced code that does not parse. The file is left
    /// as it was.
    FixBrokeSyntax {
        rules: Vec<String>,
        span: Span,
        message: String,
    },
    /// The rules still had fixes after [`MAX_ROUNDS`] rounds; these rules had a fix in the last one.
    NoConvergence { rules: Vec<String> },
}

impl fmt::Display for LintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LintError::Syntax { message, .. } => write!(f, "syntax error: {message}"),
            LintError::FixBrokeSyntax { rules, message, .. } => write!(
                f,
                "the fixes of {} produced a syntax error: {message}",
                rules.join(", ")
            ),
            LintError::NoConvergence { rules } => write!(
                f,
                "fixes did not settle after {MAX_ROUNDS} rounds ({})",
                rules.join(", ")
            ),
        }
    }
}

impl std::error::Error for LintError {}

/// The most fix rounds per file, as in Ruff. Each round is one parse.
pub const MAX_ROUNDS: usize = 100;

/// What [`Linter::fix`] did to one file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fixed {
    pub source: String,
    /// The problems whose fix was applied, in the order applied. Each span refers to the source of its own round.
    pub applied: Vec<Problem>,
    /// The problems left in the fixed source: rules without a fix, and fixes that cannot apply.
    pub remaining: Vec<Problem>,
    /// The number of rounds that changed the source.
    pub rounds: usize,
}

/// The reports of one walk, in the order the rules made them, and the ID of the rule behind each. Every rule writes
/// into the one `reports` list, so a node costs no list of its own.
#[derive(Default)]
struct Found {
    reports: Vec<Report>,
    rules: Vec<&'static str>,
}

/// A set of rules for one PHP version.
pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
    /// For each [`NodeKind`] (as `u8`), the rules to call on it with their IDs, in rule order.
    by_kind: Vec<Vec<(&'static str, usize)>>,
    php_version: u32,
}

impl Linter {
    /// Rules whose `min_php_version` is above `php_version` are dropped.
    pub fn new(rules: Vec<Box<dyn Rule>>, php_version: u32) -> Self {
        let rules: Vec<Box<dyn Rule>> = rules
            .into_iter()
            .filter(|r| r.min_php_version() <= php_version)
            .collect();
        let mut by_kind = vec![Vec::new(); usize::from(u8::MAX) + 1];
        for (i, rule) in rules.iter().enumerate() {
            match rule.targets() {
                Some(kinds) => kinds
                    .iter()
                    .for_each(|&k| by_kind[k as usize].push((rule.id(), i))),
                None => by_kind.iter_mut().for_each(|r| r.push((rule.id(), i))),
            }
        }
        Linter {
            rules,
            by_kind,
            php_version,
        }
    }

    /// The IDs of the rules that run, in order.
    pub fn rule_ids(&self) -> Vec<&'static str> {
        self.rules.iter().map(|r| r.id()).collect()
    }

    /// Parse `source` once and run every rule over the tree. Problems are sorted by start offset; at the same offset
    /// an outer node comes before an inner one, and rules keep their order.
    pub fn check(&self, path: &Path, source: &str) -> Result<Vec<Problem>, LintError> {
        // SAFETY: `file` and the tree parsed from it borrow `source` and are dropped before this function returns;
        // nothing built from them outlives `source`. Mago's `File` only takes a `'static` borrow.
        let contents: &'static [u8] = unsafe { &*std::ptr::from_ref::<[u8]>(source.as_bytes()) };
        let file = File {
            id: FileId::zero(),
            name: Cow::Borrowed(b""),
            path: None,
            file_type: FileType::Host,
            contents: Cow::Borrowed(contents),
            size: source.len() as u32,
            lines: Vec::new(),
        };
        let arena = LocalArena::new();
        let program = mago_syntax::parser::parse_file_borrowed_with_settings(
            &arena,
            &file,
            ParserSettings::default(),
        );
        if let Some(error) = program.errors.first() {
            return Err(LintError::Syntax {
                span: syntax::span(error.span()),
                message: error.to_string(),
            });
        }
        let ctx = Context {
            source,
            path,
            php_version: self.php_version,
        };
        let mut state = walk::State {
            ancestors: Vec::new(),
            found: Found::default(),
        };
        walk::Walk {
            linter: self,
            ctx: &ctx,
        }
        .walk_program(program, &mut state);
        let found = state.found;
        let mut found: Vec<(&'static str, Report)> =
            found.rules.into_iter().zip(found.reports).collect();
        found.sort_by_key(|(_, report)| report.span.start);
        Ok(found
            .into_iter()
            .map(|(rule, report)| Problem {
                rule: rule.to_string(),
                path: PathBuf::from(path),
                span: report.span,
                message: report.message,
                help: report.help,
                level: Level::Error,
                fix: report.fix,
            })
            .collect())
    }

    /// Apply every fix, round after round, until no rule has a fix that applies. Each round parses the new source
    /// once. Fixes apply in problem order; one that overlaps an earlier one waits for the next round.
    pub fn fix(&self, path: &Path, source: &str) -> Result<Fixed, LintError> {
        let mut current = source.to_string();
        let mut applied = Vec::new();
        let mut last_rules = Vec::new();
        let mut round = 0;
        loop {
            let problems = self.check(path, &current).map_err(|e| match e {
                LintError::Syntax { span, message } if round > 0 => LintError::FixBrokeSyntax {
                    rules: last_rules.clone(),
                    span,
                    message,
                },
                e => e,
            })?;
            let with_fix: Vec<&Problem> = problems.iter().filter(|p| p.fix.is_some()).collect();
            let fixes: Vec<&Fix> = with_fix.iter().filter_map(|p| p.fix.as_ref()).collect();
            let result = apply_fixes(&current, &fixes);
            if result.applied.is_empty() {
                return Ok(Fixed {
                    source: current,
                    applied,
                    remaining: problems,
                    rounds: round,
                });
            }
            last_rules = rule_ids(result.applied.iter().map(|&i| with_fix[i]));
            if round == MAX_ROUNDS {
                return Err(LintError::NoConvergence { rules: last_rules });
            }
            applied.extend(result.applied.iter().map(|&i| with_fix[i].clone()));
            current = result.source;
            round += 1;
        }
    }
}

/// The distinct rule IDs of `problems`, in first-seen order.
fn rule_ids<'a>(problems: impl Iterator<Item = &'a Problem>) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for p in problems {
        if !ids.contains(&p.rule) {
            ids.push(p.rule.clone());
        }
    }
    ids
}
