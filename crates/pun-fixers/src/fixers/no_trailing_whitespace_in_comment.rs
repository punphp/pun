//! Port of `PhpCsFixer\Fixer\Comment\NoTrailingWhitespaceInCommentFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{php_compat::ltrim_chars, Kind::*, Token, Tokens};
use std::path::Path;

struct NoTrailingWhitespaceInCommentFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(NoTrailingWhitespaceInCommentFixer)
}

impl Fixer for NoTrailingWhitespaceInCommentFixer {
    fn name(&self) -> &'static str {
        "no_trailing_whitespace_in_comment"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_COMMENT, T_DOC_COMMENT])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut index = 0;
        while index < tokens.count() {
            let token = tokens[index].clone();
            if token.is_kind(T_DOC_COMMENT) {
                tokens.set(index, Token::new(T_DOC_COMMENT, Preg::replace("/(*ANY)[\\h]+$/m", "", &token.content)));
            } else if token.is_kind(T_COMMENT) {
                if token.content.starts_with(b"/*") {
                    tokens.set(index, Token::new(T_COMMENT, Preg::replace("/(*ANY)[\\h]+$/m", "", &token.content)));
                } else if index + 1 < tokens.count() && tokens[index + 1].is_whitespace(None) {
                    let trimmed_content = ltrim_chars(&tokens[index + 1].content, b" \t").to_vec();
                    tokens.ensure_whitespace_at_index(index + 1, 0, &trimmed_content);
                }
            }
            index += 1;
        }
    }
}
