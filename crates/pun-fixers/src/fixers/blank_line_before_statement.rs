//! Port of `PhpCsFixer\Fixer\Whitespace\BlankLineBeforeStatementFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind, Kind::*, Token, Tokens, TokensAnalyzer, php_compat::rtrim_chars};
use serde_json::Value;
use std::path::Path;

const TOKEN_MAP: &[(&str, Kind)] = &[
    ("break", T_BREAK), ("case", T_CASE), ("continue", T_CONTINUE), ("declare", T_DECLARE), ("default", T_DEFAULT),
    ("do", T_DO), ("exit", T_EXIT), ("for", T_FOR), ("foreach", T_FOREACH), ("goto", T_GOTO), ("if", T_IF),
    ("include", T_INCLUDE), ("include_once", T_INCLUDE_ONCE), ("phpdoc", T_DOC_COMMENT), ("require", T_REQUIRE),
    ("require_once", T_REQUIRE_ONCE), ("return", T_RETURN), ("switch", T_SWITCH), ("throw", T_THROW), ("try", T_TRY),
    ("while", T_WHILE), ("yield", T_YIELD), ("yield_from", T_YIELD_FROM),
];

struct BlankLineBeforeStatementFixer {
    fix_token_map: Vec<Kind>,
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    let mut f = BlankLineBeforeStatementFixer { fix_token_map: Vec::new(), whitespaces_config: Default::default() };
    f.configure(&Value::Null).unwrap();
    Box::new(f)
}

fn newlines(s: &[u8]) -> usize {
    s.iter().filter(|&&b| b == b'\n').count()
}

impl BlankLineBeforeStatementFixer {
    fn get_insert_blank_line_index(tokens: &Tokens, mut index: usize) -> usize {
        while index > 0 {
            if tokens[index - 1].is_whitespace(None) && newlines(tokens[index - 1].get_content()) > 1 {
                break;
            }
            let prev_index = tokens.get_prev_non_whitespace(index, None).unwrap();
            if !tokens[prev_index].is_comment() {
                break;
            }
            if !tokens[prev_index - 1].is_whitespace(None) {
                break;
            }
            if newlines(tokens[prev_index - 1].get_content()) != 1 {
                break;
            }
            index = prev_index;
        }
        index
    }

    fn should_add_blank_line(tokens: &Tokens, prev_non_whitespace: usize) -> bool {
        let prev_non_whitespace_token = &tokens[prev_non_whitespace];
        if prev_non_whitespace_token.is_comment() {
            for j in (0..prev_non_whitespace).rev() {
                if tokens[j].get_content().contains(&b'\n') {
                    return false;
                }
                if tokens[j].is_whitespace(None) || tokens[j].is_comment() {
                    continue;
                }
                return tokens[j].equals_any(&[";".into(), "}".into()], true);
            }
        }
        prev_non_whitespace_token.equals_any(&[";".into(), "}".into()], true)
    }

    fn insert_blank_line(&self, tokens: &mut Tokens, index: usize) {
        let prev_index = index - 1;
        let prev_token = &tokens[prev_index];
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        if prev_token.is_whitespace(None) {
            let newlines_count = newlines(prev_token.get_content());
            if newlines_count == 0 {
                let t = Token::new(T_WHITESPACE, [rtrim_chars(prev_token.get_content(), b" \t"), line_ending, line_ending].concat());
                tokens.set(prev_index, t);
            } else if newlines_count == 1 {
                let t = Token::new(T_WHITESPACE, [line_ending, prev_token.get_content()].concat());
                tokens.set(prev_index, t);
            }
        } else {
            tokens.insert_at(index, vec![Token::new(T_WHITESPACE, [line_ending, line_ending].concat())]);
        }
    }
}

impl Fixer for BlankLineBeforeStatementFixer {
    fn name(&self) -> &'static str {
        "blank_line_before_statement"
    }
    fn priority(&self) -> i32 {
        // Must run after NoExtraBlankLinesFixer, NoUselessElseFixer, NoUselessReturnFixer, ReturnAssignmentFixer,
        // YieldFromArrayToYieldsFixer.
        -21
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["statements"])?;
        let keys: Vec<&str> = TOKEN_MAP.iter().map(|(k, _)| *k).collect();
        let statements = r.strings("statements", &["break", "continue", "declare", "return", "throw", "try"], &keys)?;
        // `$fixTokenMap[$key] = ...` then array_values: duplicates keep their first position.
        let mut fix_token_map: Vec<(&str, Kind)> = Vec::new();
        for key in &statements {
            let kind = TOKEN_MAP.iter().find(|(k, _)| k == key).unwrap().1;
            if !fix_token_map.iter().any(|(k, _)| k == key) {
                fix_token_map.push((TOKEN_MAP.iter().find(|(k, _)| k == key).unwrap().0, kind));
            }
        }
        self.fix_token_map = fix_token_map.into_iter().map(|(_, k)| k).collect();
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&self.fix_token_map)
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = tokens.count() - 1;
        while index > 0 {
            let token = &tokens[index];
            if !token.is_given_kind(&self.fix_token_map) {
                index -= 1;
                continue;
            }
            let analyzer = TokensAnalyzer::new(tokens);
            if token.is_kind(T_WHILE) && analyzer.is_while_part_of_do_while(index) {
                index -= 1;
                continue;
            }
            if token.is_kind(T_CASE) && analyzer.is_enum_case(index) {
                index -= 1;
                continue;
            }
            let insert_blank_line_index = Self::get_insert_blank_line_index(tokens, index);
            let prev_non_whitespace = tokens.get_prev_non_whitespace(insert_blank_line_index, None).unwrap();
            if Self::should_add_blank_line(tokens, prev_non_whitespace) {
                self.insert_blank_line(tokens, insert_blank_line_index);
            }
            index = prev_non_whitespace;
            if index == 0 {
                break;
            }
            index -= 1;
        }
    }
}
