//! Port of `PhpCsFixer\Fixer\PhpUnit\PhpUnitFqcnAnnotationFixer`.
use crate::analyzers::abstract_php_unit_fixer as base;
use crate::analyzers::preg::Preg;
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::path::Path;

struct PhpUnitFqcnAnnotationFixer;

pub fn fixer() -> Box<dyn Fixer> {
    Box::new(PhpUnitFqcnAnnotationFixer)
}

impl Fixer for PhpUnitFqcnAnnotationFixer {
    fn name(&self) -> &'static str {
        "php_unit_fqcn_annotation"
    }
    fn priority(&self) -> i32 {
        // Must run before NoUnusedImportsFixer, PhpdocOrderByValueFixer.
        -9
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        base::is_candidate(tokens)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        base::apply_fix(tokens, apply_php_unit_class_fix);
    }
}

fn apply_php_unit_class_fix(tokens: &mut Tokens, start_index: usize, end_index: usize) {
    let start_index = tokens
        .get_prev_token_of_kind(start_index, &[T_DOC_COMMENT.into()], true)
        .unwrap_or(start_index);
    for index in start_index..end_index {
        if tokens[index].is_kind(T_DOC_COMMENT) {
            let content = Preg::replace(
                r"~^(\s*\*\s*@(?:expectedException|covers|coversDefaultClass|uses)\h+)(?!(?:self|static)::)(\w.*)$~m",
                r"$1\\$2",
                &tokens[index].content,
            );
            tokens.set(index, Token::new(T_DOC_COMMENT, content));
        }
    }
}
