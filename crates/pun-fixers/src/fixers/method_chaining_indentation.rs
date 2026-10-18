//! Port of `PhpCsFixer\Fixer\Whitespace\MethodChainingIndentationFixer`.
use crate::analyzers::preg::Preg;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens, token::OBJECT_OPERATOR_KINDS};
use std::path::Path;

#[derive(Default)]
struct MethodChainingIndentationFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(MethodChainingIndentationFixer::default())
}

impl MethodChainingIndentationFixer {
    /// `index`: index of the first token on the line to indent.
    fn get_expected_indent_at(&self, tokens: &Tokens, index: usize) -> Vec<u8> {
        let index = tokens.get_prev_meaningful_token(index).unwrap();
        let indent = self.whitespaces_config.get_indent().as_bytes();
        let mut i = index as isize;
        while i >= 0 {
            let mut iu = i as usize;
            if tokens[iu].equals(")", true) {
                iu = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, iu);
                i = iu as isize;
            }
            if let Some(current_indent) = Self::get_indent_at(tokens, iu) {
                if Self::current_line_requires_extra_indent_level(tokens, iu, index) {
                    return [&current_indent[..], indent].concat();
                }
                return current_indent;
            }
            i -= 1;
        }
        indent.to_vec()
    }

    /// `index`: position of the object operator token ("->" or "?->").
    fn can_be_moved_to_next_line(index: usize, tokens: &Tokens) -> bool {
        let prev_meaningful = tokens.get_prev_meaningful_token(index).unwrap();
        let mut has_comment_before = false;
        let mut i = index - 1;
        while i > prev_meaningful {
            if tokens[i].is_comment() {
                has_comment_before = true;
                i -= 1;
                continue;
            }
            if tokens[i].is_whitespace(None) && Preg::r#match("/\\R/", tokens[i].get_content()) {
                return has_comment_before;
            }
            i -= 1;
        }
        false
    }

    /// `index`: index of the indentation token.
    fn get_indent_at(tokens: &Tokens, index: usize) -> Option<Vec<u8>> {
        let content = Self::get_indent_content_at(tokens, index);
        Preg::match_groups("/\\R{1}(\\h*)$/", &content).map(|m| m.str(1).to_vec())
    }

    fn get_indent_content_at(tokens: &Tokens, index: usize) -> Vec<u8> {
        if !tokens[index].is_given_kind(&[T_WHITESPACE, T_INLINE_HTML]) {
            return Vec::new();
        }
        let mut content = tokens[index].get_content().to_vec();
        if tokens[index].is_whitespace(None) && tokens[index - 1].is_kind(T_OPEN_TAG) {
            content = [tokens[index - 1].get_content(), &content].concat();
        }
        if Preg::r#match("/\\R/", &content) {
            return content;
        }
        Vec::new()
    }

    /// `start`: index of first meaningful token on previous line; `end`: index of last token on previous line.
    fn current_line_requires_extra_indent_level(tokens: &Tokens, start: usize, end: usize) -> bool {
        let first_meaningful = tokens.get_next_meaningful_token(start).unwrap();
        if tokens[first_meaningful].is_object_operator() {
            let third_meaningful = tokens
                .get_next_meaningful_token(
                    tokens.get_next_meaningful_token(first_meaningful).unwrap(),
                )
                .unwrap();
            return tokens[third_meaningful].equals("(", true)
                && tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, third_meaningful) > end;
        }
        !tokens[end].equals(")", true)
            || tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, end) >= start
    }
}

impl Fixer for MethodChainingIndentationFixer {
    fn name(&self) -> &'static str {
        "method_chaining_indentation"
    }
    fn priority(&self) -> i32 {
        // Must run after NoSpaceAroundDoubleColonFixer.
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(OBJECT_OPERATOR_KINDS)
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let line_ending = self.whitespaces_config.get_line_ending().as_bytes();
        let count = tokens.count();
        let mut index = 1;
        while index < count {
            if !tokens[index].is_object_operator() {
                index += 1;
                continue;
            }
            let end_parenthesis_index = tokens.get_next_token_of_kind(
                index,
                &["(".into(), ";".into(), ",".into(), T_CLOSE_TAG.into()],
                true,
            );
            let previous_end_parenthesis_index =
                tokens.get_prev_token_of_kind(index, &[")".into()], true);
            let Some(mut end_parenthesis_index) = end_parenthesis_index else {
                index += 1;
                continue;
            };
            if !tokens[end_parenthesis_index].equals("(", true)
                && previous_end_parenthesis_index.is_none()
            {
                index += 1;
                continue;
            }
            if Self::can_be_moved_to_next_line(index, tokens) {
                let newline = Token::new(T_WHITESPACE, line_ending);
                if tokens[index - 1].is_whitespace(None) {
                    tokens.set(index - 1, newline);
                } else {
                    tokens.insert_at(index, vec![newline]);
                    index += 1;
                    end_parenthesis_index += 1;
                }
            }
            let Some(current_indent) = Self::get_indent_at(tokens, index - 1) else {
                index += 1;
                continue;
            };
            let expected_indent = self.get_expected_indent_at(tokens, index);
            if current_indent != expected_indent {
                tokens.set(
                    index - 1,
                    Token::new(T_WHITESPACE, [line_ending, &expected_indent].concat()),
                );
            }
            if !tokens[end_parenthesis_index].equals("(", true) {
                index += 1;
                continue;
            }
            let end_parenthesis_index =
                tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, end_parenthesis_index);
            let pattern = [b"/(\\R)".as_slice(), &current_indent, b"(\\h*)$/D"].concat();
            let replacement = [b"$1".as_slice(), &expected_indent, b"$2"].concat();
            for search_index in index + 1..end_parenthesis_index {
                let search_token = &tokens[search_index];
                if !search_token.is_whitespace(None) {
                    continue;
                }
                let content = search_token.get_content();
                if !Preg::r#match("/\\R/", content) {
                    continue;
                }
                let content = Preg::replace(&pattern, &replacement, content);
                let t = Token {
                    id: search_token.get_id(),
                    content,
                };
                tokens.set(search_index, t);
            }
            index += 1;
        }
    }
}
