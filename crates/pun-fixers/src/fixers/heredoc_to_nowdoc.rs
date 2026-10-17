//! Port of `PhpCsFixer\Fixer\StringNotation\HeredocToNowdocFixer`.
use crate::analyzers::docblock::replace_all;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{Kind, Token, Tokens};
use std::path::Path;

struct HeredocToNowdocFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(HeredocToNowdocFixer)
}

impl Fixer for HeredocToNowdocFixer {
    fn name(&self) -> &'static str {
        "heredoc_to_nowdoc"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_START_HEREDOC)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            let token = &tokens[index];
            if !token.is_kind(Kind::T_START_HEREDOC) || token.content.contains(&b'\'') {
                continue;
            }
            if tokens[index + 1].is_kind(Kind::T_END_HEREDOC) {
                let t = convert_to_nowdoc(&tokens[index]);
                tokens.set(index, t);
                continue;
            }
            if !tokens[index + 1].is_kind(Kind::T_ENCAPSED_AND_WHITESPACE)
                || !tokens[index + 2].is_kind(Kind::T_END_HEREDOC)
            {
                continue;
            }
            let content = tokens[index + 1].content.clone();
            if Preg::r#match(r"/(?<!\\)(?:\\{2})*\\(?![$\\])/", &content) {
                continue;
            }
            let t = convert_to_nowdoc(&tokens[index]);
            tokens.set(index, t);
            let content = replace_all(&replace_all(&content, br"\\", br"\"), br"\$", b"$");
            let id = tokens[index + 1].id.unwrap();
            tokens.set(index + 1, Token::new(id, content));
        }
    }
}

fn convert_to_nowdoc(token: &Token) -> Token {
    Token::new(
        token.id.unwrap(),
        Preg::replace(
            r#"/^([Bb]?<<<)(\h*)"?([^\s"]+)"?/"#,
            "$1$2'$3'",
            &token.content,
        ),
    )
}
