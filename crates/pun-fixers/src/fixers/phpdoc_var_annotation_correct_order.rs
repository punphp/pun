//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocVarAnnotationCorrectOrderFixer`.
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct PhpdocVarAnnotationCorrectOrderFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocVarAnnotationCorrectOrderFixer)
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

impl Fixer for PhpdocVarAnnotationCorrectOrderFixer {
    fn name(&self) -> &'static str {
        "phpdoc_var_annotation_correct_order"
    }
    fn priority(&self) -> i32 {
        // Must run before PhpdocAlignFixer.
        // Must run after AlignMultilineCommentFixer, CommentToPhpdocFixer, PhpdocIndentFixer, PhpdocScalarFixer,
        // PhpdocToCommentFixer, PhpdocTypesFixer.
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(T_DOC_COMMENT) {
                continue;
            }
            let content = tokens[index].get_content();
            let lower = strtolower(content);
            if !contains(&lower, b"@var") && !contains(&lower, b"@type") {
                continue;
            }
            let new_content = Preg::replace(
                r"/(@(?:type|var)\s*)(\$\S+)(\h+)([^\$](?:[^<\s]|<[^>]*>)*)(\s|\*)/i",
                "$1$4$3$2$5",
                content,
            );
            if new_content == content {
                continue;
            }
            tokens.set(index, Token::new(T_DOC_COMMENT, new_content));
        }
    }
}
