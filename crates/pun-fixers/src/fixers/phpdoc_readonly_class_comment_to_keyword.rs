//! Port of `PhpCsFixer\Fixer\ClassNotation\PhpdocReadonlyClassCommentToKeywordFixer`.
use crate::analyzers::docblock::DocBlock;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct PhpdocReadonlyClassCommentToKeywordFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpdocReadonlyClassCommentToKeywordFixer)
}

impl Fixer for PhpdocReadonlyClassCommentToKeywordFixer {
    fn name(&self) -> &'static str {
        "phpdoc_readonly_class_comment_to_keyword"
    }
    fn priority(&self) -> i32 {
        4
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        // `\PHP_VERSION_ID >= 8_02_00`: always true on the PHP 8.5 oracle.
        tokens.is_token_kind_found(T_DOC_COMMENT)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        // `foreach ($tokens as $index => $token)` walks the collection as it grows.
        let mut main_index = 0;
        while main_index < tokens.count() {
            let current = main_index;
            main_index += 1;
            if !tokens[current].is_kind(T_DOC_COMMENT) {
                continue;
            }
            let doc = DocBlock::new(&tokens[current].content);
            let annotations = doc.get_annotations_of_type(&["readonly"]);
            if annotations.is_empty() {
                continue;
            }
            for annotation in annotations {
                annotation.remove();
            }
            let mut index = tokens.get_next_meaningful_token(current).unwrap();
            let mut add_readonly = true;
            while tokens[index].is_given_kind(&[
                T_ABSTRACT,
                T_FINAL,
                T_PRIVATE,
                T_PUBLIC,
                T_PROTECTED,
                T_READONLY,
            ]) {
                if tokens[index].is_kind(T_READONLY) {
                    add_readonly = false;
                }
                index = tokens.get_next_meaningful_token(index).unwrap();
            }
            if !tokens[index].is_kind(T_CLASS) {
                continue;
            }
            if add_readonly {
                tokens.insert_at(
                    index,
                    vec![
                        Token::new(T_READONLY, "readonly"),
                        Token::new(T_WHITESPACE, " "),
                    ],
                );
            }
            let new_content = doc.get_content();
            if new_content.is_empty() {
                tokens.clear_token_and_merge_surrounding_whitespace(current);
                continue;
            }
            tokens.set(current, Token::new(T_DOC_COMMENT, new_content));
        }
    }
}
