//! Port of `PhpCsFixer\Fixer\Semicolon\MultilineWhitespaceBeforeSemicolonsFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens};
use serde_json::Value;
use std::path::Path;

const STRATEGY_NO_MULTI_LINE: &str = "no_multi_line";
const STRATEGY_NEW_LINE_FOR_CHAINED_CALLS: &str = "new_line_for_chained_calls";

#[derive(Default)]
struct MultilineWhitespaceBeforeSemicolonsFixer {
    strategy: String,
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    crate::configured(MultilineWhitespaceBeforeSemicolonsFixer::default())
}

impl MultilineWhitespaceBeforeSemicolonsFixer {
    fn get_new_line_index(&self, mut index: usize, tokens: &Tokens) -> usize {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        let count = tokens.count();
        while index < count {
            if !tokens[index].is_whitespace(None) && !tokens[index].is_comment() {
                break;
            }
            if tokens[index]
                .get_content()
                .windows(line_ending.len())
                .any(|w| w == line_ending)
            {
                return index;
            }
            index += 1;
        }
        index
    }

    fn get_previous_significant_token_index(mut index: usize, tokens: &Tokens) -> usize {
        let stop_tokens = [
            T_LNUMBER,
            T_DNUMBER,
            T_STRING,
            T_VARIABLE,
            T_CONSTANT_ENCAPSED_STRING,
        ];
        while index > 0 {
            if tokens[index].is_given_kind(&stop_tokens) || tokens[index].equals(")", true) {
                return index;
            }
            index -= 1;
        }
        index
    }

    /// Checks if the semicolon closes a multiline call and returns the whitespace of the first call at `index`.
    fn find_whitespace_before_first_call(mut index: usize, tokens: &Tokens) -> Option<Vec<u8>> {
        let mut is_multiline_call = false;
        let mut prev_index = tokens.get_prev_meaningful_token(index).unwrap();
        while !tokens[prev_index].equals_any(
            &[
                ";".into(),
                ":".into(),
                "{".into(),
                "}".into(),
                T_OPEN_TAG.into(),
                T_OPEN_TAG_WITH_ECHO.into(),
                T_ELSE.into(),
            ],
            true,
        ) {
            index = prev_index;
            prev_index = tokens.get_prev_meaningful_token(index).unwrap();
            if let Some((block_type, false)) = Tokens::detect_block_type(&tokens[index]) {
                prev_index = tokens.find_block_start(block_type, index);
                continue;
            }
            if tokens[index].is_object_operator() || tokens[index].is_kind(T_DOUBLE_COLON) {
                prev_index = tokens.get_prev_meaningful_token(index).unwrap();
                is_multiline_call =
                    is_multiline_call || tokens.is_partial_code_multiline(prev_index, index);
            }
        }
        is_multiline_call.then(|| WhitespacesAnalyzer::detect_indent(tokens, index))
    }
}

impl Fixer for MultilineWhitespaceBeforeSemicolonsFixer {
    fn name(&self) -> &'static str {
        "multiline_whitespace_before_semicolons"
    }
    fn priority(&self) -> i32 {
        // Must run before SpaceAfterSemicolonFixer.
        // Must run after CombineConsecutiveIssetsFixer, GetClassToClassKeywordFixer, NoEmptyStatementFixer,
        // SimplifiedIfReturnFixer, SingleImportPerStatementFixer.
        0
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["strategy"])?;
        self.strategy = r.string(
            "strategy",
            STRATEGY_NO_MULTI_LINE,
            &[STRATEGY_NO_MULTI_LINE, STRATEGY_NEW_LINE_FOR_CHAINED_CALLS],
        )?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(";")
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        let count = tokens.count();
        let mut index = 0;
        while index < count {
            if tokens[index].is_kind(T_CONST) {
                index = tokens
                    .get_next_token_of_kind(index, &[";".into()], true)
                    .unwrap();
                index += 1;
                continue;
            }
            if !tokens[index].equals(";", true) {
                index += 1;
                continue;
            }
            let previous_index = index - 1;
            let previous = tokens[previous_index].clone();
            let indent = Self::find_whitespace_before_first_call(index, tokens);
            if self.strategy == STRATEGY_NEW_LINE_FOR_CHAINED_CALLS
                && let Some(indent) = indent
            {
                let wanted = [line_ending, &indent].concat();
                if previous.is_whitespace(None) && previous.get_content() == wanted.as_slice() {
                    index += 1;
                    continue;
                }
                if previous.is_whitespace(None) {
                    tokens.clear_at(previous_index);
                }
                tokens.clear_at(index);
                index = self.get_new_line_index(index, tokens);
                let newline = Token::new(T_WHITESPACE, wanted);
                tokens.insert_at(index, vec![newline, Token::char(";")]);
                index += 1;
            } else {
                if !previous.is_whitespace(None) || !previous.get_content().contains(&b'\n') {
                    index += 1;
                    continue;
                }
                let content = previous.get_content();
                if content.starts_with(line_ending) && tokens[index - 2].is_comment() {
                    // if there is comment between closing parenthesis and semicolon
                    tokens.clear_at(previous_index);
                    tokens.clear_at(index);
                    let significant_token_index =
                        Self::get_previous_significant_token_index(index, tokens);
                    tokens.insert_at(significant_token_index + 1, vec![Token::char(";")]);
                } else {
                    // if there is whitespace between closing bracket and semicolon, just remove it
                    tokens.clear_at(previous_index);
                }
            }
            index += 1;
        }
    }
}
