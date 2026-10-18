//! Port of `PhpCsFixer\Fixer\FunctionNotation\MethodArgumentSpaceFixer`.
use crate::analyzers::arguments_analyzer::ArgumentsAnalyzer;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::Result;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens, php_compat::ltrim_chars};
use serde_json::Value;
use std::path::Path;

struct MethodArgumentSpaceFixer {
    keep_multiple_spaces_after_comma: bool,
    on_multiline: String,
    after_heredoc: bool,
    attribute_placement: String,
    whitespaces_config: WhitespacesFixerConfig,
}

pub fn fixer() -> Box<dyn Fixer> {
    let mut f = MethodArgumentSpaceFixer {
        keep_multiple_spaces_after_comma: false,
        on_multiline: String::new(),
        after_heredoc: false,
        attribute_placement: String::new(),
        whitespaces_config: Default::default(),
    };
    f.configure(&Value::Null).unwrap();
    Box::new(f)
}

fn is_newline(token: &Token) -> bool {
    token.is_whitespace(None) && token.get_content().contains(&b'\n')
}

impl MethodArgumentSpaceFixer {
    /// Fix arguments spacing for given function; returns whether the function is multiline.
    fn fix_function(&self, tokens: &mut Tokens, start_function_index: usize) -> bool {
        let mut is_multiline = false;
        let end_function_index =
            tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, start_function_index);
        let first_whitespace_index = Self::find_whitespace_index_after_parenthesis(
            tokens,
            start_function_index,
            end_function_index,
        );
        let last_whitespace_index = Self::find_whitespace_index_after_parenthesis(
            tokens,
            end_function_index,
            start_function_index,
        );
        for index in [first_whitespace_index, last_whitespace_index] {
            let Some(index) = index else { continue };
            if !Preg::r#match("/\\R/", tokens[index].get_content()) {
                continue;
            }
            if self.on_multiline != "ensure_single_line" {
                is_multiline = true;
                continue;
            }
            let new_lines_removed = Self::ensure_single_line(tokens, index);
            if !new_lines_removed {
                is_multiline = true;
            }
        }
        let mut index = end_function_index - 1;
        while index > start_function_index {
            let token = &tokens[index];
            if token.equals(")", true) {
                index = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, index);
            } else if token.is_kind(CT_ARRAY_BRACKET_CLOSE) {
                index = tokens.find_block_start(Tokens::BLOCK_TYPE_ARRAY_BRACKET, index);
            } else if token.equals("}", true) {
                index = tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, index);
            } else if token.equals(",", true) {
                self.fix_space(tokens, index);
                if !is_multiline && is_newline(&tokens[index + 1]) {
                    is_multiline = true;
                }
            }
            index -= 1;
        }
        is_multiline
    }

    fn find_whitespace_index_after_parenthesis(
        tokens: &Tokens,
        start_parenthesis_index: usize,
        end_parenthesis_index: usize,
    ) -> Option<usize> {
        let direction: isize = if end_parenthesis_index > start_parenthesis_index {
            1
        } else {
            -1
        };
        let start_index = start_parenthesis_index.wrapping_add_signed(direction);
        let end_index = end_parenthesis_index.wrapping_add_signed(-direction);
        let mut index = start_index;
        while index != end_index {
            let token = &tokens[index];
            if token.is_whitespace(None) {
                return Some(index);
            }
            if !token.is_comment() {
                break;
            }
            index = index.wrapping_add_signed(direction);
        }
        None
    }

    /// Whether newlines were removed from the whitespace token.
    fn ensure_single_line(tokens: &mut Tokens, index: usize) -> bool {
        let previous_token = &tokens[index - 1];
        if previous_token.is_comment() && !previous_token.get_content().starts_with(b"/*") {
            return false;
        }
        let content = Preg::replace("/\\R\\h*/", "", tokens[index].get_content());
        tokens.ensure_whitespace_at_index(index, 0, &content);
        true
    }

    fn ensure_function_fully_multiline(&self, tokens: &mut Tokens, start_function_index: usize) {
        let mut search_index = start_function_index;
        let mut prev_whitespace_token_index;
        loop {
            prev_whitespace_token_index = tokens.get_prev_token_of_kind(
                search_index,
                &[
                    T_ENCAPSED_AND_WHITESPACE.into(),
                    T_INLINE_HTML.into(),
                    T_WHITESPACE.into(),
                ],
                true,
            );
            match prev_whitespace_token_index {
                Some(p) if !tokens[p].get_content().contains(&b'\n') => search_index = p,
                _ => break,
            }
        }
        let existing_indentation: Vec<u8> = match prev_whitespace_token_index {
            None => Vec::new(),
            Some(p) if !tokens[p].is_kind(T_WHITESPACE) => return,
            Some(p) => {
                // the search loop only stops on a token containing "\n" (upstream's `false === strrpos` arm is dead)
                let c = tokens[p].get_content();
                c[c.iter().rposition(|&b| b == b'\n').unwrap() + 1..].to_vec()
            }
        };
        let indentation = [
            &existing_indentation[..],
            self.whitespaces_config.get_indent().as_bytes(),
        ]
        .concat();
        let mut end_function_index =
            tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, start_function_index);
        let at = if tokens[end_function_index - 1].is_whitespace(None) {
            end_function_index - 1
        } else {
            end_function_index
        };
        let ws = [
            self.whitespaces_config.get_line_ending().as_bytes(),
            &existing_indentation,
        ]
        .concat();
        if tokens.ensure_whitespace_at_index(at, 0, &ws) {
            end_function_index += 1;
        }
        let mut index = end_function_index - 1;
        while index > start_function_index {
            let token = &tokens[index];
            if token.equals(")", true) {
                index = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, index);
                index -= 1;
                continue;
            }
            if token.is_kind(CT_ARRAY_BRACKET_CLOSE) {
                index = tokens.find_block_start(Tokens::BLOCK_TYPE_ARRAY_BRACKET, index);
                index -= 1;
                continue;
            }
            if token.equals("}", true) {
                index = tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, index);
                index -= 1;
                continue;
            }
            if tokens[tokens.get_next_meaningful_token(index).unwrap()].equals(")", true) {
                index -= 1;
                continue;
            }
            if token.is_kind(CT_ATTRIBUTE_CLOSE) {
                if self.attribute_placement == "standalone" {
                    self.fix_newline(tokens, index, &indentation, true);
                } else if self.attribute_placement == "same_line" {
                    Self::ensure_single_line(tokens, index + 1);
                    tokens.ensure_whitespace_at_index(index + 1, 0, b" ");
                }
                index = tokens.find_block_start(Tokens::BLOCK_TYPE_ATTRIBUTE, index);
                index -= 1;
                continue;
            }
            if token.equals(",", true) {
                self.fix_newline(tokens, index, &indentation, true);
            }
            index -= 1;
        }
        self.fix_newline(tokens, start_function_index, &indentation, false);
    }

    fn fix_newline(&self, tokens: &mut Tokens, index: usize, indentation: &[u8], _override: bool) {
        if tokens[index + 1].is_comment() {
            return;
        }
        let ws = [
            self.whitespaces_config.get_line_ending().as_bytes(),
            indentation,
        ]
        .concat();
        if tokens[index + 2].is_comment() {
            let next_meaningful_token_index = tokens.get_next_meaningful_token(index + 2).unwrap();
            if !is_newline(&tokens[next_meaningful_token_index - 1]) {
                if tokens[next_meaningful_token_index - 1].is_whitespace(None) {
                    tokens.clear_at(next_meaningful_token_index - 1);
                }
                tokens.ensure_whitespace_at_index(next_meaningful_token_index, 0, &ws);
            }
            return;
        }
        // (upstream returns here when the next meaningful token is `)`; never true: commas before `)` are skipped by
        // the caller, an attribute is followed by its parameter, and `(` only with at least one argument)
        tokens.ensure_whitespace_at_index(index + 1, 0, &ws);
    }

    fn fix_space(&self, tokens: &mut Tokens, index: usize) {
        if tokens[index - 1].is_whitespace(None) {
            let prev_index = tokens.get_prev_non_whitespace(index - 1, None).unwrap();
            if !tokens[prev_index].equals(",", true)
                && !tokens[prev_index].is_comment()
                && (self.after_heredoc || !tokens[prev_index].is_kind(T_END_HEREDOC))
            {
                tokens.clear_at(index - 1);
            }
        }
        let next_index = index + 1;
        let next_token = &tokens[next_index];
        // Two cases for fix space after comma (exclude multiline comments)
        //  1) multiple spaces after comma
        //  2) no space after comma
        if next_token.is_whitespace(None) {
            let mut new_content = next_token.get_content().to_vec();
            if self.on_multiline == "ensure_single_line" {
                new_content = Preg::replace("/\\R/", "", &new_content);
            }
            if (!self.keep_multiple_spaces_after_comma || Preg::r#match("/\\R/", &new_content))
                && !Self::is_comment_last_line_token(tokens, index + 2)
            {
                new_content = ltrim_chars(&new_content, b" \t").to_vec();
            }
            let t = Token::new(
                T_WHITESPACE,
                if new_content.is_empty() {
                    b" ".to_vec()
                } else {
                    new_content
                },
            );
            tokens.set(next_index, t);
            return;
        }
        if !Self::is_comment_last_line_token(tokens, index + 1) {
            tokens.insert_at(index + 1, vec![Token::new(T_WHITESPACE, " ")]);
        }
    }

    fn is_comment_last_line_token(tokens: &Tokens, index: usize) -> bool {
        if !tokens[index].is_comment() || !tokens[index + 1].is_whitespace(None) {
            return false;
        }
        let content = tokens[index + 1].get_content();
        content != ltrim_chars(content, b"\r\n")
    }

    fn argument_content_is_multiline(
        tokens: &Tokens,
        open_parenthesis: usize,
        close_parenthesis: usize,
    ) -> bool {
        let mut index = open_parenthesis + 1;
        while index < close_parenthesis {
            if tokens[index].is_comment() {
                return true;
            }
            if index == open_parenthesis + 1 || index == close_parenthesis - 1 {
                index += 1;
                continue;
            }
            if tokens[index].equals("(", true) {
                index = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index) + 1;
                continue;
            }
            if tokens[index].is_kind(CT_ARRAY_BRACKET_OPEN) {
                index = tokens.find_block_end(Tokens::BLOCK_TYPE_ARRAY_BRACKET, index) + 1;
                continue;
            }
            if tokens[index].equals("{", true) {
                index = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, index) + 1;
                continue;
            }
            if is_newline(&tokens[index]) {
                return true;
            }
            index += 1;
        }
        false
    }

    fn ensure_single_line_for_parentheses(
        tokens: &mut Tokens,
        open_parenthesis: usize,
        close_parenthesis: usize,
    ) {
        if Self::argument_content_is_multiline(tokens, open_parenthesis, close_parenthesis) {
            return;
        }
        let mut index = close_parenthesis - 1;
        while index > open_parenthesis {
            if tokens[index].equals(")", true) {
                index = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, index);
            } else if tokens[index].is_kind(CT_ARRAY_BRACKET_CLOSE) {
                index = tokens.find_block_start(Tokens::BLOCK_TYPE_ARRAY_BRACKET, index);
            } else if tokens[index].equals("}", true) {
                index = tokens.find_block_start(Tokens::BLOCK_TYPE_BRACE, index);
            } else if tokens[index].is_whitespace(None) {
                let previous_token = &tokens[index - 1];
                if !(previous_token.is_comment()
                    && !previous_token.get_content().starts_with(b"/*"))
                {
                    let content = Preg::replace("/\\R\\h*/", "", tokens[index].get_content());
                    tokens.ensure_whitespace_at_index(index, 0, &content);
                }
            }
            index -= 1;
        }
    }
}

impl Fixer for MethodArgumentSpaceFixer {
    fn name(&self) -> &'static str {
        "method_argument_space"
    }
    fn priority(&self) -> i32 {
        // Must run before ArrayIndentationFixer, NoTrailingCommaInSinglelineFixer, StatementIndentationFixer.
        // Must run after CombineNestedDirnameFixer, FunctionDeclarationFixer, ImplodeCallFixer, LambdaNotUsedImportFixer,
        // NoMultilineWhitespaceAroundDoubleArrowFixer, NoUselessSprintfFixer, PowToExponentiationFixer, StrictParamFixer.
        30
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let r = resolve(
            self.name(),
            options,
            &[
                "keep_multiple_spaces_after_comma",
                "on_multiline",
                "after_heredoc",
                "attribute_placement",
            ],
        )?;
        self.keep_multiple_spaces_after_comma =
            r.bool("keep_multiple_spaces_after_comma", false)?;
        self.on_multiline = r.string(
            "on_multiline",
            "ensure_fully_multiline",
            &[
                "ignore",
                "ensure_single_line",
                "ensure_fully_multiline",
                "ensure_single_line_for_single_argument",
            ],
        )?;
        // Future::getV4OrV3(true, false)
        self.after_heredoc = r.bool("after_heredoc", false)?;
        self.attribute_placement = r.string(
            "attribute_placement",
            "standalone",
            &["ignore", "same_line", "standalone"],
        )?;
        Ok(())
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found("(")
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let arguments_analyzer = ArgumentsAnalyzer;
        let expected_tokens = [T_LIST, T_FUNCTION, CT_USE_LAMBDA, T_FN, T_CLASS];
        let token_count = tokens.count();
        for index in 1..token_count {
            if !tokens[index].equals("(", true) {
                continue;
            }
            let meaningful_token_before_parenthesis =
                tokens[tokens.get_prev_meaningful_token(index).unwrap()].clone();
            if meaningful_token_before_parenthesis.is_keyword()
                && !meaningful_token_before_parenthesis.is_given_kind(&expected_tokens)
            {
                continue;
            }
            let is_multiline = self.fix_function(tokens, index);
            if is_multiline
                && (self.on_multiline == "ensure_fully_multiline"
                    || self.on_multiline == "ensure_single_line_for_single_argument")
                && !meaningful_token_before_parenthesis.is_kind(T_LIST)
            {
                if self.on_multiline == "ensure_single_line_for_single_argument" {
                    let end_function_index =
                        tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index);
                    if arguments_analyzer.count_arguments(tokens, index, end_function_index) == 1 {
                        Self::ensure_single_line_for_parentheses(tokens, index, end_function_index);
                    } else {
                        self.ensure_function_fully_multiline(tokens, index);
                    }
                } else {
                    self.ensure_function_fully_multiline(tokens, index);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Both rejected by the real fixer's configure() ("... is invalid. Accepted values are: ...").
    #[test]
    fn configure_rejects_unknown_values() {
        assert!(fixer().configure(&json!({"on_multiline": "foo"})).is_err());
        assert!(
            fixer()
                .configure(&json!({"attribute_placement": "x"}))
                .is_err()
        );
    }
}
