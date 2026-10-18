//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocTrimFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::{Kind, Token, Tokens};
use std::path::Path;

struct PhpdocTrimFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocTrimFixer)
}

impl Fixer for PhpdocTrimFixer {
    fn name(&self) -> &'static str {
        "phpdoc_trim"
    }
    fn priority(&self) -> i32 {
        -5
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(Kind::T_DOC_COMMENT) {
                continue;
            }
            let content = fix_start(tokens[index].get_content());
            let content = fix_end(&content);
            tokens.set(index, Token::new(Kind::T_DOC_COMMENT, content));
        }
    }
}

fn fix_start(content: &[u8]) -> Vec<u8> {
    Preg::replace(
        r"~
                (^/\*\*)            # DocComment begin
                (?:
                    \R\h*(?:\*\h*)? # lines without useful content
                    (?!\R\h*\*/)    # not followed by a DocComment end
                )+
                (\R\h*(?:\*\h*)?\S) # first line with useful content
            ~x",
        "$1$2",
        content,
    )
}

fn fix_end(content: &[u8]) -> Vec<u8> {
    Preg::replace(
        r"~
                (\R\h*(?:\*\h*)?\S.*?) # last line with useful content
                (?:
                    (?<!/\*\*)         # not preceded by a DocComment start
                    \R\h*(?:\*\h*)?    # lines without useful content
                )+
                (\R\h*\*/$)            # DocComment end
            ~xu",
        "$1$2",
        content,
    )
}
