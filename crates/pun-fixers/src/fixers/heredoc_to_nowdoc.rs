//! Port of `PhpCsFixer\Fixer\StringNotation\HeredocToNowdocFixer`.
use pun_tokens::php_compat::str_replace;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{Kind, Token, Tokens};
use std::path::Path;

struct HeredocToNowdocFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
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
            if !token.is_kind(Kind::T_START_HEREDOC) || token.get_content().contains(&b'\'') {
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
            let content = tokens[index + 1].get_content().to_vec();
            if Preg::r#match(r"/(?<!\\)(?:\\{2})*\\(?![$\\])/", &content) {
                continue;
            }
            let t = convert_to_nowdoc(&tokens[index]);
            tokens.set(index, t);
            let content = str_replace(br"\$", b"$", &str_replace(br"\\", br"\", &content));
            let id = tokens[index + 1].get_id().unwrap();
            tokens.set(index + 1, Token::new(id, content));
        }
    }
}

fn convert_to_nowdoc(token: &Token) -> Token {
    Token::new(
        token.get_id().unwrap(),
        Preg::replace(
            r#"/^([Bb]?<<<)(\h*)"?([^\s"]+)"?/"#,
            "$1$2'$3'",
            token.get_content(),
        ),
    )
}
