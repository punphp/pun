//! Port of `PhpCsFixer\Fixer\Casing\IntegerLiteralCaseFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct IntegerLiteralCaseFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(IntegerLiteralCaseFixer)
}

impl Fixer for IntegerLiteralCaseFixer {
    fn name(&self) -> &'static str {
        "integer_literal_case"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_LNUMBER)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.len() {
            if !tokens[index].is_kind(T_LNUMBER) {
                continue;
            }
            let content = &tokens[index].content;
            // Preg::replaceCallback('#^0([boxBOX])([0-9a-fA-F_]+)$#', ...); T_LNUMBER never holds a "\n".
            let matches = content.len() > 2
                && content[0] == b'0'
                && b"boxBOX".contains(&content[1])
                && content[2..]
                    .iter()
                    .all(|c| c.is_ascii_hexdigit() || *c == b'_');
            if !matches {
                continue;
            }
            let new_content = [
                &b"0"[..],
                &[content[1].to_ascii_lowercase()],
                &content[2..].to_ascii_uppercase(),
            ]
            .concat();
            if *content == new_content {
                continue;
            }
            tokens.set(index, Token::new(T_LNUMBER, new_content));
        }
    }
}
