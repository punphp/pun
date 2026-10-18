//! Port of `PhpCsFixer\Fixer\Whitespace\LineEndingFixer`.
use crate::analyzers::preg::Preg;
use pun_format::{Fixer, WhitespacesFixerConfig};
use pun_tokens::{Kind::*, Token, Tokens};
use std::path::Path;

#[derive(Default)]
struct LineEndingFixer {
    whitespaces_config: WhitespacesFixerConfig,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(LineEndingFixer::default())
}

impl Fixer for LineEndingFixer {
    fn name(&self) -> &'static str {
        "line_ending"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, _tokens: &Tokens) -> bool {
        true
    }
    fn set_whitespaces_config(&mut self, config: &WhitespacesFixerConfig) {
        self.whitespaces_config = config.clone();
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let ending = self.whitespaces_config.get_line_ending();
        // `\R` other than "\n" needs one of these bytes (\r, \v, \f, NEL as \x85 or \xC2\x85, U+2028/9 as \xE2..);
        // without them replacing `\R` by "\n" is the identity.
        let only_lf = |c: &[u8]| {
            ending == "\n"
                && !c
                    .iter()
                    .any(|b| matches!(b, b'\r' | 0x0b | 0x0c | 0x85 | 0xE2))
        };
        for index in 0..tokens.count() {
            let token = &tokens[index];
            if only_lf(&token.content) {
                continue;
            }
            if token.is_kind(T_ENCAPSED_AND_WHITESPACE) {
                if tokens[tokens.get_next_meaningful_token(index).unwrap()].is_kind(T_END_HEREDOC) {
                    let t = Token {
                        id: token.get_id(),
                        content: Preg::replace("#\\R#", ending, token.get_content()),
                    };
                    tokens.set(index, t);
                }
                continue;
            }
            if token.is_given_kind(&[
                T_CLOSE_TAG,
                T_COMMENT,
                T_DOC_COMMENT,
                T_OPEN_TAG,
                T_START_HEREDOC,
                T_WHITESPACE,
            ]) {
                let t = Token {
                    id: token.get_id(),
                    content: Preg::replace("#\\R#", ending, token.get_content()),
                };
                tokens.set(index, t);
            }
        }
    }
}
