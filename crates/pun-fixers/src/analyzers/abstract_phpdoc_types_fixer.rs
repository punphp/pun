//! Port of `PhpCsFixer\AbstractPhpdocTypesFixer` (applyFix + fixType); subclasses pass `normalize`.
use crate::analyzers::docblock::{Annotation, DocBlock, TAGS_WITH_TYPES, TypeExpression};
use pun_tokens::{Kind, Token, Tokens};

pub fn is_candidate(tokens: &Tokens) -> bool {
    tokens.is_token_kind_found(Kind::T_DOC_COMMENT)
}

pub fn apply_fix(tokens: &mut Tokens, normalize: &dyn Fn(&[u8]) -> Vec<u8>) {
    for index in 0..tokens.count() {
        if !tokens[index].is_kind(Kind::T_DOC_COMMENT) {
            continue;
        }
        let doc = DocBlock::new(&tokens[index].content);
        let annotations = doc.get_annotations_of_type(TAGS_WITH_TYPES);
        if annotations.is_empty() {
            continue;
        }
        for annotation in annotations {
            fix_type(annotation, normalize);
        }
        tokens.set(index, Token::new(Kind::T_DOC_COMMENT, doc.get_content()));
    }
}

fn fix_type(annotation: &Annotation, normalize: &dyn Fn(&[u8]) -> Vec<u8>) {
    let Some(type_expression) = annotation.get_type_expression() else { return };
    let new = type_expression.map_types(&mut |t| if !t.is_composite_type() { TypeExpression::plain(&normalize(t.to_string())) } else { t });
    annotation.set_types(&[new.to_string().to_vec()]);
}
