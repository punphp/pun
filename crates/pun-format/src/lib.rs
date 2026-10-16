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
    /// `WhitespacesAwareFixerInterface::setWhitespacesConfig`; only whitespace-aware fixers store it.
    fn set_whitespaces_config(&mut self, _config: &WhitespacesFixerConfig) {}
}

/// `PhpCsFixer\WhitespacesFixerConfig` (Pint never overrides the defaults: four spaces, "\n").
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WhitespacesFixerConfig {
    indent: String,
    line_ending: String,
}

impl Default for WhitespacesFixerConfig {
    fn default() -> Self {
        WhitespacesFixerConfig { indent: "    ".into(), line_ending: "\n".into() }
    }
}

impl WhitespacesFixerConfig {
    pub fn new(indent: &str, line_ending: &str) -> Result<Self> {
        if !["  ", "    ", "\t"].contains(&indent) {
            bail!("Invalid \"indent\" param, expected tab or two or four spaces.");
        }
        if !["\n", "\r\n"].contains(&line_ending) {
            bail!("Invalid \"lineEnding\" param, expected \"\\n\" or \"\\r\\n\".");
        }
        Ok(WhitespacesFixerConfig { indent: indent.into(), line_ending: line_ending.into() })
    }

    pub fn get_indent(&self) -> &str {
        &self.indent
    }

    pub fn get_line_ending(&self) -> &str {
        &self.line_ending
    }
}

/// Fixers PHP-CS-Fixer 3.95 marks deprecated (DeprecatedFixerInterface) and their successors. Pun does not port them.
pub const DEPRECATED: &[(&str, &[&str])] = &[
    ("braces", &["single_space_around_construct", "control_structure_braces", "control_structure_continuation_position", "declare_parentheses", "no_multiple_statements_per_line", "braces_position", "statement_indentation", "no_extra_blank_lines"]),
    ("class_keyword_remove", &[]),
    ("compact_nullable_typehint", &["compact_nullable_type_declaration"]),
    ("curly_braces_position", &["braces_position"]),
    ("escape_implicit_backslashes", &["string_implicit_backslashes"]),
    ("function_typehint_space", &["type_declaration_spaces"]),
    ("native_function_type_declaration_casing", &["native_type_declaration_casing"]),
    ("new_with_braces", &["new_with_parentheses"]),
    ("no_blank_lines_before_namespace", &["blank_lines_before_namespace"]),
    ("no_spaces_inside_parenthesis", &["spaces_inside_parentheses"]),
    ("no_trailing_comma_in_list_call", &["no_trailing_comma_in_singleline"]),
    ("no_trailing_comma_in_singleline_array", &["no_trailing_comma_in_singleline"]),
    ("no_trailing_comma_in_singleline_function_call", &["no_trailing_comma_in_singleline"]),
    ("no_unneeded_curly_braces", &["no_unneeded_braces"]),
    ("phpdoc_types_no_duplicates", &["phpdoc_no_duplicate_types"]),
    ("single_blank_line_before_namespace", &["blank_lines_before_namespace"]),
    ("single_space_after_construct", &["single_space_around_construct"]),
    ("visibility_required", &["modifier_keywords"]),
];

/// AbstractProxyFixer: the successor's behaviour and priority under the deprecated name (which affects sort order).
struct Proxy {
    name: &'static str,
    inner: Box<dyn Fixer>,
}

impl Fixer for Proxy {
    fn name(&self) -> &'static str {
        self.name
    }
    fn priority(&self) -> i32 {
        self.inner.priority()
    }
    // is_risky / configure / set_whitespaces_config are never called on the proxy: the inner fixer is configured
    // before wrapping, and modifier_keywords is not risky (the trait default).
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        self.inner.is_candidate(tokens)
    }
    fn supports(&self, file: &Path) -> bool {
        self.inner.supports(file)
    }
    fn fix(&self, file: &Path, tokens: &mut Tokens) {
        self.inner.fix(file, tokens)
    }
}

/// Enabled fixers, sorted exactly like `Utils::sortFixers` (priority desc, then name via strcmp).
pub struct Pipeline {
    fixers: Vec<Box<dyn Fixer>>,
    /// Enabled rules with no native implementation, in config order.
    pub not_ported: Vec<String>,
}

#[derive(Debug)]
pub struct Fixed {
    pub code: Vec<u8>,
    pub applied: Vec<&'static str>,
}

impl Pipeline {
    /// `factory` returns a fresh set of every native fixer (pun_fixers::all); rules not in it are reported as not ported.
    /// Deprecated PHP-CS-Fixer rules are refused (see DEPRECATED), except `visibility_required`, which Pint users still
    /// enable and which upstream runs as a plain proxy of `modifier_keywords`.
    pub fn new(rules: &[Rule], factory: &dyn Fn() -> Vec<Box<dyn Fixer>>) -> Result<Self> {
        let mut fixers: Vec<Box<dyn Fixer>> = Vec::new();
        let mut not_ported = Vec::new();
        for rule in rules {
            let mut target = rule.name.as_str();
            if let Some((_, successors)) = DEPRECATED.iter().find(|(n, _)| *n == target) {
                if target != "visibility_required" {
                    let hint = match successors {
                        [] => "Remove it; it has no replacement.".to_string(),
                        s => format!("Use {} instead.", s.iter().map(|s| format!("\"{s}\"")).collect::<Vec<_>>().join(", ")),
                    };
                    bail!("Rule \"{target}\" is deprecated in PHP-CS-Fixer and not supported by Pun. {hint}");
                }
                target = successors[0];
            }
            match factory().into_iter().find(|f| f.name() == target) {
                Some(mut f) => {
                    // FixerFactory::setWhitespacesConfig on every WhitespacesAwareFixerInterface; Pint keeps the defaults.
                    f.set_whitespaces_config(&WhitespacesFixerConfig::default());
                    f.configure(&rule.options)?;
                    if target != rule.name {
                        f = Box::new(Proxy { name: "visibility_required", inner: f });
                    }
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
            // AbstractFixer::fix: `0 < $tokens->count() && isCandidate && supports` (Pint/laravel_blade can empty the list).
            if tokens.count() == 0 || !f.supports(file) || !f.is_candidate(&tokens) {
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
    use pun_tokens::{Kind, Token};

    /// Test fixer: `edit` runs on every candidate; `file` names it doesn't support.
    struct F {
        name: &'static str,
        priority: i32,
        unsupported: &'static str,
        edit: fn(&mut Tokens),
        ws: Option<WhitespacesFixerConfig>,
    }
    impl Fixer for F {
        fn name(&self) -> &'static str {
            self.name
        }
        fn priority(&self) -> i32 {
            self.priority
        }
        fn is_candidate(&self, tokens: &Tokens) -> bool {
            tokens.is_token_kind_found(Kind::T_STRING)
        }
        fn supports(&self, file: &Path) -> bool {
            !file.ends_with(self.unsupported)
        }
        fn fix(&self, _: &Path, tokens: &mut Tokens) {
            assert_eq!(self.ws, Some(WhitespacesFixerConfig::default()), "the pipeline sets the whitespace config");
            (self.edit)(tokens)
        }
        fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
            self.ws = Some(config.clone());
        }
    }
    fn f(name: &'static str, priority: i32, edit: fn(&mut Tokens)) -> Box<dyn Fixer> {
        Box::new(F { name, priority, unsupported: "skip.php", edit, ws: None })
    }
    /// A fixer relying on the trait defaults (no options, not risky, supports everything, ignores whitespace config).
    struct Plain;
    impl Fixer for Plain {
        fn name(&self) -> &'static str {
            "plain"
        }
        fn priority(&self) -> i32 {
            0
        }
        fn is_candidate(&self, _: &Tokens) -> bool {
            true
        }
        fn fix(&self, _: &Path, _: &mut Tokens) {}
    }
    fn rule(n: &str) -> Rule {
        Rule { name: n.into(), options: Value::Null }
    }
    fn upper(t: &mut Tokens) {
        for i in 0..t.count() {
            if t[i].is_kind(Kind::T_STRING) {
                let c = t[i].get_content().to_ascii_uppercase();
                t.set(i, Token::new(Kind::T_STRING, c));
            }
        }
    }
    fn drop_space(t: &mut Tokens) {
        if let Some(i) = (0..t.count()).find(|&i| t[i].is_kind(Kind::T_WHITESPACE) && t[i].get_content() == b" ") {
            t.clear_at(i);
        }
    }
    fn round_trip(t: &mut Tokens) {
        let i = (0..t.count()).find(|&i| t[i].is_kind(Kind::T_STRING)).unwrap();
        let old = t[i].clone();
        t.set(i, Token::new(Kind::T_STRING, "tmp"));
        t.set(i, old);
    }
    fn none(_: &mut Tokens) {}
    fn factory() -> Vec<Box<dyn Fixer>> {
        vec![f("z", 1, none), f("upper", 5, upper), f("drop_space", 5, drop_space), f("round_trip", 9, round_trip), f("unused", 9, none), Box::new(Plain)]
    }

    #[test]
    fn sorts_like_php_cs_fixer() {
        // Utils::sortFixers: priority desc, then name (strcmp); unknown rules are reported, not fatal.
        let p = Pipeline::new(&[rule("upper"), rule("drop_space"), rule("z"), rule("missing"), rule("round_trip"), rule("plain")], &factory).unwrap();
        assert_eq!(p.fixers().map(|f| f.name()).collect::<Vec<_>>(), ["round_trip", "drop_space", "upper", "z", "plain"]);
        assert_eq!(p.not_ported, ["missing"]);
        assert!(!Plain.is_risky() && Plain.supports(Path::new("x.php")));
        assert_eq!(Plain.configure(&serde_json::json!({"a": 1})).unwrap_err().to_string(), "[plain] Is not configurable.");
        let plain = Pipeline::new(&[rule("plain")], &|| vec![Box::new(Plain)]).unwrap();
        assert!(plain.run(Path::new("x.php"), b"<?php foo();").unwrap().is_none());
    }

    #[test]
    fn runs_like_runner_fix_file() {
        let p = Pipeline::new(&[rule("upper"), rule("drop_space"), rule("round_trip"), rule("z")], &factory).unwrap();
        // Each changed fixer is recorded once, in run order; empty tokens are cleared between fixers.
        let fixed = p.run(Path::new("a.php"), b"<?php foo (bar);\nbaz();\n").unwrap().unwrap();
        assert_eq!(String::from_utf8(fixed.code).unwrap(), "<?php FOO(BAR);\nBAZ();\n");
        assert_eq!(fixed.applied, ["round_trip", "drop_space", "upper"]);
        // Changed-but-identical code and code with no candidate tokens report no change.
        let only_round_trip = Pipeline::new(&[rule("round_trip")], &factory).unwrap();
        assert!(only_round_trip.run(Path::new("a.php"), b"<?php foo();").unwrap().is_none());
        assert!(p.run(Path::new("a.php"), b"<?php\n$a = 1;\n").unwrap().is_none());
        // supports() false skips the fixer.
        let only_upper = Pipeline::new(&[rule("upper")], &factory).unwrap();
        assert!(only_upper.run(Path::new("dir/skip.php"), b"<?php foo();").unwrap().is_none());
        // Syntax errors: in the input (TYPE_INVALID) vs produced by the fixers (TYPE_LINT).
        let e = p.run(Path::new("a.php"), b"<?php\nfunction (\n").unwrap_err();
        // (The line is mago's error position; PHP reports this one on line 3, mago on line 2 — see docs/COVERAGE.md.)
        assert!(!e.after_fix && e.message.starts_with("Parse error: "), "{}", e.message);
        let only_drop = Pipeline::new(&[rule("drop_space")], &factory).unwrap();
        let e = only_drop.run(Path::new("a.php"), b"<?php\nfunction a() {}").unwrap_err();
        assert!(e.after_fix && e.message.starts_with("Parse error: "), "{}", e.message);
    }

    #[test]
    fn whitespaces_config() {
        // Messages from PhpCsFixer\WhitespacesFixerConfig.
        assert_eq!(WhitespacesFixerConfig::new("   ", "\n").unwrap_err().to_string(), "Invalid \"indent\" param, expected tab or two or four spaces.");
        assert_eq!(WhitespacesFixerConfig::new("\t", "\r").unwrap_err().to_string(), r#"Invalid "lineEnding" param, expected "\n" or "\r\n"."#);
        let c = WhitespacesFixerConfig::new("\t", "\r\n").unwrap();
        assert_eq!((c.get_indent(), c.get_line_ending()), ("\t", "\r\n"));
        let d = WhitespacesFixerConfig::default();
        assert_eq!((d.get_indent(), d.get_line_ending()), ("    ", "\n"));
    }

    #[test]
    fn deprecated_rules() {
        let all = || -> Vec<Box<dyn Fixer>> { vec![f("modifier_keywords", 56, upper), f("new_with_parentheses", 37, none)] };
        let err = Pipeline::new(&[rule("new_with_braces")], &all).err().unwrap().to_string();
        assert_eq!(err, "Rule \"new_with_braces\" is deprecated in PHP-CS-Fixer and not supported by Pun. Use \"new_with_parentheses\" instead.");
        let err = Pipeline::new(&[rule("braces")], &all).err().unwrap().to_string();
        assert!(err.ends_with("Use \"single_space_around_construct\", \"control_structure_braces\", \"control_structure_continuation_position\", \"declare_parentheses\", \"no_multiple_statements_per_line\", \"braces_position\", \"statement_indentation\", \"no_extra_blank_lines\" instead."), "{err}");
        assert!(Pipeline::new(&[rule("class_keyword_remove")], &all).err().unwrap().to_string().ends_with("Remove it; it has no replacement."));
        // visibility_required runs a second modifier_keywords instance under its own name, sorted after the preset one,
        // and behaves exactly like it (AbstractProxyFixer).
        let p = Pipeline::new(&[rule("visibility_required")], &all).unwrap();
        assert_eq!(p.fixers().map(|f| (f.name(), f.priority())).collect::<Vec<_>>(), [("visibility_required", 56)]);
        let fixed = p.run(Path::new("a.php"), b"<?php foo();").unwrap().unwrap();
        assert_eq!((fixed.code, fixed.applied), (b"<?php FOO();".to_vec(), vec!["visibility_required"]));
        assert!(p.run(Path::new("skip.php"), b"<?php foo();").unwrap().is_none());
        assert!(p.run(Path::new("a.php"), b"<?php $a;").unwrap().is_none());
        let p = Pipeline::new(&[rule("visibility_required"), rule("modifier_keywords")], &all).unwrap();
        assert_eq!(p.fixers().map(|f| f.name()).collect::<Vec<_>>(), ["modifier_keywords", "visibility_required"]);
    }

    #[test]
    fn pun_engine() {
        // Some(new) only when mago-formatter changed the code: its own output comes back as None.
        let out = format_pun(b"<?php\necho   1;", "8.4").unwrap().unwrap();
        assert_ne!(out, b"<?php\necho   1;");
        assert!(format_pun(&out, "8.5").unwrap().is_none());
        assert!(format_pun(b"<?php\nfunction (", "x").unwrap_err().message.starts_with("Parse error: "));
    }
}
