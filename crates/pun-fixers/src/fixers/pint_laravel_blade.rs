//! Port of Pint's `App\Fixers\LaravelBlade\Fixer` (+ `App\BladeFormatter`, `App\Support\Prettier`):
//! formats `*.blade.php` files with prettier (prettier-plugin-blade, resolved from the project's
//! node_modules like Pint does) wrapped in Pint's pre/post formatters.
#[path = "pint_laravel_blade/alpine_mask_patterns.rs"]
mod alpine_mask_patterns;
#[path = "pint_laravel_blade/collapse_single_attribute.rs"]
mod collapse_single_attribute;
#[path = "pint_laravel_blade/embedded_blade_masker.rs"]
mod embedded_blade_masker;
#[path = "pint_laravel_blade/escaped_directive_spacing.rs"]
mod escaped_directive_spacing;
#[path = "pint_laravel_blade/line_formatters.rs"]
mod line_formatters;
#[path = "pint_laravel_blade/not_operator_spacing.rs"]
mod not_operator_spacing;
#[path = "pint_laravel_blade/php.rs"]
mod php;
#[path = "pint_laravel_blade/php_block_formatting.rs"]
mod php_block_formatting;

use anyhow::Result;
use line_formatters::*;
use pun_format::Fixer;
use pun_tokens::Tokens;
use std::path::Path;

/// `preset`: the run's preset (Pint's `ConfigurationJsonRepository::preset()`: `--preset`, else pint.json's `preset`,
/// else "laravel"); embedded PHP is formatted with that preset's rules.
struct LaravelBladeFixer {
    preset: String,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(LaravelBladeFixer {
        preset: "laravel".into(),
    })
}

impl Fixer for LaravelBladeFixer {
    fn name(&self) -> &'static str {
        "Pint/laravel_blade"
    }
    fn priority(&self) -> i32 {
        -2
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn set_preset(&mut self, preset: &str) {
        self.preset = preset.into();
    }
    fn fix(&self, file: &Path, tokens: &mut Tokens) {
        // SplFileInfo::getRealPath()
        let path = std::fs::canonicalize(file).unwrap_or_else(|_| file.to_path_buf());
        let path_str = path.to_string_lossy().into_owned();
        if !path_str.ends_with(".blade.php") {
            return;
        }
        let content = tokens.generate_code();
        if is_ignored(&path_str) {
            return;
        }
        // PrettierException (or a failed masker regex): the runner reports the file as errored with the message and
        // leaves it untouched. Pun's fixers panic where upstream throws.
        let content = format(&path_str, &content, &self.preset).unwrap_or_else(|e| panic!("{e}"));
        // Tokens::setCode() rejects an empty string, so clear the tokens directly.
        if content.is_empty() {
            for index in 0..tokens.count() {
                tokens.clear_at(index);
            }
            tokens.clear_empty_tokens();
            return;
        }
        tokens.set_code(&content);
    }
}

/// Ignorables: Envoy, BoostGuidelines, EmailView.
fn is_ignored(path: &str) -> bool {
    let basename = path.rsplit('/').next().unwrap_or(path);
    let path = path.replace('\\', "/");
    matches!(basename, "envoy.blade.php" | "Envoy.blade.php")
        || path.contains("resources/boost/guidelines/")
        || path.contains("resources/views/emails/")
        || path.contains("resources/views/mail/")
}

/// `BladeFormatter::format`: pre-formatters, prettier, then post-formatters in declaration order. A masker that
/// cannot restore its placeholders (`UnrestorableContentException`) discards the run: the file stays as it was.
fn format(path: &str, content: &[u8], preset: &str) -> Result<Vec<u8>> {
    let mut spacing = escaped_directive_spacing::EscapedDirectiveSpacing::default();
    spacing.pre_format(content);
    let mut masker = embedded_blade_masker::EmbeddedBladeMasker::default();
    let masked = masker.pre_format(content).map_err(anyhow::Error::msg)?;
    let mut alpine = alpine_mask_patterns::AlpineMaskPatterns::default();
    let masked = alpine.pre_format(&masked).map_err(anyhow::Error::msg)?;
    let formatted = crate::prettier::format(path, &masked)?;
    let formatted = spacing.post_format(&formatted).map_err(anyhow::Error::msg)?;
    let formatted = strip_sensitive_leading_blank_lines::post_format(&formatted);
    let formatted = not_operator_spacing::post_format(&formatted);
    let formatted = collapse_short_slots::post_format(&formatted);
    let formatted = collapse_single_attribute::post_format(&formatted);
    let formatted = join_dangling_close_bracket::post_format(&formatted);
    let formatted = join_dangling_open_bracket::post_format(&formatted);
    let formatted = dedent_hugged_terminator::post_format(&formatted);
    let restored = masker
        .post_format(&formatted)
        .and_then(|formatted| alpine.post_format(&formatted));
    let Ok(formatted) = restored else {
        return Ok(content.to_vec());
    };
    let blocks = php_block_formatting::PhpBlockFormatting::new(
        php_block_formatting::PhpFragmentFormatter::new(preset.into()),
    );
    Ok(blocks.post_format(&formatted))
}

/// Each formatter step against Pint's own class on the same input (tests/blade_formatters.json, from
/// scripts/oracle-blade-formatter.php), for shapes the golden files in tests/blade don't reach.
/// PhpBlockFormatting runs the laravel preset's fixers, so they must be compiled in.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_blade_formatters() {
        let cases: Vec<serde_json::Value> =
            serde_json::from_str(include_str!("../../tests/blade_formatters.json")).unwrap();
        for (i, case) in cases.iter().enumerate() {
            let formatter = case["formatter"].as_str().unwrap();
            let input = case["input"].as_str().unwrap().as_bytes();
            let mut masker = embedded_blade_masker::EmbeddedBladeMasker::default();
            let mut alpine = alpine_mask_patterns::AlpineMaskPatterns::default();
            let mut spacing = escaped_directive_spacing::EscapedDirectiveSpacing::default();
            let got = match formatter {
                "StripSensitiveLeadingBlankLines" => {
                    strip_sensitive_leading_blank_lines::post_format(input)
                }
                "NotOperatorSpacing" => not_operator_spacing::post_format(input),
                "CollapseShortSlots" => collapse_short_slots::post_format(input),
                "CollapseSingleAttribute" => collapse_single_attribute::post_format(input),
                "JoinDanglingCloseBracket" => join_dangling_close_bracket::post_format(input),
                "JoinDanglingOpenBracket" => join_dangling_open_bracket::post_format(input),
                "DedentHuggedTerminator" => dedent_hugged_terminator::post_format(input),
                "EmbeddedBladeMasker" => masker.pre_format(input).unwrap(),
                "AlpineMaskPatterns" => alpine.pre_format(input).unwrap(),
                "EscapedDirectiveSpacing" => {
                    spacing.pre_format(input);
                    input.to_vec()
                }
                _ => php_block_formatting::PhpBlockFormatting::new(
                    php_block_formatting::PhpFragmentFormatter::new(
                        case.get("preset")
                            .map_or("laravel", |p| p.as_str().unwrap())
                            .into(),
                    ),
                )
                .post_format(input),
            };
            assert_eq!(
                String::from_utf8_lossy(&got),
                case["expected"].as_str().unwrap(),
                "case {i} {formatter}"
            );
            if let Some(post_input) = case.get("post_input") {
                let post_input = post_input.as_str().unwrap().as_bytes();
                // `null`: postFormat threw UnrestorableContentException.
                let got = match formatter {
                    "AlpineMaskPatterns" => alpine.post_format(post_input).ok(),
                    "EscapedDirectiveSpacing" => Some(spacing.post_format(post_input).unwrap()),
                    _ => masker.post_format(post_input).ok(),
                };
                assert_eq!(
                    got.map(|got| String::from_utf8_lossy(&got).into_owned()),
                    case["expected_post"].as_str().map(str::to_owned),
                    "case {i} {formatter} postFormat"
                );
            }
        }
    }

    /// An unclosed `<script>` with 8 quoted attributes in a 34 KB template: the masker's `<style|script>` pattern
    /// exhausts PHP's backtrack limit (`preg_last_error_msg()` on PHP 8.5: "Backtrack limit exhausted"). Pint writes an
    /// empty file there; Pun fails the file the way a PrettierException does (a panic is the runner's per-file error)
    /// and leaves its tokens untouched. The failure comes before prettier, so no worker starts.
    #[test]
    fn masker_regex_failure_fails_the_file() {
        let attributes: String = (1..=8).map(|i| format!(" data-a{i}=\"v{i}\"")).collect();
        let mut code = format!("<script{attributes}>\nvar x = 1;\n<div>\n");
        while code.len() < 34000 {
            code.push_str("    <p>Hello world line</p>\n");
        }
        code.push_str("</div>\n");
        let mut tokens = Tokens::from_code(code.as_bytes());
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            fixer().fix(Path::new("repro.blade.php"), &mut tokens)
        }))
        .unwrap_err();
        assert_eq!(
            panic.downcast_ref::<String>().unwrap(),
            "preg_replace_callback(): Backtrack limit exhausted"
        );
        assert_eq!(tokens.generate_code(), code.as_bytes());
    }
}
