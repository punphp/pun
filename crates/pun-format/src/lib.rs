//! Fixer pipeline (PHP-CS-Fixer's Runner::fixFile, single pass, priority order) and the two engines.

use anyhow::{Result, bail};
use mago_span::HasSpan;
use pun_config::Rule;
use serde_json::Value;
use std::path::Path;

pub use pun_tokens::Tokens;

/// A PHP-CS-Fixer fixer (`FixerInterface` + `ConfigurableFixerInterface`).
pub trait Fixer: Send + Sync {
    /// PHP-CS-Fixer rule name, e.g. `no_unused_imports` or `Pint/laravel_blade`.
    fn name(&self) -> &'static str;
    fn priority(&self) -> i32;
    fn is_risky(&self) -> bool {
        false
    }
    /// Cheap token-kind check, run before `fix`.
    fn is_candidate(&self, tokens: &Tokens) -> bool;
    fn supports(&self, _file: &Path) -> bool {
        true
    }
    /// Called once with the rule's options; `Null` means the rule was enabled with `true` (defaults).
    fn configure(&mut self, options: &Value) -> Result<()> {
        if options.is_null() { Ok(()) } else { bail!("[{}] Is not configurable.", self.name()) }
    }
    fn fix(&self, file: &Path, tokens: &mut Tokens);
}

/// Enabled fixers, sorted exactly like `Utils::sortFixers` (priority desc, then name via strcmp).
pub struct Pipeline {
    fixers: Vec<Box<dyn Fixer>>,
    /// Enabled rules with no native implementation, in config order.
    pub not_ported: Vec<String>,
}

pub struct Fixed {
    pub code: Vec<u8>,
    pub applied: Vec<&'static str>,
}

impl Pipeline {
    /// `available` is every native fixer (from pun-fixers); only those named in `rules` are kept and configured.
    pub fn new(rules: &[Rule], mut available: Vec<Box<dyn Fixer>>) -> Result<Self> {
        let mut fixers = Vec::new();
        let mut not_ported = Vec::new();
        for rule in rules {
            match available.iter().position(|f| f.name() == rule.name) {
                Some(i) => {
                    let mut f = available.swap_remove(i);
                    f.configure(&rule.options)?;
                    fixers.push(f);
                }
                None => not_ported.push(rule.name.clone()),
            }
        }
        fixers.sort_by(|a, b| b.priority().cmp(&a.priority()).then_with(|| a.name().as_bytes().cmp(b.name().as_bytes())));
        Ok(Pipeline { fixers, not_ported })
    }

    pub fn fixers(&self) -> impl Iterator<Item = &dyn Fixer> {
        self.fixers.iter().map(|f| f.as_ref())
    }

    /// Runner::fixFile: lint, run each fixer once, return `None` when the code did not change.
    pub fn run(&self, file: &Path, code: &[u8]) -> Result<Option<Fixed>, LintError> {
        lint(code)?;
        let mut tokens = Tokens::from_code(code);
        let mut applied = Vec::new();
        for f in &self.fixers {
            if !f.supports(file) || !f.is_candidate(&tokens) {
                continue;
            }
            f.fix(file, &mut tokens);
            if tokens.is_changed() {
                tokens.clear_empty_tokens();
                tokens.clear_changed();
                applied.push(f.name());
            }
        }
        if applied.is_empty() {
            return Ok(None);
        }
        let new = tokens.generate_code();
        if new == code {
            return Ok(None);
        }
        lint(&new).map_err(|e| LintError { after_fix: true, ..e })?;
        Ok(Some(Fixed { code: new, applied }))
    }
}

/// A syntax error (PHP-CS-Fixer `LintingException`). `after_fix` = the fixers produced invalid code.
#[derive(Debug, Clone)]
pub struct LintError {
    pub message: String,
    pub after_fix: bool,
}

/// Syntax check standing in for `token_get_all(TOKEN_PARSE)`. Message shape matches PHP-CS-Fixer's
/// TokenizerLintingResult; the wording after "Parse error:" is mago's, not PHP's.
pub fn lint(code: &[u8]) -> Result<(), LintError> {
    let arena = mago_allocator::LocalArena::new();
    let program = mago_syntax::parser::parse_file_content(&arena, mago_database::file::FileId::zero(), code);
    match program.errors.first() {
        None => Ok(()),
        Some(e) => {
            let offset = e.span().start.offset as usize;
            let line = code[..offset.min(code.len())].iter().filter(|&&b| b == b'\n').count() + 1;
            Err(LintError { message: format!("Parse error: {e} on line {line}."), after_fix: false })
        }
    }
}

/// `--engine=pun`: mago-formatter with its Pint preset (whole-file reprint, not byte-identical to Pint).
pub fn format_pun(code: &[u8], php: &str) -> Result<Option<Vec<u8>>, LintError> {
    let (major, minor) = php.split_once('.').unwrap_or(("8", "4"));
    let version = mago_php_version::PHPVersion::new(major.parse().unwrap_or(8), minor.parse().unwrap_or(4), 0);
    let arena = mago_allocator::LocalArena::new();
    let settings = mago_formatter::presets::FormatterPreset::Pint.settings();
    let out = mago_formatter::Formatter::new(&arena, version, settings)
        .format_code("stdin.php".as_bytes().into(), code.to_vec().into())
        .map_err(|e| LintError { message: format!("Parse error: {e}."), after_fix: false })?;
    Ok((out != code).then(|| out.to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct F(&'static str, i32);
    impl Fixer for F {
        fn name(&self) -> &'static str {
            self.0
        }
        fn priority(&self) -> i32 {
            self.1
        }
        fn is_candidate(&self, _: &Tokens) -> bool {
            true
        }
        fn fix(&self, _: &Path, _: &mut Tokens) {}
    }

    #[test]
    fn sorts_like_php_cs_fixer() {
        let rule = |n: &str| Rule { name: n.into(), options: Value::Null };
        let rules = [rule("b"), rule("a"), rule("z"), rule("missing")];
        let p = Pipeline::new(&rules, vec![Box::new(F("z", 1)), Box::new(F("b", 5)), Box::new(F("a", 5)), Box::new(F("unused", 9))]).unwrap();
        assert_eq!(p.fixers().map(|f| f.name()).collect::<Vec<_>>(), ["a", "b", "z"]);
        assert_eq!(p.not_ported, ["missing"]);
        assert!(p.run(Path::new("x.php"), b"<?php\nfunction (\n").is_err());
        assert!(p.run(Path::new("x.php"), b"<?php\necho 1;\n").unwrap().is_none());
        assert!(format_pun(b"<?php\necho   1;", "8.4").unwrap().is_some());
    }
}
