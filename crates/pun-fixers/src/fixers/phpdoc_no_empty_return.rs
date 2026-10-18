//! Port of `PhpCsFixer\Fixer\Phpdoc\PhpdocNoEmptyReturnFixer`.
use crate::analyzers::docblock::DocBlock;
use pun_format::Fixer;
use pun_tokens::php_compat::strtolower;
use pun_tokens::{Kind, Token, Tokens};
use std::path::Path;

struct PhpdocNoEmptyReturnFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocNoEmptyReturnFixer)
}

impl Fixer for PhpdocNoEmptyReturnFixer {
    fn name(&self) -> &'static str {
        "phpdoc_no_empty_return"
    }
    fn priority(&self) -> i32 {
        4
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(Kind::T_DOC_COMMENT) {
                continue;
            }
            let doc = DocBlock::new(tokens[index].get_content());
            let annotations = doc.get_annotations_of_type(&["return"]);
            if annotations.is_empty() {
                continue;
            }
            for annotation in annotations {
                // fixAnnotation: getNormalizedTypes() is the lowercased types; one of them, `null` or `void`.
                let types = annotation.get_types();
                if types.len() == 1 && matches!(&strtolower(&types[0])[..], b"null" | b"void") {
                    annotation.remove();
                }
            }
            let content = doc.get_content();
            if content == tokens[index].get_content() {
                continue;
            }
            if content.is_empty() {
                tokens.clear_token_and_merge_surrounding_whitespace(index);
                continue;
            }
            tokens.set(index, Token::new(Kind::T_DOC_COMMENT, content));
        }
    }
}
