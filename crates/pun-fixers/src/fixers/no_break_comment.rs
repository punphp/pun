//! Port of `PhpCsFixer\Fixer\ControlStructure\NoBreakCommentFixer`.
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::{Preg, preg_quote};
use crate::analyzers::whitespaces_analyzer::WhitespacesAnalyzer;
use anyhow::{Result, bail};
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind, Kind::*, Token, Tokens, TokensAnalyzer};
use serde_json::Value;
use std::path::Path;

const STRUCTURE_KINDS: &[Kind] = &[
    T_FOR, T_FOREACH, T_WHILE, T_IF, T_ELSEIF, T_SWITCH, T_FUNCTION, T_MATCH,
];

#[derive(Default)]
struct NoBreakCommentFixer {
    comment_text: String,
    /// isNoBreakComment's pattern for `comment_text`, built once per configuration.
    comment_pattern: String,
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    crate::configured(NoBreakCommentFixer {
        comment_text: "no break".into(),
        comment_pattern: comment_pattern("no break"),
        whitespaces_config: Default::default(),
    })
}

fn comment_pattern(comment_text: &str) -> String {
    let text = String::from_utf8(preg_quote(comment_text.as_bytes(), Some(b'~'))).unwrap();
    format!("~^((//|#)\\s*{text}\\s*)|(/\\*\\*?\\s*{text}(\\s+.*)*\\*/)$~i")
}

impl Fixer for NoBreakCommentFixer {
    fn name(&self) -> &'static str {
        "no_break_comment"
    }
    /// Must run after NoUselessElseFixer.
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_SWITCH)
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["comment_text"])?;
        let text = o.string("comment_text", "no break", &[])?;
        if Preg::r#match("/\\R/", &text) {
            bail!(
                "[no_break_comment] Invalid configuration: The comment text must not contain new lines."
            );
        }
        self.comment_text = text
            .trim_end_matches([' ', '\t', '\n', '\r', '\0', '\x0B'])
            .into();
        self.comment_pattern = comment_pattern(&self.comment_text);
        Ok(())
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in (0..tokens.len()).rev() {
            if tokens[index].is_kind(T_DEFAULT) {
                if tokens[tokens.get_next_meaningful_token(index).unwrap()].is_kind(T_DOUBLE_ARROW)
                {
                    continue; // this is "default" from "match"
                }
            } else if !tokens[index].is_kind(T_CASE)
                || TokensAnalyzer::new(tokens).is_enum_case(index)
            {
                continue;
            }
            let case_position = tokens
                .get_next_token_of_kind(index, &[":".into(), ";".into()], true)
                .unwrap();
            self.fix_case(tokens, case_position);
        }
    }
}

/// PHP `str_ireplace($text, $text, $subject)`: every ASCII-case-insensitive occurrence becomes `text`.
fn str_ireplace_same(text: &[u8], subject: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(subject.len());
    let mut i = 0;
    while i < subject.len() {
        if subject.len() - i >= text.len() && subject[i..i + text.len()].eq_ignore_ascii_case(text)
        {
            out.extend_from_slice(text);
            i += text.len();
        } else {
            out.push(subject[i]);
            i += 1;
        }
    }
    out
}

impl NoBreakCommentFixer {
    fn fix_case(&self, tokens: &mut Tokens, case_position: usize) {
        let mut empty = true;
        let mut fall_through = true;
        let mut comment_position: Option<usize> = None;
        let max = tokens.len();
        let mut i = case_position + 1;
        while i < max {
            if tokens[i].is_given_kind(STRUCTURE_KINDS)
                || tokens[i].is_given_kind(&[T_ELSE, T_DO, T_CLASS])
            {
                empty = false;
                i = get_structure_end(tokens, i) + 1;
                continue;
            }
            if tokens[i].is_given_kind(&[T_BREAK, T_CONTINUE, T_RETURN, T_EXIT, T_GOTO]) {
                fall_through = false;
                i += 1;
                continue;
            }
            if tokens[i].is_kind(T_THROW) {
                let previous_index = tokens.get_prev_meaningful_token(i).unwrap();
                if previous_index == case_position
                    || tokens[previous_index].equals_any(
                        &["{".into(), ";".into(), "}".into(), T_OPEN_TAG.into()],
                        true,
                    )
                {
                    fall_through = false;
                }
                i += 1;
                continue;
            }
            if tokens[i].equals("}", true) || tokens[i].is_kind(T_ENDSWITCH) {
                if let Some(c) = comment_position {
                    self.remove_comment(tokens, c);
                }
                break;
            }
            if self.is_no_break_comment(&tokens[i]) {
                comment_position = Some(i);
                i += 1;
                continue;
            }
            if tokens[i].is_given_kind(&[T_CASE, T_DEFAULT]) {
                if !empty && fall_through {
                    if let Some(c) = comment_position
                        && tokens.get_prev_non_whitespace(i, None) != Some(c)
                    {
                        self.remove_comment(tokens, c);
                        comment_position = None;
                    }
                    match comment_position {
                        None => self.insert_comment_at(tokens, i),
                        Some(c) => {
                            let content = str_ireplace_same(
                                self.comment_text.as_bytes(),
                                tokens[c].get_content(),
                            );
                            tokens.set(c, Token::new(tokens[c].get_id().unwrap(), content));
                            self.ensure_new_line_at(tokens, c);
                        }
                    }
                } else if let Some(c) = comment_position {
                    self.remove_comment(tokens, c);
                }
                break;
            }
            if !tokens[i].is_given_kind(&[T_COMMENT, T_WHITESPACE]) {
                empty = false;
            }
            i += 1;
        }
    }

    fn is_no_break_comment(&self, token: &Token) -> bool {
        if !token.is_comment() {
            return false;
        }
        Preg::r#match(&self.comment_pattern, token.get_content())
    }

    fn insert_comment_at(&self, tokens: &mut Tokens, case_position: usize) {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        let mut newline_position = self.ensure_new_line_at(tokens, case_position);
        let newline_token = tokens[newline_position].clone();
        let mut nb_newlines = newline_token
            .get_content()
            .windows(line_ending.len())
            .filter(|w| *w == line_ending)
            .count();

        if newline_token.is_kind(T_OPEN_TAG) && Preg::r#match("/\\R/", newline_token.get_content())
        {
            nb_newlines += 1;
        } else if tokens[newline_position - 1].is_kind(T_OPEN_TAG)
            && Preg::r#match("/\\R/", tokens[newline_position - 1].get_content())
        {
            nb_newlines += 1;
            if !Preg::r#match("/\\R/", newline_token.get_content()) {
                let content = [line_ending, newline_token.get_content()].concat();
                tokens.set(
                    newline_position,
                    Token::new(newline_token.get_id().unwrap(), content),
                );
            }
        }

        if nb_newlines > 1 {
            let m =
                Preg::match_groups("/^(.*?)(\\R\\h*)$/s", newline_token.get_content()).unwrap();
            let indent = WhitespacesAnalyzer::detect_indent(tokens, newline_position - 1);
            let content = [m.str(1), line_ending, &indent].concat();
            tokens.set(
                newline_position,
                Token::new(newline_token.get_id().unwrap(), content),
            );
            newline_position += 1;
            tokens.insert_at(newline_position, vec![Token::new(T_WHITESPACE, m.str(2))]);
        }

        let comment = format!("// {}", self.comment_text);
        tokens.insert_at(newline_position, vec![Token::new(T_COMMENT, comment)]);
        self.ensure_new_line_at(tokens, newline_position);
    }

    /// Returns the newline token position.
    fn ensure_new_line_at(&self, tokens: &mut Tokens, position: usize) -> usize {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        let mut content = [
            line_ending,
            &WhitespacesAnalyzer::detect_indent(tokens, position),
        ]
        .concat();
        let whitespace_token = tokens[position - 1].clone();

        if !whitespace_token.is_kind(T_WHITESPACE) {
            if whitespace_token.is_kind(T_OPEN_TAG) {
                content = Preg::replace("/\\R/", "", &content);
                if !Preg::r#match("/\\R/", whitespace_token.get_content()) {
                    let open =
                        Preg::replace("/\\s+$/", line_ending, whitespace_token.get_content());
                    tokens.set(position - 1, Token::new(T_OPEN_TAG, open));
                }
            }
            if !content.is_empty() {
                tokens.insert_at(position, vec![Token::new(T_WHITESPACE, content)]);
                return position;
            }
            return position - 1;
        }

        if tokens[position - 2].is_kind(T_OPEN_TAG)
            && Preg::r#match("/\\R/", tokens[position - 2].get_content())
        {
            content = Preg::replace("/^\\R/", "", &content);
        }
        if !Preg::r#match("/\\R/", whitespace_token.get_content()) {
            tokens.set(position - 1, Token::new(T_WHITESPACE, content));
        }
        position - 1
    }

    fn remove_comment(&self, tokens: &mut Tokens, comment_position: usize) {
        let (whitespace_position, regex) = if tokens[tokens
            .get_prev_non_whitespace(comment_position, None)
            .unwrap()]
        .is_kind(T_OPEN_TAG)
        {
            (comment_position + 1, "/^\\R\\h*/")
        } else {
            (comment_position - 1, "/\\R\\h*$/")
        };
        if tokens[whitespace_position].is_kind(T_WHITESPACE) {
            let content = Preg::replace(regex, "", tokens[whitespace_position].get_content());
            tokens.ensure_whitespace_at_index(whitespace_position, 0, &content);
        }
        tokens.clear_token_and_merge_surrounding_whitespace(comment_position);
    }
}

fn get_structure_end(tokens: &Tokens, mut position: usize) -> usize {
    let initial_token = tokens[position].clone();
    if initial_token.is_given_kind(STRUCTURE_KINDS) {
        let open = tokens
            .get_next_token_of_kind(position, &["(".into()], true)
            .unwrap();
        position = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
    } else if initial_token.is_kind(T_CLASS) {
        let open_parenthesis_position = tokens.get_next_meaningful_token(position).unwrap();
        if tokens[open_parenthesis_position].get_content() == b"(" {
            position =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open_parenthesis_position);
        }
    }

    position = if initial_token.is_kind(T_FUNCTION) {
        tokens
            .get_next_token_of_kind(position, &["{".into()], true)
            .unwrap()
    } else {
        tokens.get_next_meaningful_token(position).unwrap()
    };

    if tokens[position].get_content() != b"{" {
        return tokens
            .get_next_token_of_kind(position, &[";".into()], true)
            .unwrap();
    }

    position = tokens.find_block_end(Tokens::BLOCK_TYPE_BRACE, position);

    if initial_token.is_kind(T_DO) {
        let open = tokens
            .get_next_token_of_kind(position, &["(".into()], true)
            .unwrap();
        position = tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, open);
        return tokens
            .get_next_token_of_kind(position, &[";".into()], true)
            .unwrap();
    }
    position
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // NoBreakCommentFixerTest::provideInvalidConfigurationCases (PHP-CS-Fixer 3.95).
    #[test]
    fn configure_validation() {
        let err = |c: Value| fixer().configure(&c).unwrap_err().to_string();
        for text in ["No\nbreak", "No\r\nbreak", "No\rbreak"] {
            assert_eq!(
                err(json!({ "comment_text": text })),
                "[no_break_comment] Invalid configuration: The comment text must not contain new lines."
            );
        }
        assert_eq!(
            err(json!({"foo": true})),
            "[no_break_comment] Invalid configuration: The option \"foo\" does not exist. Defined options are: \"comment_text\"."
        );
    }
}
