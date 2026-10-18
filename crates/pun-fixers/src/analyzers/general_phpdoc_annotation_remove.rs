//! Port of `PhpCsFixer\Fixer\Phpdoc\GeneralPhpdocAnnotationRemoveFixer`, the proxy target of
//! phpdoc_no_access / phpdoc_no_package (AbstractProxyFixer with a single proxied fixer). Both configure it with
//! `case_sensitive => true` and a non-empty `annotations`, and only run it on candidates, so only that path is ported.
use crate::analyzers::docblock::DocBlock;
use pun_tokens::{Kind, Token, Tokens};

pub(crate) struct GeneralPhpdocAnnotationRemove {
    pub annotations: &'static [&'static str],
}

impl GeneralPhpdocAnnotationRemove {
    pub(crate) const PRIORITY: i32 = 10;

    pub(crate) fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
    }

    pub(crate) fn fix(&self, tokens: &mut Tokens) {
        for index in 0..tokens.count() {
            if !tokens[index].is_kind(Kind::T_DOC_COMMENT) {
                continue;
            }
            let doc = DocBlock::new(tokens[index].get_content());
            let annotations = doc.get_annotations_of_type(self.annotations);
            if annotations.is_empty() {
                continue;
            }
            for a in annotations {
                a.remove();
            }
            let content = doc.get_content();
            if content.is_empty() {
                tokens.clear_token_and_merge_surrounding_whitespace(index);
            } else {
                tokens.set(index, Token::new(Kind::T_DOC_COMMENT, content));
            }
        }
    }
}
