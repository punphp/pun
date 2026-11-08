//! Port of `PhpCsFixer\Fixer\Whitespace\NoExtraBlankLinesFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::{PREG_SPLIT_DELIM_CAPTURE, PREG_SPLIT_NO_EMPTY, Preg};
use crate::analyzers::switch_analyzer::SwitchAnalyzer;
use anyhow::Result;
use indexmap::IndexMap;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind, Kind::*, Token, Tokens, TokensAnalyzer, php_compat::rtrim_chars};
use serde_json::Value;
use std::path::Path;

const AVAILABLE_TOKENS: &[&str] = &[
    "attribute",
    "break",
    "case",
    "comma",
    "continue",
    "curly_brace_block",
    "default",
    "extra",
    "parenthesis_brace_block",
    "return",
    "square_brace_block",
    "switch",
    "throw",
    "use",
    "use_trait",
];

#[derive(Clone, Copy)]
enum Callback {
    FixAfterToken,
    FixAfterCaseToken,
    FixAfterThrowToken,
    FixStructureOpenCloseIfMultiLine,
    RemoveBetweenUse,
    RemoveMultipleBlankLines,
}

struct NoExtraBlankLinesFixer {
    token_kind_callback_map: IndexMap<Kind, Callback>,
    token_equals_map: Vec<(&'static str, Callback)>,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(NoExtraBlankLinesFixer {
        token_kind_callback_map: IndexMap::new(),
        token_equals_map: Vec::new(),
        whitespaces_config: Default::default(),
    })
}

struct Run<'a> {
    fixer: &'a NoExtraBlankLinesFixer,
    tokens: &'a mut Tokens,
}

impl Run<'_> {
    fn call(&mut self, callback: Callback, index: usize) {
        match callback {
            Callback::FixAfterToken => self.fix_after_token(index),
            Callback::FixAfterCaseToken => self.fix_after_case_token(index),
            Callback::FixAfterThrowToken => self.fix_after_throw_token(index),
            Callback::FixStructureOpenCloseIfMultiLine => {
                self.fix_structure_open_close_if_multi_line(index);
            }
            Callback::RemoveBetweenUse => self.remove_between_use(index),
            Callback::RemoveMultipleBlankLines => self.remove_multiple_blank_lines(index),
        }
    }

    fn fix_by_token(&mut self, index: usize) {
        let token = &self.tokens[index];
        for (&kind, &callback) in &self.fixer.token_kind_callback_map {
            if token.is_kind(kind) {
                self.call(callback, index);
                return;
            }
        }
        for &(equals, callback) in &self.fixer.token_equals_map {
            if token.equals(equals, true) {
                self.call(callback, index);
                return;
            }
        }
    }

    fn remove_between_use(&mut self, index: usize) {
        let next =
            self.tokens
                .get_next_token_of_kind(index, &[";".into(), T_CLOSE_TAG.into()], true);
        let Some(next) = next else { return };
        if self.tokens[next].is_kind(T_CLOSE_TAG) {
            return;
        }
        let next_use_candidate = self.tokens.get_next_meaningful_token(next);
        let Some(n) = next_use_candidate else { return };
        if !self.tokens[n].is_kind(self.tokens[index].get_id().unwrap())
            || !self.contains_linebreak(index, n)
        {
            return;
        }
        self.remove_empty_lines_after_line_with_token_at(next);
    }

    fn remove_multiple_blank_lines(&mut self, index: usize) {
        let prev = &self.tokens[index - 1];
        let expected = if prev.is_kind(T_OPEN_TAG) && Preg::r#match("/\\R$/", prev.get_content()) {
            1
        } else {
            2
        };
        let parts = Preg::split(
            "/(.*\\R)/",
            self.tokens[index].get_content(),
            -1,
            PREG_SPLIT_DELIM_CAPTURE | PREG_SPLIT_NO_EMPTY,
        );
        let count = parts.len();
        if count > expected {
            let mut content: Vec<u8> = parts[..expected]
                .iter()
                .flat_map(|p| p.text.iter().copied())
                .collect();
            content.extend_from_slice(rtrim_chars(&parts[count - 1].text, b"\r\n"));
            self.tokens.set(index, Token::new(T_WHITESPACE, content));
        }
    }

    fn fix_after_token(&mut self, index: usize) {
        let mut i = index - 1;
        while i > 0 {
            let t = &self.tokens[i];
            if t.is_kind(T_FUNCTION) && TokensAnalyzer::new(self.tokens).is_lambda(i) {
                return;
            }
            if t.is_kind(T_CLASS) && TokensAnalyzer::new(self.tokens).is_anonymous_class(i) {
                return;
            }
            if t.is_given_kind(&[CT_ARRAY_BRACKET_OPEN, CT_DESTRUCTURING_BRACKET_OPEN]) {
                return;
            }
            if t.is_whitespace(None) && t.get_content().contains(&b'\n') {
                break;
            }
            i -= 1;
        }
        self.remove_empty_lines_after_line_with_token_at(index);
    }

    fn fix_after_case_token(&mut self, index: usize) {
        let enum_switch_index = self
            .tokens
            .get_prev_token_of_kind(index, &[T_SWITCH.into(), T_ENUM.into()], true)
            .unwrap();
        if !self.tokens[enum_switch_index].is_kind(T_SWITCH) {
            return;
        }
        self.remove_empty_lines_after_line_with_token_at(index);
    }

    fn fix_after_throw_token(&mut self, index: usize) {
        let prev_index = self.tokens.get_prev_meaningful_token(index).unwrap();
        if !self.tokens[prev_index].equals_any(
            &[
                ";".into(),
                "{".into(),
                "}".into(),
                ":".into(),
                T_OPEN_TAG.into(),
            ],
            true,
        ) {
            return;
        }
        if self.tokens[prev_index].equals(":", true)
            && !SwitchAnalyzer::belongs_to_switch(self.tokens, prev_index)
        {
            return;
        }
        self.fix_after_token(index);
    }

    /// Remove white line(s) after the index of a block type, but only if the block is not on one line.
    fn fix_structure_open_close_if_multi_line(&mut self, index: usize) {
        let (block_type, _) = Tokens::detect_block_type(&self.tokens[index]).unwrap();
        let body_end = self.tokens.find_block_end(block_type, index);
        let mut i = body_end - 1;
        while i >= index {
            if self.tokens[i].get_content().contains(&b'\n') {
                self.remove_empty_lines_after_line_with_token_at(i);
                self.remove_empty_lines_after_line_with_token_at(index);
                break;
            }
            i -= 1; // index >= 1 (a block opener is never token 0), so this cannot underflow
        }
    }

    fn remove_empty_lines_after_line_with_token_at(&mut self, index: usize) {
        let mut parentheses_depth = 0i64;
        let token_count = self.tokens.count();
        let mut end = index;
        while end < token_count {
            let t = &self.tokens[end];
            if t.equals("(", true) {
                parentheses_depth += 1;
                end += 1;
                continue;
            }
            if t.equals(")", true) {
                parentheses_depth -= 1;
                if parentheses_depth < 0 {
                    return;
                }
                end += 1;
                continue;
            }
            if t.equals("}", true) || t.get_content().contains(&b'\n') {
                break;
            }
            end += 1;
        }
        if end == token_count {
            return;
        }
        let ending = self.fixer.whitespaces_config.get_line_ending();
        let mut i = end;
        while i < token_count && self.tokens[i].is_whitespace(None) {
            let content = self.tokens[i].get_content();
            if content.contains(&b'\n') {
                let new_content = Preg::replace("/^.*\\R(\\h*)$/s", format!("{ending}$1"), content);
                self.tokens.set(i, Token::new(T_WHITESPACE, new_content));
            }
            i += 1;
        }
    }

    fn contains_linebreak(&self, start_index: usize, end_index: usize) -> bool {
        (start_index + 1..=end_index)
            .rev()
            .any(|i| Preg::r#match("/\\R/", self.tokens[i].get_content()))
    }
}

impl Fixer for NoExtraBlankLinesFixer {
    fn name(&self) -> &'static str {
        "no_extra_blank_lines"
    }
    fn priority(&self) -> i32 {
        // Must run before BlankLineBeforeStatementFixer.
        // Must run after ClassAttributesSeparationFixer, CombineConsecutiveUnsetsFixer, EmptyLoopBodyFixer, ...
        -20
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(self.name(), options, &["tokens"])?;
        // "use_trait" only triggers a deprecation upstream.
        let tokens_configuration = r.strings("tokens", &["extra"], AVAILABLE_TOKENS)?;
        let mut token_equals_map = Vec::new();
        if tokens_configuration.iter().any(|t| t == "comma") {
            token_equals_map.push((",", Callback::FixAfterToken));
        }
        if tokens_configuration
            .iter()
            .any(|t| t == "curly_brace_block")
        {
            token_equals_map.push(("{", Callback::FixStructureOpenCloseIfMultiLine)); // i.e. not: CT::T_ARRAY_INDEX_BRACE_OPEN
        }
        if tokens_configuration
            .iter()
            .any(|t| t == "parenthesis_brace_block")
        {
            token_equals_map.push(("(", Callback::FixStructureOpenCloseIfMultiLine)); // i.e. not: CT::T_CLASS_INSTANTIATION_PARENTHESIS_OPEN
        }
        let config_map = |c: &str| -> Option<(Kind, Callback)> {
            Some(match c {
                "attribute" => (CT_ATTRIBUTE_CLOSE, Callback::FixAfterToken),
                "break" => (T_BREAK, Callback::FixAfterToken),
                "case" => (T_CASE, Callback::FixAfterCaseToken),
                "continue" => (T_CONTINUE, Callback::FixAfterToken),
                "default" => (T_DEFAULT, Callback::FixAfterToken),
                "extra" => (T_WHITESPACE, Callback::RemoveMultipleBlankLines),
                "return" => (T_RETURN, Callback::FixAfterToken),
                "square_brace_block" => (
                    CT_ARRAY_BRACKET_OPEN,
                    Callback::FixStructureOpenCloseIfMultiLine,
                ),
                "switch" => (T_SWITCH, Callback::FixAfterToken),
                "throw" => (T_THROW, Callback::FixAfterThrowToken),
                "use" => (T_USE, Callback::RemoveBetweenUse),
                "use_trait" => (CT_USE_TRAIT, Callback::RemoveBetweenUse),
                _ => return None,
            })
        };
        // PHP array assignment: a repeated kind keeps its first position
        let token_kind_callback_map = tokens_configuration.iter().filter_map(|c| config_map(c)).collect();
        self.token_kind_callback_map = token_kind_callback_map;
        self.token_equals_map = token_equals_map;
        Ok(())
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut run = Run {
            fixer: self,
            tokens,
        };
        let mut index = run.tokens.count() - 1;
        while index > 0 {
            run.fix_by_token(index);
            index -= 1;
        }
    }
}
