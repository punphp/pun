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
            if !tokens[index].is_comment() {
                index += 1;
                continue;
            }
            let token = tokens[index].clone();
            if token.is_kind(T_DOC_COMMENT) {
                if !may_have_trailing_h(&token.content) {
                    index += 1;
                    continue;
                }
                tokens.set(index, Token::new(T_DOC_COMMENT, Preg::replace("/(*ANY)[\\h]+$/m", "", &token.content)));
            } else {
                if token.content.starts_with(b"/*") {
                    if !may_have_trailing_h(&token.content) {
                        index += 1;
                        continue;
                    }
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

/// `/(*ANY)[\h]+$/m` can only match before a line break or at the end; for ASCII text that means a space or tab
/// followed by CR, LF, VT, FF or the end. Non-ASCII text (Unicode `\h` and line breaks) always takes the regex.
fn may_have_trailing_h(c: &[u8]) -> bool {
    let h = |b: &u8| matches!(b, b' ' | b'\t');
    !c.is_ascii() || c.last().is_some_and(h) || c.windows(2).any(|w| h(&w[0]) && matches!(w[1], b'\n' | b'\r' | 0x0b | 0x0c))
}
