//! Port of `PhpCsFixer\Fixer\FunctionNotation\SingleLineThrowFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use std::path::Path;

struct SingleLineThrowFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(SingleLineThrowFixer)
}

/// REMOVE_WHITESPACE_AFTER_TOKENS + REMOVE_WHITESPACE_AROUND_TOKENS.
const PREVIOUS_TOKENS_TO_CLEAR: [Proto<'static>; 3] = [
    Proto::Char(b"["),
    Proto::Char(b"("),
    Proto::Kind(T_DOUBLE_COLON),
];
/// REMOVE_WHITESPACE_AROUND_TOKENS + REMOVE_WHITESPACE_BEFORE_TOKENS.
const NEXT_TOKENS_TO_CLEAR: [Proto<'static>; 6] = [
    Proto::Char(b"("),
    Proto::Kind(T_DOUBLE_COLON),
    Proto::Char(b")"),
    Proto::Char(b"]"),
    Proto::Char(b","),
    Proto::Char(b";"),
];
const END_TOKENS: [Proto<'static>; 5] = [
    Proto::Char(b")"),
    Proto::Char(b"]"),
    Proto::Char(b","),
    Proto::Char(b";"),
    Proto::Kind(T_CLOSE_TAG),
];

impl Fixer for SingleLineThrowFixer {
    fn name(&self) -> &'static str {
        "single_line_throw"
    }
    fn priority(&self) -> i32 {
        36
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_THROW)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_THROW) {
                continue;
            }
            let mut end_candidate_index = tokens.get_next_meaningful_token(index).unwrap();
            while !tokens[end_candidate_index].equals_any(&END_TOKENS, true) {
                if let Some((block_type, is_start)) =
                    Tokens::detect_block_type(&tokens[end_candidate_index])
                {
                    if block_type == Tokens::BLOCK_TYPE_BRACE || !is_start {
                        break;
                    }
                    end_candidate_index = tokens.find_block_end(block_type, end_candidate_index);
                }
                end_candidate_index = tokens
                    .get_next_meaningful_token(end_candidate_index)
                    .unwrap();
            }
            let end = tokens
                .get_prev_meaningful_token(end_candidate_index)
                .unwrap();
            trim_new_lines(tokens, index, end);
        }
    }
}

fn trim_new_lines(tokens: &mut Tokens, start_index: usize, end_index: usize) {
    for index in start_index..end_index {
        let mut content = tokens[index].get_content().to_vec();
        if tokens[index].is_kind(T_COMMENT) {
            if let Some(rest) = content.strip_prefix(b"//") {
                content = [b"/*", rest, b" */"].concat();
                tokens.clear_at(index + 1);
            } else if let Some(rest) = content.strip_prefix(b"#") {
                content = [b"/*", rest, b" */"].concat();
                tokens.clear_at(index + 1);
            } else if Preg::r#match(r"/\R/", &content) {
                content = Preg::replace(r"/\R/", " ", &content);
            }
            tokens.set(index, Token::new(T_COMMENT, content));
            continue;
        }
        if !tokens[index].is_kind(T_WHITESPACE) {
            continue;
        }
        if !Preg::r#match(r"/\R/", &content) {
            continue;
        }
        let prev_index = tokens.get_non_empty_sibling(index, -1).unwrap();
        if is_previous_token_to_clear(&tokens[prev_index]) {
            tokens.clear_at(index);
            continue;
        }
        let next_index = tokens.get_non_empty_sibling(index, 1).unwrap();
        if is_next_token_to_clear(&tokens[next_index]) && !tokens[prev_index].is_kind(T_FUNCTION) {
            tokens.clear_at(index);
            continue;
        }
        tokens.set(index, Token::new(T_WHITESPACE, " "));
    }
}

fn is_previous_token_to_clear(token: &Token) -> bool {
    token.equals_any(&PREVIOUS_TOKENS_TO_CLEAR, true) || token.is_object_operator()
}

fn is_next_token_to_clear(token: &Token) -> bool {
    token.equals_any(&NEXT_TOKENS_TO_CLEAR, true) || token.is_object_operator()
}
