//! Port of `PhpCsFixer\Fixer\Comment\MultilineCommentOpeningClosingFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct MultilineCommentOpeningClosingFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(MultilineCommentOpeningClosingFixer)
}

impl Fixer for MultilineCommentOpeningClosingFixer {
    fn name(&self) -> &'static str {
        "multiline_comment_opening_closing"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_COMMENT, T_DOC_COMMENT])
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            let token = &tokens[index];
            let original_content = token.get_content();
            if !token.is_kind(T_DOC_COMMENT)
                && !(token.is_kind(T_COMMENT) && original_content.starts_with(b"/*"))
            {
                continue;
            }
            let mut new_content = original_content.to_vec();
            // Fix opening
            if token.is_kind(T_COMMENT) {
                new_content = Preg::replace(r"/^\/\*{2,}(?!\/)/", "/*", &new_content);
            }
            // Fix closing
            new_content = Preg::replace(r"/(?<!\/)\*{2,}\/$/", "*/", &new_content);
            if new_content != original_content {
                let id = token.id.unwrap();
                tokens.set(index, Token::new(id, new_content));
            }
        }
    }
}
