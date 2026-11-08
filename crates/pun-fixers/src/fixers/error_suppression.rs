//! Port of `PhpCsFixer\Fixer\LanguageConstruct\ErrorSuppressionFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::functions_analyzer::FunctionsAnalyzer;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct ErrorSuppressionFixer {
    mute_deprecation_error: bool,
    noise_remaining_usages: bool,
    noise_remaining_usages_exclude: Vec<String>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(ErrorSuppressionFixer {
        mute_deprecation_error: true,
        noise_remaining_usages: false,
        noise_remaining_usages_exclude: vec![],
    })
}

impl ErrorSuppressionFixer {
    fn is_deprecation_error_call(tokens: &Tokens, index: usize) -> bool {
        if strtolower(tokens[index].get_content()) != b"trigger_error" {
            return false;
        }
        let open = tokens
            .get_next_token_of_kind(index, &[T_STRING.into(), "(".into()], true)
            .unwrap();
        let end_brace_index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
        let mut prev_index = tokens.get_prev_meaningful_token(end_brace_index).unwrap();
        if tokens[prev_index].equals(",", true) {
            prev_index = tokens.get_prev_meaningful_token(prev_index).unwrap();
        }
        tokens[prev_index].equals((T_STRING, "E_USER_DEPRECATED"), true)
    }
}

impl Fixer for ErrorSuppressionFixer {
    fn name(&self) -> &'static str {
        "error_suppression"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn configure(&mut self, options: &Value) -> anyhow::Result<()> {
        let c = resolve(
            self.name(),
            options,
            &[
                "mute_deprecation_error",
                "noise_remaining_usages",
                "noise_remaining_usages_exclude",
            ],
        )?;
        self.mute_deprecation_error = c.bool("mute_deprecation_error", true)?;
        self.noise_remaining_usages = c.bool("noise_remaining_usages", false)?;
        self.noise_remaining_usages_exclude =
            c.strings("noise_remaining_usages_exclude", &[], &[])?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_STRING)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let functions_analyzer = FunctionsAnalyzer::new();
        let excluded_functions: Vec<Vec<u8>> = self
            .noise_remaining_usages_exclude
            .iter()
            .map(|f| strtolower(f.as_bytes()))
            .collect();
        let mut index = tokens.len() as isize - 1;
        while index >= 0 {
            let i = index as usize;
            index -= 1;
            if self.noise_remaining_usages && tokens[i].equals("@", true) {
                tokens.clear_at(i);
                continue;
            }
            if !functions_analyzer.is_global_function_call(tokens, i) {
                continue;
            }
            let function_index = i;
            let mut start_index = i;
            let mut prev_index = tokens.get_prev_meaningful_token(i).unwrap();
            if tokens[prev_index].is_kind(T_NS_SEPARATOR) {
                start_index = prev_index;
                prev_index = tokens.get_prev_meaningful_token(start_index).unwrap();
            }
            // `$index = $prevIndex;` then the loop's `--$index`.
            index = prev_index as isize - 1;
            if Self::is_deprecation_error_call(tokens, function_index) {
                if !self.mute_deprecation_error || tokens[prev_index].equals("@", true) {
                    continue;
                }
                tokens.insert_at(start_index, vec![Token::char("@")]);
                continue;
            }
            if !tokens[prev_index].equals("@", true) {
                continue;
            }
            if self.noise_remaining_usages
                && !excluded_functions.contains(&tokens[function_index].get_content().to_vec())
            {
                tokens.clear_at(prev_index);
            }
        }
    }
}
