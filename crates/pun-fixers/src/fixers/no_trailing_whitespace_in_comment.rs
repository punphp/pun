//! Port of `PhpCsFixer\Fixer\Comment\NoTrailingWhitespaceInCommentFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{Kind::*, Token, Tokens, php_compat::ltrim_chars};
use std::path::Path;

struct NoTrailingWhitespaceInCommentFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
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
            let token = &tokens[index];
            // A doc comment or a `/* */` comment keeps its kind; a `//` or `#` comment trims the next whitespace.
            let block_kind = if token.is_kind(T_DOC_COMMENT) {
                Some(T_DOC_COMMENT)
            } else {
                token.get_content().starts_with(b"/*").then_some(T_COMMENT)
            };
            if let Some(kind) = block_kind {
                if may_have_trailing_h(token.get_content()) {
                    let content = Preg::replace("/(*ANY)[\\h]+$/m", "", token.get_content());
                    tokens.set(index, Token::new(kind, content));
                }
            } else if index + 1 < tokens.count() && tokens[index + 1].is_whitespace(None) {
                let trimmed_content = ltrim_chars(tokens[index + 1].get_content(), b" \t").to_vec();
                tokens.ensure_whitespace_at_index(index + 1, 0, &trimmed_content);
            }
            index += 1;
        }
    }
}

/// `/(*ANY)[\h]+$/m` can only match before a line break or at the end; for ASCII text that means a space or tab
/// followed by CR, LF, VT, FF or the end. Non-ASCII text (Unicode `\h` and line breaks) always takes the regex.
fn may_have_trailing_h(c: &[u8]) -> bool {
    let h = |b: &u8| matches!(b, b' ' | b'\t');
    !c.is_ascii()
        || c.last().is_some_and(h)
        || c.windows(2)
            .any(|w| h(&w[0]) && matches!(w[1], b'\n' | b'\r' | 0x0b | 0x0c))
}
